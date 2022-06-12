use crate::{section::*, time::millis};
use core::cell::RefCell;
use core::default::Default;
use core::option::Option;
use core::option::Option::*;
use core::panic;
use arduino_hal::{hal::{Usart, port::{PE0, PE1}}, pac::USART0, port::{Pin, mode::{Input, Output}}, clock::MHz16};
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub const SENSOR_ACTIVE: bool = false;

pub enum SensorEnum<'l, W: 'l, R: 'l>
where
    W: OutputPin,
    R: InputPin,
{
    StartSensor(&'l RefCell<Sensor<'l, W, R>>),
    EndSensor(&'l RefCell<Sensor<'l, W, R>>),
}

pub struct Sensor<'l, W, R>
where
    W: OutputPin,
    R: InputPin,
{
    id: u8,
    pin: R,
    last_state: bool,
    last_time: u64,
    start_section_owners: [Option<&'l RefCell<Section<'l, W, R>>>; 2],
    end_section_owners: [Option<&'l RefCell<Section<'l, W, R>>>; 2],
}

impl<'l, W, R> Sensor<'l, W, R>
where
    W: OutputPin,
    R: InputPin,
{
    pub fn get_state(&self) -> bool {
        match self.pin.is_high() {
            Ok(state) => state,
            Err(_) => panic!("read failed"),
        }
    }

    pub fn add_start_owner(&mut self, section: &'l RefCell<Section<'l, W, R>>) {
        for option in &mut self.start_section_owners {
            if option.is_none() {
                *option = Some(section);
                return;
            }
        }
        panic!("no more than two start owners possible");
    }

    pub fn add_end_owner(&mut self, section: &'l RefCell<Section<'l, W, R>>) {
        for option in &mut self.end_section_owners {
            if option.is_none() {
                *option = Some(section);
                return;
            }
        }
        panic!("no more than two end owners possible");
    }

    pub fn check_pin_change(&mut self, serial: Option<&RefCell<Usart<USART0, Pin<Input, PE0>, Pin<Output, PE1>, MHz16>>>) {
        let state = self.get_state();
        let time = millis();
        #[cfg(debug_assertions)]
        {
            if let Some(serial) = serial {
                ufmt::uwriteln!(&mut serial.borrow_mut(), "state: {}, last_state: {}, time: {}, last_time: {}", state, self.last_state, time, self.last_time).unwrap();
            }
        }
        if state == SENSOR_ACTIVE && time - self.last_time >= 1_000 {//&& state != self.last_state {
            #[cfg(debug_assertions)]
            {
                if let Some(serial) = serial {
                    ufmt::uwriteln!(&mut serial.borrow_mut(), "sensor {} detected; time: {}; last_time: {}; state: {}, last_state: {}", self.id, time, self.last_time, state, self.last_state).unwrap();
                }
            }
            for start_section_owner in self.start_section_owners {
                match start_section_owner {
                    Some(start_section) => start_section.borrow_mut().start_sensor_callback(),
                    None => break,
                }
            }

            for end_section_owner in self.end_section_owners {
                match end_section_owner {
                    Some(end_section) => end_section.borrow_mut().end_sensor_callback(),
                    None => break,
                }
            }
            self.last_time = time;
        }
        self.last_state = state;
    }

    pub fn new(pin: R, id: u8) -> Sensor<'l, W, R> {
        Sensor {
            id,
            pin,
            last_state: !SENSOR_ACTIVE,
            last_time: 0,
            start_section_owners: Default::default(),
            end_section_owners: Default::default(),
        }
    }
}
