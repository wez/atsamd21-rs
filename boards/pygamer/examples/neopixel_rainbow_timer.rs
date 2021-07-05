//! Rotate all neopixel leds through a rainbow. Uses a Timer as a timer source.
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml
//!
//! Note: This is jittery these days and probably not a good choice until
//! debugged

#![no_std]
#![no_main]

use bsp::{entry, hal, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::{clock::GenericClockController, delay::Delay, timer::TimerCounter};
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::SmartLedsWrite;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = Pins::new(peripherals.PORT).split();

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK);
    timer.start(3.mhz());

    let mut neopixel = pins.neopixel.init(timer);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        for j in 0..255u8 {
            let colors = [
                // stagger the color changes across all 5 leds evenly, 255/5=51
                // and have them safely wrap over when they go above 255
                hsv2rgb(Hsv {
                    hue: j,
                    sat: 255,
                    val: 32,
                }),
                hsv2rgb(Hsv {
                    hue: j.wrapping_add(51),
                    sat: 255,
                    val: 32,
                }),
                hsv2rgb(Hsv {
                    hue: j.wrapping_add(102),
                    sat: 255,
                    val: 32,
                }),
                hsv2rgb(Hsv {
                    hue: j.wrapping_add(153),
                    sat: 255,
                    val: 32,
                }),
                hsv2rgb(Hsv {
                    hue: j.wrapping_add(204),
                    sat: 255,
                    val: 32,
                }),
            ];
            neopixel.write(colors.iter().cloned()).unwrap();
            delay.delay_ms(5u8);
        }
    }
}
