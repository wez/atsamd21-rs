#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use embedded_hal as ehal;
pub use hal::pac;

use hal::clock::GenericClockController;
use hal::sercom::{
    v2::{
        spi,
        uart::{self, BaudMode, Oversampling},
        Sercom0, Sercom4,
    },
    I2CMaster3,
};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_pins!(
    PA03 {
        /// AREF pin - has 1uF capacitor to ground
        name: aref
    }
    PA02 {
        /// Analog pin 0.
        /// Can act as a true analog output as it has a DAC (which is not
        /// currently supported by this hal) as well as input.
        name: a0,
        aliases: {
            AlternateB: A0
        }
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
        /// Pin 0, rx
        name: d0
        aliases: {
            AlternateC: UartRx
        }
    }
    PA10 {
        /// Pin 1, tx
        name: d1
        aliases: {
            AlternateC: UartTx
        }
    }
    PA15 {
        /// Pin 5, PWM capable
        name: d5
        aliases: {
            AlternateE: D5Pwm
        }
    }
    PA20 {
        /// Pin 6, PWM capable
        name: d6
    }
    PA07 {
        /// Pin 9, PWM capable.  Also analog input (A7)
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
        /// Pin 13, which is also attached to the red LED. PWM capable.
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

    PB11 {
        /// The SPI SCLK
        name: sclk
        aliases: {
            AlternateD: Sclk
        }
    }
    PB10 {
        /// The SPI MOSI
        name: mosi
        aliases: {
            AlternateD: Mosi
        }
    }
    PA12 {
        /// The SPI MISO
        name: miso
        aliases: {
            AlternateD: Miso
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

    #[cfg(all(feature = "rfm", not(feature = "express")))]
    PA06 {
        /// SPI chip select for the RFM module
        name: rfm_cs
    }
    #[cfg(all(feature = "rfm", not(feature = "express"), not(feature = "adalogger")))]
    PA08 {
        /// Reset for the RFM module
        name: rfm_reset
    }
    #[cfg(all(feature = "rfm", not(feature = "express")))]
    PA09 {
        /// Interrupt from the RFM module
        name: rfm_irq
    }

    #[cfg(all(feature = "express", not(feature = "rfm")))]
    PA06 {
        /// Neopixel data
        name: neopixel
    }

    #[cfg(all(feature = "express", not(feature = "rfm")))]
    PA09 {
        /// SPI clock for the external flash
        name: flash_sclk
    }
    #[cfg(all(feature = "express", not(feature = "rfm"), not(feature = "adalogger")))]
    PA08 {
        /// SPI MOSI for the external flash
        name: flash_mosi
    }
    #[cfg(feature = "express")]
    PA14 {
        /// SPI MISO for the external flash
        name: flash_miso
    }
    #[cfg(feature = "express")]
    PA13 {
        /// SPI chip select for the external flash
        name: flash_cs
    }

    #[cfg(all(feature = "adalogger", not(feature = "rfm"), not(feature = "express")))]
    PA08 {
        /// SD card SPI chip select
        name: sd_cs
    },
    PA21 {
        /// SD card detect
        name: sd_cd
    },
);

/// SPI pads for the labelled SPI peripheral
///
/// You can use these pads with other, user-defined [`spi::Config`]urations.
pub type SpiPads = spi::Pads<Sercom4, Miso, Mosi, Sclk>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>>;

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM4 and configures it for use as an
/// SPI Master in SPI Mode 0.
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

pub type UartPads = uart::Pads<Sercom0, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

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
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(pm, sercom0, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
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
