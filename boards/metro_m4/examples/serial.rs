#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate metro_m4 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

#[macro_use(block)]
extern crate nb;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::gclk::genctrl::SRC_A;
use hal::pac::gclk::pchctrl::GEN_A;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::sercom::{PadPin, Sercom3Pad0, Sercom3Pad1, UART3};

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
    clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
    let gclk2 = clocks
        .get_gclk(GEN_A::GCLK2)
        .expect("Could not get clock 2");

    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let rx: Sercom3Pad1<_> = pins
        .d0
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let tx: Sercom3Pad0<_> = pins
        .d1
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let uart_clk = clocks
        .sercom3_core(&gclk2)
        .expect("Could not configure sercom3 clock");

    let mut uart = UART3::new(
        &uart_clk,
        9600.Hz(),
        peripherals.SERCOM3,
        &mut peripherals.MCLK,
        (rx, tx),
    );

    loop {
        for byte in b"Hello, world!" {
            // NOTE `block!` blocks until `uart.write()` completes and returns
            // `Result<(), Error>`
            block!(uart.write(*byte)).unwrap();
        }
        delay.delay_ms(1000u16);
    }
}
