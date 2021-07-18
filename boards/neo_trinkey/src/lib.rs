#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use embedded_hal as ehal;
pub use hal::pac;

// use hal::clock::GenericClockController;
// use hal::sercom::v2::{spi, Sercom4};
// use hal::sercom::{I2CMaster3, UART0};
// use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_pins!(
    PA03 {
        name: touch1,
        aliases: {
            AlternateB: Touch1
        }
    },

    PA05 {
        name: neo_pixel,
        aliases: {
            AlternateB: NeoPixel
        }
    },

    PA07 {
        name: touch2,
        aliases: {
            AlternateB: Touch2
        }
    },
);

