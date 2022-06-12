use core::cell::RefCell;

use embedded_hal::{blocking::i2c::SevenBitAddress, digital::v2::OutputPin};

use crate::intersection::IntersectionActionDirection;

use embedded_hal::blocking::i2c;

/// Struct representing a servo which can be set to two angles
pub struct Servo<'l, I2C, S>
where
    S: OutputPin,
    I2C: i2c::Write<SevenBitAddress>,
{
    /// The output pin to which the servo is connected to
    pin: S,
    /// The angle to set the servo to when the servo should be set to right
    right_angle: u8,
    /// The angle to set the servo to when the servo should be set to left
    left_angle: u8,

    i2c: &'l RefCell<I2C>,

    id: u8,

    address: u8,
}

impl<'l, I2C, S> Servo<'l, I2C, S>
where
    I2C: i2c::Write<SevenBitAddress>,
    S: OutputPin,
{
    /// Sets the direction for the next pulse
    ///
    /// # Arguments
    ///
    /// * `direction` - The direction to set the servo in the next pulse to
    pub fn set_direction(&mut self, direction: &IntersectionActionDirection) {
        let angle = match direction {
            IntersectionActionDirection::Right => self.right_angle,
            IntersectionActionDirection::Left => self.left_angle,
        };
        let id = self.id;
        if self
            .i2c
            .borrow_mut()
            .write(self.address as u8, &[id, angle])
            .is_err()
        {
            panic!("sending servo direction via i2c failed");
        }
    }

    /// Returns a new servo with the given pin and right and left angles
    ///
    /// Sets the servo to the default direction right
    ///
    /// # Arguments
    ///
    /// * `pin` - the output pin to which the servo is connected
    /// * `right_angle` - the angle to set the servo to when the servo should be set to right
    /// * `left_angle` - the angle to set the servo to when the servo should be set to left
    pub fn new(
        pin: S,
        right_angle: u8,
        left_angle: u8,
        i2c: &'l RefCell<I2C>,
        id: u8,
        address: u8,
    ) -> Servo<'l, I2C, S> {
        // check bounds of angles
        if !(right_angle <= 180 && left_angle <= 180) {
            panic!("angle must be between 0 and 180");
        }

        // set to right angle as default
        let mut servo = Servo {
            pin,
            right_angle,
            left_angle,
            i2c,
            address,
            id,
        };

        servo.set_direction(&IntersectionActionDirection::Right);
        servo
    }
}
