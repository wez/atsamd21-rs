//! # Version 1 of the GPIO module
//!
//! This module is a compatibility shim that allows existing code to use the new
//! [v2](super::v2) module. This API will eventually be deprecated and removed.
//!
//! Working with GPIO pins.
//! The pins are associated with the PORT hardware. This module defines a
//! [split](GpioExt::split) method on the [PORT](crate::target_device::PORT)
//! type that is used to safely reference the individual pin configuration.
//! The IO pins can be switched into alternate function modes, which
//! routes the pins to different peripherals depending on the mode
//! for the pin.  The pin configuration is reflected through the
//! use of type states to make the interface (ideally, or at least practically)
//! impossible to misuse.

use crate::target_device::PORT;

use hal::digital::v2::OutputPin;

#[cfg(feature = "unproven")]
use hal::digital::v2::{InputPin, StatefulOutputPin, ToggleableOutputPin};

use crate::gpio::v2::{self, Alternate, AlternateConfig, AnyPin, OutputConfig};
pub use crate::gpio::v2::{PinId, PinMode};
use crate::typelevel::Sealed;

//==============================================================================
// Type definitions
//==============================================================================

/// Represents a pin configured for input.
/// The MODE type is typically one of `Floating`, `PullDown` or
/// `PullUp`.
pub type Input<MODE> = v2::Input<MODE>;

/// Represents a pin configured for interrupt.
/// The MODE type is one of `Floating`, `PullDown` or `PullUp`.
pub type Interrupt<MODE> = v2::Interrupt<MODE>;

/// Represents a pin configured for output.
/// The MODE type is typically one of `PushPull`, or
/// `OpenDrain`.
pub type Output<MODE> = v2::Output<MODE>;

// The following collection of types is used to encode the
// state of the pin at compile time and helps to avoid misuse.

/// Floating Input
pub type Floating = v2::Floating;
/// Pulled down Input
pub type PullDown = v2::PullDown;
/// Pulled up Input
pub type PullUp = v2::PullUp;

/// Totem Pole aka Push-Pull
pub type PushPull = v2::PushPull;
/// Open drain output.
/// The SAMD5x/E5x chips don't actually have open drain outputs.
/// This option was added by mistake. It is currently an alias of `PushPull`
pub type OpenDrain = v2::PushPull;
/// Open drain output, which can be read when not driven
/// The SAMD5x/E5x chips don't actually have open drain outputs.
/// This option actually represents a readable `PushPull` output
pub type ReadableOpenDrain = v2::Readable;

/// Peripheral Function B
pub type PfB = v2::AlternateB;
/// Peripheral Function C
pub type PfC = v2::AlternateC;
/// Peripheral Function D
pub type PfD = v2::AlternateD;
/// Peripheral Function E
pub type PfE = v2::AlternateE;
/// Peripheral Function F
pub type PfF = v2::AlternateF;
/// Peripheral Function G
pub type PfG = v2::AlternateG;
/// Peripheral Function H
#[cfg(any(feature = "samd21", feature = "min-samd51g"))]
pub type PfH = v2::AlternateH;
/// Peripheral Function I
#[cfg(feature = "min-samd51g")]
pub type PfI = v2::AlternateI;
/// Peripheral Function J
#[cfg(feature = "min-samd51g")]
pub type PfJ = v2::AlternateJ;
/// Peripheral Function K
#[cfg(feature = "min-samd51g")]
pub type PfK = v2::AlternateK;
/// Peripheral Function L
#[cfg(feature = "min-samd51g")]
pub type PfL = v2::AlternateL;
/// Peripheral Function M
#[cfg(feature = "min-samd51g")]
pub type PfM = v2::AlternateM;
/// Peripheral Function N
#[cfg(feature = "min-samd51g")]
pub type PfN = v2::AlternateN;

//==============================================================================
// Pin
//==============================================================================

/// Represents a GPIO pin with a corresponding [`PinId`] and [`PinMode`]
///
/// The [`v2::Pin`] type provides many of the same inherent functions, but it
/// does so without requiring the `PORT` as an argument, breaking backwards
/// compatibility.
///
/// `v1` [`Pin`] type is a newtype wrapper for [`v2::Pin`]s. To aid in
/// compatibility, the `v1` [`Pin`] types also implement [`AnyPin`]. [`From`] &
/// [`Into`] conversions are provided between the two pin types.
pub struct Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    pub(crate) pin: v2::Pin<I, M>,
}

impl<I, M> Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    #[inline]
    unsafe fn new() -> Pin<I, M> {
        Pin {
            pin: v2::Pin::new(),
        }
    }

    #[inline]
    pub fn into_mode<N: PinMode>(mut self) -> Pin<I, N> {
        // Change pin mode unconditionally, to match the original `v1` behavior
        self.pin.regs.change_mode::<N>();
        // Safe because we drop the existing Pin
        unsafe { Pin::new() }
    }

    /// Configures the pin to operate as a floating input
    #[allow(unused_variables)]
    #[inline]
    pub fn into_floating_input(self, port: &mut Port) -> Pin<I, Input<Floating>> {
        self.into_mode()
    }

    /// Configures the pin to operate as a pulled down input pin
    #[allow(unused_variables)]
    #[inline]
    pub fn into_pull_down_input(self, port: &mut Port) -> Pin<I, Input<PullDown>> {
        self.into_mode()
    }

    /// Configures the pin to operate as a pulled up input pin
    #[allow(unused_variables)]
    #[inline]
    pub fn into_pull_up_input(self, port: &mut Port) -> Pin<I, Input<PullUp>> {
        self.into_mode()
    }

    /// Configures the pin to operate as a floating interrupt
    #[allow(unused_variables)]
    #[inline]
    pub fn into_floating_interrupt(self, port: &mut Port) -> Pin<I, Interrupt<Floating>> {
        self.into_mode()
    }

    /// Configures the pin to operate as a pulled down interrupt pin
    #[allow(unused_variables)]
    #[inline]
    pub fn into_pull_down_interrupt(self, port: &mut Port) -> Pin<I, Interrupt<PullDown>> {
        self.into_mode()
    }

    /// Configures the pin to operate as a pulled up interrupt pin
    #[allow(unused_variables)]
    #[inline]
    pub fn into_pull_up_interrupt(self, port: &mut Port) -> Pin<I, Interrupt<PullUp>> {
        self.into_mode()
    }

    /// Configures the pin to operate as an open drain output
    #[allow(unused_variables)]
    #[inline]
    pub fn into_open_drain_output(self, port: &mut Port) -> Pin<I, Output<OpenDrain>> {
        self.into_mode()
    }

    /// Configures the pin to operate as an open drain output which can be read
    #[allow(unused_variables)]
    #[inline]
    pub fn into_readable_open_drain_output(
        self,
        port: &mut Port,
    ) -> Pin<I, Output<ReadableOpenDrain>> {
        self.into_mode()
    }

    /// Configures the pin to operate as a push-pull output
    #[allow(unused_variables)]
    #[inline]
    pub fn into_push_pull_output(self, port: &mut Port) -> Pin<I, Output<PushPull>> {
        self.into_mode()
    }

    #[inline]
    fn into_alternate<C: AlternateConfig>(self) -> Pin<I, Alternate<C>> {
        self.into_mode()
    }

    /// Configures the pin to operate with a peripheral
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_b(self, port: &mut Port) -> Pin<I, PfB> {
        self.into_alternate()
    }

    /// Configures the pin to operate with a peripheral
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_c(self, port: &mut Port) -> Pin<I, PfC> {
        self.into_alternate()
    }

    /// Configures the pin to operate with a peripheral
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_d(self, port: &mut Port) -> Pin<I, PfD> {
        self.into_alternate()
    }

    /// Configures the pin to operate with a peripheral
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_e(self, port: &mut Port) -> Pin<I, PfE> {
        self.into_alternate()
    }

    /// Configures the pin to operate with a peripheral
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_f(self, port: &mut Port) -> Pin<I, PfF> {
        self.into_alternate()
    }

    /// Configures the pin to operate with a peripheral
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_g(self, port: &mut Port) -> Pin<I, PfG> {
        self.into_alternate()
    }

    /// Configures the pin to operate with a peripheral
    #[cfg(any(feature = "samd21", feature = "min-samd51g"))]
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_h(self, port: &mut Port) -> Pin<I, PfH> {
        self.into_alternate()
    }

    /// Configures the pin to operate with a peripheral
    #[cfg(feature = "min-samd51g")]
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_i(self, port: &mut Port) -> Pin<I, PfI> {
        self.into_alternate()
    }

    /// Configures the pin to operate with a peripheral
    #[cfg(feature = "min-samd51g")]
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_j(self, port: &mut Port) -> Pin<I, PfJ> {
        self.into_alternate()
    }

    /// Configures the pin to operate with a peripheral
    #[cfg(feature = "min-samd51g")]
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_k(self, port: &mut Port) -> Pin<I, PfK> {
        self.into_alternate()
    }

    /// Configures the pin to operate with a peripheral
    #[cfg(feature = "min-samd51g")]
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_l(self, port: &mut Port) -> Pin<I, PfL> {
        self.into_alternate()
    }

    /// Configures the pin to operate with a peripheral
    #[cfg(feature = "min-samd51g")]
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_m(self, port: &mut Port) -> Pin<I, PfM> {
        self.into_alternate()
    }

    /// Configures the pin to operate with a peripheral
    #[cfg(feature = "min-samd51g")]
    #[allow(unused_variables)]
    #[inline]
    pub fn into_function_n(self, port: &mut Port) -> Pin<I, PfN> {
        self.into_alternate()
    }
}

//==============================================================================
//  AnyPin
//==============================================================================

impl<I, M> Sealed for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
}

/// Implement [`AnyPin`] for `v1` [`Pin`] types to enhance compatibility with
/// [`v2::Pin`]s
impl<I, M> AnyPin for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    type Id = I;
    type Mode = M;
}

/// Type alias to recover the specific `v1` [`Pin`] type from an implementation
/// of [`AnyPin`]
///
/// By definition, `P == SpecificPin<P>` for all `P: AnyPin`.
pub type SpecificPin<P> = Pin<<P as AnyPin>::Id, <P as AnyPin>::Mode>;

impl<P: AnyPin> AsRef<P> for SpecificPin<P> {
    #[inline]
    fn as_ref(&self) -> &P {
        // SAFETY: This is guaranteed to be safe, because P == SpecificPin<P>
        // Transmuting between `v1` and `v2` `Pin` types is also safe, because
        // single-field, newtype structs are guaranteed to have the same layout
        // as the field, even for repr(Rust).
        unsafe { core::mem::transmute(self) }
    }
}

impl<P: AnyPin> AsMut<P> for SpecificPin<P> {
    #[inline]
    fn as_mut(&mut self) -> &mut P {
        // SAFETY: This is guaranteed to be safe, because P == SpecificPin<P>
        // Transmuting between `v1` and `v2` `Pin` types is also safe, because
        // single-field, newtype structs are guaranteed to have the same layout
        // as the field, even for repr(Rust).
        unsafe { core::mem::transmute(self) }
    }
}

//==============================================================================
// IntoFunction
//==============================================================================

/// A trait that makes it easier to generically manage
/// converting a pin from its current state into some
/// other functional mode.  The configuration change
/// requires exclusive access to the Port hardware,
/// which is why this isn't simply the standard `Into`
/// trait.
pub trait IntoFunction<T> {
    /// Consume the pin and configure it to operate in
    /// the mode T.
    fn into_function(self, port: &mut Port) -> T;
}

impl<I, M, C> IntoFunction<Pin<I, Alternate<C>>> for Pin<I, M>
where
    I: PinId,
    M: PinMode,
    C: AlternateConfig,
{
    #[allow(unused_variables)]
    #[inline]
    fn into_function(self, port: &mut Port) -> Pin<I, Alternate<C>> {
        self.into_alternate()
    }
}

//==============================================================================
//  Conversions
//==============================================================================

/// Convert from a `v2::Pin` to a `v1::Pin`
impl<I, M> From<v2::Pin<I, M>> for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    fn from(pin: v2::Pin<I, M>) -> Pin<I, M> {
        Pin { pin }
    }
}

/// Convert from a `v1::Pin` to a `v2::Pin`
impl<I, M> From<Pin<I, M>> for v2::Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    fn from(pin: Pin<I, M>) -> v2::Pin<I, M> {
        pin.pin
    }
}

//==============================================================================
//  Embedded HAL traits
//==============================================================================

impl<I: PinId> Pin<I, Output<OpenDrain>> {
    /// Control state of the internal pull up
    ///
    /// This function shouldn't exist. It is not possible to enable a pull-up
    /// resistor in an output mode.
    #[allow(unused_variables)]
    #[inline]
    pub fn internal_pull_up(&mut self, port: &mut Port, on: bool) {
        unimplemented!();
    }
}

impl<I, M> Pin<I, Output<M>>
where
    I: PinId,
    M: OutputConfig,
{
    /// Toggle the logic level of the pin; if it is currently
    /// high, set it low and vice versa.
    #[inline]
    pub fn toggle(&mut self) {
        self.pin._toggle();
    }
}

#[cfg(feature = "unproven")]
impl<I, M> ToggleableOutputPin for Pin<I, Output<M>>
where
    I: PinId,
    M: OutputConfig,
{
    // TODO: switch to ! when it’s stable
    type Error = ();

    #[inline]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin._toggle();
        Ok(())
    }
}

#[cfg(feature = "unproven")]
impl<I: PinId> InputPin for Pin<I, Output<ReadableOpenDrain>> {
    // TODO: switch to ! when it’s stable
    type Error = ();

    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.pin._is_high())
    }

    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.pin._is_low())
    }
}

#[cfg(feature = "unproven")]
impl<I, M> InputPin for Pin<I, Input<M>>
where
    I: PinId,
    M: crate::gpio::v2::pin::InputConfig,
{
    // TODO: switch to ! when it’s stable
    type Error = ();

    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.pin._is_high())
    }

    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.pin._is_low())
    }
}

#[cfg(feature = "unproven")]
impl<I, M> StatefulOutputPin for Pin<I, Output<M>>
where
    I: PinId,
    M: OutputConfig,
{
    #[inline]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.pin._is_set_high())
    }

    #[inline]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.pin._is_set_low())
    }
}

impl<I, M> OutputPin for Pin<I, Output<M>>
where
    I: PinId,
    M: OutputConfig,
{
    // TODO: switch to ! when it’s stable
    type Error = ();

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin._set_high();
        Ok(())
    }

    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin._set_low();
        Ok(())
    }
}

//==============================================================================
//  Port & Parts
//==============================================================================

/// The GpioExt trait allows splitting the PORT hardware into
/// its constituent pin parts.
pub trait GpioExt {
    type Parts;

    /// Consume and split the device into its constitent parts
    fn split(self) -> Self::Parts;
}

/// Opaque port reference
pub struct Port {
    _0: (),
}

macro_rules! port {
    ([
       $(
           $( #[$cfg:meta] )?
           ($PinId:ident, $PinType:ident),
        )+
    ]) => {
        $crate::paste::item! {
            $(
                $( #[$cfg] )?
                /// Represents the IO pin with the matching name
                pub type $PinType<M> = Pin<v2::$PinId, M>;
            )+

            /// Holds the GPIO Port peripheral and broken out pin instances
            pub struct Parts {
                /// Opaque port reference
                pub port: Port,
                $(
                    $( #[$cfg] )?
                    pub [<$PinType:lower>]: $PinType<Input<Floating>>,
                )+
            }

            impl GpioExt for PORT {
                type Parts = Parts;

                /// Split the PORT peripheral into discrete pins
                fn split(self) -> Parts {
                    Parts {
                        port: Port {_0: ()},
                        // Safe because we only create one `Pin` per `PinId`
                        $(
                            $( #[$cfg] )?
                            [<$PinType:lower>]: unsafe { Pin::<v2::$PinId, _>::new() },
                        )+
                    }
                }
            }
        }
    };
}

port!([
    #[cfg(not(feature = "samd11"))]
    (PA00, Pa0),
    #[cfg(not(feature = "samd11"))]
    (PA01, Pa1),
    (PA02, Pa2),
    #[cfg(not(feature = "samd11c"))]
    (PA03, Pa3),
    (PA04, Pa4),
    (PA05, Pa5),
    #[cfg(not(feature = "samd11c"))]
    (PA06, Pa6),
    #[cfg(not(feature = "samd11c"))]
    (PA07, Pa7),
    (PA08, Pa8),
    (PA09, Pa9),
    #[cfg(not(feature = "samd11"))]
    (PA10, Pa10),
    #[cfg(not(feature = "samd11"))]
    (PA11, Pa11),
    #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
    (PA12, Pa12),
    #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
    (PA13, Pa13),
    (PA14, Pa14),
    (PA15, Pa15),
    #[cfg(not(feature = "samd11c"))]
    (PA16, Pa16),
    #[cfg(not(feature = "samd11"))]
    (PA17, Pa17),
    #[cfg(not(feature = "samd11"))]
    (PA18, Pa18),
    #[cfg(not(feature = "samd11"))]
    (PA19, Pa19),
    #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
    (PA20, Pa20),
    #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
    (PA21, Pa21),
    #[cfg(not(feature = "samd11c"))]
    (PA22, Pa22),
    #[cfg(not(feature = "samd11c"))]
    (PA23, Pa23),
    (PA24, Pa24),
    (PA25, Pa25),
    #[cfg(not(feature = "samd11"))]
    (PA27, Pa27),
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    (PA28, Pa28),
    (PA30, Pa30),
    (PA31, Pa31),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB00, Pb0),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB01, Pb1),
    #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
    (PB02, Pb2),
    #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
    (PB03, Pb3),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB04, Pb4),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB05, Pb5),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB06, Pb6),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB07, Pb7),
    #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
    (PB08, Pb8),
    #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
    (PB09, Pb9),
    #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
    (PB10, Pb10),
    #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
    (PB11, Pb11),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB12, Pb12),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB13, Pb13),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB14, Pb14),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB15, Pb15),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB16, Pb16),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB17, Pb17),
    #[cfg(any(feature = "min-samd51n"))]
    (PB18, Pb18),
    #[cfg(any(feature = "min-samd51n"))]
    (PB19, Pb19),
    #[cfg(any(feature = "min-samd51n"))]
    (PB20, Pb20),
    #[cfg(any(feature = "min-samd51n"))]
    (PB21, Pb21),
    #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
    (PB22, Pb22),
    #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
    (PB23, Pb23),
    #[cfg(any(feature = "min-samd51n"))]
    (PB24, Pb24),
    #[cfg(any(feature = "min-samd51n"))]
    (PB25, Pb25),
    #[cfg(any(feature = "min-samd51p"))]
    (PB26, Pb26),
    #[cfg(any(feature = "min-samd51p"))]
    (PB27, Pb27),
    #[cfg(any(feature = "min-samd51p"))]
    (PB28, Pb28),
    #[cfg(any(feature = "min-samd51p"))]
    (PB29, Pb29),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB30, Pb30),
    #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
    (PB31, Pb31),
    #[cfg(any(feature = "min-samd51n"))]
    (PC00, Pc0),
    #[cfg(any(feature = "min-samd51n"))]
    (PC01, Pc1),
    #[cfg(any(feature = "min-samd51n"))]
    (PC02, Pc2),
    #[cfg(any(feature = "min-samd51n"))]
    (PC03, Pc3),
    #[cfg(any(feature = "min-samd51p"))]
    (PC04, Pc4),
    #[cfg(any(feature = "min-samd51n"))]
    (PC05, Pc5),
    #[cfg(any(feature = "min-samd51n"))]
    (PC06, Pc6),
    #[cfg(any(feature = "min-samd51n"))]
    (PC07, Pc7),
    #[cfg(any(feature = "min-samd51n"))]
    (PC10, Pc10),
    #[cfg(any(feature = "min-samd51n"))]
    (PC11, Pc11),
    #[cfg(any(feature = "min-samd51n"))]
    (PC12, Pc12),
    #[cfg(any(feature = "min-samd51n"))]
    (PC13, Pc13),
    #[cfg(any(feature = "min-samd51n"))]
    (PC14, Pc14),
    #[cfg(any(feature = "min-samd51n"))]
    (PC15, Pc15),
    #[cfg(any(feature = "min-samd51n"))]
    (PC16, Pc16),
    #[cfg(any(feature = "min-samd51n"))]
    (PC17, Pc17),
    #[cfg(any(feature = "min-samd51n"))]
    (PC18, Pc18),
    #[cfg(any(feature = "min-samd51n"))]
    (PC19, Pc19),
    #[cfg(any(feature = "min-samd51n"))]
    (PC20, Pc20),
    #[cfg(any(feature = "min-samd51n"))]
    (PC21, Pc21),
    #[cfg(any(feature = "min-samd51p"))]
    (PC22, Pc22),
    #[cfg(any(feature = "min-samd51p"))]
    (PC23, Pc23),
    #[cfg(any(feature = "min-samd51n"))]
    (PC24, Pc24),
    #[cfg(any(feature = "min-samd51n"))]
    (PC25, Pc25),
    #[cfg(any(feature = "min-samd51n"))]
    (PC26, Pc26),
    #[cfg(any(feature = "min-samd51n"))]
    (PC27, Pc27),
    #[cfg(any(feature = "min-samd51n"))]
    (PC28, Pc28),
    #[cfg(any(feature = "min-samd51p"))]
    (PC30, Pc30),
    #[cfg(any(feature = "min-samd51p"))]
    (PC31, Pc31),
    #[cfg(any(feature = "min-samd51p"))]
    (PD00, Pd0),
    #[cfg(any(feature = "min-samd51p"))]
    (PD01, Pd1),
    #[cfg(any(feature = "min-samd51p"))]
    (PD08, Pd8),
    #[cfg(any(feature = "min-samd51p"))]
    (PD09, Pd9),
    #[cfg(any(feature = "min-samd51p"))]
    (PD10, Pd10),
    #[cfg(any(feature = "min-samd51p"))]
    (PD11, Pd11),
    #[cfg(any(feature = "min-samd51p"))]
    (PD12, Pd12),
    #[cfg(any(feature = "min-samd51p"))]
    (PD20, Pd20),
    #[cfg(any(feature = "min-samd51p"))]
    (PD21, Pd21),
]);

//==============================================================================
//  Define pins macro
//==============================================================================

/// This macro is a helper for defining a `Pins` type in a board support
/// crate.  This type is used to provide more meaningful aliases for the
/// various GPIO pins for a given board.
#[macro_export]
macro_rules! define_pins {
    ($(#[$topattr:meta])* struct $Type:ident,
     target_device: $target_device:ident,
     $( $(#[$attr:meta])* pin $name:ident = $pin_ident:ident),+ , ) => {

$crate::paste::item! {
    $(#[$topattr])*
    pub struct $Type {
        /// Opaque port reference
        pub port: Port,

        $(
            $(#[$attr])*
            pub $name: gpio::[<P $pin_ident>]<Input<Floating>>
        ),+
    }
}

impl $Type {
    /// Returns the pins for the device
    $crate::paste::item! {
        pub fn new(port: $target_device::PORT) -> Self {
            let pins = port.split();
            $Type {
                port: pins.port,
                $(
                $(#[$attr])*
                $name: pins.[<p $pin_ident>]
                ),+
            }
        }
    }
}
}}
