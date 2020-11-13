//! Endlessly fill the screen with characters from the alphabet, and blink the
//! red LED connected to pin13
//!
//! This example is for the _Adafruit Feather M0_ series boards connected to a
//! Adafruit OLED Featherwing, which has 128x32 pixel resolution.  I2C is used
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
//! ssd1306_terminalmode_128x32_i2c`

#![no_std]
#![no_main]

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

use ssd1306::prelude::*;
use ssd1306::Builder;

use core::fmt::Write;

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
    let mut disp: TerminalMode<_> = Builder::new()
        .size(DisplaySize::Display128x32)
        .connect_i2c(i2c)
        .into();

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
