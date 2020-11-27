//! Reads the adc and prints to rtt every second.
//!
//! Requires `cargo install probe-run`
//!
//! probe-run builds, uploads, and runs your code on device and in combination
//! with rtt-target and panic-probe prints debug and panic information to your
//! console. Its used for short running sessions like seeing the results of a
//! calculation or a measurement, a panic message or backtrace of an error right
//! on your command line. You can also force an exit with a
//! cortex_m::asm::bkpt()
//!
//! `cargo run --release --example adc --features=unproven`

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate embedded_hal;
extern crate samd11_bare as hal;

use hal::adc::Adc;
use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    let mut pins = hal::Pins::new(peripherals.PORT);

    let mut adc = Adc::adc(peripherals.ADC, &mut peripherals.PM, &mut clocks);
    let mut a0 = pins.d1.into_function_b(&mut pins.port);

    loop {
        let data: u16 = adc.read(&mut a0).unwrap();
        rprintln!("{}", data);
        delay.delay_ms(1000u16);
    }
}
