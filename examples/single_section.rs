#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use sensor_caller::SensorCaller;

mod section;
mod stopper;
mod sensor;
mod time;
mod sensor_caller;

use crate::section::*;
use crate::stopper::*;
use crate::sensor::*;
use crate::sensor::SensorEnum::*;
use crate::time::micros;

use arduino_hal::port::{Pin, mode::{Output, Input}};
use core::cell::RefCell;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    // stopper 1 setup
    let stopper_1_pin = pins.d13.into_output().downgrade();
    let stopper_1: RefCell<Stopper<Pin<Output>>> = RefCell::new(Stopper::new(stopper_1_pin));
    
    // sensor 1 setup
    let sensor_1_pin = pins.d0.into_pull_up_input().forget_imode().downgrade();
    let sensor_1: RefCell<Sensor<Pin<Output>, Pin<Input>>> = RefCell::new(Sensor::new(sensor_1_pin));
    
    // section 1 setup
    let section_1: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new());
    let sensor_2_pin = pins.d1.into_pull_up_input().forget_imode().downgrade();
    let sensor_2: RefCell<Sensor<Pin<Output>, Pin<Input>>> = RefCell::new(Sensor::new(sensor_2_pin));
    section_1.borrow_mut().set_self_reference(&section_1);
    section_1.borrow_mut().add_sensor(StartSensor(&sensor_1));
    section_1.borrow_mut().add_sensor(EndSensor(&sensor_2));
    section_1.borrow_mut().add_stopper(&stopper_1);
    
    // sensor caller setup
    let sensors = [&sensor_1, &sensor_2];
    let mut sensor_caller = SensorCaller::new(&sensors);
    
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
            // call the servo caller
            last_4us = current;
        }
        
        let current = micros();
        if current - last_100ms > 100_000 {
            // call the sensor caller
            sensor_caller.call();
            last_100ms = current;
        }
        
        let current = micros();
        if current - last_1000ms > 1_000_000 {
            // call the intersection
            last_1000ms = current;
        }
    }
}