use core::cell::RefCell;
use core::default::Default;
use core::option::Option;
use core::option::Option::*;
use core::panic;
use arduino_hal::hal::Usart;
use arduino_hal::clock::MHz16;
use arduino_hal::hal::port::PE0;
use arduino_hal::hal::port::PE1;
use arduino_hal::pac::USART0;
use arduino_hal::port::Pin;
use arduino_hal::port::mode::Input;
use arduino_hal::port::mode::Output;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;

use crate::sensor::SensorEnum::*;
use crate::sensor::*;
use crate::stopper::*;

pub struct Section<'l, W: 'l, R: 'l>
where
    R: InputPin,
    W: OutputPin,
{
    serial: &'l RefCell<Usart<USART0, Pin<Input, PE0>, Pin<Output, PE1>, MHz16>>,
    id: u8,
    locks: i8,
    start_sensors: [Option<&'l RefCell<Sensor<'l, W, R>>>; 2],
    end_sensors: [Option<&'l RefCell<Sensor<'l, W, R>>>; 2],
    stoppers: [Option<&'l RefCell<Stopper<W>>>; 2],
    self_reference: Option<&'l RefCell<Section<'l, W, R>>>,
}

impl<'l, W, R> Section<'l, W, R>
where
    W: OutputPin,
    R: InputPin,
{
    pub fn new(id: u8, serial: &'l RefCell<Usart<USART0, Pin<Input, PE0>, Pin<Output, PE1>, MHz16>>) -> Self {
        Section {
            serial,
            id,
            locks: 0,
            start_sensors: Default::default(),
            end_sensors: Default::default(),
            stoppers: Default::default(),
            self_reference: None,
        }
    }

    pub fn set_self_reference(&mut self, self_reference: &'l RefCell<Section<'l, W, R>>) {
        self.self_reference = Some(self_reference);
    }

    pub fn start_sensor_callback(&mut self) {
        ufmt::uwriteln!(&mut self.serial.borrow_mut(), "section {} start sensor detected", self.id).unwrap();
        self.locks += 1;
        for stopper in &self.stoppers {
            match stopper {
                Some(stopper) => stopper.borrow_mut().lock(),
                None => return,
            }
        }
    }

    pub fn end_sensor_callback(&mut self) {
        ufmt::uwriteln!(&mut self.serial.borrow_mut(), "section {} end sensor detected", self.id).unwrap();
        self.locks -= 1;
        for stopper in &self.stoppers {
            match stopper {
                Some(stopper) => stopper.borrow_mut().release(),
                None => return,
            }
        }
    }

    pub fn add_sensor(&mut self, sensor: SensorEnum<'l, W, R>) {
        if let Some(self_reference) = self.self_reference {
            match sensor {
                StartSensor(sensor) => {
                    sensor.borrow_mut().add_start_owner(self_reference);
                    for sensor_loop in &mut self.start_sensors {
                        if sensor_loop.is_none() {
                            *sensor_loop = Some(sensor);
                            return;
                        }
                    }
                }
                EndSensor(sensor) => {
                    sensor.borrow_mut().add_end_owner(self_reference);
                    for sensor_loop in &mut self.end_sensors {
                        if sensor_loop.is_none() {
                            *sensor_loop = Some(sensor);
                            return;
                        }
                    }
                }
            }
            panic!("no more than 2 start or end sensors allowed");
        } else {
            panic!("no self reference set");
        }
    }

    pub fn add_stopper(&mut self, stopper: &'l RefCell<Stopper<W>>) {
        for stopper_loop in &mut self.stoppers {
            if stopper_loop.is_none() {
                *stopper_loop = Some(stopper);
                return;
            }
        }
        panic!("no more than two stoppers allowed");
    }
}
