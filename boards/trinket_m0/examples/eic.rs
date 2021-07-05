#![no_std]
#![no_main]

/// This example is intended to be used with a pushbutton connected between D0
/// and ground.  The LED should toggle when the button is pressed (perhaps more
/// than once due to the lack of debouncing).
use panic_halt as _;
use trinket_m0 as hal;

use hal::clock::GenericClockController;
use hal::eic::{pin::Sense, EIC};
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let _core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut led = pins.d13.into_push_pull_output(&mut pins.port);
    led.set_high().unwrap();

    let gclk0 = clocks.gclk0();
    let clock = clocks.eic(&gclk0).unwrap();
    let mut eic = EIC::init(&mut peripherals.PM, clock, peripherals.EIC);

    let mut d3 = pins.d3.into_pull_up_ei(&mut pins.port);
    d3.sense(&mut eic, Sense::FALL);
    d3.enable_interrupt(&mut eic);

    loop {
        if d3.is_interrupt() {
            d3.clear_interrupt();
            led.toggle();
        }
    }
}
