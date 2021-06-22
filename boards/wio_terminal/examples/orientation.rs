#![no_std]
#![no_main]

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use eg::image::{Image, ImageRaw, ImageRawLE};
use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};

use wio::accelerometer::{self, Tracker};
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins, Sets};

// The height and width of the RAW image of Ferris, which can be found at
// 'assets/ferris.raw'.
const IMG_HEIGHT: u32 = 64;
const IMG_WIDTH: u32 = 86;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let pins = Pins::new(peripherals.PORT);
    let mut sets: Sets = pins.split();

    // Initialize the LIS3DH accelerometer, and create the orientation tracker.
    // The calibration value for Tracker was obtained experimentally, as directed in
    // the documentation.
    let mut lis3dh = sets.accelerometer.init(
        &mut clocks,
        peripherals.SERCOM4,
        &mut peripherals.MCLK,
        &mut sets.port,
    );
    let mut tracker = Tracker::new(3700.0);

    // Initialize the ILI9341-based LCD display. Create a black backdrop the size of
    // the screen, load an image of Ferris from a RAW file, and draw it to the
    // screen.
    // By default, the display is in the LandscapeFlipped orientation.
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            58.mhz(),
            &mut delay,
        )
        .unwrap();

    // The display's resolution is 320x240. I'm too lazy to deal with orientation
    // for something as trivial as a backdrop, so it's larger than the display in
    // one dimension.
    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();
    let backdrop =
        Rectangle::with_corners(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
    backdrop.draw(&mut display).unwrap();

    // Load the RAW image file into a renderable format. Determine the coordinate to
    // center the image on the display, and draw the image there.
    let image_data: ImageRawLE<Rgb565> =
        ImageRaw::new(include_bytes!("../assets/ferris.raw"), IMG_WIDTH);
    let position = center_image(display.width(), display.height());
    let image = Image::new(&image_data, position);
    image.draw(&mut display).unwrap();

    // Determine a baseline orientation. This doesn't really matter initially, but
    // will be updated periodically.
    let acceleration = lis3dh.accel_raw().unwrap();
    let mut prev_orientation = tracker.update(acceleration);

    loop {
        // Get the current orientation of the device.
        let acceleration = lis3dh.accel_raw().unwrap();
        let orientation = tracker.update(acceleration);

        // If the orientation hasn't changed, we have nothing left to do.
        if orientation == prev_orientation {
            continue;
        }

        // Attempt to convert the accelerometer orientation to a display orientation. If
        // successful, set the display to the correct orientation and re-center the
        // image.
        if let Some(mode) = try_convert_orientation(orientation) {
            display.set_orientation(mode).unwrap();
            prev_orientation = orientation;

            let position = center_image(display.width(), display.height());
            let image = Image::new(&image_data, position);

            backdrop.draw(&mut display).unwrap();
            image.draw(&mut display).unwrap();
        }
    }
}

// Given the width and height of the display, return the coordinate which would
// center the image, based on its dimensions defined at the top of the file.
fn center_image(width: usize, height: usize) -> Point {
    let x = (width as i32 / 2) - (IMG_WIDTH as i32 / 2);
    let y = (height as i32 / 2) - (IMG_HEIGHT as i32 / 2);
    Point::new(x, y)
}

// Given an accelerometer orientation, return the equivalent display
// orientation, if one exists.
fn try_convert_orientation(
    orientation: accelerometer::Orientation,
) -> Option<ili9341::Orientation> {
    use accelerometer::Orientation;

    match orientation {
        Orientation::LandscapeUp => Some(ili9341::Orientation::LandscapeFlipped),
        Orientation::LandscapeDown => Some(ili9341::Orientation::Landscape),
        Orientation::PortraitUp => Some(ili9341::Orientation::Portrait),
        Orientation::PortraitDown => Some(ili9341::Orientation::PortraitFlipped),
        _ => None,
    }
}
