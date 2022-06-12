// dependency imports
use arduino_hal::{port::{
    mode::{Input, Output},
    Pin,
}, hal::{Atmega, port::{PE0, PE1}, Usart}, pac::USART0, clock::MHz16};
use core::cell::RefCell;

// crate imports
use crate::{sensor::Sensor, time::millis};

/// Struct which calls multiple sensors but has to be called itself regularily
pub struct SensorCaller<'l> {
    /// array slice of the sensors it should call
    sensors: &'l [&'l RefCell<Sensor<'l, Pin<Output>, Pin<Input>>>],
    last_time: u64,
}

impl<'l> SensorCaller<'l> {
    /// Returns a new SensorCaller with the given sensors
    ///
    /// # Arguments
    ///
    /// * `sensors` - array slice of the sensors it should call
    pub fn new(
        sensors: &'l [&'l RefCell<Sensor<'l, Pin<Output>, Pin<Input>>>],
    ) -> SensorCaller<'l> {
        SensorCaller { sensors, last_time: 0 }
    }

    /// Calls all the sensors
    pub fn call(&self, serial: Option<&RefCell<Usart<USART0, Pin<Input, PE0>, Pin<Output, PE1>, MHz16>>>) {
        let time = millis();
        let debug = time - self.last_time >= 100;
        let debug = false;
        for sensor in self.sensors {
            sensor.borrow_mut().check_pin_change(serial);
        }
    }
}
