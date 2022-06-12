#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

// the imports from dependencies
use core::cell::RefCell;
use arduino_hal::port::{
    mode::{Input, Output},
    Pin,
};
// only need the prelude when building with profile debug
#[cfg(debug_assertions)]
use arduino_hal::prelude::*;


/// Module which contains the struct intersection which represents a intersection with traffic lights and servos and stoppers to control the cars depending on the traffic light phase
pub mod intersection;
/// Module which contains the struct light which represents a traffic light
pub mod lights;
/// Module which contains the struct section which controls multiple stoppers depending on multiple sensors to keep the distance between cars
pub mod section;
/// Module which contains the struct sensor to read a sensor which can callback multiple sections it belongs to
pub mod sensor;
/// Module which contains the struct senso_caller to call multiple sensors from a loop
pub mod sensor_caller;
/// Module which contains the struct servo to control servos
pub mod servo;
/// Module which contains the struct servo_caller to call multiple servos from a loop
pub mod servo_caller;
/// Module which contains the struct stopper to control a stopper in a car system which stops cars
pub mod stopper;
/// Module which contains the functions for millis
pub mod time;

// the imports from crate
use crate::lights::Light;
use crate::sensor_caller::SensorCaller;
use crate::intersection::*;
use crate::section::*;
use crate::sensor::SensorEnum::*;
use crate::sensor::*;
use crate::servo::*;
use crate::servo_caller::ServoCaller;
use crate::stopper::*;
use crate::time::micros;

#[arduino_hal::entry]
fn main() -> ! {
    // initiate peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);


    // stopper 1 setup
    let stopper_1_pin = pins.a0.into_output().downgrade();
    let stopper_1 = RefCell::new(Stopper::new(stopper_1_pin));

    // stopper 2 setup
    let stopper_2_pin = pins.a1.into_output().downgrade();
    let stopper_2 = RefCell::new(Stopper::new(stopper_2_pin));

    // stopper 3 setup
    let stopper_3_pin = pins.a2.into_output().downgrade();
    let stopper_3 = RefCell::new(Stopper::new(stopper_3_pin));

    // stopper 4 setup
    let stopper_4_pin = pins.a3.into_output().downgrade();
    let stopper_4 = RefCell::new(Stopper::new(stopper_4_pin));

    // stopper 5 setup
    let stopper_5_pin = pins.a4.into_output().downgrade();
    let stopper_5 = RefCell::new(Stopper::new(stopper_5_pin));

    // stopper 6 setup
    let stopper_6_pin = pins.a5.into_output().downgrade();
    let stopper_6 = RefCell::new(Stopper::new(stopper_6_pin));

    // stopper 7 setup
    let stopper_7_pin = pins.a6.into_output().downgrade();
    let stopper_7 = RefCell::new(Stopper::new(stopper_7_pin));


    // left servo setup (servo 1)
    let left_servo_pin = pins.a7.into_output().downgrade();
    let left_servo: RefCell<Servo<Pin<Output>>> = RefCell::new(Servo::new(left_servo_pin, 30, 90));

    // right servo setup (servo 1)
    let right_servo_pin = pins.a8.into_output().downgrade();
    let right_servo: RefCell<Servo<Pin<Output>>> =
        RefCell::new(Servo::new(right_servo_pin, 30, 90));

    // upper servo setup (servo 3)
    let upper_servo_pin = pins.a9.into_output().downgrade();
    let upper_servo: RefCell<Servo<Pin<Output>>> =
        RefCell::new(Servo::new(upper_servo_pin, 30, 90));


    // left light setup
    let left_light_green_pin = pins.d0.into_output().downgrade();
    let left_light_yellow_pin = pins.a10.into_output().downgrade();
    let left_light_red_pin = pins.a11.into_output().downgrade();
    let left_light = Light::new(
        left_light_green_pin,
        left_light_yellow_pin,
        left_light_red_pin,
    );

    // right light setup
    let right_light_green_pin = pins.a12.into_output().downgrade();
    let right_light_yellow_pin = pins.a13.into_output().downgrade();
    let right_light_red_pin = pins.a14.into_output().downgrade();
    let right_light = Light::new(
        right_light_green_pin,
        right_light_yellow_pin,
        right_light_red_pin,
    );

    // upper light setup
    let right_light_green_pin = pins.a15.into_output().downgrade();
    let right_light_yellow_pin = pins.d2.into_output().downgrade();
    let upper_light_red_pin = pins.d1.into_output().downgrade();
    let upper_light = Light::new(
        right_light_green_pin,
        right_light_yellow_pin,
        upper_light_red_pin,
    );


    // intersection arm left setup
    let left_intersection_arm = IntersectionArm {
        servo: &left_servo,
        entry_stopper: &stopper_1,
        light: left_light,
    };

    // intersection arm right setup
    let right_intersection_arm = IntersectionArm {
        servo: &right_servo,
        entry_stopper: &stopper_2,
        light: right_light,
    };

    // intersection arm upper setup
    let upper_intersection_arm = IntersectionArm {
        servo: &upper_servo,
        entry_stopper: &stopper_3,
        light: upper_light,
    };

    // intersection states setup
    let intersection_states = DefaultIntersectionStates::new();

    // intersection setup
    let mut intersection = Intersection::new(
        left_intersection_arm,
        right_intersection_arm,
        upper_intersection_arm,
        intersection_states,
    );


    // sensor 1 setup
    let sensor_1_pin = pins.d14.into_pull_up_input().forget_imode().downgrade();
    let sensor_1: RefCell<Sensor<Pin<Output>, Pin<Input>>> =
        RefCell::new(Sensor::new(sensor_1_pin));

    // sensor 2 setup
    let sensor_2_pin = pins.d15.into_pull_up_input().forget_imode().downgrade();
    let sensor_2: RefCell<Sensor<Pin<Output>, Pin<Input>>> =
        RefCell::new(Sensor::new(sensor_2_pin));

    // sensor 3 setup
    let sensor_3_pin = pins.d16.into_pull_up_input().forget_imode().downgrade();
    let sensor_3: RefCell<Sensor<Pin<Output>, Pin<Input>>> =
        RefCell::new(Sensor::new(sensor_3_pin));

    // sensor 4 setup
    let sensor_4_pin = pins.d17.into_pull_up_input().forget_imode().downgrade();
    let sensor_4: RefCell<Sensor<Pin<Output>, Pin<Input>>> =
        RefCell::new(Sensor::new(sensor_4_pin));

    // sensor 5 setup
    let sensor_5_pin = pins.d18.into_pull_up_input().forget_imode().downgrade();
    let sensor_5: RefCell<Sensor<Pin<Output>, Pin<Input>>> =
        RefCell::new(Sensor::new(sensor_5_pin));

    // sensor 6 setup
    let sensor_6_pin = pins.d19.into_pull_up_input().forget_imode().downgrade();
    let sensor_6: RefCell<Sensor<Pin<Output>, Pin<Input>>> =
        RefCell::new(Sensor::new(sensor_6_pin));

    // sensor 7 setup
    let sensor_7_pin = pins.d20.into_pull_up_input().forget_imode().downgrade();
    let sensor_7: RefCell<Sensor<Pin<Output>, Pin<Input>>> =
        RefCell::new(Sensor::new(sensor_7_pin));


    // section 1 setup
    let section_1: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new());
    section_1.borrow_mut().set_self_reference(&section_1);
    section_1.borrow_mut().add_sensor(StartSensor(&sensor_6));
    section_1.borrow_mut().add_stopper(&stopper_1);
    section_1.borrow_mut().add_sensor(EndSensor(&sensor_3));
    section_1.borrow_mut().add_sensor(EndSensor(&sensor_2));

    // section 2 setup
    let section_2: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new());
    section_2.borrow_mut().set_self_reference(&section_2);
    section_2.borrow_mut().add_sensor(StartSensor(&sensor_3));
    section_2.borrow_mut().add_stopper(&stopper_6);
    section_2.borrow_mut().add_sensor(EndSensor(&sensor_6));

    // section 3 setup
    let section_3: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new());
    section_3.borrow_mut().set_self_reference(&section_3);
    section_3.borrow_mut().add_sensor(StartSensor(&sensor_4));
    section_3.borrow_mut().add_stopper(&stopper_2);
    section_3.borrow_mut().add_sensor(EndSensor(&sensor_3));
    section_3.borrow_mut().add_sensor(EndSensor(&sensor_1));

    // section 4 setup
    let section_4: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new());
    section_4.borrow_mut().set_self_reference(&section_4);
    section_4.borrow_mut().add_sensor(StartSensor(&sensor_5));
    section_4.borrow_mut().add_stopper(&stopper_4);
    section_4.borrow_mut().add_sensor(EndSensor(&sensor_4));

    // section 5 setup
    let section_5: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new());
    section_5.borrow_mut().set_self_reference(&section_5);
    section_5.borrow_mut().add_sensor(StartSensor(&sensor_2));
    section_5.borrow_mut().add_stopper(&stopper_5);
    section_5.borrow_mut().add_sensor(EndSensor(&sensor_5));

    // section 6 setup
    let section_6: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new());
    section_6.borrow_mut().set_self_reference(&section_6);
    section_6.borrow_mut().add_sensor(StartSensor(&sensor_1));
    section_6.borrow_mut().add_stopper(&stopper_7);
    section_6.borrow_mut().add_sensor(EndSensor(&sensor_7));

    // section 7 setup
    let section_7: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new());
    section_7.borrow_mut().set_self_reference(&section_7);
    section_7.borrow_mut().add_sensor(StartSensor(&sensor_7));
    section_7.borrow_mut().add_stopper(&stopper_3);
    section_7.borrow_mut().add_sensor(EndSensor(&sensor_1));
    section_7.borrow_mut().add_sensor(EndSensor(&sensor_2));


    // sensor caller setup
    let sensors = [
        &sensor_1, &sensor_2, &sensor_3, &sensor_4, &sensor_5, &sensor_6, &sensor_7,
    ];
    let sensor_caller = SensorCaller::new(&sensors);
    

    // servo caller setup
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
        // 4us loop
        let current = micros();
        if current - last_4us > 4 {
            // call the servo caller
            servo_caller.call();
            last_4us = current;
        }

        // 100ms loop
        let current = micros();
        if current - last_100ms > 100_000 {
            // call the sensor caller
            sensor_caller.call();
            last_100ms = current;
        }

        // 1000ms loop
        let current = micros();
        if current - last_1000ms > 1_000_000 {
            // call the intersection
            intersection.call();
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
        let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
        // print out panic location
        ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();
        if let Some(loc) = info.location() {
            ufmt::uwriteln!(
                &mut serial,
                "  At {}:{}:{}\r",
                loc.file(),
                loc.line(),
                loc.column(),
            ).void_unwrap();
        }
    }
    
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}