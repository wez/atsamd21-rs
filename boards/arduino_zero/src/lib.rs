#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

#[cfg(feature = "panic_halt")]
pub extern crate panic_halt;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

use hal::prelude::*;
use hal::*;

pub use hal::common::*;
pub use hal::samd21::*;
pub use hal::target_device as pac;

use gpio::{Floating, Input, PfC, Port};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin, SPIMaster1, UART0};
use hal::time::Hertz;

// The docs could be further improved with details of the specific channels etc
define_pins!(
    /// Maps the pins to their arduino names and the numbers printed on the board.
    /// Information from: <https://github.com/arduino/ArduinoCore-samd/blob/master/variants/mkrzero/variant.cpp>
    struct Pins,
    target_device: target_device,

    /// Digital 2: TC
    pin d2 = a14,

    /// Digital 3: PWM, TCC
    pin d3 = a9,

    /// Digital 4: PWM
    pin d4 = a8,

    /// Digital 5: PWM
    pin d5 = a15,

    /// Digital 6: PWM
    pin d6 = a20,

    /// Digital 7: PWM, TCC
    pin d7 = a21,

    /// Digital 8: PWM
    pin d8 = a6,

    /// Digital 9: PWM
    pin d9 = a7,

    /// Digital 9: PWM
    pin d10 = a18,

    /// SPI MOSI: PWM, TCC
    pin mosi = a16,

    /// Digital 13/LED/SPI SCK: ON-BOARD-LED
    pin led_sck = a17,

    /// SPI MISO: PWM, TC
    pin miso = a19,

    /// SDA
    pin sda = a22,

    /// SCL
    pin scl = a23,

    /// RX
    pin rx = a11,

    /// TX
    pin tx = a10,

    /// Analog 0: DAC
    pin a0 = a2,

    /// Analog 1
    pin a1 = b8,

    /// Analog 2
    pin a2 = b9,

    /// Analog 3: PWM, TCC
    pin a3 = a4,

    /// Analog 4: PWM, TCC
    pin a4 = a5,

    /// Analog 5
    pin a5 = b2,

    pin usb_n = a24,
    pin usb_p = a25,
    pin usb_id = a28,
    pin aref = a3,

    pin tx_led = a27,
    pin rx_led = b3,

    pin xin32 = a0,
    pin xout32 = a1,
);

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: gpio::Pa24<Input<Floating>>,
    dp: gpio::Pa25<Input<Floating>>,
    port: &mut Port,
) -> UsbBusAllocator<UsbBus> {
    use gpio::IntoFunction;

    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    UsbBusAllocator::new(UsbBus::new(
        usb_clock,
        pm,
        dm.into_function(port),
        dp.into_function(port),
        usb,
    ))
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom3: pac::SERCOM3,
    pm: &mut pac::PM,
    sda: gpio::Pa22<Input<Floating>>,
    scl: gpio::Pa23<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster3<hal::sercom::Sercom3Pad0<gpio::Pa22<PfC>>, hal::sercom::Sercom3Pad1<gpio::Pa23<PfC>>>
{
    let gclk0 = clocks.gclk0();
    I2CMaster3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom3,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    rx: gpio::Pa11<Input<Floating>>,
    tx: gpio::Pa10<Input<Floating>>,
    port: &mut Port,
) -> UART0<
    hal::sercom::Sercom0Pad3<gpio::Pa11<PfC>>,
    hal::sercom::Sercom0Pad2<gpio::Pa10<PfC>>,
    (),
    (),
> {
    let gclk0 = clocks.gclk0();

    UART0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        baud.into(),
        sercom0,
        pm,
        (rx.into_pad(port), tx.into_pad(port)),
    )
}

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM4 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom1: pac::SERCOM1,
    pm: &mut pac::PM,
    sck: gpio::Pa17<Input<Floating>>,
    mosi: gpio::Pa16<Input<Floating>>,
    miso: gpio::Pa19<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster1<
    hal::sercom::Sercom1Pad3<gpio::Pa19<PfC>>,
    hal::sercom::Sercom1Pad0<gpio::Pa16<PfC>>,
    hal::sercom::Sercom1Pad1<gpio::Pa17<PfC>>,
> {
    let gclk0 = clocks.gclk0();

    SPIMaster1::new(
        &clocks.sercom1_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom1,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}
