#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_halt;
extern crate samd11_bare as hal;

#[macro_use(block)]
extern crate nb;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use hal::pac::gclk::clkctrl::GEN_A;
use hal::pac::gclk::genctrl::SRC_A;
use hal::sercom::{PadPin, Sercom0Pad0, Sercom0Pad1, UART0};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL48M, false);
    let gclk2 = clocks
        .get_gclk(GEN_A::GCLK2)
        .expect("Could not get clock 2");

    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let rx: Sercom0Pad1<_> = pins
        .d1
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let tx: Sercom0Pad0<_> = pins
        .d14
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);

    let uart_clk = clocks
        .sercom0_core(&gclk2)
        .expect("Could not configure sercom0 clock");

    let mut uart = UART0::new(
        &uart_clk,
        9600.hz(),
        peripherals.SERCOM0,
        &mut peripherals.PM,
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
