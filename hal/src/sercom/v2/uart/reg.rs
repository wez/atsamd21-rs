//! Register-level access to UART configuration

use super::{BaudMode, BitOrder, CharSizeEnum, Parity, StopBits};

use crate::sercom::v2::*;
use crate::target_device as pac;

#[cfg(any(feature = "samd11", feature = "samd21"))]
use pac::sercom0::usart::ctrla::MODE_A;

#[cfg(feature = "min-samd51g")]
use pac::sercom0::usart_int::ctrla::MODE_A;

use crate::time::Hertz;

pub struct Registers<S: Sercom> {
    sercom: S,
}

// SAFETY: It is safe to implement Sync for Registers, because it erases the
// interior mutability of the PAC SERCOM struct.
unsafe impl<S: Sercom> Sync for Registers<S> {}

impl<S: Sercom> Registers<S> {
    /// Create a new `Registers` instance
    #[inline]
    pub(super) fn new(sercom: S) -> Self {
        Self { sercom }
    }

    /// Helper function to access the underlying `USART` from the given `SERCOM`
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    #[inline]
    fn usart(&self) -> &pac::sercom0::USART {
        self.sercom.usart()
    }

    /// Helper function to access the underlying `USART_INT` from the given
    /// `SERCOM`
    #[cfg(feature = "min-samd51g")]
    #[inline]
    fn usart(&self) -> &pac::sercom0::USART_INT {
        self.sercom.usart_int()
    }

    #[cfg(feature = "dma")]
    /// Get a pointer to the `DATA` register
    pub(super) fn data_ptr<T>(&self) -> *mut T {
        self.usart().data.as_ptr() as *mut _
    }

    /// Free the `Registers` struct and return the underlying `Sercom` instance
    #[inline]
    pub(super) fn free(self) -> S {
        self.sercom
    }

    /// Reset the SERCOM peripheral
    #[inline]
    pub(super) fn swrst(&mut self) {
        self.usart().ctrla.write(|w| w.swrst().set_bit());
        while self.usart().syncbusy.read().swrst().bit_is_set() {}
    }

    /// Configure the SERCOM to use internal clock mode
    #[inline]
    pub(super) fn configure_mode(&mut self) {
        self.usart()
            .ctrla
            .modify(|_, w| w.mode().variant(MODE_A::USART_INT_CLK));
    }

    /// Configure the `SERCOM`'s Pads according to RXPO and TXPO
    #[inline]
    pub(super) fn configure_pads(&mut self, rxpo: u8, txpo: u8) {
        self.usart().ctrla.modify(|_, w| unsafe {
            w.rxpo().bits(rxpo);
            w.txpo().bits(txpo)
        });
    }

    /// Configure the character size
    #[inline]
    pub(super) fn set_char_size(&mut self, bits: u8) {
        self.usart()
            .ctrlb
            .modify(|_, w| unsafe { w.chsize().bits(bits) });
    }

    /// Get the current character size setting
    #[inline]
    pub(super) fn get_char_size(&self) -> CharSizeEnum {
        self.usart().ctrlb.read().chsize().bits().into()
    }

    /// Change the bit order of transmission (MSB/LSB first)
    #[inline]
    pub(super) fn set_bit_order(&mut self, bit_order: BitOrder) {
        self.usart()
            .ctrla
            .modify(|_, w| w.dord().bit(bit_order.into()));
    }

    /// Get the current bit order
    #[inline]
    pub(super) fn get_bit_order(&self) -> BitOrder {
        self.usart().ctrla.read().dord().bit().into()
    }

    /// Change the parity setting
    #[inline]
    pub(super) fn set_parity(&mut self, parity: Parity) {
        // Use only the first two available settings in the FORM field.
        // Ignore auto-baud options.
        let enabled = match parity {
            Parity::None => false,
            Parity::Odd => {
                self.usart().ctrlb.modify(|_, w| w.pmode().bit(true));
                true
            }
            Parity::Even => {
                self.usart().ctrlb.modify(|_, w| w.pmode().bit(false));
                true
            }
        };

        self.usart()
            .ctrla
            .modify(|_, w| unsafe { w.form().bits(enabled as u8) });
    }

    /// Get the current parity setting
    #[inline]
    pub(super) fn get_parity(&self) -> Parity {
        let form = self.usart().ctrla.read().form().bits();
        let enabled = form == 0x1 || form == 0x5;

        if !enabled {
            return Parity::None;
        }

        let pmode = self.usart().ctrlb.read().pmode().bit();

        match pmode {
            false => Parity::Even,
            true => Parity::Odd,
        }
    }

    /// Change the stop bit setting
    #[inline]
    pub(super) fn set_stop_bits(&mut self, stop_bits: StopBits) {
        self.usart()
            .ctrlb
            .modify(|_, w| w.sbmode().bit(stop_bits.into()));
    }

    /// Get the current stop bit setting
    #[inline]
    pub(super) fn get_stop_bits(&self) -> StopBits {
        self.usart().ctrlb.read().sbmode().bit().into()
    }

    /// Enable or disable the start of frame detector.
    ///
    /// When set, the UART will generate interrupts for
    /// RXC and/or RXS if these interrupt flags have been enabled.
    #[inline]
    pub(super) fn set_start_of_frame_detection(&mut self, enabled: bool) {
        self.usart().ctrlb.modify(|_, w| w.sfde().bit(enabled));
    }

    /// Get the current SOF detector setting
    #[inline]
    pub(super) fn get_start_of_frame_detection(&self) -> bool {
        self.usart().ctrlb.read().sfde().bit()
    }

    /// Enable or disable the collision detector.
    ///
    /// When set, the UART will detect collisions and update the
    /// corresponding flag in the STATUS register.
    #[inline]
    pub(super) fn set_collision_detection(&mut self, enabled: bool) {
        self.usart().ctrlb.modify(|_, w| w.colden().bit(enabled));
    }

    /// Get the current collision detector setting
    #[inline]
    pub(super) fn get_collision_detection(&self) -> bool {
        self.usart().ctrlb.read().colden().bit()
    }

    /// Set the baud rate
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// GCLK frequency/oversampling. Values outside this range will saturate at
    /// the maximum supported baud rate.
    ///
    /// Note that 3x oversampling is not supported.
    #[inline]
    pub(super) fn set_baud<B: Into<Hertz>>(&mut self, freq: Hertz, baud: B, mode: BaudMode) {
        let baud: Hertz = baud.into();
        let usart = self.usart();

        usart
            .ctrla
            .modify(|_, w| unsafe { w.sampr().bits(mode.sampr()) });

        match mode {
            BaudMode::Arithmetic(n) => {
                let baud = calculate_baud_asynchronous_arithm(baud.0, freq.0, n as u8);
                unsafe { usart.baud_usartfp_mode().write(|w| w.baud().bits(baud)) };
            }

            BaudMode::Fractional(n) => {
                let (baud, frac) = calculate_baud_asynchronous_fractional(baud.0, freq.0, n as u8);
                unsafe {
                    usart.baud_frac_mode().write(|w| {
                        w.fp().bits(frac);
                        w.baud().bits(baud)
                    });
                }
            }
        };
    }

    /// Get the contents of the `BAUD` register and the current baud mode. Note
    /// that only the CONTENTS of `BAUD` are returned, and not the actual baud
    /// rate. Refer to the datasheet to convert the `BAUD` register contents
    /// into a baud rate.
    #[inline]
    pub(super) fn get_baud(&self) -> (u16, BaudMode) {
        let baud = self.usart().baud_usartfp_mode().read().bits();
        let mode = BaudMode::get_sampr(self.usart().ctrla.read().sampr().bits());
        (baud, mode)
    }

    /// Control the buffer overflow notification
    ///
    /// If set to true, an [`RxError::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    pub(super) fn set_immediate_overflow_notification(&mut self, set: bool) {
        self.usart().ctrla.modify(|_, w| w.ibon().bit(set));
    }

    /// Get the current immediate overflow notification setting
    #[inline]
    pub(super) fn get_immediate_overflow_notification(&self) -> bool {
        self.usart().ctrla.read().ibon().bit()
    }

    /// Run in standby mode
    ///
    /// When set, the UART peripheral will run in standby mode. See the
    /// datasheet for more details.
    #[inline]
    pub(super) fn set_run_in_standby(&mut self, set: bool) {
        self.usart().ctrla.modify(|_, w| w.runstdby().bit(set));
    }

    /// Get the current run in standby mode
    #[inline]
    pub(super) fn get_run_in_standby(&self) -> bool {
        self.usart().ctrla.read().runstdby().bit()
    }

    /// Enable or disable IrDA encoding. The pulse length controls the minimum
    // pulse length that is required for a pulse to be accepted by the IrDA
    /// receiver with regards to the serial engine clock period.
    /// See datasheet for more information.
    #[inline]
    pub(super) fn set_irda_encoding(&mut self, pulse_length: Option<u8>) {
        match pulse_length {
            Some(l) => {
                self.usart().rxpl.write(|w| unsafe { w.rxpl().bits(l) });
                self.usart().ctrlb.modify(|_, w| w.enc().bit(true));
            }
            None => {
                self.usart().ctrlb.modify(|_, w| w.enc().bit(false));
            }
        }
    }

    /// Get the current IrDA encoding setting. The return type is the pulse
    /// length wrapped in an [`Option`].
    #[inline]
    pub(super) fn get_irda_encoding(&self) -> Option<u8> {
        if !self.usart().ctrlb.read().enc().bit() {
            return None;
        }

        Some(self.usart().rxpl.read().bits())
    }

    /// Clear specified interrupt flags
    #[inline]
    pub fn clear_flags(&mut self, bits: u8) {
        self.usart().intflag.modify(|_, w| unsafe { w.bits(bits) });
    }

    /// Read interrupt flags
    #[inline]
    pub fn read_flags(&self) -> u8 {
        self.usart().intflag.read().bits()
    }

    /// Enable specified interrupts
    #[inline]
    pub fn enable_interrupts(&mut self, bits: u8) {
        self.usart().intenset.write(|w| unsafe { w.bits(bits) });
    }

    /// Disable specified interrupts
    #[inline]
    pub fn disable_interrupts(&mut self, bits: u8) {
        self.usart().intenclr.write(|w| unsafe { w.bits(bits) });
    }

    /// Clear specified status flags
    #[inline]
    pub fn clear_status(&mut self, bits: u16) {
        self.usart().status.modify(|_, w| unsafe { w.bits(bits) });
    }

    /// Read status flags
    #[inline]
    pub fn read_status(&self) -> u16 {
        self.usart().status.read().bits()
    }

    /// Read from the `DATA` register
    #[inline]
    pub unsafe fn read_data(&mut self) -> super::DataSize {
        self.usart().data.read().data().bits()
    }

    /// Write to the `DATA` register
    #[inline]
    pub unsafe fn write_data(&mut self, data: super::DataSize) {
        self.usart().data.write(|w| w.data().bits(data))
    }

    /// Enable the UART peripheral
    ///
    /// UART transactions are not possible until the peripheral is enabled.
    #[inline]
    pub(super) fn enable(&mut self) {
        let usart = self.usart();

        // Enable RX
        usart.ctrlb.modify(|_, w| w.rxen().set_bit());
        while usart.syncbusy.read().ctrlb().bit_is_set() {}

        // Enable TX
        usart.ctrlb.modify(|_, w| w.txen().set_bit());
        while usart.syncbusy.read().ctrlb().bit_is_set() {}

        // Globally enable peripheral
        self.enable_peripheral(true);
    }

    #[inline]
    pub(super) fn disable(&mut self) {
        let usart = self.usart();

        // Disable RX
        usart.ctrlb.modify(|_, w| w.rxen().clear_bit());
        while usart.syncbusy.read().ctrlb().bit_is_set() {}

        // Disable TX
        usart.ctrlb.modify(|_, w| w.txen().clear_bit());
        while usart.syncbusy.read().ctrlb().bit_is_set() {}

        self.enable_peripheral(false);
    }

    /// Enable or disable the SERCOM peripheral, and wait for the ENABLE bit to
    /// synchronize.
    pub(super) fn enable_peripheral(&mut self, enable: bool) {
        self.usart().ctrla.modify(|_, w| w.enable().bit(enable));
        while self.usart().syncbusy.read().enable().bit_is_set() {}
    }
}

/// Calculate baudrate value using the asynchronous arithmetic method (Table
/// 24-2)
#[inline]
fn calculate_baud_asynchronous_arithm(baudrate: u32, clk_freq: u32, n_samples: u8) -> u16 {
    const SHIFT: u8 = 32;
    let sample_rate = (n_samples as u64 * baudrate as u64) << SHIFT;
    let ratio = sample_rate / clk_freq as u64;
    let scale = (1u64 << SHIFT) - ratio;
    let baud_calculated = (65536u64 * scale) >> SHIFT;
    baud_calculated as u16
}

/// Calculate baudrate value using the asynchronous frational method (Table
/// 24-2)
#[inline]
fn calculate_baud_asynchronous_fractional(
    baudrate: u32,
    clk_freq: u32,
    n_samples: u8,
) -> (u16, u8) {
    let baud_mult = (clk_freq * 8) / (n_samples as u32 * baudrate);
    ((baud_mult / 8) as u16, (baud_mult % 8) as u8)
}
