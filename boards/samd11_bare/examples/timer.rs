#![no_std]
#![no_main]

extern crate panic_halt;
extern crate samd11_bare as hal;

extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate nb;

use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::Peripherals;
use hal::prelude::*;
use hal::time::{Hertz, Nanoseconds};
use hal::timer::TimerCounter;
use nb::block;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc1_tc2(&gclk0).unwrap();
    let mut timer = TimerCounter::tc1_(&timer_clock, peripherals.TC1, &mut peripherals.PM);
    timer.start(Hertz(1u32).to_duration::<Nanoseconds>().unwrap());

    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut d2 = pins.d2.into_open_drain_output(&mut pins.port);

    loop {
        d2.set_high().unwrap();
        block!(timer.wait()).ok();
        d2.set_low().unwrap();
        block!(timer.wait()).ok();
    }
}
