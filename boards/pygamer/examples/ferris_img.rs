//! Place a bitmap image on the screen. Convert png to .bmp
//! * Resize and export images directly from image editor by saving as .bmp and
//!   choosing 16bit R5 G6 B5
//! * OR Convert with imagemagick: convert rustacean-flat-noshadow.png -type
//!   truecolor -define bmp:subtype=RGB565 -depth 16 -strip -resize 86x64
//!   ferris.bmp

#![no_std]
#![no_main]

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer::{self as hal, entry, pac, Pins};

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::{image::Image, pixelcolor::Rgb565};
use hal::clock::GenericClockController;
use pac::{CorePeripherals, Peripherals};
use tinybmp::Bmp;

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
    let mut pins = Pins::new(peripherals.PORT).split();
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    let (mut display, _backlight) = pins
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM4,
            &mut peripherals.MCLK,
            peripherals.TC2,
            &mut delay,
            &mut pins.port,
        )
        .unwrap();

    // black out the screen
    Rectangle::with_corners(Point::new(0, 0), Point::new(160, 128))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

    let raw_image: Bmp<Rgb565> = Bmp::from_slice(include_bytes!("../assets/ferris.bmp")).unwrap();
    let ferris = Image::new(&raw_image, Point::new(32, 32));

    ferris.draw(&mut display).unwrap();
    loop {}
}
