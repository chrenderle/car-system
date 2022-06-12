use core::cell::RefCell;
use embedded_hal::blocking::i2c;
use embedded_hal::blocking::i2c::SevenBitAddress;
use embedded_hal::digital::v2::OutputPin;

use crate::intersection::IntersectionActionDirection::*;
use crate::intersection::IntersectionActionLight::*;
use crate::lights::*;
use crate::servo::Servo;
use crate::stopper::Stopper;

const LONG_STATE_TIME: u32 = 18;
const SHORT_STATE_TIME: u32 = 2;

pub trait CustomIterator {
    fn next(&mut self) -> &IntersectionState;
    fn current(&self) -> &IntersectionState;
}

pub struct IntersectionArm<'l, I2C, W, S>
where
    I2C: i2c::Write<SevenBitAddress>,
    W: OutputPin,
    S: OutputPin,
{
    pub entry_stopper: &'l RefCell<Stopper<W>>,
    pub light: Light<W>,
    pub servo: &'l RefCell<Servo<'l, I2C, S>>,
}

pub enum IntersectionActionDirection {
    Right,
    Left,
}

pub enum IntersectionActionLight {
    Green(IntersectionActionDirection),
    Yellow,
    Red,
    RedYellow,
    Off,
}

pub struct IntersectionState {
    left_action: IntersectionActionLight,
    right_action: IntersectionActionLight,
    upper_action: IntersectionActionLight,
    duration: u32,
}

pub struct DefaultIntersectionStates {
    states: [IntersectionState; 8],
    count: usize,
}

impl DefaultIntersectionStates {
    pub fn new() -> DefaultIntersectionStates {
        let states: [IntersectionState; 8] = [
            // Phase 1
            IntersectionState {
                left_action: Green(Right),
                right_action: Green(Right),
                upper_action: Green(Right),
                duration: LONG_STATE_TIME,
            },
            IntersectionState {
                left_action: Green(Right),
                right_action: Yellow,
                upper_action: Green(Right),
                duration: SHORT_STATE_TIME,
            },
            // Phase 2
            IntersectionState {
                left_action: Green(Left),
                right_action: Red,
                upper_action: Green(Right),
                duration: LONG_STATE_TIME,
            },
            IntersectionState {
                left_action: Green(Left),
                right_action: RedYellow,
                upper_action: Yellow,
                duration: SHORT_STATE_TIME,
            },
            // Phase 3
            IntersectionState {
                left_action: Green(Right),
                right_action: Green(Left),
                upper_action: Red,
                duration: LONG_STATE_TIME,
            },
            IntersectionState {
                left_action: Yellow,
                right_action: Green(Left),
                upper_action: RedYellow,
                duration: SHORT_STATE_TIME,
            },
            // Phase 4
            IntersectionState {
                left_action: Red,
                right_action: Green(Right),
                upper_action: Green(Left),
                duration: LONG_STATE_TIME,
            },
            IntersectionState {
                left_action: RedYellow,
                right_action: Green(Right),
                upper_action: Green(Left),
                duration: SHORT_STATE_TIME,
            },
        ];
        DefaultIntersectionStates { states, count: 7 }
    }
}

impl CustomIterator for DefaultIntersectionStates {
    fn next(&mut self) -> &IntersectionState {
        self.count += 1;
        if self.count == self.states.len() {
            self.count = 0;
        }
        &self.states[self.count]
    }

    fn current(&self) -> &IntersectionState {
        &self.states[self.count]
    }
}

pub struct Intersection<'l, I2C, I, W, S>
where
    I2C: i2c::Write<SevenBitAddress>,
    I: CustomIterator,
    W: OutputPin,
    S: OutputPin,
{
    left_arm: IntersectionArm<'l, I2C, W, S>,
    right_arm: IntersectionArm<'l, I2C, W, S>,
    upper_arm: IntersectionArm<'l, I2C, W, S>,
    states: I,
    time_counter: u32,
}

impl<'l, I2C, I, W, S> Intersection<'l, I2C, I, W, S>
where
    I2C: i2c::Write<SevenBitAddress>,
    I: CustomIterator,
    W: OutputPin,
    S: OutputPin,
{
    pub fn new(
        left_arm: IntersectionArm<'l, I2C, W, S>,
        right_arm: IntersectionArm<'l, I2C, W, S>,
        upper_arm: IntersectionArm<'l, I2C, W, S>,
        states: I,
    ) -> Intersection<'l, I2C, I, W, S> {
        let mut intersection = Intersection {
            left_arm,
            right_arm,
            upper_arm,
            states,
            time_counter: 0,
        };
        intersection.execute_next_state();
        intersection
    }

    pub fn call(&mut self) {
        self.time_counter += 1;
        if self.time_counter >= self.states.current().duration {
            self.time_counter = 0;
            self.execute_next_state();
        }
    }

    fn execute_next_state(&mut self) {
        let state = self.states.next();

        // take action for right intersection arm stoppers
        match state.right_action {
            Green(_) => {
                self.right_arm
                    .entry_stopper
                    .borrow_mut()
                    .intersection_release();
            }
            Yellow => {
                self.right_arm
                    .entry_stopper
                    .borrow_mut()
                    .intersection_lock();
            }
            _ => (),
        }

        // take action for left intersection arm stoppers
        match state.left_action {
            Green(_) => {
                self.left_arm
                    .entry_stopper
                    .borrow_mut()
                    .intersection_release();
            }
            Yellow => {
                self.left_arm.entry_stopper.borrow_mut().intersection_lock();
            }
            _ => (),
        }

        // take action for upper intersection arm stoppers
        match state.upper_action {
            Green(_) => {
                self.upper_arm
                    .entry_stopper
                    .borrow_mut()
                    .intersection_release();
            }
            Yellow => {
                self.upper_arm
                    .entry_stopper
                    .borrow_mut()
                    .intersection_lock();
            }
            _ => (),
        }

        // take action for intersection lights
        self.right_arm.light.set_state(&state.right_action);
        self.left_arm.light.set_state(&state.left_action);
        self.upper_arm.light.set_state(&state.upper_action);

        // take action for intersection servos
        if let IntersectionActionLight::Green(direction) = &state.right_action {
            self.right_arm.servo.borrow_mut().set_direction(direction);
        }
        if let IntersectionActionLight::Green(direction) = &state.left_action {
            self.left_arm.servo.borrow_mut().set_direction(direction);
        }
        if let IntersectionActionLight::Green(direction) = &state.upper_action {
            self.upper_arm.servo.borrow_mut().set_direction(direction);
        }
    }
}
