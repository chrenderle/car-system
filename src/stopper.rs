use embedded_hal::digital::v2::{OutputPin, PinState};

pub const STOPPER_ACTIVE: bool = false;

/// Struct which controls a stopper which stops cars
pub struct Stopper<W>
where
    W: OutputPin,
{
    /// The pin connected to the stopper
    pin: W,
    /// The number of locks set by sections through lock() and released by release()
    number_locks: usize,
    /// If the intersection locked the stopper
    ///
    /// Overwrites the number_locks
    intersection_lock: bool,
}

impl<W> Stopper<W>
where
    W: OutputPin,
{
    /// Writes to the pin of the stopper according to the locks
    ///
    /// # Panic
    /// Panics when writing the pin failes
    fn write_pin(&mut self) {
        #[allow(clippy::redundant_pattern_matching)]
        if let Err(_) = self.pin.set_state(PinState::from(match self.get_state() {
            STOPPER_ACTIVE => true,
            _ => false,
        })) {
            panic!("write failed");
        }
    }

    /// Returns the current state which depends on the locks (number_locks and intersection_lock)
    pub fn get_state(&self) -> bool {
        self.number_locks > 0 || self.intersection_lock
    }

    /// Locks the stopper by increasing number_locks by one and then calling write_pin()
    ///
    /// Only calles write_pin() when a change occured
    pub fn lock(&mut self) {
        self.number_locks += 1;

        // only call write_pin() if the stopper changes the lock state
        if self.number_locks == 1 && !self.intersection_lock {
            self.write_pin();
        }
    }

    /// Releases the stopper by decreasing number_locks by one if number_locks is 0
    ///
    /// Only calles write_pin() if a change occured
    pub fn release(&mut self) {
        if self.number_locks > 0 {
            self.number_locks -= 1;
        }

        // only call write_pin() if the stopper changes the lock state
        if self.number_locks == 0 && !self.intersection_lock {
            self.write_pin();
        }
    }

    /// Locks the stopper overwriting number_locks
    ///
    /// Only meant to be called from a intersection
    pub fn intersection_lock(&mut self) {
        self.intersection_lock = true;

        // only call write_pin() if the stopper changes the lock state
        if self.number_locks > 0 {
            self.write_pin();
        }
    }

    /// Releases the intersection lock overwrite
    ///
    /// Only meant to be called from a intersection
    pub fn intersection_release(&mut self) {
        self.intersection_lock = false;

        // only call write_pin() if the stopper changes the lock state
        if self.number_locks == 0 {
            self.write_pin();
        }
    }

    /// Returns a stopper with the given pin
    pub fn new(pin: W) -> Self {
        let mut stopper = Stopper {
            pin,
            number_locks: 0,
            intersection_lock: false,
        };
        stopper.write_pin();
        stopper
    }
}
