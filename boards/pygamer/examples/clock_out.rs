//! Use a hardware timer to wiggle a pin in the background.

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::clock::GenericClockController;
use hal::gpio::v2::AlternateM;
use pac::gclk::genctrl::SRC_A::DPLL0;
use pac::gclk::pchctrl::GEN_A::GCLK2;
use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = Pins::new(peripherals.PORT);

    //3mhz
    let _gclk2 = clocks
        .configure_gclk_divider_and_source(GCLK2, 40, DPLL0, false)
        .unwrap();
    pins.d5.into_mode::<AlternateM>();
    loop {}
}
