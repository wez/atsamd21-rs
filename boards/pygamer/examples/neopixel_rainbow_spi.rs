//! Rotate all neopixel leds through a rainbow. Uses a luckily placed set of SPI
//! pins as a timer source.
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::prelude::*;
use hal::sercom::v2::spi;
use hal::{clock::GenericClockController, delay::Delay};
use pac::{CorePeripherals, Peripherals};
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::SmartLedsWrite;
use ws2812_spi as ws2812;

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
    let pins = Pins::new(peripherals.PORT);
    let mclk = &mut peripherals.MCLK;
    let gclk = clocks.gclk0();
    let clock = &clocks.sercom2_core(&gclk).unwrap();
    let sercom2 = peripherals.SERCOM2;
    let pads = spi::Pads::default()
        .data_in(pins.sda)
        .data_out(pins.neopixel)
        .sclk(pins.scl);
    let spi = spi::Config::new(mclk, sercom2, pads, clock.freq())
        .baud(3.mhz())
        .enable();
    let mut neopixel = ws2812::Ws2812::new(spi);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        for j in 0..255u8 {
            let colors = [
                // split the color changes across all 5 leds evenly, 255/5=51
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
