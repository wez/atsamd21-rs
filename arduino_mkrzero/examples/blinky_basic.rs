#![no_std]
#![no_main]

extern crate arduino_mkrzero as hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::{CorePeripherals, entry, Peripherals};

entry!(main);

fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::new(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::pins(peripherals.PORT);
    let mut led = pins.led_builtin.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        delay.delay_ms(200u8);
        led.set_high();
        delay.delay_ms(200u8);
        led.set_low();
    }
}
