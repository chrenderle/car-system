#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::cell::RefCell;

mod section;
mod stopper;
mod sensor;
mod servo;
mod intersection;
mod lights;
mod time;
mod servo_caller;

use crate::servo_caller::ServoCaller;
use crate::servo::*;
use crate::time::micros as micros;
use ufmt::derive::uDebug;

use arduino_hal::port::{Pin, mode::{Output}};

#[arduino_hal::entry]
fn main() -> ! {
    // initiate peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let servo_1_pin = pins.d13.into_output().downgrade();
    let servo_1: RefCell<Servo<Pin<Output>>> = RefCell::new(Servo::new(servo_1_pin, 80, 90));
    
    // create servo array
    let servos = [&servo_1];
    let mut servo_caller = ServoCaller::new(&servos);
    let mut servo_pin = pins.d2.into_output();
    loop {
        servo_pin.set_low();
        arduino_hal::delay_us(1_000);
        servo_pin.set_high();
        arduino_hal::delay_us(19_000);
    }
//    let mut led = pins.d13.into_output();

    // initiate micros
    crate::time::micros_init(dp.TC0);
    // enable interrupts globally
    unsafe { avr_device::interrupt::enable() };
    
    let mut last_4us: u32 = micros();
    let mut last_100ms: u32 = micros();
    let mut last_1000ms: u32 = micros();
    
    loop {
        let current = micros();
        if last_4us + 4 < current {
            servo_caller.call();
            last_4us = current;
        }
        
        let current = micros();
        if last_100ms + 100_000 < current {
            // call sensor caller
            last_100ms = current;
        }
        
        let current = micros();
        if last_1000ms + 1_000_000 < current {
            // call intersection
            last_1000ms = current;
        }
    }
}

#[allow(unused_variables)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // disable interrupts
    avr_device::interrupt::disable();
    
    // get the peripherals so we can access serial and the LED
    // UNSAFE: because main already has references to the peripherals this is an unsafe operation
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    // print out panic location
    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").unwrap();
    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        ).unwrap();
    }
    
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}