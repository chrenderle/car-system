use core::cell::RefCell;
use arduino_hal::{port::{mode::{Output, Input}, Pin}, hal::{Usart, Atmega, port::{PE0, PE1}}, pac::USART0};

use crate::{servo::Servo, time::micros};

/// Struct which calls multiple servos but has to be called itself regularily
pub struct ServoCaller<'l> {
    /// Array slice of the servos it should call regularily
    servos: &'l [&'l RefCell<Servo<Pin<Output>>>],
    us_counter: u32,
    last_us: u32,
}

impl<'l> ServoCaller<'l> {
    /// Returns a new servo caller with the given servos
    /// 
    /// # Arguments
    /// 
    /// * `servos` - Array slice of the servos to call
    pub fn new(servos: &'l [&'l RefCell<Servo<Pin<Output>>>]) -> ServoCaller<'l> {
        ServoCaller {
            servos,
            us_counter: 0,
            last_us: 0,
        }
    }

    /// Calls the servos according to the pulse length they specify
    pub fn call(&mut self) {
        // the time the loop is called
        let current_loop = micros() / 1000;

        // increment us_counter by the time passed since the last call
        self.us_counter += current_loop - self.last_us;

        // set the last_us time to the current call time
        self.last_us = current_loop;


        for servo in self.servos {
            // if 20ms passed run new loop/pulse
            if self.us_counter >= 20_000 {
                self.us_counter = 0;
                servo.borrow_mut().start_pulse();
            }

            // if the pulse passed end the pulse (must be between 1000us and 2000us)
            else if self.us_counter >= servo.borrow().get_pulse_length() as u32 && self.us_counter < 20_000 {
                servo.borrow_mut().pulse_end();
            } 
        }
    }
}
