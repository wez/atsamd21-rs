#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use hal::pac;

use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, UART0};
use hal::sercom::v2::{spi, Sercom4};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

// The docs could be further improved with details of the specific channels etc
atsamd_hal::bsp_pins!(
    // ---------- Left Hand Side ----------

    // A0_PA02_AIN0
    PA02 {
        name: a0,
        aliases: {
            AlternateB: AIn0
        }
    }

    // AREF
    PA03 {
        name: aref,
    }

    // A1_PB08_AIN2
    PB08 {
        name: a1
        aliases: {
            AlternateB: AIn1
        }
    },

    // A2_PB09_AIN2
    PB09 {
        name: a2
        aliases: {
            AlternateB: AIn2
        }
    },

    // A3_PB04_AIN2
    PA04 {
        name: a3
        aliases: {
            AlternateB: AIn3
        }
    },

    // A4_PB05_AIN2
    PA05 {
        name: a4
        aliases: {
            AlternateB: AIn4
        }
    },

    // D8_DGI_GPIO2/VBAT
    // ToDo: What is this?
    PA06 {
        name: vbat
    },

    // D9_DGI_GPIO3
    PA07 {
        name: d9
    },

    // D1/TX_PA10_TCC0-W2
    PA10 {
        name: d1
        aliases: {
            AlternateC: UartTx
        }
    },

    // D0/RX_PA11_TCC0-W3
    PA11 {
        name: d0
        aliases: {
            AlternateC: UartRx
        }
    },

    // D23_PB10_S4_SPI_MOSI
    PB10 {
        name: d23
        aliases: {
            AlternateD: Mosi
        }
    },

    // D24_PB11_S4_SPI_SCK
    PB11 {
        name: d24
        aliases: {
            AlternateD: Sck
        }
    },

    // D22_PA12_S4_SPI_MISO
    PA12 {
        name: d22
        aliases: {
            AlternateD: Miso
        }
    },

    // D5_PA15_TCC0-W5
    PA15 {
        name: d5
    },

    // ---------- Right Hand Side ----------

    // PB03_RX_LED
    // ToDo: What is this?
    PB03 {
        name: rx_led
    },

    // A5_PB02_AIN10
    PB02 {
        name: a5
    },

    // PA31_SWDIO
    // ToDo: Is this ever useful in software?
    PA31 {
        name: swdio
    },

    // PA30_SWCLK
    // ToDo: Is this ever useful in software?
    PA30 {
        name: swclk
    },

    // PA27_TX_LED
    // ToDo: What is this?
    PA27 {
        name: tx_led
    },

    // UART1/TX_PB12_A4 (Tx for W600)
    PB23 {
        name: w600_tx
    },

    // UART1/RX_PB11_A3 (Rx for W600)
    PB22 {
        name: w600_rx
    },

    // SAMD21_D+ (USB Data Plus)
    PA25 {
        name: usb_dp
        aliases: {
            AlternateG: UsbDp
        }
    },

    // SAMD21_D- (USB Data Minus)
    PA24 {
        name: usb_dm
        aliases: {
            AlternateG: UsbDm
        }
    },

    // D33/SCL/PA23
    PA23 {
        name: d33,
        aliases: {
            AlternateC: Scl
        }
    },

    // D32/SDA/PA22
    PA22 {
        name: d32,
        aliases: {
            AlternateC: Sda
        }
    },

    // RESET_W600
    PA21 {
        name: reset_w600
    },

    // D6_PA20_TCC0-W6
    PA20 {
        name: d6
    },

    // D12_MISO
    PA19 {
        name: d12
    },

    // D10_SS
    PA18 {
        name: d10
    },

    // D13_SCK
    PA17 {
        name: d13
    },

    // D11_MOSI
    PA16 {
        name: d11
    },
);

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

type SpiPads = spi::Pads<Sercom4, Miso, Mosi, Sck>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>>;

/// Convenience function for setting up the D24/SCK, D23/MOSI, and D22/MISO pins
/// as a SPI Master.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom4: pac::SERCOM4,
    pm: &mut pac::PM,
    sck: impl Into<Sck>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom4_core(&gclk0).unwrap();
    let freq = clock.freq();
    let pads = spi::Pads::default()
        .data_in(miso.into())
        .data_out(mosi.into())
        .sclk(sck.into());
    spi::Config::new(pm, sercom4, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

/// UART device for the labelled RX & TX pins
pub type Uart = UART0<UartRx, UartTx, (), ()>;

/// Convenience for setting up the D0 and D1 pins to
/// operate as UART RX/TX (respectively) running at the specified baud.
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
    dm: UsbDm,
    dp: UsbDp,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    UsbBusAllocator::new(UsbBus::new(usb_clock, pm, dm, dp, usb))
}
