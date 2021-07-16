#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;

use wio_lite_w600 as bsp;

use bsp::hal;
use bsp::entry;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = hal::Pins::new(peripherals.PORT);

    // You will need to connect an led to D13 with an appropriate resistor
    let mut led = pins.d13.into_push_pull_output();

    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        led.set_high().unwrap();
        delay.delay_ms(100u16);
        led.set_low().unwrap();
        delay.delay_ms(100u16);
    }
}
