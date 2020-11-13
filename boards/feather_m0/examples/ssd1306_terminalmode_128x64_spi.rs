//! Endlessly fill the screen with characters from the alphabet, and blink the
//! red LED connected to pin13.
//!
//! This example is for the _Adafruit Feather M0_ series boards connected to a
//! generic SSD1306 display module with 128x64 pixel resolution, that uses SPI
//! to communicate between the processor board and the display module.
//!
//! This example is based on:
//! - https://github.com/jamwaffles/ssd1306/blob/master/examples/terminal_i2c.rs
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
//! Adafruit SPI OLED module that was used for this demo:
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
//! ssd1306_terminalmode_128x64_spi`

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

    // NOTE the `DisplaySize` enum comes from the ssd1306 package,
    // and currently only supports certain display sizes; see
    // https://jamwaffles.github.io/ssd1306/master/ssd1306/prelude/enum.DisplaySize.html
    // - Display128x64 is the default, just being explicit here
    let mut disp: TerminalMode<_> = Builder::new()
        .size(DisplaySize::Display128x64)
        .connect_spi(spi, dc)
        .into();

    disp.reset(&mut rst, &mut delay).unwrap();
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
