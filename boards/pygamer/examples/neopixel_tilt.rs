//! LIS3DH accelerometer example. Move the neopixel led by tilting left and
//! right.
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
use hal::time::KiloHertz;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};
use lis3dh::{accelerometer::Accelerometer, Lis3dh};
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

    // neopixels
    let timer = SpinTimer::new(4);
    let mut neopixel = pins.neopixel.init(timer);

    // i2c
    let i2c = pins.i2c.init(
        &mut clocks,
        KiloHertz(400),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
    );

    let mut lis3dh = Lis3dh::new(i2c, 0x19).unwrap();
    lis3dh.set_range(lis3dh::Range::G2).unwrap();
    lis3dh.set_datarate(lis3dh::DataRate::Hz_100).unwrap();

    let mut state = TiltState::new();

    loop {
        let lis = lis3dh.acceleration().unwrap();

        let (pos, j) = state.update(lis.x);

        // iterate through neopixels and paint the one led
        let _ = neopixel.write((0..5).map(|i| {
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

        //don't update faster than the accell is reading
        delay.delay_ms(10u8);
    }
}

pub struct TiltState {
    pos: usize,
    tilt: i16,
    j: u8,
}

impl TiltState {
    // start at the middle pixel
    const fn new() -> TiltState {
        TiltState {
            pos: 2,
            tilt: 0,
            j: 0,
        }
    }

    fn update(&mut self, value: i16) -> (usize, u8) {
        //what about like.. momentum, more angle or longer its been at angle stops
        // slower like.. steps larger so it gets easier. also on a bigger number
        // tilt?

        // naive solution.. threshold tilt
        // better.. delay filter?

        // actually 2 thresholds, first you have to be tilted enough (gt / lt 1000) to
        // be counted
        if value > 1000 {
            self.tilt += 1;
        } else if value < -1000 {
            self.tilt -= 1;
        }

        // then we need threshold amount of counted tilts to inc/dec position
        if self.tilt.abs() > 20 {
            //todo clamp is nightly
            if self.tilt.is_negative() {
                if self.pos > 0 {
                    self.pos -= 1;
                }
            } else {
                if self.pos < 4 {
                    self.pos += 1;
                }
            }
            self.tilt = 0;
        }

        //incremement the hue easing
        self.j = self.j.wrapping_add(1);
        (self.pos, self.j)
    }
}
