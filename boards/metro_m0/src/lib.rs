#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use embedded_hal as ehal;
pub use hal::pac;

use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::sercom::v2::{spi, Sercom4, Sercom5};
use hal::sercom::{I2CMaster3, UART0};
use hal::time::{Hertz, MegaHertz};

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_pins!(
    PA02 {
        /// Analog pin 0.  Can act as a true analog output
        /// as it has a DAC (which is not currently supported
        /// by this hal) as well as input.
        name: a0
    }
    PB08 {
        /// Analog Pin 1
        name: a1
    }
    PB09 {
        /// Analog Pin 2
        name: a2
    }
    PA04 {
        /// Analog Pin 3
        name: a3
    }
    PA05 {
        /// Analog Pin 4
        name: a4
    }
    PB02 {
        /// Analog Pin 5
        name: a5
    }
    PA11 {
        /// Pin 0, UART RX.  Also analog input (A6)
        name: d0
        aliases: {
            AlternateC: UartRx
        }
    }
    PA10 {
        /// Pin 1, UART TX.  Also analog input (A7)
        name: d1
        aliases: {
            AlternateC: UartTx
        }
    }
    PA14 {
        /// Pin 2
        name: d2
    }
    PA09 {
        /// Pin 3, PWM capable
        name: d3
    }
    PA08 {
        /// Pin 4, PWM capable.  Also analog input (A8)
        name: d4
    }
    PA15 {
        /// Pin 5, PWM capable.  Also analog input (A9)
        name: d5
    }
    PA20 {
        /// Pin 6, PWM capable
        name: d6
    }
    PA21 {
        /// Pin 7
        name: d7
    }
    PA06 {
        /// Pin 8, PWM capable.  Also analog input (A10)
        name: d8
    }
    PA07 {
        /// Pin 9, PWM capable.  Also analog input (A11)
        name: d9
    }
    PA18 {
        /// Pin 10, PWM capable
        name: d10
    }
    PA16 {
        /// Pin 11, PWM capable
        name: d11
    }
    PA19 {
        /// Pin 12, PWM capable
        name: d12
    }
    PA17 {
        /// Digital pin number 13, which is also attached to
        /// the red LED.  PWM capable.
        name: d13
        aliases: {
            PushPullOutput: RedLed
        }
    }
    PA22 {
        /// The I2C data line
        name: sda
        aliases: {
            AlternateC: Sda
        }
    }
    PA23 {
        /// The I2C clock line
        name: scl
        aliases: {
            AlternateC: Scl
        }
    }
    PA30 {
        /// The data line attached to the neopixel.
        /// Is also attached to SWCLK.
        name: neopixel
    }
    PB11 {
        /// The SPI SCK attached the to 2x3 header
        name: sck
        aliases: {
            AlternateD: Sclk
        }
    }
    PB10 {
        /// The SPI MOSI attached the to 2x3 header
        name: mosi
        aliases: {
            AlternateD: Mosi
        }
    }
    PA12 {
        /// The SPI MISO attached the to 2x3 header
        name: miso
        aliases: {
            AlternateD: Miso
        }
    }
    PB23 {
        /// The SCK pin attached to the on-board SPI flash
        name: flash_sck
        aliases: {
            AlternateD: FlashSclk
        }
    }
    PB22 {
        /// The MOSI pin attached to the on-board SPI flash
        name: flash_mosi
        aliases: {
            AlternateD: FlashMosi
        }
    }
    PB03 {
        /// The MISO pin attached to the on-board SPI flash
        name: flash_miso
        aliases: {
            AlternateD: FlashMiso
        }
    }
    PA13 {
        /// The CS pin attached to the on-board SPI flash
        name: flash_cs
        aliases: {
            PushPullOutput: FlashCs
        }
    }
    PA24 {
        /// The USB D- pad
        name: usb_dm
        aliases: {
            AlternateG: UsbDm
        }
    }
    PA25 {
        /// The USB D+ pad
        name: usb_dp
        aliases: {
            AlternateG: UsbDp
        }
    }
);

/// SPI pads for the labelled SPI peripheral
///
/// You can use these pads with other, user-defined [`spi::Config`]urations.
pub type SpiPads = spi::Pads<Sercom4, Miso, Mosi, Sclk>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>>;

/// Convenience for setting up the 2x3 header block for SPI.
/// This powers up SERCOM4 and configures it for use as an
/// SPI Master in SPI Mode 0.
/// Unlike the `flash_spi_master` function, this
/// one does not accept a CS pin; configuring a pin for CS
/// is the responsibility of the caller, because it could be
/// any OutputPin, or even a pulled up line on the slave.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom4: pac::SERCOM4,
    pm: &mut pac::PM,
    sclk: impl Into<Sclk>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom4_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sclk.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);
    spi::Config::new(pm, sercom4, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

/// SPI pads for the flash chip
pub type FlashPads = spi::Pads<Sercom5, FlashMiso, FlashMosi, FlashSclk>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type FlashSpi = (spi::Spi<spi::Config<FlashPads>>, FlashCs);

/// Convenience for accessing the on-board SPI Flash device.
/// This powers up SERCOM5 and configures it for use as an
/// SPI Master.
pub fn flash_spi_master(
    clocks: &mut GenericClockController,
    sercom5: pac::SERCOM5,
    pm: &mut pac::PM,
    sclk: impl Into<FlashSclk>,
    mosi: impl Into<FlashMosi>,
    miso: impl Into<FlashMiso>,
    cs: impl Into<FlashCs>,
) -> FlashSpi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom5_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sclk, mut cs) = (miso.into(), mosi.into(), sclk.into(), cs.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sclk);
    let spi = spi::Config::new(pm, sercom5, pads, freq)
        .baud(MegaHertz(48))
        .spi_mode(spi::MODE_0)
        .enable();

    cs.set_high().unwrap();

    (spi, cs)
}

/// I2C master for the labelled SDA & SCL pins
pub type I2C = I2CMaster3<Sda, Scl>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom3: pac::SERCOM3,
    pm: &mut pac::PM,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2C {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom3_core(&gclk0).unwrap();
    let baud = baud.into();
    let sda = sda.into();
    let scl = scl.into();
    I2CMaster3::new(clock, baud, sercom3, pm, sda, scl)
}

/// UART device for the labelled RX & TX pins
pub type Uart = UART0<UartRx, UartTx, (), ()>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    uart_rx: impl Into<UartRx>,
    uart_tx: impl Into<UartTx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom0_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = (uart_rx.into(), uart_tx.into());
    UART0::new(clock, baud, sercom0, pm, pads)
}

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(clock, pm, dm, dp, usb))
}
