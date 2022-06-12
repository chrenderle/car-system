use embedded_hal::digital::v2::{OutputPin, PinState};

use crate::intersection::IntersectionActionLight;
use crate::intersection::IntersectionActionLight::*;

pub const LIGHT_ACTIVE: bool = true;

/// Structure to represent a traffic light
pub struct Light<W>
where
    W: OutputPin,
{
    /// the output pin representing the green light
    green_light: W,
    /// the output pin representing the yellow light
    yellow_light: W,
    /// the output pin representing the red light
    red_light: W,
}

impl<W> Light<W>
where
    W: OutputPin,
{
    /// Returns a traffic light
    ///
    /// # Arguments
    ///
    /// * `green_light` - the output pin which represents the green light
    /// * `yellow_light` - the output pin which represents the yellow light
    /// * `red_light` - the output pin which represents the red light
    pub fn new(green_light: W, yellow_light: W, red_light: W) -> Light<W> {
        let mut light = Light {
            green_light,
            yellow_light,
            red_light,
        };
        light.set_state(&Off);
        light
    }

    /// Sets the state for the traffic light
    ///
    /// # Arguments
    ///
    /// * `state` - reference to the state which should be set
    pub fn set_state(&mut self, state: &IntersectionActionLight) {
        let light_states = match state {
            Green(_) => (LIGHT_ACTIVE, !LIGHT_ACTIVE, !LIGHT_ACTIVE),
            Yellow => (!LIGHT_ACTIVE, LIGHT_ACTIVE, !LIGHT_ACTIVE),
            Red => (!LIGHT_ACTIVE, !LIGHT_ACTIVE, LIGHT_ACTIVE),
            RedYellow => (!LIGHT_ACTIVE, LIGHT_ACTIVE, LIGHT_ACTIVE),
            Off => (!LIGHT_ACTIVE, !LIGHT_ACTIVE, !LIGHT_ACTIVE),
        };

        let green_result = self.green_light.set_state(PinState::from(light_states.0));
        let yellow_result = self.yellow_light.set_state(PinState::from(light_states.1));
        let red_result = self.red_light.set_state(PinState::from(light_states.2));

        for result in [green_result, yellow_result, red_result] {
            if result.is_err() {
                panic!("write pin failed");
            }
        }
    }
}
