#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::cell::RefCell;

use panic_halt as _;

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

use arduino_hal::port::{Pin, mode::{Output}};

#[arduino_hal::entry]
fn main() -> ! {
    // initiate peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let servo_1_pin = pins.d12.into_output().downgrade();
    let servo_1: RefCell<Servo<Pin<Output>>> = RefCell::new(Servo::new(servo_1_pin, 30, 90));
    
    // create servo array
    let servos = [&servo_1];
    let mut servo_caller = ServoCaller::new(&servos);

    // initiate micros
    crate::time::micros_init(dp.TC0);
    // enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    let mut last_4us: u32 = 0;
    let mut last_100ms: u32 = 0;
    let mut last_1000ms: u32 = 0;
    
    loop {
        let current = micros();
        if current - last_4us > 4 {
            servo_caller.call();
            last_4us = current;
        }
        
        let current = micros();
        if current - last_100ms > 100_000 {
            // call sensor caller
            last_100ms = current;
        }
        
        let current = micros();
        if current - last_1000ms > 1_000_000 {
            // call intersection
            last_1000ms = current;
        }
    }
}