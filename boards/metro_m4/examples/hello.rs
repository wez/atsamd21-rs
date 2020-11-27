#![no_std]
#![no_main]

extern crate cortex_m;
extern crate metro_m4 as hal;

use panic_probe as _; //for probe-run
use rtt_target::{rprintln, rtt_init_print};
//use panic_rtt as _; //for cargo embed

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    loop {
        // panic!('goodbye');
        rprintln!("hellooooo");
        cortex_m::asm::bkpt();
    }
}
