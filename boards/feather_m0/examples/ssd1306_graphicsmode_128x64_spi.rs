//! Draw a square, circle and triangle bounded inside of a box on the screen
//! borders of an OLED using the `embedded_graphics` crate, and blink the red
//! LED connected to pin13
//!
//! This example is for the _Adafruit Feather M0_ series boards connected to
//! an SPI OLED display board based on the SSD1306 chipset which has 128x64
//! pixel resolution.
//!
//! This example is based on:
//! - https://github.com/jamwaffles/ssd1306/blob/master/examples/graphics.rs
//! - https://github.com/atsamd-rs/atsamd/blob/master/boards/itsybitsy_m0/examples/spi_ssd1306.rs
//!
//! You can either use the USB port on the Feather + BOSSAC or a SWD
//! debugger/programmer to load this example onto the Feather M0 board.
//!
//! Adafruit Feather M0 Boards:
//! - Feather M0 Express: https://www.adafruit.com/product/3403
//! - Feather M0 Adalogger: https://www.adafruit.com/product/2796
//! - Feather M0 Basic: https://www.adafruit.com/product/2772
//!
//! Adafruit OLED module that was used for this demo:
//! - https://www.adafruit.com/product/938
//!
//! All Adafruit OLED modules (SPI/I2C)
//! - https://www.adafruit.com/category/98
//!
//! Other "generic" SPI SSD1306 modules found on Amazon/Ebay/Ali* should work
//! with this example as well.
//!
//! For this demo, it's assumed that the SSD1306 display is the only device on
//! the SPI bus, so the "CS" pin of the display needs to be connected to GND
//! for the demo to work.
//!
//! Wiring connections for the Adafruit/generic OLED SSD1306 displays:
//!
//! ```
//! Feather M0  -> SPI SSD1306 OLED Display
//! ---------------------------------------
//!         GND -> GND
//!         3V3 -> VCC
//!         SCK -> Clk (Adafruit) or D0 (generic)
//!        MOSI -> Data (Adafruit) or D1 (generic)
//!    PA7 (D9) -> Rst (Adafruit) or RES (generic)
//!   PA20 (D6) -> DC
//!         GND -> CS (see note above about grounding 'CS' pin)
//! ```
//!
//! Build this example with: `cargo build --example
//! ssd1306_graphicsmode_128x64_spi`

#![no_std]
#![no_main]

extern crate feather_m0 as hal;
extern crate panic_halt;

extern crate embedded_graphics;
extern crate ssd1306;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Rectangle, Triangle};
use embedded_graphics::style::PrimitiveStyleBuilder;
use ssd1306::prelude::*;
use ssd1306::Builder;

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
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let spi = hal::spi_master(
        &mut clocks,
        10_000_000.Hz(),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.sck,
        pins.mosi,
        pins.miso,
        &mut pins.port,
    );

    let dc = pins.d6.into_open_drain_output(&mut pins.port);
    let mut rst = pins.d9.into_open_drain_output(&mut pins.port);

    // default DisplaySize: 128x64 pixels; see the
    // ssd1306::builder::Builder::new() method definition for more info
    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();

    disp.reset(&mut rst, &mut delay).unwrap();
    disp.init().unwrap();

    let yoffset = 24;
    let x_max = 127;
    let y_max = 63;

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    // screen outline
    Rectangle::new(Point::new(0, 0), Point::new(x_max, y_max))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // triangle
    // 'Triangle' requires 'embedded_graphics' 0.6 or newer...
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(style)
    .draw(&mut disp)
    .unwrap();

    // square
    Rectangle::new(Point::new(54, yoffset), Point::new(54 + 16, 16 + yoffset))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // circle
    Circle::new(Point::new(100, 8 + yoffset), 8)
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    loop {
        delay.delay_ms(400_u16);
        red_led.set_high().unwrap();
        delay.delay_ms(400_u16);
        red_led.set_low().unwrap();
    }
}
