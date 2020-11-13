/// Consolidated common logic for dealing with ATSAMD SPI peripherals.
use crate::hal::spi::{Mode, Phase, Polarity};
use embedded_time::rate::{Extensions, Hertz};

#[cfg(any(feature = "samd11", feature = "samd21"))]
use crate::target_device::sercom0::SPI;

#[cfg(any(
    feature = "samd51",
    feature = "same51",
    feature = "same53",
    feature = "same54"
))]
use crate::target_device::sercom0::SPIM as SPI;

pub trait CommonSpi {
    /// Helper for accessing the spi member of the sercom instance
    fn spi(&self) -> &SPI;

    /// Helper for accessing the spi member of the sercom instance
    fn spi_mut(&mut self) -> &SPI;

    /// Disable the SPI
    fn disable(&mut self) {
        self.spi_mut().ctrla.modify(|_, w| w.enable().clear_bit());
        // wait for configuration to take effect
        while self.spi().syncbusy.read().enable().bit_is_set() {}
    }

    /// Enable the SPI
    fn enable(&mut self) {
        self.spi_mut().ctrla.modify(|_, w| w.enable().set_bit());
        // wait for configuration to take effect
        while self.spi().syncbusy.read().enable().bit_is_set() {}
    }

    /// Set the polarity (CPOL) and phase (CPHA) of the SPI
    fn set_mode(&mut self, mode: Mode) {
        self.disable();
        self.spi_mut().ctrla.modify(|_, w| {
            match mode.polarity {
                Polarity::IdleLow => w.cpol().clear_bit(),
                Polarity::IdleHigh => w.cpol().set_bit(),
            };

            match mode.phase {
                Phase::CaptureOnFirstTransition => w.cpha().clear_bit(),
                Phase::CaptureOnSecondTransition => w.cpha().set_bit(),
            }
        });
        self.enable();
    }

    /// Method for calculating the output frequency given our baud settings.
    ///
    /// for synchronous SERCOM peripherals, the calculation for the final
    /// frequency is `f_baud = f_ref / (2 * (BAUD + 1))`.
    fn freq<F: Into<Hertz>>(&self, src_clock_freq: Hertz) -> Hertz {
        let baud: u8 = self.spi().baud.read().bits();
        (src_clock_freq.0 / (2_u32 * (baud as u32 + 1_u32))).Hz()
    }

    /// Helper for calculating our baudrate register
    ///
    /// for synchronous SERCOM peripherals, the calculation for this
    /// register is `BAUD = f_ref / (2 * f_baud) - 1`.
    #[inline]
    fn calculate_baud<F: Into<Hertz>>(freq: F, src_clock_freq: Hertz) -> u8 {
        let baud = (src_clock_freq.0 / (2 * freq.into().0) - 1) as u8;
        baud
    }
}
