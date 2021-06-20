//! Joystick y controls the color of a neopixel while Joystick x moves it
//! left and right around the center neopixel
//! Select and Start control a second neopixel left and right while it is
//! automatically rotating through the color wheel
//! When they overlap, joystick takes precedence
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Keys, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use bsp::util::map_from;
use hal::adc::Adc;
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};
use pac::gclk::pchctrl::GEN_A::GCLK11;
use pac::{CorePeripherals, Peripherals};
use smart_leds::hsv::{hsv2rgb, Hsv, RGB8};
use smart_leds::SmartLedsWrite;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core_peripherals = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core_peripherals.SYST, &mut clocks);
    let pins = Pins::new(peripherals.PORT).split();

    let mut buttons = pins.buttons.init();

    let mut adc1 = Adc::adc1(peripherals.ADC1, &mut peripherals.MCLK, &mut clocks, GCLK11);
    let mut joystick = pins.joystick.init();

    // neopixels
    let timer = SpinTimer::new(4);

    let mut neopixel = pins.neopixel.init(timer);

    const NUM_LEDS: usize = 5;
    let mut pos_button: usize = 2;
    let mut color_button: u8 = 0;
    loop {
        let (x, y) = joystick.read(&mut adc1);

        // map up/down to control rainbow color 0-255
        let color_joy = map_from(y as i16, (0, 4095), (0, 255)) as u8;

        // map left/right to neopixel position 0-4
        // joystick is not quite linear, rests at second pixel
        // shifting up by 500 seems to help
        let pos_joy = map_from(x as i16 + 500, (0, 4595), (0, 4)) as usize;

        for event in buttons.events() {
            match event {
                Keys::SelectDown => {
                    if pos_button > 0 {
                        pos_button -= 1;
                    }
                }
                Keys::StartDown => {
                    if pos_button < 4 {
                        pos_button += 1;
                    }
                }
                _ => {}
            }
        }

        //finally paint the two leds at position, accel priority
        let _ = neopixel.write((0..NUM_LEDS).map(|i| {
            if i == pos_joy {
                hsv2rgb(Hsv {
                    hue: color_joy,
                    sat: 255,
                    val: 32,
                })
            } else if i == pos_button {
                hsv2rgb(Hsv {
                    hue: color_button,
                    sat: 255,
                    val: 32,
                })
            } else {
                RGB8::default()
            }
        }));

        //incremement the hue easing
        color_button = color_button.wrapping_add(1);

        delay.delay_ms(5u8);
    }
}
