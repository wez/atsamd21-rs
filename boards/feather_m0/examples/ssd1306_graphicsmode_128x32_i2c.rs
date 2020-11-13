#![no_std]
#![no_main]

//! Draw a square, circle and triangle on an SSD1306-based Adafruit OLED
//! Featherwing display using the `embedded_graphics` crate.
//!
//! This example is for the _Adafruit Feather M0_ series boards connected to a
//! Adafruit OLED Featherwing, which has 128x32 pixel resolution.  I2C is used
//! to communicate between the processor board and the display module.
//!
//! This example is based on:
//! - https://github.com/jamwaffles/ssd1306/blob/master/examples/graphics_i2c_128x32.rs
//! - https://github.com/atsamd-rs/atsamd/blob/master/boards/itsybitsy_m0/examples/i2c_ssd1306.rs
//!
//! You can either use the USB port on the Feather + BOSSAC or a SWD
//! debugger/programmer to load this example onto the Feather M0 board.
//!
//! Adafruit Feather M0 Boards:
//! - Feather M0 Express: https://www.adafruit.com/product/3403
//! - Feather M0 Adalogger: https://www.adafruit.com/product/2796
//! - Feather M0 Basic: https://www.adafruit.com/product/2772
//!
//! Adafruit OLED Featherwing:
//! - https://www.adafruit.com/product/2900
//!
//! Other generic SSD1306 modules will work with this demo as well.
//!
//! Wiring connections for the Adafruit OLED Featherwing:
//!
//! ```
//! OLED Featherwing -> Feather M0
//!     GND -> GND
//!     3v  -> 3v3
//! GPIOSDA -> PA22 (SDA)
//! GPIOSCL -> PA23 (SCL)
//! ```
//!
//! Build this example with: `cargo build --example
//! ssd1306_graphicsmode_128x32_i2c`

extern crate embedded_graphics;
extern crate feather_m0 as hal;
extern crate ssd1306;

// how to panic...
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

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

    let i2c = hal::i2c_master(
        &mut clocks,
        400_000.Hz(),
        peripherals.SERCOM3,
        &mut peripherals.PM,
        pins.sda,
        pins.scl,
        &mut pins.port,
    );

    // NOTE the `DisplaySize` enum comes from the ssd1306 package,
    // and currently only supports certain display sizes; see
    // https://jamwaffles.github.io/ssd1306/master/ssd1306/prelude/enum.DisplaySize.html
    let mut disp: GraphicsMode<_> = Builder::new()
        .size(DisplaySize::Display128x32)
        .connect_i2c(i2c)
        .into();

    disp.init().unwrap();
    disp.flush().unwrap();

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    let yoffset = 8;
    let x_max = 127;
    let y_max = 31;

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
    Rectangle::new(Point::new(58, yoffset), Point::new(58 + 16, 16 + yoffset))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // circle
    Circle::new(Point::new(108, yoffset + 8), 8)
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    // blink the onboard blinkenlight (digital pin 13)
    loop {
        delay.delay_ms(200u8);
        red_led.set_high().unwrap();
        delay.delay_ms(200u8);
        red_led.set_low().unwrap();
    }
}
