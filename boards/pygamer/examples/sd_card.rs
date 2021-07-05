//! Place a series of bitmap image on the screen from the sd card.
//! Install Imagemagick and convert 3 pngs from https://rustacean.net/ to centered 86x64 size .raw bytes (where 11008 is 86x64x2)
//! convert -resize 86x64^ -gravity center -extent 86x64 -background black
//! rustacean-orig-noshadow.png -flip -type truecolor -define bmp:subtype=RGB565
//! -depth 16 -strip ferris.bmp && tail -c 11008 ferris.bmp > ferris.raw convert
//! -resize 86x64^ -gravity center -extent 86x64 -background black
//! rustacean-flat-gesture.png -flip -type truecolor -define bmp:subtype=RGB565
//! -depth 16 -strip ferris1.bmp && tail -c 11008 ferris1.bmp > ferris1.raw
//! convert -resize 86x64^ -gravity center -extent 86x64 -background black
//! rustacean-flat-happy.png -flip -type truecolor -define bmp:subtype=RGB565
//! -depth 16 -strip ferris2.bmp && tail -c 11008 ferris2.bmp > ferris2.raw
//! cp *.raw /Volumes/SDCARD/

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins, RedLed};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::prelude::*;
use embedded_graphics::{egrectangle, primitive_style};
use embedded_graphics::{image::Image, image::ImageRaw, image::ImageRawLE};
use embedded_hal::digital::v1_compat::OldOutputPin;
use embedded_sdmmc::{TimeSource, Timestamp, VolumeIdx};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::time::MegaHertz;
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
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let pins = Pins::new(peripherals.PORT).split();

    let mut red_led: RedLed = pins.led_pin.into();

    let sdmmc_cs: OldOutputPin<_> = pins.sd_cs_pin.into_push_pull_output().into();
    let sdmmc_spi = pins.spi.init(
        &mut clocks,
        MegaHertz(3),
        peripherals.SERCOM1,
        &mut peripherals.MCLK,
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
        )
        .unwrap();

    egrectangle!(
        top_left = (0, 0),
        bottom_right = (160, 128),
        style = primitive_style!(stroke_width = 0, fill_color = RgbColor::BLACK)
    )
    .draw(&mut display)
    .unwrap();

    cont.device().init().unwrap();
    let mut volume = cont.get_volume(VolumeIdx(0)).unwrap();
    let dir = cont.open_root_dir(&volume).unwrap();

    let mut scratch = [0u8; 11008];

    //"animation" order of files to open
    let images = ["ferris.raw", "ferris1.raw", "ferris2.raw", "ferris1.raw"];

    loop {
        for image in images.iter() {
            if let Ok(mut f) =
                cont.open_file_in_dir(&mut volume, &dir, image, embedded_sdmmc::Mode::ReadOnly)
            {
                let _ = cont.read(&volume, &mut f, &mut scratch);

                let raw_image: ImageRawLE<Rgb565> = ImageRaw::new(&scratch, 86, 64);
                let ferris: Image<_, Rgb565> = Image::new(&raw_image, Point::new(32, 32));

                let _ = ferris.draw(&mut display);

                cont.close_file(&volume, f).ok();
            } else {
                red_led.set_high().ok();
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
