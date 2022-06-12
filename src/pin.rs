//use core::prelude::rust_2024::derive;
use core::todo;
//use arduino_hal::port::Pin as APin;
//use arduino_hal::port::mode::{Output, Input};
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::InputPin;


/*pub struct ServoPin<'l, W> where W: OutputPin {
    pin: &'l W,
}

pub trait ServoPinT<'l, W> where W: OutputPin {
    fn new(pin: &'l W) -> Self;
    fn write_angle(&mut self, angle: u16);
}

impl<'l, W> ServoPinT<'l, W> for ServoPin<'l, W> {
    fn new(pin: &'l W) -> ServoPin<'l, W> {
        // TODO setup servo pin
        todo!();
        //Pin { pin }
    }
    
    fn write_angle(&mut self, angle: u16) {
        todo!();
        // TODO implement
    }
}

pub struct ReadPin<'l, R> where R: InputPin {
    pin: &'l R,
}

pub trait ReadPinT<'l, R> where R: InputPin {
    fn new(pin: &'l R) -> Self;
    fn read(&self) -> bool;
}

impl<'l, R> ReadPinT<'l, R> for ReadPin<'l, R> where R: InputPin {
    fn new(pin: &'l R) -> ReadPin<'l, R> {
        ReadPin { pin }
    }

    fn read(&self) -> bool {
        match 
        self.pin.is_high()
    }
}

pub struct WritePin<'l, W> where W: OutputPin {
    pin: &'l mut W,
}

pub trait WritePinT<'l, W> where W: OutputPin {
    fn new(pin: &'l mut W) -> Self;
    fn write(&mut self, state: bool);
}

impl<'l, W> WritePinT<'l, W> for WritePin<'l, W> where W: OutputPin {
    fn new(pin: &'l mut W) -> WritePin<'l, W> {
        WritePin { pin }
    }

    fn write(&mut self, state: bool) {
        match state {
            true => self.pin.set_high(),
            false => self.pin.set_low(),
        }
    }
}*/