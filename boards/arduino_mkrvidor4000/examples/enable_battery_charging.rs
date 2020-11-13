#![no_std]
#![no_main]

extern crate arduino_mkrvidor4000 as hal;

use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::pad::PadPin;
use hal::prelude::*;
use hal::sercom::I2CMaster0;

// https://www.ti.com/lit/ds/symlink/bq24195l.pdf
const PMIC_ADDRESS: u8 = 0x6B;
const PMIC_POWER_ON_CONFIGURATION_REGISTER: u8 = 0x01;
// Configure for Charge Battery + Minimum System Voltage Limit: 3.5V
const PMIC_POWER_ON_CONFIGURATION: u8 = 0b00011011;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let _core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut _led = pins.led_builtin.into_open_drain_output(&mut pins.port);
    let gclk0 = clocks.gclk0();

    let mut i2c: I2CMaster0<
        hal::sercom::Sercom0Pad0<hal::gpio::Pa8<hal::gpio::PfC>>,
        hal::sercom::Sercom0Pad1<hal::gpio::Pa9<hal::gpio::PfC>>,
    > = I2CMaster0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        100_000.Hz(),
        peripherals.SERCOM0,
        &mut peripherals.PM,
        // Arduino MKR Vidor 4000 has I2C on pins PA08, PA09
        pins.sda.into_pad(&mut pins.port),
        pins.scl.into_pad(&mut pins.port),
    );

    i2c.write(
        PMIC_ADDRESS,
        &[
            PMIC_POWER_ON_CONFIGURATION_REGISTER,
            PMIC_POWER_ON_CONFIGURATION,
        ],
    )
    .unwrap();
    loop {}
}
