use core::convert::TryInto;

use embedded_hal::spi;

#[cfg(any(feature = "samd11", feature = "samd21"))]
use crate::pac::sercom0::SPI;
#[cfg(feature = "min-samd51g")]
use crate::pac::sercom0::SPIM;

#[cfg(any(feature = "samd11", feature = "samd21"))]
use crate::pac::sercom0::spi::ctrla::MODE_A;
#[cfg(feature = "min-samd51g")]
use crate::pac::sercom0::spim::ctrla::MODE_A;

use crate::sercom::v2::Sercom;
use crate::time::Hertz;

use super::{BitOrder, DataWidth, Error, Flags, Phase, Polarity, Status};

//==============================================================================
// Registers
//==============================================================================

/// Define a task-focused register interface for SPI peripherals
///
/// This struct acts to define a task-focused, rather than register-focused, API
/// for a SERCOM configured as an SPI peripheral. It also serves to erase the
/// inherent interior mutability of the PAC [`Sercom`] type, which allows it to
/// implement [`Sync`].
pub(super) struct Registers<S: Sercom> {
    pub sercom: S,
}

// Safety: The [`Registers`] struct erases interior mutability, so this is now
// safe.
unsafe impl<S: Sercom> Sync for Registers<S> {}

impl<S: Sercom> Registers<S> {
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    #[inline]
    pub fn spi(&self) -> &SPI {
        self.sercom.spi()
    }

    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn spi(&self) -> &SPIM {
        self.sercom.spim()
    }

    /// Reset the SERCOM peripheral
    #[inline]
    pub fn reset(&mut self) {
        self.spi().ctrla.write(|w| w.swrst().set_bit());
        while self.spi().syncbusy.read().swrst().bit_is_set() {}
    }

    /// Configure the DIPO and DOPO values
    #[inline]
    pub fn set_dipo_dopo(&mut self, dipo_dopo: (u8, u8)) {
        let (dipo, dopo) = dipo_dopo;
        self.spi().ctrla.modify(|_, w| unsafe {
            w.dipo().bits(dipo);
            w.dopo().bits(dopo)
        });
    }

    /// Configure the SPI operating mode
    ///
    /// For maximum flexibility, this module chooses to always operate in 32-bit
    /// extension mode. The LENGTH counter is used to control the number of byes
    /// in each SPI transaction. Due to a hardware bug, ICSPACE must be at least
    /// one. See the silicon errata for more details.
    #[inline]
    pub fn set_op_mode(&mut self, mode: MODE_A, mssen: bool) -> () {
        self.spi().ctrla.modify(|_, w| w.mode().variant(mode));
        self.spi().ctrlb.modify(|_, w| w.mssen().bit(mssen));
        #[cfg(feature = "min-samd51g")]
        self.spi().ctrlc.write(|w| unsafe {
            w.data32b().data_trans_32bit();
            w.icspace().bits(1)
        });
        while self.spi().syncbusy.read().ctrlb().bit_is_set() {}
    }

    /// Return the current transaction length
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn get_length(&self) -> u8 {
        self.spi().length.read().len().bits()
    }

    /// Set the transaction length
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn set_length(&mut self, length: u8) {
        let length = if length == 0 { 1 } else { length };
        self.spi().length.write(|w| unsafe {
            w.len().bits(length);
            w.lenen().set_bit()
        });
        while self.spi().syncbusy.read().length().bit_is_set() {}
    }

    /// Set the character size
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    #[inline]
    pub fn set_char_size(&mut self, bits: u8) {
        self.spi()
            .ctrlb
            .modify(|_, w| unsafe { w.chsize().bits(bits) });
    }

    /// Get the clock polarity
    #[inline]
    pub fn get_cpol(&self) -> Polarity {
        let cpol = self.spi().ctrla.read().cpol().bit();
        match cpol {
            false => Polarity::IdleLow,
            true => Polarity::IdleHigh,
        }
    }

    /// Set the clock polarity
    #[inline]
    pub fn set_cpol(&mut self, cpol: Polarity) {
        let cpol = match cpol {
            Polarity::IdleLow => false,
            Polarity::IdleHigh => true,
        };
        self.spi().ctrla.modify(|_, w| w.cpol().bit(cpol));
    }

    /// Get the clock phase
    #[inline]
    pub fn get_cpha(&self) -> Phase {
        let cpha = self.spi().ctrla.read().cpha().bit();
        match cpha {
            false => Phase::CaptureOnFirstTransition,
            true => Phase::CaptureOnSecondTransition,
        }
    }

    /// Set the clock phase
    #[inline]
    pub fn set_cpha(&mut self, cpha: Phase) {
        let cpha = match cpha {
            Phase::CaptureOnFirstTransition => false,
            Phase::CaptureOnSecondTransition => true,
        };
        self.spi().ctrla.modify(|_, w| w.cpha().bit(cpha));
    }

    /// Get the SPI mode (clock polarity & phase)
    #[inline]
    pub fn get_spi_mode(&self) -> spi::Mode {
        let reg = self.spi().ctrla.read();
        let cpol = reg.cpol().bit();
        let cpha = reg.cpha().bit();
        let polarity = match cpol {
            false => Polarity::IdleLow,
            true => Polarity::IdleHigh,
        };
        let phase = match cpha {
            false => Phase::CaptureOnFirstTransition,
            true => Phase::CaptureOnSecondTransition,
        };
        spi::Mode { polarity, phase }
    }

    /// Set the SPI mode (clock polarity & phase)
    #[inline]
    pub fn set_spi_mode(&mut self, mode: spi::Mode) {
        let cpol = match mode.polarity {
            Polarity::IdleLow => false,
            Polarity::IdleHigh => true,
        };
        let cpha = match mode.phase {
            Phase::CaptureOnFirstTransition => false,
            Phase::CaptureOnSecondTransition => true,
        };
        self.spi().ctrla.modify(|_, w| {
            w.cpol().bit(cpol);
            w.cpha().bit(cpha)
        });
    }

    /// Get the bit order of transmission (MSB/LSB first)
    #[inline]
    pub fn get_bit_order(&self) -> BitOrder {
        let order = self.spi().ctrla.read().dord().bits();
        match order {
            false => BitOrder::MsbFirst,
            true => BitOrder::LsbFirst,
        }
    }

    /// Set the bit order of transmission (MSB/LSB first)
    #[inline]
    pub fn set_bit_order(&mut self, order: BitOrder) {
        let order = match order {
            BitOrder::MsbFirst => false,
            BitOrder::LsbFirst => true,
        };
        self.spi().ctrla.modify(|_, w| w.dord().bit(order));
    }

    /// Get the baud rate
    #[inline]
    pub fn get_baud(&mut self, freq: Hertz) -> Hertz {
        let baud = self.spi().baud.read().baud().bits() as u32 + 1;
        Hertz(freq.0 / 2 / baud)
    }

    /// Set the baud rate
    #[inline]
    pub fn set_baud(&mut self, freq: Hertz, baud: impl Into<Hertz>) {
        let baud = baud.into().0;
        let baud = if baud == 0 { 1 } else { baud };
        let bits = (freq.0 / 2 / baud).saturating_sub(1);
        let bits = bits.try_into().unwrap_or(u8::MAX);
        self.spi()
            .baud
            .modify(|_, w| unsafe { w.baud().bits(bits) });
    }

    /// Get the enable state of the immediate buffer overflow notification
    #[inline]
    pub fn get_ibon(&self) -> bool {
        self.spi().ctrla.read().ibon().bit()
    }

    /// Set the enable state of the immediate buffer overflow notification
    #[inline]
    pub fn set_ibon(&mut self, enabled: bool) {
        self.spi().ctrla.modify(|_, w| w.ibon().bit(enabled));
    }

    /// Get run in standby mode
    #[inline]
    pub fn get_run_in_standby(&self) -> bool {
        self.spi().ctrla.read().runstdby().bit()
    }

    /// Set run in standby mode
    #[inline]
    pub fn set_run_in_standby(&mut self, set: bool) {
        self.spi().ctrla.modify(|_, w| w.runstdby().bit(set));
    }

    /// Enable interrupts for the specified flags
    #[inline]
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.spi()
            .intenset
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Disable interrupts for the specified flags
    #[inline]
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.spi()
            .intenclr
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Enable the receiver
    #[inline]
    pub fn rx_enable(&mut self) {
        self.spi().ctrlb.modify(|_, w| w.rxen().set_bit());
        while self.spi().syncbusy.read().ctrlb().bit_is_set() {}
    }

    /// Disable the receiver
    #[inline]
    pub fn rx_disable(&mut self) {
        self.spi().ctrlb.modify(|_, w| w.rxen().clear_bit());
        while self.spi().syncbusy.read().ctrlb().bit_is_set() {}
    }

    /// Enable the peripheral
    #[inline]
    pub fn enable(&mut self) {
        self.spi().ctrla.modify(|_, w| w.enable().set_bit());
        while self.spi().syncbusy.read().enable().bit_is_set() {}
    }

    /// Disable the peripheral
    #[inline]
    pub fn disable(&mut self) {
        self.spi().ctrla.modify(|_, w| w.enable().clear_bit());
        while self.spi().syncbusy.read().enable().bit_is_set() {}
    }

    /// Read from the `DATA` register
    #[inline]
    pub fn read_data(&mut self) -> DataWidth {
        self.spi().data.read().data().bits()
    }

    /// Write to the `DATA` register
    #[inline]
    pub fn write_data(&mut self, data: DataWidth) {
        // Safety: All bit patterns are memory safe
        self.spi().data.write(|w| unsafe { w.data().bits(data) })
    }

    /// Read the interrupt flags
    #[inline]
    pub fn read_flags(&self) -> Flags {
        let bits = self.spi().intflag.read().bits();
        Flags::from_bits_truncate(bits)
    }

    /// Clear interrupt flags
    #[inline]
    pub fn clear_flags(&mut self, flags: Flags) {
        unsafe { self.spi().intflag.write(|w| w.bits(flags.bits())) };
    }

    /// Read the error status flags
    #[inline]
    pub fn read_status(&self) -> Status {
        let bits = self.spi().status.read().bits();
        Status::from_bits_truncate(bits)
    }

    /// Clear error status flags
    #[inline]
    pub fn clear_status(&mut self, status: Status) {
        unsafe { self.spi().status.write(|w| w.bits(status.bits())) };
    }

    /// Try to read the interrupt flags, but first check the error status flags.
    #[inline]
    pub fn read_flags_errors(&self) -> Result<Flags, Error> {
        self.read_status().try_into()?;
        Ok(self.read_flags())
    }
}
