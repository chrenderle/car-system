#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use sensor_caller::SensorCaller;
use time::millis;

mod intersection;
mod lights;
mod section;
mod sensor;
mod sensor_caller;
mod servo;
mod stopper;
mod time;

use crate::section::*;
use crate::sensor::SensorEnum::*;
use crate::sensor::*;
use crate::stopper::*;

use arduino_hal::port::{
    mode::{Input, Output},
    Pin,
};
use core::cell::RefCell;

#[arduino_hal::entry]
fn main() -> ! {
    // setup of peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // setup i2c
    let mut i2c = RefCell::new(arduino_hal::I2c::new(
        dp.TWI,
        pins.d20.into_pull_up_input(),
        pins.d21.into_pull_up_input(),
        50000,
    ));

    // setup integrated led
    let mut led = pins.d13.into_output();
    led.set_low();

    // setup serial
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // initiate micros
    crate::time::millis_init(dp.TC0);
    // enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    let mut last_1000ms: u64 = 0;

    loop {
        // call the sensor caller

        let current = millis();
        if last_1000ms + 1_000 < current {
            led.toggle();
            // call the intersection
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

    #[cfg(debug_assertions)]
    {
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
            )
            .unwrap();
        }
    }

    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}