//! Demonstrates reading and writing to the onboard GD25Q64C 8MiB flash.
//! The entire chip is erased, some data written, and then read back.
//! The Red LED blink incessantly if the data written + read back
//! was not the same.
//!
//! | device  | block   | sector   | page    | unit    |
//! |---------|---------|----------|---------|---------|
//! | 8388608 | 64/32K  | 4096     | 256     | bytes   |
//! | 32768   | 256/128 | 16       | -       | pages   |
//! | 2048    | 16/8    | -        | -       | sectors |
//! | 128/256 | -       | -        | -       | blocks  |
//! http://www.gigadevice.com/datasheet/gd25q64c/
//!
//! Note:: Write Enable bit will return to reset by the following situation:
//! -Power-Up
//! -Write Disable (WRDI)
//! -Write Status Register (WRSR)
//! -Page Program (PP)
//! -Sector Erase (SE)/ Block Erase (BE) / Chip Erase (CE)
//! -Software reset (66H+99H)

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::qspi::{self, Command};
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

    let sets = Pins::new(peripherals.PORT).split();

    let mut flash = sets.flash.init(&mut peripherals.MCLK, peripherals.QSPI);

    // Startup delay. Can't find documented but Adafruit use 5ms
    delay.delay_ms(5u8);
    // Reset. It is recommended to check the BUSY(WIP?) bit and the SUS before reset
    wait_ready(&mut flash);
    flash.run_command(Command::EnableReset).unwrap();
    flash.run_command(Command::Reset).unwrap();
    // tRST(30μs) to reset. During this period, no command will be accepted
    delay.delay_ms(1u8);

    // Check for GD25Q64C JEDEC ID
    let mut read_buf = [0u8; 3];
    flash.read_command(Command::ReadId, &mut read_buf).unwrap();
    assert_eq!(read_buf, [0x17, 0x40, 0xc8]);

    // 120MHz / 2 = 60mhz
    // faster than 104mhz at 3.3v would require High Performance Mode
    flash.set_clk_divider(2);

    // Enable Quad SPI mode. Requires write enable. Check WIP.
    flash.run_command(Command::WriteEnable).unwrap();
    flash.write_command(Command::WriteStatus2, &[0x02]).unwrap();
    wait_ready(&mut flash);

    // Chip Erase. Requires write enable. Check WIP.
    flash.run_command(Command::WriteEnable).unwrap();
    flash.erase_command(Command::EraseChip, 0x0).unwrap();
    // Worst case up to 140 seconds!
    wait_ready(&mut flash);

    // Page Program. Requires write enable. Check WIP.
    // If more than 256 bytes are sent to the device, previously latched data
    // are discarded and the last 256 data bytes are guaranteed to be
    // programmed correctly within the same page. If less than 256 data
    // bytes are sent to device, they are correctly programmed at the
    // requested addresses without having any effects on the other bytes of
    // the same page

    let write_buf = [0x0d, 0xd0, 0x01, 0xc0];
    flash.run_command(Command::WriteEnable).unwrap();
    flash.write_memory(0, &write_buf);
    wait_ready(&mut flash);

    // Read back data
    // datasheet claims 6BH needs a single dummy byte, but doesnt work then
    // adafruit uses 8, and the underlying implementation uses 8 atm as well
    let mut read_buf = [0u8; 4];
    flash.read_memory(0, &mut read_buf);
    assert_eq!(read_buf, write_buf);

    loop {}
}

/// Wait for the write-in-progress and suspended write/erase.
fn wait_ready(flash: &mut qspi::Qspi<qspi::OneShot>) {
    while flash_status(flash, Command::ReadStatus) & 0x01 != 0 {}
    while flash_status(flash, Command::ReadStatus2) & 0x80 != 0 {}
}

/// Returns the contents of the status register indicated by cmd.
fn flash_status(flash: &mut qspi::Qspi<qspi::OneShot>, cmd: Command) -> u8 {
    let mut out = [0u8; 1];
    flash.read_command(cmd, &mut out).ok().unwrap();
    out[0]
}
