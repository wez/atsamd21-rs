//! UART [`Config`] definition and implementation\

use super::{
    BaudMode, BitOrder, CharSize, CharSizeEnum, DataSize, DynCharSize, EightBit, FixedCharSize,
    Parity, Registers, StopBits, Uart, ValidConfig, ValidPads,
};
use crate::{
    sercom::v2::*,
    target_device as pac,
    time::Hertz,
    typelevel::{Is, Sealed},
};
use core::marker::PhantomData;
use num_traits::{AsPrimitive, PrimInt};

//=============================================================================
// Config
//=============================================================================

/// A configurable, disabled UART peripheral
///
/// This `struct` represents a configurable UART peripheral in its disabled
/// state. It is generic over the set of [`Pads`] and [`CharSize`].
/// Upon creation, the [`Config`] takes ownership of the
/// [`Sercom`] and resets it, returning it configured as an UART peripheral
/// with a default configuration:
///
/// * [`EightBit`]
/// * No parity
/// * One stop bit
/// * LSB-first
///
/// [`Config`] uses a builder-pattern API to configure the peripheral,
/// culminating in a call to [`enable`], which consumes the [`Config`] and
/// returns enabled [`Uart`]. The [`enable`] method is
/// restricted to [`ValidConfig`]s.
///
/// [`enable`]: Config::enable
/// [`Pads`]: super::Pads
pub struct Config<P, C = EightBit>
where
    P: ValidPads,
    C: CharSize,
{
    pub(super) registers: Registers<P::Sercom>,
    pads: P,
    chsize: PhantomData<C>,
    freq: Hertz,
}

/// Clock type needed to create a new [`Config`]. [`PM`](pac::PM) for thumbv6m
/// targets.
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub type Clock = pac::PM;

/// Clock type needed to create a new [`Config`]. [`MCLK`](pac::MCLK) for
/// thumbv7em targets.
#[cfg(feature = "min-samd51g")]
pub type Clock = pac::MCLK;

impl<P: ValidPads> Config<P> {
    /// Create a new [`Config`] in the default configuration
    ///
    /// This function will enable the corresponding APB clock, reset the
    /// [`Sercom`] peripheral, and return a [`Config`] in the default
    /// configuration:
    ///
    /// * [`EightBit`] [`CharSize`]
    /// * No parity
    /// * One stop bit
    /// * LSB-first
    ///
    /// [`Config`] takes ownership of the [`Sercom`] and [`Pads`](super::Pads).
    ///
    /// Users must configure GCLK manually. The `freq` parameter represents the
    /// GCLK frequency for this [`Sercom`] instance.
    #[inline]
    pub fn new(clk: &Clock, mut sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        sercom.enable_apb_clock(clk);
        Self::default(sercom, pads, freq).bit_order(BitOrder::LsbFirst)
    }

    /// Create a new [`Config`] in the default configuration
    #[inline]
    fn default(sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        let mut registers = Registers::new(sercom);
        registers.swrst();

        // Enable internal clock mode
        registers.configure_mode();
        registers.configure_pads(P::RXPO as u8, P::TXPO as u8);
        registers.set_char_size(EightBit::BITS);

        Self {
            registers,
            pads,
            chsize: PhantomData,
            freq: freq.into(),
        }
    }
}

impl<P, C> Config<P, C>
where
    P: ValidPads,
    C: CharSize,
{
    /// Change the [`Config`] [`CharSize`]
    #[inline]
    fn change<C2>(self) -> Config<P, C2>
    where
        C2: CharSize,
    {
        Config {
            registers: self.registers,
            pads: self.pads,
            chsize: PhantomData,
            freq: self.freq,
        }
    }

    /// Trigger the [`Sercom`]'s SWRST and return a [`Config`] in the
    /// default configuration.
    #[inline]
    pub fn reset(self) -> Config<P> {
        Config::default(self.registers.free(), self.pads, self.freq)
    }

    /// Consume the [`Config`], reset the peripheral, and return the [`Sercom`]
    /// and [`Pads`](super::Pads)
    #[inline]
    pub fn free(mut self) -> (P::Sercom, P) {
        self.registers.swrst();
        (self.registers.free(), self.pads)
    }

    /// Change the [`CharSize`].
    #[inline]
    pub fn char_size<C2: FixedCharSize>(mut self) -> Config<P, C2> {
        self.registers.set_char_size(C2::BITS);
        self.change()
    }

    /// Change the [`CharSize`] to [`DynCharSize`]. The UART's character
    /// size will be changed to the provided `C2`, which is a [`FixedCharSize`]
    /// type, and can then be changed dynamically on an enabled [`Uart`]
    /// without changing the underlying [`Config`]'s type through the
    /// [`reconfigure`](Uart::reconfigure) method.
    #[inline]
    pub fn dyn_char_size<C2: FixedCharSize>(mut self) -> Config<P, DynCharSize> {
        self.registers.set_char_size(C2::BITS);
        self.change()
    }

    /// Change the bit order of transmission (builder pattern version)
    #[inline]
    pub fn bit_order(mut self, bit_order: BitOrder) -> Self {
        self.set_bit_order(bit_order);
        self
    }

    /// Change the bit order of transmission (setter version)
    #[inline]
    pub fn set_bit_order(&mut self, bit_order: BitOrder) {
        self.registers.set_bit_order(bit_order);
    }

    /// Get the current bit order
    #[inline]
    pub fn get_bit_order(&self) -> BitOrder {
        self.registers.get_bit_order()
    }

    /// Change the parity setting (builder pattern version)
    #[inline]
    pub fn parity(mut self, parity: Parity) -> Self {
        self.set_parity(parity);
        self
    }

    /// Change the parity setting (setter version)
    #[inline]
    pub fn set_parity(&mut self, parity: Parity) {
        self.registers.set_parity(parity);
    }

    /// Get the current parity setting
    #[inline]
    pub fn get_parity(&self) -> Parity {
        self.registers.get_parity()
    }

    /// Change the stop bit setting (builder pattern version)
    #[inline]
    pub fn stop_bits(mut self, stop_bits: StopBits) -> Self {
        self.set_stop_bits(stop_bits);
        self
    }

    /// Change the stop bit setting (setter version)
    #[inline]
    pub fn set_stop_bits(&mut self, stop_bits: StopBits) {
        self.registers.set_stop_bits(stop_bits);
    }

    /// Get the current stop bit setting
    #[inline]
    pub fn get_stop_bits(&self) -> StopBits {
        self.registers.get_stop_bits()
    }

    /// Enable or disable the start of frame detector (builder pattern version)
    ///
    /// When set, the UART will generate interrupts for
    /// RXC and/or RXS if these interrupt flags have been enabled.
    #[inline]
    pub fn start_of_frame_detection(mut self, enabled: bool) -> Self {
        self.set_start_of_frame_detection(enabled);
        self
    }

    /// Enable or disable the start of frame detector (setter version)
    ///
    /// When set, the UART will generate interrupts for
    /// RXC and/or RXS if these interrupt flags have been enabled.
    #[inline]
    pub fn set_start_of_frame_detection(&mut self, enabled: bool) {
        self.registers.set_start_of_frame_detection(enabled);
    }

    /// Get the current SOF detector setting
    #[inline]
    pub fn get_start_of_frame_detection(&self) -> bool {
        self.registers.get_start_of_frame_detection()
    }

    /// Enable or disable the collision detector (builder pattern version)
    ///
    /// When set, the UART will detect collisions and update the
    /// corresponding flag in the STATUS register.
    #[inline]
    pub fn collision_detection(mut self, enabled: bool) -> Self {
        self.set_collision_detection(enabled);
        self
    }

    /// Enable or disable the collision detector (setter version)
    ///
    /// When set, the UART will detect collisions and update the
    /// corresponding flag in the STATUS register.
    #[inline]
    pub fn set_collision_detection(&mut self, enabled: bool) {
        self.registers.set_collision_detection(enabled);
    }

    /// Get the current collision detector setting
    #[inline]
    pub fn get_collision_detection(&self) -> bool {
        self.registers.get_collision_detection()
    }

    /// Set the baud rate (builder pattern version)
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// GCLK frequency/oversampling. Values outside this range will saturate at
    /// the maximum supported baud rate.
    ///
    /// Note that 3x oversampling is not supported.
    #[inline]
    pub fn baud<B: Into<Hertz>>(mut self, baud: B, mode: BaudMode) -> Self {
        self.set_baud(baud, mode);
        self
    }

    /// Set the baud rate (setter version)
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// GCLK frequency/oversampling. Values outside this range will saturate at
    /// the maximum supported baud rate.
    ///
    /// Note that 3x oversampling is not supported.
    #[inline]
    pub fn set_baud<B: Into<Hertz>>(&mut self, baud: B, mode: BaudMode) {
        self.registers.set_baud(self.freq, baud, mode);
    }

    /// Get the contents of the `BAUD` register and the current baud mode. Note
    /// that only the CONTENTS of `BAUD` are returned, and not the actual baud
    /// rate. Refer to the datasheet to convert the `BAUD` register contents
    /// into a baud rate.
    #[inline]
    pub fn get_baud(&self) -> (u16, BaudMode) {
        self.registers.get_baud()
    }

    /// Control the buffer overflow notification (builder pattern version)
    ///
    /// If set to true, an [`Error::Overflow`](super::Error::Overflow) will be
    /// issued as soon as an overflow occurs. Otherwise, it will not be
    /// issued until its place within the data stream.
    #[inline]
    pub fn immediate_overflow_notification(mut self, set: bool) -> Self {
        self.set_immediate_overflow_notification(set);
        self
    }

    /// Control the buffer overflow notification (setter version)
    ///
    /// If set to true, an [`Error::Overflow`](super::Error::Overflow) will be
    /// issued as soon as an overflow occurs. Otherwise, it will not be
    /// issued until its place within the data stream.
    #[inline]
    pub fn set_immediate_overflow_notification(&mut self, set: bool) {
        self.registers.set_immediate_overflow_notification(set);
    }

    /// Get the current immediate overflow notification setting
    #[inline]
    pub fn get_immediate_overflow_notification(&self) -> bool {
        self.registers.get_immediate_overflow_notification()
    }

    /// Run in standby mode (builder pattern version)
    ///
    /// When set, the UART peripheral will run in standby mode. See the
    /// datasheet for more details.
    #[inline]
    pub fn run_in_standby(mut self, set: bool) -> Self {
        self.set_run_in_standby(set);
        self
    }

    /// Run in standby mode (setter version)
    ///
    /// When set, the UART peripheral will run in standby mode. See the
    /// datasheet for more details.
    #[inline]
    pub fn set_run_in_standby(&mut self, set: bool) {
        self.registers.set_run_in_standby(set);
    }

    /// Get the current run in standby mode
    #[inline]
    pub fn get_run_in_standby(&self) -> bool {
        self.registers.get_run_in_standby()
    }

    /// Enable or disable IrDA encoding (builder pattern version)
    ///
    /// The pulse length controls the minimum pulse length that is required for
    /// a pulse to be accepted by the IrDA receiver with regards to the
    /// serial engine clock period. See datasheet for more information.
    #[inline]
    pub fn irda_encoding(mut self, pulse_length: Option<u8>) -> Self {
        self.set_irda_encoding(pulse_length);
        self
    }

    /// Enable or disable IrDA encoding (setter version)
    ///
    /// The pulse length controls the minimum pulse length that is required for
    /// a pulse to be accepted by the IrDA receiver with regards to the
    /// serial engine clock period. See datasheet for more information.
    #[inline]
    pub fn set_irda_encoding(&mut self, pulse_length: Option<u8>) {
        self.registers.set_irda_encoding(pulse_length);
    }

    /// Get the current IrDA encoding setting. The return type is the pulse
    /// length wrapped in an [`Option`].
    #[inline]
    pub fn get_irda_encoding(&self) -> Option<u8> {
        self.registers.get_irda_encoding()
    }
}

impl<P: ValidPads> Config<P, DynCharSize> {
    /// Dynamically change the character size
    #[inline]
    pub fn set_dyn_char_size(&mut self, char_size: CharSizeEnum) {
        self.registers.set_char_size(char_size as u8);
    }

    /// Get the current character size setting
    pub fn get_char_size(&self) -> CharSizeEnum {
        self.registers.get_char_size()
    }
}

impl<P, C> Config<P, C>
where
    P: ValidPads,
    C: CharSize,
    Self: ValidConfig,
{
    /// Enable the UART peripheral and return a [`Uart`] struct.
    ///
    /// UART transactions are not possible until the peripheral is enabled.
    /// This method is limited to [`ValidConfig`]s
    #[inline]
    pub fn enable(mut self) -> Uart<Self, P::Capability> {
        self.registers.enable();
        Uart {
            config: self,
            capability: PhantomData,
        }
    }
}

//=============================================================================
// AnyConfig
//=============================================================================

/// Type class for all possible [`Config`] types
///
/// This trait uses the [`AnyKind`] trait pattern to create a [type class] for
/// [`Config`] types. See the `AnyKind` documentation for more details on the
/// pattern.
///
/// In addition to the normal, `AnyKind` associated types. This trait also
/// copies the [`Sercom`] and `Word` types, to make it easier to apply
/// bounds to these types at the next level of abstraction.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-patter
pub trait AnyConfig: Sealed + Is<Type = SpecificConfig<Self>> {
    type Sercom: Sercom;
    type Pads: ValidPads<Sercom = Self::Sercom>;
    type Word: 'static + PrimInt + AsPrimitive<DataSize>;
    type CharSize: CharSize<Word = Self::Word>;
}

/// Type alias to recover the specific [`Config`] type from an implementation of
/// [`AnyConfig`]
pub type SpecificConfig<C> = Config<<C as AnyConfig>::Pads, <C as AnyConfig>::CharSize>;

/// Type alias to recover the specific [`Sercom`] type from an implementation of
/// [`AnyConfig`]
pub type ConfigSercom<C> = <C as AnyConfig>::Sercom;

impl<P, C> AsRef<Self> for Config<P, C>
where
    P: ValidPads,
    C: CharSize,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<P, C> AsMut<Self> for Config<P, C>
where
    P: ValidPads,
    C: CharSize,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<P, C> Sealed for Config<P, C>
where
    P: ValidPads,
    C: CharSize,
{
}

impl<P, C> AnyConfig for Config<P, C>
where
    P: ValidPads,
    C: CharSize,
{
    type Sercom = P::Sercom;
    type Word = C::Word;
    type Pads = P;
    type CharSize = C;
}
