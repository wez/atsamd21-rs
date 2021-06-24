#![no_std]
#![no_main]

extern crate arduino_nano33iot as hal;
extern crate embedded_graphics;
extern crate st7735_lcd;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::time::MegaHertz;

use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

use st7735_lcd::Orientation;

const BOOT_DELAY_MS: u16 = 100;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    delay.delay_ms(BOOT_DELAY_MS);

    let spi = hal::spi_master(
        &mut clocks,
        MegaHertz(16),
        peripherals.SERCOM1,
        &mut peripherals.PM,
        pins.led_sck,
        pins.mosi,
        pins.miso,
        &mut pins.port,
    );

    let dc = pins.d6.into_open_drain_output(&mut pins.port);
    let rst = pins.d9.into_open_drain_output(&mut pins.port);

    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);

    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();

    Rectangle::with_corners(Point::new(0, 0), Point::new(160, 128))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    disp.set_offset(0, 25);

    // draw ferris
    let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("assets/ferris.raw"), 86);
    let image = Image::new(&image_raw, Point::new(34, 8));
    image.draw(&mut disp).unwrap();

    loop {}
}
