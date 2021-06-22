#![no_std]
#![no_main]

/// Demonstrates reading and writing to the onboard 4MB flash.
/// The entire chip is erased, some data written, and then read back.
/// The Blue LED blink incessantly if the data written + read back
/// was not the same.
use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::qspi::{self, Command};
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins, Sets};

use core::fmt::Write;
use eg::mono_font::{ascii::FONT_6X12, MonoTextStyle};
use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::{PrimitiveStyleBuilder, Rectangle};
use eg::text::Text;
use heapless::consts::U256;
use heapless::String;

use bitfield::bitfield;

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
    let mut sets: Sets = Pins::new(peripherals.PORT).split();

    // Set up the display so we can log our progress.
    let (display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            24.mhz(),
            &mut delay,
        )
        .unwrap();
    let mut terminal = Terminal::new(display);
    let mut textbuffer = String::<U256>::new();

    let mut user_led = sets.user_led.into_open_drain_output(&mut sets.port);
    user_led.set_high().unwrap();

    let mut flash = sets
        .flash
        .init(&mut peripherals.MCLK, &mut sets.port, peripherals.QSPI);

    // We don't know the current state of the chip, so lets chill out and
    // reset it.
    delay.delay_ms(15u8);
    wait_ready(&mut flash); // Wait for any pending writes.
    flash.run_command(Command::EnableReset).unwrap();
    flash.run_command(Command::Reset).unwrap();
    delay.delay_ms(15u8);

    // 120MHz / (3-1) = 60mhz
    // w25q can do frequencies of up to 80MHz
    flash.set_clk_divider(3);

    // Enable Quad SPI mode.
    if !Status2(flash_status(&mut flash, Command::ReadStatus2)).quad_en() {
        wait_ready(&mut flash);
        flash.run_command(Command::WriteEnable).unwrap();
        flash
            .write_command(Command::WriteStatus, &[0x00, 0x02])
            .unwrap();
    }

    // Erase the entire chip.
    wait_ready(&mut flash);
    flash.run_command(Command::WriteEnable).unwrap();
    terminal.write_str("erasing chip, please wait...");
    flash.erase_command(Command::EraseChip, 0x0).unwrap();
    wait_ready(&mut flash);
    terminal.write_str("DONE.\n");

    // Print out the first 4 bytes.
    let mut read_buf = [0u8; 4];
    flash.read_memory(0, &mut read_buf);
    writeln!(textbuffer, "post-erase read value: {:?}\n", read_buf).unwrap();
    terminal.write_str(textbuffer.as_str());
    textbuffer.truncate(0);

    // Write out 4 bytes to address 0.
    let write_buf = [0x0, 0xff, 0xaa, 0x11];
    wait_ready(&mut flash);
    flash.run_command(Command::WriteEnable).unwrap();
    flash.write_memory(0, &write_buf);
    writeln!(textbuffer, "Wrote {:?} to address 0.\n", write_buf).unwrap();
    terminal.write_str(textbuffer.as_str());
    textbuffer.truncate(0);

    // Read out the first 4 bytes of address 0.
    let mut read_buf = [0u8; 4];
    wait_ready(&mut flash);
    flash.read_memory(0, &mut read_buf);
    writeln!(textbuffer, "post-write read value: {:?}\n", read_buf).unwrap();
    terminal.write_str(textbuffer.as_str());
    textbuffer.truncate(0);

    if read_buf != write_buf {
        // If we did not read back the same data flash the status
        // LED.
        loop {
            user_led.toggle();
            delay.delay_ms(200u8);
        }
    }

    // So far, everything has worked fine in OneShot mode. Lets try XIP mode.
    //
    // Write out 4 bytes to address 0x800.
    let write_buf = [0x1, 0xaa, 0xce, 0x4];
    wait_ready(&mut flash);
    flash.run_command(Command::WriteEnable).unwrap();
    flash.write_memory(0x800, &write_buf);
    writeln!(textbuffer, "Wrote {:?} to address 0x800.\n", write_buf).unwrap();
    terminal.write_str(textbuffer.as_str());
    textbuffer.truncate(0);

    // Switch to XIP mode and read those 4 bytes natively.
    let flash = flash.into_xip();
    let mut read_buf = [0u8; 4];
    unsafe {
        core::ptr::copy(
            (0x04000000 + 0x800) as *mut u8,
            read_buf.as_mut_ptr(),
            read_buf.len(),
        );
    }
    writeln!(textbuffer, "XIP read value: {:?}\n", read_buf).unwrap();
    terminal.write_str(textbuffer.as_str());
    textbuffer.truncate(0);

    if read_buf != write_buf {
        // If we did not read back the same data flash the status
        // LED.
        loop {
            user_led.toggle();
            delay.delay_ms(200u8);
        }
    }

    // Switch back to OneShot mode and make sure we read the same.
    let mut flash = flash.into_oneshot();
    let mut read_buf = [0u8; 4];
    wait_ready(&mut flash);
    flash.read_memory(0x800, &mut read_buf);
    writeln!(textbuffer, "post-XIP read value: {:?}\n", read_buf).unwrap();
    terminal.write_str(textbuffer.as_str());
    textbuffer.truncate(0);

    if read_buf != write_buf {
        // If we did not read back the same data flash the status
        // LED.
        loop {
            user_led.toggle();
            delay.delay_ms(200u8);
        }
    }
    user_led.set_low().unwrap();
    loop {}
}

/// Handly helper for logging text to the screen.
struct Terminal<'a> {
    text_style: MonoTextStyle<'a, Rgb565>,
    cursor: Point,
    display: wio::LCD,
}

impl<'a> Terminal<'a> {
    pub fn new(mut display: wio::LCD) -> Self {
        // Clear the screen.
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build();
        let backdrop =
            Rectangle::with_corners(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
        backdrop.draw(&mut display).ok().unwrap();

        Self {
            text_style: MonoTextStyle::new(&FONT_6X12, Rgb565::WHITE),
            cursor: Point::new(0, 0),
            display,
        }
    }

    pub fn write_str(&mut self, str: &str) {
        for character in str.chars() {
            self.write_character(character);
        }
    }

    pub fn write_character(&mut self, c: char) {
        if self.cursor.x >= 320 || c == '\n' {
            self.cursor = Point::new(0, self.cursor.y + FONT_6X12.character_size.height as i32);
        }
        if self.cursor.y >= 240 {
            // Clear the screen.
            let style = PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build();
            let backdrop =
                Rectangle::with_corners(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
            backdrop.draw(&mut self.display).ok().unwrap();
            self.cursor = Point::new(0, 0);
        }

        if c != '\n' {
            let mut buf = [0u8; 8];
            Text::new(c.encode_utf8(&mut buf), self.cursor, self.text_style)
                .draw(&mut self.display)
                .ok()
                .unwrap();

            self.cursor.x += (FONT_6X12.character_size.width + FONT_6X12.character_spacing) as i32;
        }
    }
}

bitfield! {
    /// Status register 1.
    struct Status1(u8);
    impl Debug;
    pub busy, _: 0;
    pub write_en, _: 1;
    pub block_protect, _: 4, 2;
    pub tb_protect, _: 5;
    pub sector_block_protect, _: 6;
    pub srp, _ : 7;
}

bitfield! {
    /// Status register 2.
    struct Status2(u8);
    impl Debug;
    pub sr_lock, _: 0;
    pub quad_en, _: 1;
    pub lock_bits, _: 5, 3;
    pub cmp_protect, _: 6;
    pub suspend, _: 7;
}

/// Wait for the write-in-progress and suspended write/erase.
fn wait_ready(flash: &mut qspi::Qspi<qspi::OneShot>) {
    while Status1(flash_status(flash, Command::ReadStatus)).busy() {}
    while Status2(flash_status(flash, Command::ReadStatus2)).suspend() {}
}

/// Returns the contents of the status register indicated by cmd.
fn flash_status(flash: &mut qspi::Qspi<qspi::OneShot>, cmd: Command) -> u8 {
    let mut out = [0u8; 1];
    flash.read_command(cmd, &mut out).ok().unwrap();
    out[0]
}
