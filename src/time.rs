use core::cell::Cell;

const PRESCALER: u64 = 1024;
const TIMER_COUNTS: u64 = 125;

const MILLIS_INCREMENT: u64 = PRESCALER * TIMER_COUNTS / 16000;

static MILLIS_COUNTER: avr_device::interrupt::Mutex<Cell<u64>> =
    avr_device::interrupt::Mutex::new(Cell::new(0));

pub fn millis_init(tc0: arduino_hal::pac::TC0) {
    // Configure the timer for the above interval (in CTC mode)
    // and enable its interrupt.
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| unsafe { w.bits(TIMER_COUNTS as u8) });
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
}

#[avr_device::interrupt(atmega2560)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    })
}

pub fn millis() -> u64 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
}
/*const MICROS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 2;

static MICROS_COUNTER: avr_device::interrupt::Mutex<Cell<u32>> =
    avr_device::interrupt::Mutex::new(Cell::new(0));

pub fn micros_init(tc0: arduino_hal::pac::TC0) {
    // configure the timer for the above interval (in CTC mode)
    // and enable its interrupt
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| unsafe { w.bits(TIMER_COUNTS as u8) });
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MICROS_COUNTER.borrow(cs).set(0);
    });
}

#[avr_device::interrupt(atmega2560)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MICROS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MICROS_INCREMENT);
    })
}

pub fn micros() -> u32 {
    avr_device::interrupt::free(|cs| MICROS_COUNTER.borrow(cs).get())
}*/
