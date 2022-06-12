use embedded_hal::digital::v2::{InputPin, OutputPin};
use core::cell::RefCell;

pub struct Pin<'l> {
    state: &'l RefCell<bool>,
}

impl<'l> Pin<'l> {
    pub fn new(state: &'l RefCell<bool>) -> Pin<'l> {
        Pin { state }
    }
}

impl InputPin for Pin<'_> {
    type Error = ();
    
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(*self.state.borrow())
    }
    
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!*self.state.borrow())
    }
}

impl OutputPin for Pin<'_> {
    type Error = ();
    
    fn set_high(&mut self) -> Result<(), Self::Error> {
        *self.state.borrow_mut() = true;
        Ok(())
    }
    
    fn set_low(&mut self) -> Result<(), Self::Error> {
        *self.state.borrow_mut() = false;
        Ok(())
    }
}