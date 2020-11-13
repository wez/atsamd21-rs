#![no_std]
#![no_main]

/// Makes the wio_terminal read the SD card and print the filenames
/// of the first few entries.
use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use eg::fonts::{Font12x16, Text};
use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::rectangle::Rectangle;
use eg::style::{PrimitiveStyleBuilder, TextStyle};

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins, Sets};

use core::fmt::Write;
use heapless::consts::U128;
use heapless::String;

use embedded_sdmmc::{TimeSource, Timestamp, VolumeIdx};
use wio::SDCardController;

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

    let (mut cont, _sd_present) = sets
        .sd_card
        .init(
            &mut clocks,
            peripherals.SERCOM6,
            &mut peripherals.MCLK,
            &mut sets.port,
            Clock,
        )
        .unwrap();

    // Initialize the ILI9341-based LCD display. Create a black backdrop the size of
    // the screen.
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            &mut delay,
        )
        .unwrap();

    let style = TextStyle::new(Font12x16, Rgb565::WHITE);

    loop {
        match cont.device().init() {
            Ok(_) => {
                // Now that we have initialized, we can run the SPI bus at
                // a reasonable speed.
                cont.set_baud(20_000_000.Hz());

                let mut data = String::<U128>::new();
                write!(data, "OK! ").unwrap();
                match cont.device().card_size_bytes() {
                    Ok(size) => writeln!(data, "{}Mb", size / 1024 / 1024).unwrap(),
                    Err(e) => writeln!(data, "Err: {:?}", e).unwrap(),
                }
                Text::new(data.as_str(), Point::new(4, 2))
                    .into_styled(style)
                    .draw(&mut display)
                    .ok()
                    .unwrap();

                if let Err(e) = print_contents(&mut cont, &mut display) {
                    let mut data = String::<U128>::new();
                    writeln!(data, "Err: {:?}", e).unwrap();
                    Text::new(data.as_str(), Point::new(4, 20))
                        .into_styled(style)
                        .draw(&mut display)
                        .ok()
                        .unwrap();
                }
            }
            Err(e) => {
                let mut data = String::<U128>::new();
                writeln!(data, "Error!: {:?}", e).unwrap();
                Text::new(data.as_str(), Point::new(4, 2))
                    .into_styled(style)
                    .draw(&mut display)
                    .ok()
                    .unwrap();
            }
        }

        delay.delay_ms(2500 as u16);
        Rectangle::new(Point::new(0, 0), Point::new(320, 240))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(Rgb565::BLACK)
                    .build(),
            )
            .draw(&mut display)
            .ok()
            .unwrap();
    }
}

fn print_contents(
    cont: &mut SDCardController<Clock>,
    lcd: &mut wio::LCD,
) -> Result<(), embedded_sdmmc::Error<embedded_sdmmc::SdMmcError>> {
    let style = TextStyle::new(Font12x16, Rgb565::WHITE);

    let volume = cont.get_volume(VolumeIdx(0))?;
    let dir = cont.open_root_dir(&volume)?;

    let mut count = 0;
    let out = cont.iterate_dir(&volume, &dir, |ent| {
        let mut data = String::<U128>::new();
        writeln!(data, "{} - {:?}", ent.name, ent.attributes).unwrap();
        Text::new(data.as_str(), Point::new(4, 20 + count * 16))
            .into_styled(style)
            .draw(lcd)
            .ok()
            .unwrap();
        count += 1;
    });
    cont.close_dir(&volume, dir);
    out
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
