#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use sensor_caller::SensorCaller;
use time::millis;

mod section;
mod stopper;
mod sensor;
mod time;
mod sensor_caller;

use crate::section::*;
use crate::stopper::*;
use crate::sensor::*;
use crate::sensor::SensorEnum::*;

use arduino_hal::port::{Pin, mode::{Output, Input}};
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
    
    // setup serial
    let adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let serial = RefCell::new(arduino_hal::default_serial!(dp, pins, 57600));
    ufmt::uwriteln!(&mut serial.borrow_mut(), "serial start").unwrap();


    // setup integrated led
    let mut led = pins.d13.into_output();
    led.set_low();
    // stopper: 41, 43, 45, 47, 49, 51, 53

    // stopper 1 setup
    let stopper_1_pin = pins.d41.into_output().downgrade();
    let stopper_1: RefCell<Stopper<Pin<Output>>> = RefCell::new(Stopper::new(stopper_1_pin));
    
    // stopper 2 setup
    let stopper_2_pin = pins.d43.into_output().downgrade();
    let stopper_2: RefCell<Stopper<Pin<Output>>> = RefCell::new(Stopper::new(stopper_2_pin));
    
    // stopper 3 setup
    let stopper_3_pin = pins.d45.into_output().downgrade();
    let stopper_3: RefCell<Stopper<Pin<Output>>> = RefCell::new(Stopper::new(stopper_3_pin));
    
    // stopper 4 setup
    let stopper_4_pin = pins.d47.into_output().downgrade();
    let stopper_4: RefCell<Stopper<Pin<Output>>> = RefCell::new(Stopper::new(stopper_4_pin));
    
    // stopper 5 setup
    let stopper_5_pin = pins.d49.into_output().downgrade();
    let stopper_5: RefCell<Stopper<Pin<Output>>> = RefCell::new(Stopper::new(stopper_5_pin));
    
    // stopper 6 setup
    let stopper_6_pin = pins.d51.into_output().downgrade();
    let stopper_6: RefCell<Stopper<Pin<Output>>> = RefCell::new(Stopper::new(stopper_6_pin));
    
    // stopper 7 setup
    let stopper_7_pin = pins.d53.into_output().downgrade();
    let stopper_7: RefCell<Stopper<Pin<Output>>> = RefCell::new(Stopper::new(stopper_7_pin));

    // sensor: a7, a6, a5, a4, a3, a2, a1
    // sensor 1 setup
    let sensor_1_pin = pins.a7.into_pull_up_input().forget_imode().downgrade();
    let sensor_1: RefCell<Sensor<Pin<Output>, Pin<Input>>> = RefCell::new(Sensor::new(sensor_1_pin, 1));
    
    // sensor 2 setup
    let sensor_2_pin = pins.a6.into_pull_up_input().forget_imode().downgrade();
    let sensor_2: RefCell<Sensor<Pin<Output>, Pin<Input>>> = RefCell::new(Sensor::new(sensor_2_pin, 2));
    
    // sensor 3 setup
    let sensor_3_pin = pins.a5.into_pull_up_input().forget_imode().downgrade();
    let sensor_3: RefCell<Sensor<Pin<Output>, Pin<Input>>> = RefCell::new(Sensor::new(sensor_3_pin, 3));
    
    // sensor 4 setup
    let sensor_4_pin = pins.a4.into_pull_up_input().forget_imode().downgrade();
    let sensor_4: RefCell<Sensor<Pin<Output>, Pin<Input>>> = RefCell::new(Sensor::new(sensor_4_pin, 4));
    
    // sensor 5 setup
    let sensor_5_pin = pins.a3.into_pull_up_input().forget_imode().downgrade();
    let sensor_5: RefCell<Sensor<Pin<Output>, Pin<Input>>> = RefCell::new(Sensor::new(sensor_5_pin, 5));
    
    // sensor 6 setup
    let sensor_6_pin = pins.a2.into_pull_up_input().forget_imode().downgrade();
    let sensor_6: RefCell<Sensor<Pin<Output>, Pin<Input>>> = RefCell::new(Sensor::new(sensor_6_pin, 6));
    
    // sensor 7 setup
    let sensor_7_pin = pins.a1.into_pull_up_input().forget_imode().downgrade();
    let sensor_7: RefCell<Sensor<Pin<Output>, Pin<Input>>> = RefCell::new(Sensor::new(sensor_7_pin, 7));
    
    // section 1 setup
    let section_1: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new(1, &serial));
    section_1.borrow_mut().set_self_reference(&section_1);
    section_1.borrow_mut().add_stopper(&stopper_1);
    section_1.borrow_mut().add_sensor(StartSensor(&sensor_3));
    section_1.borrow_mut().add_sensor(StartSensor(&sensor_2));
    section_1.borrow_mut().add_sensor(EndSensor(&sensor_5));
    section_1.borrow_mut().add_sensor(EndSensor(&sensor_6));
    
    // section 2 setup
    let section_2: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new(2, &serial));
    section_2.borrow_mut().set_self_reference(&section_2);
    section_2.borrow_mut().add_stopper(&stopper_2);
    section_2.borrow_mut().add_sensor(StartSensor(&sensor_3));
    section_2.borrow_mut().add_sensor(StartSensor(&sensor_1));
    section_2.borrow_mut().add_sensor(EndSensor(&sensor_5));
    section_2.borrow_mut().add_sensor(EndSensor(&sensor_4));
    
    // section 3 setup
    let section_3: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new(3, &serial));
    section_3.borrow_mut().set_self_reference(&section_3);
    section_3.borrow_mut().add_stopper(&stopper_3);
    section_3.borrow_mut().add_sensor(StartSensor(&sensor_1));
    section_3.borrow_mut().add_sensor(StartSensor(&sensor_2));
    section_3.borrow_mut().add_sensor(EndSensor(&sensor_4));
    section_3.borrow_mut().add_sensor(EndSensor(&sensor_6));
    
    // section 4 setup
    let section_4: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new(4, &serial));
    section_4.borrow_mut().set_self_reference(&section_4);
    section_4.borrow_mut().add_stopper(&stopper_4);
    section_4.borrow_mut().add_sensor(StartSensor(&sensor_4));
    section_4.borrow_mut().add_sensor(EndSensor(&sensor_1));
    section_4.borrow_mut().add_sensor(EndSensor(&sensor_2));
    
    // section 5 setup
    let section_5: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new(5, &serial));
    section_5.borrow_mut().set_self_reference(&section_5);
    section_5.borrow_mut().add_stopper(&stopper_5);
    section_5.borrow_mut().add_sensor(StartSensor(&sensor_5));
    section_5.borrow_mut().add_sensor(EndSensor(&sensor_3));
    section_5.borrow_mut().add_sensor(EndSensor(&sensor_2));
    
    // section 6 setup
    let section_6: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new(6, &serial));
    section_6.borrow_mut().set_self_reference(&section_6);
    section_6.borrow_mut().add_stopper(&stopper_6);
    section_6.borrow_mut().add_sensor(StartSensor(&sensor_6));
    section_6.borrow_mut().add_sensor(EndSensor(&sensor_7));
    
    // section 7 setup
    let section_7: RefCell<Section<Pin<Output>, Pin<Input>>> = RefCell::new(Section::new(7, &serial));
    section_7.borrow_mut().set_self_reference(&section_7);
    section_7.borrow_mut().add_stopper(&stopper_7);
    section_7.borrow_mut().add_sensor(StartSensor(&sensor_7));
    section_7.borrow_mut().add_sensor(EndSensor(&sensor_3));
    section_7.borrow_mut().add_sensor(EndSensor(&sensor_1));
    
    // sensor caller setup
    let sensors = [&sensor_1, &sensor_2, &sensor_3, &sensor_4, &sensor_5, &sensor_6, &sensor_7];
    let sensor_caller = SensorCaller::new(&sensors);
    
    // initiate micros
    crate::time::millis_init(dp.TC0);
    // enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    let mut last_1000ms: u64 = 0;
    let mut last_5ms: u64 = 0;

    loop {
        // call the sensor caller
        let current = millis();
        if last_5ms + 5 < current {
            sensor_caller.call(Some(&serial));
            last_5ms = current;
        }

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