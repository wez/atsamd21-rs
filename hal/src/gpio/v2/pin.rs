//! # Type-level module for GPIO pins
//!
//! This module provides a type-level API for GPIO pins. It uses the type system
//! to track the state of pins at compile-time. Representing GPIO pins in this
//! manner incurs no run-time overhead. Each [`Pin`] struct is zero-sized, so
//! there is no data to copy around. Instead, real code is generated as a side
//! effect of type transformations, and the resulting assembly is nearly
//! identical to the equivalent, hand-written C.
//!
//! To track the state of pins at compile-time, this module uses traits to
//! represent [type classes] and types as instances of those type classes. For
//! example, the trait [`InputConfig`] acts as a [type-level enum] of the
//! available input configurations, and the types [`Floating`], [`PullDown`] and
//! [`PullUp`] are its type-level variants.
//!
//! Type-level [`Pin`]s are parameterized by two type-level enums, [`PinId`] and
//! [`PinMode`].
//!
//! ```
//! pub struct Pin<I, M>
//! where
//!     I: PinId,
//!     M: PinMode,
//! {
//!     // ...
//! }
//! ```
//!
//! A `PinId` identifies a pin by it's group (A, B, C or D) and pin number. Each
//! `PinId` instance is named according to its datasheet identifier, e.g.
//! [`PA02`].
//!
//! A `PinMode` represents the various pin modes. The available `PinMode`
//! variants are [`Disabled`], [`Input`], [`Interrupt`], [`Output`] and
//! [`Alternate`], each with its own corresponding configurations.
//!
//! It is not possible for users to create new instances of a [`Pin`]. Singleton
//! instances of each pin are made available to users through the [`Pins`]
//! struct.
//!
//! To create the [`Pins`] struct, users must supply the PAC
//! [`PORT`](crate::target_device::PORT) peripheral. The [`Pins`] struct takes
//! ownership of the [`PORT`] and provides the corresponding pins. Each [`Pin`]
//! within the [`Pins`] struct can be moved out and used individually.
//!
//!
//! ```
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! ```
//!
//! Pins can be converted between modes using several different methods.
//!
//! ```
//! // Use one of the literal function names
//! let pa27 = pins.pa27.into_floating_input();
//! // Use a generic method and one of the `PinMode` variant types
//! let pa27 = pins.pa27.into_mode::<FloatingInput>();
//! // Specify the target type and use `From`/`Into`
//! let pa27: Pin<PA27, FloatingInput> = pins.pa27.into();
//! ```
//!
//! # Embedded HAL traits
//!
//! This module implements all of the embedded HAL GPIO traits for each [`Pin`]
//! in the corresponding [`PinMode`]s, namely: [`InputPin`], [`OutputPin`],
//! [`ToggleableOutputPin`] and [`StatefulOutputPin`].
//!
//! For example, you can control the logic level of an `OutputPin` like so
//!
//! ```
//! use atsamd_hal::pac::Peripherals;
//! use atsamd_hal::gpio::v2::Pins;
//! use embedded_hal::digital::v2::OutputPin;
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let mut pins = Pins::new(peripherals.PORT);
//! pins.pa27.set_high();
//! ```
//!
//! # Type-level features
//!
//! This module also provides additional, type-level tools to work with GPIO
//! pins.
//!
//! The [`OptionalPinId`] and [`OptionalPin`] traits use the [`OptionalKind`]
//! pattern to act as type-level versions of [`Option`] for `PinId` and `Pin`
//! respectively. And the [`AnyPin`] trait defines an [`AnyKind`] type class
//! for all `Pin` types.
//!
//! [type classes]: crate::typelevel#type-classes
//! [type-level enum]: crate::typelevel#type-level-enum
//! [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
//! [`AnyKind`]: crate::typelevel#anykind-trait-pattern

#![allow(clippy::zero_prefixed_literal)]

use core::convert::Infallible;
use core::marker::PhantomData;
use core::mem::transmute;

use hal::digital::v2::OutputPin;
#[cfg(feature = "unproven")]
use hal::digital::v2::{InputPin, StatefulOutputPin, ToggleableOutputPin};
use paste::paste;

use crate::target_device::PORT;

use crate::typelevel::{NoneT, Sealed};

use super::dynpin::*;
use super::reg::RegisterInterface;

//==============================================================================
//  Disabled configurations
//==============================================================================

/// Type-level enum for disabled configurations
///
/// The valid options are [`Floating`], [`PullDown`] and [`PullUp`]. See the
/// [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait DisabledConfig: Sealed {
    /// Corresponding [`DynDisabled`](super::DynDisabled)
    const DYN: DynDisabled;
}

/// Type-level variant of both [`DisabledConfig`] and [`InputConfig`]
pub enum Floating {}
/// Type-level variant of both [`DisabledConfig`] and [`InputConfig`]
pub enum PullDown {}
/// Type-level variant of both [`DisabledConfig`] and [`InputConfig`]
pub enum PullUp {}

impl Sealed for Floating {}
impl Sealed for PullDown {}
impl Sealed for PullUp {}

impl DisabledConfig for Floating {
    const DYN: DynDisabled = DynDisabled::Floating;
}
impl DisabledConfig for PullDown {
    const DYN: DynDisabled = DynDisabled::PullDown;
}
impl DisabledConfig for PullUp {
    const DYN: DynDisabled = DynDisabled::PullUp;
}

/// Type-level variant of [`PinMode`] for disabled modes
///
/// Type `C` is one of three configurations: [`Floating`], [`PullDown`] or
/// [`PullUp`]
pub struct Disabled<C: DisabledConfig> {
    cfg: PhantomData<C>,
}

impl<C: DisabledConfig> Sealed for Disabled<C> {}

/// Type-level variant of [`PinMode`] for floating disabled mode
pub type FloatingDisabled = Disabled<Floating>;

/// Type-level variant of [`PinMode`] for pull-down disabled mode
pub type PullDownDisabled = Disabled<PullDown>;

/// Type-level variant of [`PinMode`] for pull-up disabled mode
pub type PullUpDisabled = Disabled<PullUp>;

/// Type alias for the [`PinMode`] at reset
pub type Reset = FloatingDisabled;

//==============================================================================
//  Input configurations
//==============================================================================

/// Type-level enum for input configurations
///
/// The valid options are [`Floating`], [`PullDown`] and [`PullUp`]. See the
/// [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait InputConfig: Sealed {
    /// Corresponding [`DynInput`](super::DynInput)
    const DYN: DynInput;
}

impl InputConfig for Floating {
    const DYN: DynInput = DynInput::Floating;
}
impl InputConfig for PullDown {
    const DYN: DynInput = DynInput::PullDown;
}
impl InputConfig for PullUp {
    const DYN: DynInput = DynInput::PullUp;
}

/// Type-level variant of [`PinMode`] for input modes
///
/// Type `C` is one of three input configurations: [`Floating`], [`PullDown`] or
/// [`PullUp`]
pub struct Input<C: InputConfig> {
    cfg: PhantomData<C>,
}

impl<C: InputConfig> Sealed for Input<C> {}

/// Type-level variant of [`PinMode`] for floating input mode
pub type FloatingInput = Input<Floating>;

/// Type-level variant of [`PinMode`] for pull-down input mode
pub type PullDownInput = Input<PullDown>;

/// Type-level variant of [`PinMode`] for pull-up input mode
pub type PullUpInput = Input<PullUp>;

//==============================================================================
//  Interrupt configurations
//==============================================================================

/// Type-level `enum` for Interrupt configurations
pub trait InterruptConfig: Sealed {
    /// Corresponding [`DynInterrupt`](super::DynInterrupt)
    const DYN: DynInterrupt;
}

impl InterruptConfig for Floating {
    const DYN: DynInterrupt = DynInterrupt::Floating;
}
impl InterruptConfig for PullDown {
    const DYN: DynInterrupt = DynInterrupt::PullDown;
}
impl InterruptConfig for PullUp {
    const DYN: DynInterrupt = DynInterrupt::PullUp;
}

/// Type-level variant of [`PinMode`] for Interrupt modes
///
/// Type `C` is one of three Interrupt configurations: [`Floating`],
/// [`PullDown`] or [`PullUp`]
pub struct Interrupt<C: InterruptConfig> {
    cfg: PhantomData<C>,
}

impl<C: InterruptConfig> Sealed for Interrupt<C> {}

/// Type-level variant of [`PinMode`] for floating Interrupt mode
pub type FloatingInterrupt = Interrupt<Floating>;

/// Type-level variant of [`PinMode`] for pull-down Interrupt mode
pub type PullDownInterrupt = Interrupt<PullDown>;

/// Type-level variant of [`PinMode`] for pull-up Interrupt mode
pub type PullUpInterrupt = Interrupt<PullUp>;

//==============================================================================
//  Output configurations
//==============================================================================

/// Type-level enum for output configurations
///
/// The valid options are [`PushPull`] and [`Readable`]. See the [type-level
/// enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait OutputConfig: Sealed {
    /// Corresponding [`DynOutput`](super::DynOutput)
    const DYN: DynOutput;
}

/// Type-level variant of [`OutputConfig`] for a push-pull configuration
pub enum PushPull {}
/// Type-level variant of [`OutputConfig`] for a readable push-pull
/// configuration
pub enum Readable {}

impl Sealed for PushPull {}
impl Sealed for Readable {}

impl OutputConfig for PushPull {
    const DYN: DynOutput = DynOutput::PushPull;
}
impl OutputConfig for Readable {
    const DYN: DynOutput = DynOutput::Readable;
}

/// Type-level variant of [`PinMode`] for output modes
///
/// Type `C` is one of two output configurations: [`PushPull`] or [`Readable`]
pub struct Output<C: OutputConfig> {
    cfg: PhantomData<C>,
}

impl<C: OutputConfig> Sealed for Output<C> {}

/// Type-level variant of [`PinMode`] for push-pull output mode
pub type PushPullOutput = Output<PushPull>;

/// Type-level variant of [`PinMode`] for readable push-pull output mode
pub type ReadableOutput = Output<Readable>;

//==============================================================================
//  Alternate configurations
//==============================================================================

/// Type-level enum for alternate peripheral function configurations
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait AlternateConfig: Sealed {
    /// Corresponding [`DynAlternate`](super::DynAlternate)
    const DYN: DynAlternate;
}

macro_rules! alternate {
    (
        $(
            $Letter:ident
        ),+
    ) => {
        paste! {
            $(
                #[
                    doc = "Type-level variant of [`AlternateConfig`] for \
                    alternate peripheral function " $Letter
                ]
                pub enum $Letter {}
                impl Sealed for $Letter {}
                impl AlternateConfig for $Letter {
                    const DYN: DynAlternate = DynAlternate::$Letter;
                }
                #[
                    doc = "Type-level variant of [`PinMode`] for alternate \
                    peripheral function [`" $Letter "`]"
                ]
                pub type [<Alternate $Letter>] = Alternate<$Letter>;
            )+
        }
    };
}

alternate!(B, C, D, E, F, G);

#[cfg(any(feature = "samd21", feature = "min-samd51g"))]
alternate!(H);

#[cfg(feature = "min-samd51g")]
alternate!(I, J, K, L, M, N);

/// Type-level variant of [`PinMode`] for alternate peripheral functions
///
/// Type `C` is an [`AlternateConfig`]
pub struct Alternate<C: AlternateConfig> {
    cfg: PhantomData<C>,
}

impl<C: AlternateConfig> Sealed for Alternate<C> {}

//==============================================================================
//  Pin modes
//==============================================================================

/// Type-level enum representing pin modes
///
/// The valid options are [`Disabled`], [`Input`], [`Output`] and [`Alternate`].
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait PinMode: Sealed {
    /// Corresponding [`DynPinMode`](super::DynPinMode)
    const DYN: DynPinMode;
}

impl<C: DisabledConfig> PinMode for Disabled<C> {
    const DYN: DynPinMode = DynPinMode::Disabled(C::DYN);
}

impl<C: InputConfig> PinMode for Input<C> {
    const DYN: DynPinMode = DynPinMode::Input(C::DYN);
}

impl<C: InterruptConfig> PinMode for Interrupt<C> {
    const DYN: DynPinMode = DynPinMode::Interrupt(C::DYN);
}

impl<C: OutputConfig> PinMode for Output<C> {
    const DYN: DynPinMode = DynPinMode::Output(C::DYN);
}

impl<C: AlternateConfig> PinMode for Alternate<C> {
    const DYN: DynPinMode = DynPinMode::Alternate(C::DYN);
}

//==============================================================================
//  Pin IDs
//==============================================================================

/// Type-level enum for pin IDs
///
/// Valid options take the form `PXYY`, where `X` is a letter in `A`-`D` and
/// `YY` is a number between 00-31. See the [type-level enum] documentation for
/// more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait PinId: Sealed {
    /// Corresponding [`DynPinId`](super::DynPinId)
    const DYN: DynPinId;
}

macro_rules! pin_id {
    ($Group:ident, $Id:ident, $NUM:literal) => {
        paste! {
            #[doc = "Pin ID representing pin " $Id]
            pub enum $Id {}
            impl Sealed for $Id {}
            impl PinId for $Id {
                const DYN: DynPinId = DynPinId {
                    group: DynGroup::$Group,
                    num: $NUM,
                };
            }
        }
    };
}

//==============================================================================
//  OptionalPinId
//==============================================================================

/// Type-level equivalent of `Option<PinId>`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait OptionalPinId {}

impl OptionalPinId for NoneT {}

impl<I: PinId> OptionalPinId for I {}

/// Type-level equivalent of `Some(PinId)`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait SomePinId: OptionalPinId + PinId {}

impl<I: PinId> SomePinId for I {}

//==============================================================================
//  Registers
//==============================================================================

/// Provide a safe register interface for [`Pin`]s
///
/// This `struct` takes ownership of a [`PinId`] and provides an API to
/// access the corresponding regsiters.
struct Registers<I: PinId> {
    id: PhantomData<I>,
}

// [`Registers`] takes ownership of the [`PinId`], and [`Pin`] guarantees that
// each pin is a singleton, so this implementation is safe.
unsafe impl<I: PinId> RegisterInterface for Registers<I> {
    #[inline]
    fn id(&self) -> DynPinId {
        I::DYN
    }
}

impl<I: PinId> Registers<I> {
    /// Create a new instance of [`Registers`]
    ///
    /// # Safety
    ///
    /// Users must never create two simultaneous instances of this `struct` with
    /// the same [`PinId`]
    #[inline]
    unsafe fn new() -> Self {
        Registers { id: PhantomData }
    }

    /// Provide a type-level equivalent for the
    /// [`RegisterInterface::change_mode`] method.
    #[inline]
    fn change_mode<M: PinMode>(&mut self) {
        RegisterInterface::change_mode(self, M::DYN);
    }
}

//==============================================================================
//  Pin
//==============================================================================

/// A type-level GPIO pin, parameterized by [`PinId`] and [`PinMode`] types
pub struct Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    regs: Registers<I>,
    mode: PhantomData<M>,
}

impl<I, M> Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    /// Create a new [`Pin`]
    ///
    /// # Safety
    ///
    /// Each [`Pin`] must be a singleton. For a given [`PinId`], there must be
    /// at most one corresponding [`Pin`] in existence at any given time.
    /// Violating this requirement is `unsafe`.
    #[inline]
    pub(crate) unsafe fn new() -> Pin<I, M> {
        Pin {
            regs: Registers::new(),
            mode: PhantomData,
        }
    }

    /// Convert the pin to the requested [`PinMode`]
    #[inline]
    pub fn into_mode<N: PinMode>(mut self) -> Pin<I, N> {
        // Only modify registers if we are actually changing pin mode
        // This check should compile away
        if N::DYN != M::DYN {
            self.regs.change_mode::<N>();
        }
        // Safe because we drop the existing Pin
        unsafe { Pin::new() }
    }

    /// Disable the pin and set it to float
    #[inline]
    pub fn into_floating_disabled(self) -> Pin<I, FloatingDisabled> {
        self.into_mode()
    }

    /// Disable the pin and set it to pull down
    #[inline]
    pub fn into_pull_down_disabled(self) -> Pin<I, PullDownDisabled> {
        self.into_mode()
    }

    /// Disable the pin and set it to pull up
    #[inline]
    pub fn into_pull_up_disabled(self) -> Pin<I, PullUpDisabled> {
        self.into_mode()
    }

    /// Configure the pin to operate as a floating input
    #[inline]
    pub fn into_floating_input(self) -> Pin<I, FloatingInput> {
        self.into_mode()
    }

    /// Configure the pin to operate as a pulled down input
    #[inline]
    pub fn into_pull_down_input(self) -> Pin<I, PullDownInput> {
        self.into_mode()
    }

    /// Configure the pin to operate as a pulled up input
    #[inline]
    pub fn into_pull_up_input(self) -> Pin<I, PullUpInput> {
        self.into_mode()
    }

    /// Configure the pin to operate as a floating interrupt
    #[inline]
    pub fn into_floating_interrupt(self) -> Pin<I, FloatingInterrupt> {
        self.into_mode()
    }

    /// Configure the pin to operate as a pulled down interrupt
    #[inline]
    pub fn into_pull_down_interrupt(self) -> Pin<I, PullDownInterrupt> {
        self.into_mode()
    }

    /// Configure the pin to operate as a pulled up interrupt
    #[inline]
    pub fn into_pull_up_interrupt(self) -> Pin<I, PullUpInterrupt> {
        self.into_mode()
    }

    /// Configure the pin to operate as a push-pull output
    #[inline]
    pub fn into_push_pull_output(self) -> Pin<I, PushPullOutput> {
        self.into_mode()
    }

    /// Configure the pin to operate as a readable push pull output
    #[inline]
    pub fn into_readable_output(self) -> Pin<I, ReadableOutput> {
        self.into_mode()
    }

    /// Configure the pin to operate as the corresponding peripheral function.
    ///
    /// The type `C` indicates the desired peripheral function.
    #[inline]
    pub fn into_alternate<C: AlternateConfig>(self) -> Pin<I, Alternate<C>> {
        self.into_mode()
    }

    /// Read the current drive strength of the pin.
    ///
    /// The drive strength is reset to normal on every change in pin mode.
    #[inline]
    pub fn get_drive_strength(&self) -> bool {
        self.regs.read_drive_strength()
    }

    /// Set the drive strength for the pin.
    ///
    /// The drive strength is reset to normal on every change in pin mode.
    #[inline]
    pub fn set_drive_strength(&mut self, stronger: bool) {
        self.regs.write_drive_strength(stronger);
    }

    #[inline]
    pub(crate) fn _is_low(&self) -> bool {
        self.regs.read_pin() == false
    }

    #[inline]
    pub(crate) fn _is_high(&self) -> bool {
        self.regs.read_pin() == true
    }

    #[inline]
    pub(crate) fn _set_low(&mut self) {
        self.regs.write_pin(false);
    }

    #[inline]
    pub(crate) fn _set_high(&mut self) {
        self.regs.write_pin(true);
    }

    #[inline]
    pub(crate) fn _toggle(&mut self) {
        self.regs.toggle_pin();
    }

    #[inline]
    pub(crate) fn _is_set_low(&self) -> bool {
        self.regs.read_out_pin() == false
    }

    #[inline]
    pub(crate) fn _is_set_high(&self) -> bool {
        self.regs.read_out_pin() == true
    }
}

//==============================================================================
//  PinMode conversions
//==============================================================================

/// Use a recursive macro to implement [`From`](core::convert::From) for each
/// pair of [`PinMode`]s. A macro is necessary to avoid conflicting with the
/// reflexive implementation in [`core::convert`], i.e. `impl<T> From<T> for T`.
macro_rules! impl_core_convert_from {
    (
        $( #[$cfg1:meta] )?
        $Mode1:ident,
    ) => {};
    (
        #[$cfg1:meta]
        $Mode1:ident,
        $(
            $( #[$cfg2:meta] )?
            $Mode2:ident,
        )*
    ) => {
        #[$cfg1]
        impl_core_convert_from!(
            $Mode1,
            $(
                $( #[$cfg2] )?
                $Mode2,
            )*
        );
    };
    (
        $Mode1:ident,
        $(
            $( #[$cfg2:meta] )?
            $Mode2:ident,
        )*
    ) => {
        paste! {
            $(
                $( #[$cfg2] )?
                impl<I> From<Pin<I, $Mode1>> for Pin<I, $Mode2>
                where
                    I: PinId,
                {
                    #[doc = "Convert from [`" $Mode1 "`] to [`" $Mode2 "`]"]
                    #[inline]
                    fn from(pin: Pin<I, $Mode1>) -> Self {
                        pin.into_mode()
                    }
                }
                $( #[$cfg2] )?
                impl<I> From<Pin<I, $Mode2>> for Pin<I, $Mode1>
                where
                    I: PinId,
                {
                    #[doc = "Convert from [`" $Mode2 "`] to [`" $Mode1 "`]"]
                    #[inline]
                    fn from(pin: Pin<I, $Mode2>) -> Self {
                        pin.into_mode()
                    }
                }
            )*
            impl_core_convert_from!(
                $(
                    $( #[$cfg2] )?
                    $Mode2,
                )*
            );
        }
    };
}

impl_core_convert_from!(
    FloatingDisabled,
    PullDownDisabled,
    PullUpDisabled,
    FloatingInput,
    PullDownInput,
    PullUpInput,
    PushPullOutput,
    ReadableOutput,
    FloatingInterrupt,
    PullUpInterrupt,
    PullDownInterrupt,
    AlternateB,
    AlternateC,
    AlternateD,
    AlternateE,
    AlternateF,
    AlternateG,
    #[cfg(any(feature = "samd21", feature = "min-samd51g"))]
    AlternateH,
    #[cfg(feature = "min-samd51g")]
    AlternateI,
    #[cfg(feature = "min-samd51g")]
    AlternateJ,
    #[cfg(feature = "min-samd51g")]
    AlternateK,
    #[cfg(feature = "min-samd51g")]
    AlternateL,
    #[cfg(feature = "min-samd51g")]
    AlternateM,
    #[cfg(feature = "min-samd51g")]
    AlternateN,
);

//==============================================================================
//  AnyPin
//==============================================================================

/// Type class for [`Pin`] types
///
/// This trait uses the [`AnyKind`] trait pattern to create a [type class] for
/// [`Pin`] types. See the `AnyKind` documentation for more details on the
/// pattern.
///
/// ## `v1` Compatibility
///
/// Normally, this trait would use Is<Type = SpecificPin<Self>> as a super
/// trait. But doing so would restrict implementations to only the `v2` `Pin`
/// type in this module. To aid in backwards compatibility, we want to implement
/// `AnyPin` for the `v1` `Pin` type as well. This is possible for a few
/// reasons. First, both structs are zero-sized, so there is no meaningful
/// memory layout to begin with. And even if there were, the `v1` `Pin` type is
/// a newtype wrapper around a `v2` `Pin`, and single-field structs are
/// guaranteed to have the same layout as the field, even for `repr(Rust)`.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait AnyPin
where
    Self: Sealed,
    Self: From<SpecificPin<Self>>,
    Self: Into<SpecificPin<Self>>,
    Self: AsRef<SpecificPin<Self>>,
    Self: AsMut<SpecificPin<Self>>,
{
    /// [`PinId`] of the corresponding [`Pin`]
    type Id: PinId;
    /// [`PinMode`] of the corresponding [`Pin`]
    type Mode: PinMode;
}

impl<I, M> Sealed for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
}

impl<I, M> AnyPin for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    type Id = I;
    type Mode = M;
}

/// Type alias to recover the specific [`Pin`] type from an implementation of
/// [`AnyPin`]
///
/// See the [`AnyKind`] documentation for more details on the pattern.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
pub type SpecificPin<P> = Pin<<P as AnyPin>::Id, <P as AnyPin>::Mode>;

impl<P: AnyPin> AsRef<P> for SpecificPin<P> {
    #[inline]
    fn as_ref(&self) -> &P {
        // SAFETY: This is guaranteed to be safe, because P == SpecificPin<P>
        // Transmuting between `v1` and `v2` `Pin` types is also safe, because
        // both are zero-sized, and single-field, newtype structs are guaranteed
        // to have the same layout as the field anyway, even for repr(Rust).
        unsafe { transmute(self) }
    }
}

impl<P: AnyPin> AsMut<P> for SpecificPin<P> {
    #[inline]
    fn as_mut(&mut self) -> &mut P {
        // SAFETY: This is guaranteed to be safe, because P == SpecificPin<P>
        // Transmuting between `v1` and `v2` `Pin` types is also safe, because
        // both are zero-sized, and single-field, newtype structs are guaranteed
        // to have the same layout as the field anyway, even for repr(Rust).
        unsafe { transmute(self) }
    }
}

//==============================================================================
//  Optional pins
//==============================================================================

/// Type-level equivalent of `Option<PinId>`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait OptionalPin: Sealed {
    type Id: OptionalPinId;
}

impl OptionalPin for NoneT {
    type Id = NoneT;
}

impl<P: AnyPin> OptionalPin for P {
    type Id = P::Id;
}

/// Type-level equivalent of `Some(PinId)`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait SomePin: AnyPin {}
impl<P: AnyPin> SomePin for P {}

//==============================================================================
//  Embedded HAL traits
//==============================================================================

impl<I, C> OutputPin for Pin<I, Output<C>>
where
    I: PinId,
    C: OutputConfig,
{
    type Error = Infallible;
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self._set_high();
        Ok(())
    }
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self._set_low();
        Ok(())
    }
}

#[cfg(feature = "unproven")]
impl<I> InputPin for Pin<I, ReadableOutput>
where
    I: PinId,
{
    type Error = Infallible;
    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self._is_high())
    }
    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self._is_low())
    }
}

#[cfg(feature = "unproven")]
impl<I, C> InputPin for Pin<I, Input<C>>
where
    I: PinId,
    C: InputConfig,
{
    type Error = Infallible;
    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self._is_high())
    }
    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self._is_low())
    }
}

#[cfg(feature = "unproven")]
impl<I, C> ToggleableOutputPin for Pin<I, Output<C>>
where
    I: PinId,
    C: OutputConfig,
{
    type Error = Infallible;
    #[inline]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self._toggle();
        Ok(())
    }
}

#[cfg(feature = "unproven")]
impl<I, C> StatefulOutputPin for Pin<I, Output<C>>
where
    I: PinId,
    C: OutputConfig,
{
    #[inline]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self._is_set_high())
    }
    #[inline]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self._is_set_low())
    }
}

//==============================================================================
//  Pin definitions
//==============================================================================

macro_rules! pins{
    (
        $(
            $( #[$cfg:meta] )?
            $Id:ident,
        )+
    ) => {
        paste! {
            /// Collection of all the individual [`Pin`]s
            pub struct Pins {
                port: Option<PORT>,
                $(
                    #[doc = "Pin " $Id]
                    $( #[$cfg] )?
                    pub [<$Id:lower>]: Pin<$Id, Reset>,
                )+
            }
            impl Pins {
                /// Take ownership of the PAC
                /// [`PORT`](crate::target_device::PORT) and split it into
                /// discrete [`Pin`]s
                #[inline]
                pub fn new(port: PORT) -> Pins {
                    Pins {
                        port: Some(port),
                        // Safe because we only create one `Pin` per `PinId`
                        $(
                            $( #[$cfg] )?
                            [<$Id:lower>]: unsafe { Pin::new() },
                        )+
                    }
                }
                /// Take the PAC [`PORT`]
                ///
                /// The [`PORT`] can only be taken once. Subsequent calls to
                /// this function will panic.
                ///
                /// # Safety
                ///
                /// Direct access to the [`PORT`] could allow you to invalidate
                /// the compiler's type-level tracking, so it is unsafe.
                ///
                /// [`PORT`](crate::target_device::PORT)
                #[inline]
                pub unsafe fn port(&mut self) -> PORT {
                    self.port.take().unwrap()
                }
            }
        }
    };
}

macro_rules! declare_pins {
    (
        $(
            $Group:ident {
                $(
                    $( #[$cfg:meta] )?
                    ($Id:ident, $NUM:literal),
                )+
            }
        )+
    ) => {
        $(
            $(
                $( #[$cfg] )?
                pin_id!($Group, $Id, $NUM);
            )+
        )+
        pins!(
            $(
                $(
                    $( #[$cfg] )?
                    $Id,
                )+
            )+
        );
    };
}

declare_pins!(
    A {
        #[cfg(not(feature = "samd11"))]
        (PA00, 00),
        #[cfg(not(feature = "samd11"))]
        (PA01, 01),
        (PA02, 02),
        #[cfg(not(feature = "samd11c"))]
        (PA03, 03),
        (PA04, 04),
        (PA05, 05),
        #[cfg(not(feature = "samd11c"))]
        (PA06, 06),
        #[cfg(not(feature = "samd11c"))]
        (PA07, 07),
        (PA08, 08),
        (PA09, 09),
        #[cfg(not(feature = "samd11"))]
        (PA10, 10),
        #[cfg(not(feature = "samd11"))]
        (PA11, 11),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PA12, 12),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PA13, 13),
        (PA14, 14),
        (PA15, 15),
        #[cfg(not(feature = "samd11c"))]
        (PA16, 16),
        #[cfg(not(feature = "samd11"))]
        (PA17, 17),
        #[cfg(not(feature = "samd11"))]
        (PA18, 18),
        #[cfg(not(feature = "samd11"))]
        (PA19, 19),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PA20, 20),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PA21, 21),
        #[cfg(not(feature = "samd11c"))]
        (PA22, 22),
        #[cfg(not(feature = "samd11c"))]
        (PA23, 23),
        (PA24, 24),
        (PA25, 25),
        #[cfg(not(feature = "samd11"))]
        (PA27, 27),
        #[cfg(any(feature = "samd11", feature = "samd21"))]
        (PA28, 28),
        (PA30, 30),
        (PA31, 31),
    }
    B {
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB00, 00),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB01, 01),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB02, 02),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB03, 03),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB04, 04),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB05, 05),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB06, 06),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB07, 07),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB08, 08),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB09, 09),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB10, 10),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB11, 11),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB12, 12),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB13, 13),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB14, 14),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB15, 15),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB16, 16),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB17, 17),
        #[cfg(feature = "min-samd51n")]
        (PB18, 18),
        #[cfg(feature = "min-samd51n")]
        (PB19, 19),
        #[cfg(feature = "min-samd51n")]
        (PB20, 20),
        #[cfg(feature = "min-samd51n")]
        (PB21, 21),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB22, 22),
        #[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
        (PB23, 23),
        #[cfg(feature = "min-samd51n")]
        (PB24, 24),
        #[cfg(feature = "min-samd51n")]
        (PB25, 25),
        #[cfg(feature = "min-samd51p")]
        (PB26, 26),
        #[cfg(feature = "min-samd51p")]
        (PB27, 27),
        #[cfg(feature = "min-samd51p")]
        (PB28, 28),
        #[cfg(feature = "min-samd51p")]
        (PB29, 29),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB30, 30),
        #[cfg(any(feature = "min-samd21j", feature = "min-samd51j"))]
        (PB31, 31),
    }
    C {
        #[cfg(feature = "min-samd51n")]
        (PC00, 00),
        #[cfg(feature = "min-samd51n")]
        (PC01, 01),
        #[cfg(feature = "min-samd51n")]
        (PC02, 02),
        #[cfg(feature = "min-samd51n")]
        (PC03, 03),
        #[cfg(feature = "min-samd51p")]
        (PC04, 04),
        #[cfg(feature = "min-samd51n")]
        (PC05, 05),
        #[cfg(feature = "min-samd51n")]
        (PC06, 06),
        #[cfg(feature = "min-samd51n")]
        (PC07, 07),
        #[cfg(feature = "min-samd51n")]
        (PC10, 10),
        #[cfg(feature = "min-samd51n")]
        (PC11, 11),
        #[cfg(feature = "min-samd51n")]
        (PC12, 12),
        #[cfg(feature = "min-samd51n")]
        (PC13, 13),
        #[cfg(feature = "min-samd51n")]
        (PC14, 14),
        #[cfg(feature = "min-samd51n")]
        (PC15, 15),
        #[cfg(feature = "min-samd51n")]
        (PC16, 16),
        #[cfg(feature = "min-samd51n")]
        (PC17, 17),
        #[cfg(feature = "min-samd51n")]
        (PC18, 18),
        #[cfg(feature = "min-samd51n")]
        (PC19, 19),
        #[cfg(feature = "min-samd51n")]
        (PC20, 20),
        #[cfg(feature = "min-samd51n")]
        (PC21, 21),
        #[cfg(feature = "min-samd51p")]
        (PC22, 22),
        #[cfg(feature = "min-samd51p")]
        (PC23, 23),
        #[cfg(feature = "min-samd51n")]
        (PC24, 24),
        #[cfg(feature = "min-samd51n")]
        (PC25, 25),
        #[cfg(feature = "min-samd51n")]
        (PC26, 26),
        #[cfg(feature = "min-samd51n")]
        (PC27, 27),
        #[cfg(feature = "min-samd51n")]
        (PC28, 28),
        #[cfg(feature = "min-samd51p")]
        (PC30, 30),
        #[cfg(feature = "min-samd51p")]
        (PC31, 31),
    }
    D {
        #[cfg(feature = "min-samd51p")]
        (PD00, 00),
        #[cfg(feature = "min-samd51p")]
        (PD01, 01),
        #[cfg(feature = "min-samd51p")]
        (PD08, 08),
        #[cfg(feature = "min-samd51p")]
        (PD09, 09),
        #[cfg(feature = "min-samd51p")]
        (PD10, 10),
        #[cfg(feature = "min-samd51p")]
        (PD11, 11),
        #[cfg(feature = "min-samd51p")]
        (PD12, 12),
        #[cfg(feature = "min-samd51p")]
        (PD20, 20),
        #[cfg(feature = "min-samd51p")]
        (PD21, 21),
    }
);

//==============================================================================
//  bsp_pins
//==============================================================================

/// Helper macro to give meaningful names to GPIO pins
///
/// The normal [`Pins`] struct names each [`Pin`] according to its [`PinId`].
/// However, BSP authors would prefer to name each [`Pin`] according to its
/// function. This macro defines a new `Pins` struct with custom field names
/// for each [`Pin`], and it defines type aliases and constants to make it
/// easier to work with the [`Pin`]s and [`DynPin`](super::DynPin)s.
///
/// When specifying pin aliases, be sure to use a [`PinMode`]. See
/// [here](self#types) for a list of the available [`PinMode`] type aliases.
///
/// # Example
///
/// The following example macro call
///
/// ```
/// atsamd_hal::bsp_pins!(
///     #[cfg(feature = "unproven")]
///     PA24 {
///         name: led_pass,
///         aliases: {
///             AlternateH: LedPass,
///             #[cfg(feature = "usb")]
///             AlternateM: UsbPin
///         }
///     }
///     PA25 {
///         name: led_fail
///     }
/// );
/// ```
///
/// would expand to something like this
///
/// ```
/// pub struct Pins {
///     port: Option<PORT>,
///     #[cfg(feature = "unproven")]
///     pub led_pass: Pin<PA24, Reset>,
///     pub led_fail: Pin<PA25, Reset>,
/// }
///
/// impl Pins {
///
///     pub fn new(port: PORT) -> Self {
///         let pins = gpio::Pins::new(port);
///         Self {
///             port: Some(unsafe { pins.port() }),
///             #[cfg(feature = "unproven")]
///             led_pass: pins.pa24,
///             led_fail: pins.pa25,
///         }
///     }
///
///     #[inline]
///     pub unsafe fn port(&mut self) -> PORT {
///         self.port.take().unwrap()
///     }
/// }
///
/// #[cfg(feature = "unproven")]
/// pub type LedPass = Pin<PA24, AlternateH>;
///
/// #[cfg(feature = "unproven")]
/// pub const LED_PASS_ID: DynPinId = <PA24 as PinId>::DYN;
///
/// #[cfg(feature = "unproven")]
/// pub const LED_PASS_MODE: DynPinMode = <AlternateH as PinMode>::DYN;
///
/// #[cfg(feature = "unproven")]
/// #[cfg(feature = "usb")]
/// pub type UsbPin = Pin<PA24, AlternateM>;
///
/// #[cfg(feature = "unproven")]
/// #[cfg(feature = "usb")]
/// pub const USB_PIN_ID: DynPinId = <PA24 as PinId>::DYN;
///
/// #[cfg(feature = "unproven")]
/// #[cfg(feature = "usb")]
/// pub const USB_PIN_MODE: DynPinMode = <AlternateM as PinMode>::DYN;
/// ```
#[macro_export]
macro_rules! bsp_pins {
    (
        $(
            $( #[$id_cfg:meta] )*
            $Id:ident {
                $( #[$name_doc:meta] )*
                name: $name:ident $(,)?
                $(
                    aliases: {
                        $(
                            $( #[$alias_cfg:meta] )*
                            $Mode:ident: $Alias:ident
                        ),+
                    }
                )?
            } $(,)?
        )+
    ) => {
        $crate::paste::paste! {

            /// BSP replacement for the HAL
            /// [`Pins`](atsamd_hal::gpio::v2::Pins) type
            ///
            /// This type is intended to provide more meaningful names for the
            /// given pins.
            pub struct Pins {
                port: Option<$crate::target_device::PORT>,
                $(
                    $( #[$id_cfg] )*
                    $( #[$name_doc] )*
                    pub $name: $crate::gpio::v2::Pin<
                        $crate::gpio::v2::$Id,
                        $crate::gpio::v2::Reset
                    >,
                )+
            }

            impl Pins {

                /// Take ownership of the PAC [`PORT`] and split it into
                /// discrete [`Pin`]s.
                ///
                /// This struct serves as a replacement for the HAL [`Pins`]
                /// struct. It is intended to provide more meaningful names for
                /// each [`Pin`] in a BSP. Any [`Pin`] not defined by the BSP is
                /// dropped.
                ///
                /// [`PORT`](atsamd_hal::target_device::PORT)
                /// [`Pin`](atsamd_hal::gpio::v2::Pin)
                /// [`Pins`](atsamd_hal::gpio::v2::Pins)
                #[inline]
                pub fn new(port: $crate::target_device::PORT) -> Self {
                    let mut pins = $crate::gpio::v2::Pins::new(port);
                    Self {
                        port: Some(unsafe{ pins.port() }),
                        $(
                            $( #[$id_cfg] )*
                            $name: pins.[<$Id:lower>],
                        )+
                    }
                }

                /// Take the PAC [`PORT`]
                ///
                /// The [`PORT`] can only be taken once. Subsequent calls to
                /// this function will panic.
                ///
                /// # Safety
                ///
                /// Direct access to the [`PORT`] could allow you to invalidate
                /// the compiler's type-level tracking, so it is unsafe.
                ///
                /// [`PORT`](atsamd_hal::target_device::PORT)
                #[inline]
                pub unsafe fn port(&mut self) -> $crate::target_device::PORT {
                    self.port.take().unwrap()
                }
            }
            $(
                $( #[$id_cfg] )*
                $crate::bsp_pins!(@aliases, $( $( $( #[$alias_cfg] )* $Id $Mode $Alias )+ )? );
            )+
        }
    };
    ( @aliases, $( $( $( #[$attr:meta] )* $Id:ident $Mode:ident $Alias:ident )+ )? ) => {
        $crate::paste::paste! {
            $(
                $(
                    $( #[$attr] )*
                    /// Alias for a configured [`Pin`](atsamd_hal::gpio::v2::Pin)
                    pub type $Alias = $crate::gpio::v2::Pin<
                        $crate::gpio::v2::$Id,
                        $crate::gpio::v2::$Mode
                    >;

                    $( #[$attr] )*
                    #[doc = "[DynPinId](atsamd_hal::gpio::v2::DynPinId) "]
                    #[doc = "for the `" $Alias "` alias."]
                    pub const [<$Alias:snake:upper _ID>]: $crate::gpio::v2::DynPinId =
                    <$crate::gpio::v2::$Id as $crate::gpio::v2::PinId>::DYN;

                    $( #[$attr] )*
                    #[doc = "[DynPinMode](atsamd_hal::gpio::v2::DynPinMode) "]
                    #[doc = "for the `" $Alias "` alias."]
                    pub const [<$Alias:snake:upper _MODE>]: $crate::gpio::v2::DynPinMode =
                    <$crate::gpio::v2::$Mode as $crate::gpio::v2::PinMode>::DYN;
                )+
            )?
        }
    };
}
