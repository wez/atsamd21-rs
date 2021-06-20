//! Place a bitmap image on the screen.
//! Convert a png to .raw bytes
//! * With imagemagick `convert ferris.png -flip -type truecolor -define
//!   bmp:subtype=RGB565 -depth 16 -strip ferris.bmp`
//! * Or export images directly from GIMP by saving as .bmp and choosing 16bit
//!   R5 G6 B5
//! Then `tail -c 11008 ferris.bmp > ferris.raw` where c is width*height*2 and
//! our ferris.png was 86x64

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::prelude::*;
use embedded_graphics::{egrectangle, primitive_style};
use embedded_graphics::{image::Image, image::ImageRaw, image::ImageRawLE};
use hal::clock::GenericClockController;
use pac::{CorePeripherals, Peripherals};

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
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    let (mut display, _backlight) = pins
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM4,
            &mut peripherals.MCLK,
            peripherals.TC2,
            &mut delay,
        )
        .unwrap();

    egrectangle!(
        top_left = (0, 0),
        bottom_right = (160, 128),
        style = primitive_style!(stroke_width = 0, fill_color = RgbColor::BLACK)
    )
    .draw(&mut display)
    .unwrap();

    let raw_image: ImageRawLE<Rgb565> =
        ImageRaw::new(include_bytes!("../assets/ferris.raw"), 86, 64);
    let ferris: Image<_, Rgb565> = Image::new(&raw_image, Point::new(32, 32));

    ferris.draw(&mut display).unwrap();
    loop {}
}
