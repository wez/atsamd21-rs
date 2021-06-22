//! Endlessly fill the screen with characters from the alphabet, and blink the
//! red LED connected to pin13
//!
//! This example is for the _Adafruit Feather M0_ series boards connected to a
//! generic SSD1306 display module with 128x64 pixel resolution, that uses I2C
//! to communicate between the processor board and the display module.
//!
//! This example is based on:
//! - https://github.com/jamwaffles/ssd1306/blob/master/examples/terminal_i2c.rs
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
//! Adafruit sells a SSD1306 display module called a "Featherwing"
//! (https://www.adafruit.com/product/2900), which can plug in to other
//! Feather boards if the correct headers are used, or can be plugged directly
//! into a breadboard and connected with wires.  They also sell other I2C and
//! SPI "modules" that have the SSD1306 chipset and a display of some sort,
//! which you then connect to the Feather M0 with wires.  Even more SSD1306
//! modules can also be found on various shopping websites (Amazon, Ebay, //!
//! Ali*), although it may be hard to tell from item listings if the module is
//! using an SSD1306 chipset or not.
//!
//! Note that most SSD1306-backed display modules are usually either 128x64
//! pixels or 128x32 pixels in size, but there are other sizes running around
//! in the wild.
//!
//! The default display size for the Rust `ssd1306` library (if no size is
//! passed in to the constructor as a parameter) is 128x64 pixels;
//!
//! https://jamwaffles.github.io/ssd1306/master/ssd1306/builder/struct.Builder.html#method.new
//!
//! Wiring connections for the Adafruit OLED Featherwing:
//!
//! ```
//! OLED Featherwing -> Feather M0
//!     GND -> GND
//!      3v -> 3v3
//! GPIOSDA -> PA22 (SDA)
//! GPIOSCL -> PA23 (SCL)
//! ```
//!
//! Wiring connections for generic OLED modules:
//!
//! ```
//!      OLED module -> Feather M0
//! (black)  GND -> GND
//! (red)    +3V -> VCC
//! (yellow) SDA -> PA22 (SDA)
//! (green)  SCL -> PA23 (SCL)
//! ```
//! Build this example with: `cargo build --example
//! ssd1306_terminalmode_128x64_i2c`

#![no_std]
#![no_main]

use core::fmt::Write;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use bsp::hal;
use bsp::pac;
use feather_m0 as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::time::KiloHertz;
use pac::{CorePeripherals, Peripherals};

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
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led: bsp::RedLed = pins.d13.into();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let i2c = bsp::i2c_master(
        &mut clocks,
        KiloHertz(400),
        peripherals.SERCOM3,
        &mut peripherals.PM,
        pins.sda,
        pins.scl,
    );

    // NOTE the `DisplaySize` enum comes from the ssd1306 package,
    // and currently only supports certain display sizes; see
    // https://jamwaffles.github.io/ssd1306/master/ssd1306/prelude/enum.DisplaySize.html
    // - Display128x64 is the default, just being explicit here
    let interface = I2CDisplayInterface::new(i2c);
    let mut disp =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();

    disp.init().unwrap();
    let _ = disp.clear();

    /* Endless loop */
    loop {
        red_led.set_high().unwrap();
        for c in 97..123 {
            let _ = disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
            delay.delay_ms(20u8);
        }
        red_led.set_low().unwrap();
        for c in 65..91 {
            let _ = disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
            delay.delay_ms(20u8);
        }
    }
}
