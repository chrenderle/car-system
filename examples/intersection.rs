#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::cell::RefCell;

use lights::Light;
use panic_halt as _;
use sensor_caller::SensorCaller;

mod section;
mod stopper;
mod sensor;
mod servo;
mod intersection;
mod lights;
mod time;
mod servo_caller;
mod sensor_caller;

use crate::section::*;
use crate::servo_caller::ServoCaller;
use crate::stopper::*;
use crate::sensor::*;
use crate::sensor::SensorEnum::*;
use crate::servo::*;
use crate::intersection::*;
use crate::time::micros as micros;

use arduino_hal::port::{Pin, mode::{Output, Input}};

#[arduino_hal::entry]
fn main() -> ! {
    // initiate peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // left stopper setup
    let left_stopper_pin = pins.a3.into_output().downgrade();
    let left_stopper = RefCell::new(Stopper::new(left_stopper_pin));
    
    // right stopper setup
    let right_stopper_pin = pins.a4.into_output().downgrade();
    let right_stopper = RefCell::new(Stopper::new(right_stopper_pin));

    // upper stopper setup
    let upper_stopper_pin = pins.a5.into_output().downgrade();
    let upper_stopper = RefCell::new(Stopper::new(upper_stopper_pin));
    

    // left servo setup
    let left_servo_pin = pins.a6.into_output().downgrade();
    let left_servo: RefCell<Servo<Pin<Output>>> = RefCell::new(Servo::new(left_servo_pin, 30, 90));

    // right servo setup
    let right_servo_pin = pins.a7.into_output().downgrade();
    let right_servo: RefCell<Servo<Pin<Output>>> = RefCell::new(Servo::new(right_servo_pin, 30, 90));

    // upper servo setup
    let upper_servo_pin = pins.a8.into_output().downgrade();
    let upper_servo: RefCell<Servo<Pin<Output>>> = RefCell::new(Servo::new(upper_servo_pin, 30, 90));
    

    // left light setup
    let left_light_green_pin = pins.a9.into_output().downgrade();
    let left_light_yellow_pin = pins.a10.into_output().downgrade();
    let left_light_red_pin = pins.a11.into_output().downgrade();
    let left_light = Light::new(left_light_green_pin, left_light_yellow_pin, left_light_red_pin);

    // right light setup
    let right_light_green_pin = pins.a12.into_output().downgrade();
    let right_light_yellow_pin = pins.a13.into_output().downgrade();
    let right_light_red_pin = pins.a14.into_output().downgrade();
    let right_light = Light::new(right_light_green_pin, right_light_yellow_pin, right_light_red_pin);

    // upper light setup
    let right_light_green_pin = pins.a15.into_output().downgrade();
    let right_light_yellow_pin = pins.d0.into_output().downgrade();
    let upper_light_red_pin = pins.d1.into_output().downgrade();
    let upper_light = Light::new(right_light_green_pin, right_light_yellow_pin, upper_light_red_pin);
    
    
    // intersection arm left setup
    let left_intersection_arm = IntersectionArm {
        servo: &left_servo,
        entry_stopper: &left_stopper,
        light: left_light,
    };

    // intersection arm right setup
    let right_intersection_arm = IntersectionArm {
        servo: &right_servo,
        entry_stopper: &right_stopper,
        light: right_light,
    };

    // intersection arm upper setup
    let upper_intersection_arm = IntersectionArm {
        servo: &upper_servo,
        entry_stopper: &upper_stopper,
        light: upper_light,
    };
    

    // intersection states setup
    let intersection_states = DefaultIntersectionStates::new();
    

    // intersection setup
    let mut intersection = Intersection::new(left_intersection_arm, right_intersection_arm, upper_intersection_arm, intersection_states);
    

    // sensor caller setup
    let servos = [&left_servo, &right_servo, &upper_servo];
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
            // call the servo caller
            servo_caller.call();
            last_4us = current;
        }
        
        let current = micros();
        if current - last_100ms > 100_000 {
            // call the sensor caller
            last_100ms = current;
        }
        
        let current = micros();
        if current - last_1000ms > 1_000_000 {
            // call the intersection
            intersection.call();
            last_1000ms = current;
        }
    }
}