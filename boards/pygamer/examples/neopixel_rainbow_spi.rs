//! Rotate all neopixel leds through a rainbow. Uses a luckily placed set of SPI
//! pins as a timer source.
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml

#![no_std]
#![no_main]

use panic_halt as _;
use pygamer as hal;

use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::sercom::PadPin;
use hal::{clock::GenericClockController, delay::Delay};
use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
};
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
    let mut pins = hal::Pins::new(peripherals.PORT);
    let gclk = clocks.gclk0();

    let spi: hal::sercom::SPIMaster2<
        hal::sercom::Sercom2Pad0<hal::gpio::Pa12<hal::gpio::PfC>>,
        hal::sercom::Sercom2Pad3<hal::gpio::Pa15<hal::gpio::PfC>>,
        hal::sercom::Sercom2Pad1<hal::gpio::Pa13<hal::gpio::PfC>>,
    > = hal::sercom::SPIMaster2::new(
        &clocks.sercom2_core(&gclk).unwrap(),
        3_000_000.Hz(),
        embedded_hal::spi::Mode {
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
            polarity: embedded_hal::spi::Polarity::IdleLow,
        },
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        (
            pins.sda.into_pad(&mut pins.port),
            pins.neopixel.into_pad(&mut pins.port),
            pins.scl.into_pad(&mut pins.port),
        ),
    );

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
