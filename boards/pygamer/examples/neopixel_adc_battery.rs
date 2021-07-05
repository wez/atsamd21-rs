//! Display battery percentage on the neopixels.
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::adc::Adc;
use hal::pac::gclk::pchctrl::GEN_A::GCLK11;
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};
use pac::{CorePeripherals, Peripherals};
use smart_leds::{brightness, hsv::RGB8, SmartLedsWrite};

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

    let mut adc0 = Adc::adc0(peripherals.ADC0, &mut peripherals.MCLK, &mut clocks, GCLK11);
    let mut battery = pins.battery.init();

    // neopixels
    let timer = SpinTimer::new(4);
    let mut neopixel = pins.neopixel.init(timer);

    let mut delay = Delay::new(core.SYST, &mut clocks);

    //todo put this on a .. 10minute, 30min, update timer
    loop {
        let battery_data = battery.read(&mut adc0);

        let mut colors = [
            RGB8::default(),
            RGB8::default(),
            RGB8::default(),
            RGB8::default(),
            RGB8::default(),
        ];

        if battery_data < 3.6 {
            colors[0] = RGB8::from((255, 0, 0));
        } else if (battery_data >= 3.6) && (battery_data < 3.8) {
            colors[0] = RGB8::from((255, 0, 0));
            colors[1] = RGB8::from((255, 0, 0));
        } else if (battery_data >= 3.8) && (battery_data < 3.9) {
            colors[0] = RGB8::from((255, 255, 0));
            colors[1] = RGB8::from((255, 255, 0));
            colors[2] = RGB8::from((255, 255, 0));
        } else if (battery_data >= 3.9) && (battery_data < 4.0) {
            colors[0] = RGB8::from((0, 255, 0));
            colors[1] = RGB8::from((0, 255, 0));
            colors[2] = RGB8::from((0, 255, 0));
            colors[3] = RGB8::from((0, 255, 0));
        } else {
            colors[0] = RGB8::from((0, 255, 0));
            colors[1] = RGB8::from((0, 255, 0));
            colors[2] = RGB8::from((0, 255, 0));
            colors[3] = RGB8::from((0, 255, 0));
            colors[4] = RGB8::from((0, 255, 0));
        };

        neopixel
            .write(brightness(colors.iter().cloned(), 1))
            .unwrap();

        delay.delay_ms(10u8);
    }
}
