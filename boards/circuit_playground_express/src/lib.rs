#![no_std]

extern crate atsamd_hal as hal;

use hal::prelude::*;
use hal::*;

pub use hal::common::*;
pub use hal::samd21::*;
pub use hal::target_device as pac;

use gpio::{Floating, Input, Output, Port, PushPull};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster5, PadPin, SPIMaster3};
use hal::time::Hertz;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,
    /// Pin 0, rx. Also analog input (A6)
    pin rx = b9,
    /// Pin 1, tx. Also analog input (A7)
    pin tx = b8,
    /// Pin 4, button A.
    pin d4 = a28,
    /// Pin 5, button B.
    pin d5 = a14,
    /// Pin 7, slide switch.
    pin d7 = a15,
    /// Pin 11, speaker enable.
    pin d11 = a30,
    /// Digital pin number 13, which is also attached to the red LED. PWM capable.
    pin d13 = a17,
    /// The I2C SDA. Also D2 and A5.
    pin sda = b2,
    /// The I2C SCL. Also D3 and A4
    pin scl = b3,

    /// The data line attached to the neopixel. Also D8.
    pin neopixel = b23,

    /// The line attached to the speaker. Also D12 and A0.
    pin speaker = a2,

    /// The SPI SCK. Also D6 and A1
    pin sck = a5,
    /// The SPI MOSI. Also D10 and A3
    pin mosi = a7,
    /// The SPI MISO. Also D9 and A2
    pin miso = a6,

    /// The SCK pin attached to the on-board SPI flash
    pin flash_sck = a21,
    /// The MOSI pin attached to the on-board SPI flash
    pin flash_mosi = a20,
    /// The MISO pin attached to the on-board SPI flash
    pin flash_miso = a16,
    /// The CS pin attached to the on-board SPI flash
    pin flash_cs = b22,

    pin accel_sda = a0,
    pin accel_scl = a1,
);

/// Convenience for accessing the on-board SPI Flash device.
/// This powers up SERCOM5 and configures it for use as an
/// SPI Master.
pub fn flash_spi_master(
    clocks: &mut GenericClockController,
    sercom3: pac::SERCOM3,
    pm: &mut pac::PM,
    sck: gpio::Pa21<Input<Floating>>,
    mosi: gpio::Pa20<Input<Floating>>,
    miso: gpio::Pa16<Input<Floating>>,
    cs: gpio::Pb22<Input<Floating>>,
    port: &mut Port,
) -> (
    SPIMaster3<
        hal::sercom::Sercom3Pad0<gpio::Pa16<gpio::PfD>>,
        hal::sercom::Sercom3Pad2<gpio::Pa20<gpio::PfD>>,
        hal::sercom::Sercom3Pad3<gpio::Pa21<gpio::PfD>>,
    >,
    gpio::Pb22<Output<PushPull>>,
) {
    let gclk0 = clocks.gclk0();
    let flash = SPIMaster3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        48_000_000.Hz(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom3,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    );

    let mut cs = cs.into_push_pull_output(port);

    // We’re confident that set_high won’t error here because on-board
    // GPIO pins don’t error.
    cs.set_high().unwrap();

    (flash, cs)
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom5: pac::SERCOM5,
    pm: &mut pac::PM,
    sda: gpio::Pb2<Input<Floating>>,
    scl: gpio::Pb3<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster5<
    hal::sercom::Sercom5Pad0<gpio::Pb2<gpio::PfD>>,
    hal::sercom::Sercom5Pad1<gpio::Pb3<gpio::PfD>>,
> {
    let gclk0 = clocks.gclk0();
    I2CMaster5::new(
        &clocks.sercom5_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom5,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}
