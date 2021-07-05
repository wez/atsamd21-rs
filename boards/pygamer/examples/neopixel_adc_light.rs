//! Display light sensor reading on the neopixels.
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use embedded_hal::digital::v1_compat::OldOutputPin;
use hal::adc::Adc;
use hal::gpio::v2::AlternateB;
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};
use pac::gclk::pchctrl::GEN_A::GCLK11;
use pac::{CorePeripherals, Peripherals};
use smart_leds::hsv::{hsv2rgb, Hsv, RGB8};
use smart_leds::SmartLedsWrite;
use ws2812_timer_delay as ws2812;

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

    let mut adc1 = Adc::adc1(peripherals.ADC1, &mut peripherals.MCLK, &mut clocks, GCLK11);
    let mut light = pins.light.into_mode::<AlternateB>();

    let timer = SpinTimer::new(4);
    let neopixel_pin: OldOutputPin<_> = pins.neopixel.into_push_pull_output().into();
    let mut neopixel = ws2812::Ws2812::new(timer, neopixel_pin);

    let mut delay = Delay::new(core.SYST, &mut clocks);

    const NUM_LEDS: usize = 5;
    let mut j: u8 = 0;

    loop {
        let light_data: u16 = adc1.read(&mut light).unwrap();

        let pos: usize = if light_data < 100 {
            0
        } else if (light_data >= 147) && (light_data < 1048) {
            1
        } else if (light_data >= 1048) && (light_data < 3048) {
            2
        } else if (light_data >= 3048) && (light_data < 3948) {
            3
        } else {
            4
        };

        //finally paint the one led wherever the position is
        let _ = neopixel.write((0..NUM_LEDS).map(|i| {
            if i == pos {
                hsv2rgb(Hsv {
                    hue: j,
                    sat: 255,
                    val: 32,
                })
            } else {
                RGB8::default()
            }
        }));

        //incremement the hue easing
        j = j.wrapping_add(1);

        delay.delay_ms(10u8);
    }
}
