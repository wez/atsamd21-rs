//! Type-level tools to configure SERCOM pads
//!
//! This module helps configure [`Pin`]s as SERCOM pads. It provides type-level
//! tools to convert `Pin`s to the correct [`PinMode`] and to enforce type-level
//! constraints at compile-time.
//!
//! # Overview
//!
//! A SERCOM pad is defined by two types, its corresponding [`Sercom`] instance
//! and its [`PadNum`], from [`Pad0`] to [`Pad3`]. However, a given SERCOM pad
//! can usually be mapped to several possible [`PinId`]s.
//!
//! There are two primary traits defined in this module:
//! - The [`IsPad`] trait is implemented on `Pin` types that are properly
//!   configured as SERCOM pads, with `PinMode` [`AlternateC`] or
//!   [`AlternateD`]. It acts as both a [type class] for SERCOM pads and as a
//!   [type-level function] to recover the corresponding [`Sercom`] and
//!   [`PadNum`] types from the `Pin`.
//! - The [`GetPad`] trait maps each [`PinId`] to its corresponding, pad-related
//!   types. The [`PadMode`] alias uses `GetPad` to recover the corresponding
//!   `PinMode` for a given SERCOM pad, while the [`Pad`] alias recovers the
//!   configured [`Pin`] type.
//!
//! [`AlternateC`]: crate::gpio::v2::AlternateC
//! [`AlternateD`]: crate::gpio::v2::AlternateD
//! [type class]: crate::typelevel#type-classes
//! [type-level function]: crate::typelevel#type-level-functions
#![cfg_attr(
    feature = "min-samd51g",
    doc = "
# IOSET\n
\n
SAMx5x chips do not allow arbitrary combinations of `PinId` for a given
SERCOM. Instead, all `PinId`s must belong to the same IOSET. This module
defines a [type-level enum], [`IoSet`], to enforce this restriction, and the
[`InIoSet`] [type class] is responsible for labeling each `IsPad` type with
its corresponding, valid `IoSet`\\(s).\n
\n
"
)]

use paste::paste;
use seq_macro::seq;

use super::Sercom;
#[cfg(not(feature = "samd11"))]
use crate::gpio::v2::OptionalPinId;
use crate::gpio::v2::{AnyPin, OptionalPin, Pin, PinId, PinMode};
use crate::typelevel::{NoneT, Sealed};

#[cfg(any(feature = "samd11", feature = "samd21"))]
#[path = "pad/impl_pad_thumbv6m.rs"]
mod impl_pad;

#[cfg(feature = "min-samd51g")]
#[path = "pad/impl_pad_thumbv7em.rs"]
mod impl_pad;

//==============================================================================
// PadNum
//==============================================================================

/// Type-level enum representing a SERCOM pad number
///
/// It has variants [`Pad0`], [`Pad1`], [`Pad2`] & [`Pad3`]. See the [type-level
/// enum] documentation for an explanation of the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait PadNum: Sealed {}

seq!(N in 0..=3 {
    paste! {
        #[doc = "Type-level variant of [`PadNum`] representing SERCOM pad " N]
        ///
        /// See the [type-level enum] documentation for an explanation of the
        /// pattern.
        ///
        /// [type-level enum]: crate::typelevel#type-level-enum
        pub enum Pad#N {}
        impl Sealed for Pad#N {}
        impl PadNum for Pad#N {}
    }
});

//==============================================================================
// OptionalPadNum
//==============================================================================

/// Type-level equivalent of `Option<PadNum>`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait OptionalPadNum: Sealed {}

impl OptionalPadNum for NoneT {}

impl<N: PadNum> OptionalPadNum for N {}

//==============================================================================
// IsPad
//==============================================================================

/// Type class for [`Pin`]s configured as SERCOM pads
///
/// This trait serves as both a [type class] for `Pin`s configured as SERCOM
/// pads and as a [type-level function] mapping each `Pin` type to its
/// corresponding [`Sercom`] and [`PadNum`].
///
/// [type class]: crate::typelevel#type-classes
/// [type-level function]: crate::typelevel#type-level-functions
pub trait IsPad: AnyPin {
    type Sercom: Sercom;
    type PadNum: PadNum;
}

//==============================================================================
// OptionalPad
//==============================================================================

/// Type-level equivalent of `Option<Pad>`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait OptionalPad: OptionalPin {
    type PadNum: OptionalPadNum;
}

impl OptionalPad for NoneT {
    type PadNum = NoneT;
}

impl<P: IsPad> OptionalPad for P {
    type PadNum = P::PadNum;
}

/// Type-level equivalent of `Some(Pad)`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait SomePad: IsPad {}

impl<P: IsPad> SomePad for P {}

//==============================================================================
// GetPad
//==============================================================================

/// Type-level function mapping [`PinId`]s to SERCOM-pad-related types
///
/// For SAMD21 and SAMx5x chips, a [`Sercom`] and a [`PinId`] is enough
/// information to uniquely identify a pad, so this trait returns the
/// corresponding [`PadNum`] and [`PinMode`].
///
/// For SAMD11 chips, on the other hand, some `PinId`s can serve as two
/// different `PadNum`s for the *same* `Sercom`. For these chips, `GetPad`
/// requires a second type parameter to specify the `PadNum` and only returns
/// the `PinMode`.
///
/// See the documentation on [type-level functions] for more details.
///
/// [type-level functions]: crate::typelevel#type-level-functions
#[cfg(feature = "samd11")]
pub trait GetPad<S, N>
where
    S: Sercom,
    N: PadNum,
    Self: PinId,
{
    type PinMode: PinMode;
}

/// Type-level function mapping [`PinId`]s to SERCOM-pad-related types
///
/// For SAMD21 and SAMx5x chips, a [`Sercom`] and a [`PinId`] is enough
/// information to uniquely identify a pad, so this trait returns the
/// corresponding [`PadNum`] and [`PinMode`].
///
/// For SAMD11 chips, on the other hand, some `PinId`s can serve as two
/// different `PadNum`s for the *same* `Sercom`. For these chips, `GetPad`
/// requires a second type parameter to specify the `PadNum` and only returns
/// the `PinMode`.
///
/// See the documentation on [type-level functions] for more details.
///
/// [type-level functions]: crate::typelevel#type-level-functions
#[cfg(not(feature = "samd11"))]
pub trait GetPad<S>
where
    S: Sercom,
    Self: PinId,
{
    type PadNum: PadNum;
    type PinMode: PinMode;
}

//==============================================================================
// GetPad aliases
//==============================================================================

/// Type alias using [`GetPad`] to recover the [`PinMode`] for a given SERCOM
/// pad
#[cfg(feature = "samd11")]
pub type PadMode<S, N, I> = <I as GetPad<S, N>>::PinMode;

/// Type alias using [`GetPad`] to recover the [`PinMode`] for a given SERCOM
/// pad
#[cfg(not(feature = "samd11"))]
pub type PadMode<S, I> = <I as GetPad<S>>::PinMode;

/// Type alias to recover a [`Pin`] configured as a SERCOM pad in the correct
/// [`PadMode`]
#[cfg(feature = "samd11")]
pub type Pad<S, N, I> = Pin<I, PadMode<S, N, I>>;

/// Type alias to recover a [`Pin`] configured as a SERCOM pad in the correct
/// [`PadMode`]
#[cfg(not(feature = "samd11"))]
pub type Pad<S, I> = Pin<I, PadMode<S, I>>;

//==============================================================================
// GetOptionalPad
//==============================================================================

/// Type-level function mapping [`OptionalPinId`]s to their corresponding
/// [`OptionalPad`]s
///
/// This trait acts as a [type-level function] mapping `OptionalPinId`s to their
/// corresponding `OptionalPad`. In pseudo-Rust, it is the type-level equivalent
/// of starting with `Option<PinId>` and calling `.map(GetPad)` to recover an
/// `Option<Pad>`.
///
/// [type-level functions]: crate::typelevel#type-level-functions
#[cfg(not(feature = "samd11"))]
pub trait GetOptionalPad<S: Sercom>: OptionalPinId {
    type PadNum: OptionalPadNum;
    type Pad: OptionalPad;
}

#[cfg(not(feature = "samd11"))]
impl<S: Sercom> GetOptionalPad<S> for NoneT {
    type PadNum = NoneT;
    type Pad = NoneT;
}

#[cfg(not(feature = "samd11"))]
impl<S, I> GetOptionalPad<S> for I
where
    S: Sercom,
    I: PinId + GetPad<S>,
    Pad<S, I>: IsPad,
{
    type PadNum = I::PadNum;
    type Pad = Pad<S, I>;
}

//==============================================================================
// IoSet
//==============================================================================

#[cfg(feature = "min-samd51g")]
mod ioset {

    use super::*;

    /// Type-level enum representing a SERCOM IOSET
    ///
    /// See the [type-level enum] documentation for more details on the pattern.
    ///
    /// [type-level enum]: crate::typelevel#type-level-enum
    pub trait IoSet: Sealed {}

    seq!(N in 1..=6 {
        paste! {
            #[doc = "Type-level variant of [`IoSet`] representing SERCOM IOSET " N]
            ///
            /// See the [type-level enum] documentation for more details on the
            /// pattern.
            ///
            /// [type-level enum]: crate::typelevel#type-level-enum
            pub enum IoSet#N {}
            impl Sealed for IoSet#N {}
            impl IoSet for IoSet#N {}
        }
    });

    /// Type-level variant of [`IoSet`] representing an undocumented SERCOM
    /// IOSET
    ///
    /// See the [type-level enum] documentation for more details on the pattern.
    ///
    /// [type-level enum]: crate::typelevel#type-level-enum
    pub enum UndocIoSet1 {}
    impl Sealed for UndocIoSet1 {}
    impl IoSet for UndocIoSet1 {}

    /// Type class for SERCOM pads in a given [`IoSet`]
    ///
    /// This trait is used to label each [`Pin`] implementing [`IsPad`] with its
    /// corresponding [`IoSet`]\(s). Downstream types can use this trait as a
    /// [type class] to restrict [`Pin`]s to a given [`IoSet`]. See the [type
    /// class] documentation for more details on the pattern.
    ///
    /// [type class]: crate::typelevel#type-classes
    pub trait InIoSet<I>
    where
        Self: IsPad,
        I: IoSet,
    {
    }
}

#[cfg(feature = "min-samd51g")]
pub use ioset::*;
