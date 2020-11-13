#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate metro_m4 as hal;
extern crate nb;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::sercom::PadPin;
use nb::block;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut pins = hal::Pins::new(peripherals.PORT);
    let gclk = clocks.gclk0();

    let mut spi: hal::sercom::SPIMaster3<
        hal::sercom::Sercom3Pad3<hal::gpio::Pa21<hal::gpio::PfD>>,
        hal::sercom::Sercom3Pad0<hal::gpio::Pa22<hal::gpio::PfC>>,
        hal::sercom::Sercom3Pad1<hal::gpio::Pa23<hal::gpio::PfC>>,
    > = hal::sercom::SPIMaster3::new(
        &clocks.sercom3_core(&gclk).unwrap(),
        3_000_000u32.Hz(),
        embedded_hal::spi::Mode {
            polarity: embedded_hal::spi::Polarity::IdleLow,
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
        },
        peripherals.SERCOM3,
        &mut peripherals.MCLK,
        (
            pins.d8.into_pad(&mut pins.port),
            pins.d1.into_pad(&mut pins.port),
            pins.d0.into_pad(&mut pins.port),
        ),
    );

    loop {
        for byte in b"Hello, world!" {
            block!(spi.send(*byte)).unwrap();
        }
        delay.delay_ms(1000u16);
    }
}
