//! Place a bitmap image on the screen. Ferris pngs from https://rustacean.net/
//! Convert png to .bmp bytes
//! * Resize and export images directly from image editor by saving as .bmp and
//!   choosing 16bit R5 G6 B5
//! * OR Convert with imagemagick: convert rustacean-flat-noshadow.png -type
//!   truecolor -define bmp:subtype=RGB565 -depth 16 -strip -resize 86x64
//!   ferris.bmp
//!
//! cp ferris*.bmp /Volumes/SDCARD/

#![no_std]
#![no_main]

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer::{self as hal, entry, pac, Pins};

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::{image::Image, pixelcolor::Rgb565};
use embedded_hal::digital::v1_compat::OldOutputPin;
use embedded_sdmmc::{TimeSource, Timestamp, VolumeIdx};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::time::MegaHertz;
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
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut pins = Pins::new(peripherals.PORT).split();

    let mut red_led = pins.led_pin.into_open_drain_output(&mut pins.port);

    let sdmmc_cs: OldOutputPin<_> = pins.sd_cs_pin.into_push_pull_output(&mut pins.port).into();
    let sdmmc_spi = pins.spi.init(
        &mut clocks,
        MegaHertz(3),
        peripherals.SERCOM1,
        &mut peripherals.MCLK,
        &mut pins.port,
    );
    let mut cont =
        embedded_sdmmc::Controller::new(embedded_sdmmc::SdMmcSpi::new(sdmmc_spi, sdmmc_cs), Clock);

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

    cont.device().init().unwrap();
    let mut volume = cont.get_volume(VolumeIdx(0)).unwrap();
    let dir = cont.open_root_dir(&volume).unwrap();

    let mut scratch = [0u8; 11008];

    // "animation" order of files to open
    let images = ["ferris.bmp", "ferris1.bmp", "ferris2.bmp", "ferris1.bmp"];

    loop {
        for image in images.iter() {
            if let Ok(mut f) =
                cont.open_file_in_dir(&mut volume, &dir, image, embedded_sdmmc::Mode::ReadOnly)
            {
                let _ = cont.read(&volume, &mut f, &mut scratch);

                let raw_image: Bmp<Rgb565> = Bmp::from_slice(&scratch).unwrap();

                let ferris = Image::new(&raw_image, Point::new(32, 32));

                let _ = ferris.draw(&mut display);

                cont.close_file(&volume, f).ok();
            } else {
                let _ = red_led.set_high();
            }
            delay.delay_ms(200u8);
        }
    }
}

struct Clock;

impl TimeSource for Clock {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}
