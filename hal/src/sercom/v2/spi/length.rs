//! Define a trait to track the transaction [`Length`], which represents the
//! [`Config`] [`Size`] for SAMx5x chips
//!
//! This module defines the `Size` trait for SAMx5x chips. These chips always
//! operate in 32-bit extension mode and use the hardware `LENGTH` counter to
//! track the length of each transaction, in bytes. See the [`Length`]
//! documentation for more details.
//!
//! [`Config`]: super::Config
//! [`Size`]: super::Size

use seq_macro::seq;
use typenum::{Unsigned, U0, U1, U2, U3, U4};

use crate::typelevel::Sealed;

//=============================================================================
// Transaction length
//=============================================================================

/// Type-level enum representing the SPI transaction length, in bytes
///
/// This trait acts as both a [type-level enum], forming a type class for
/// transaction lengths, as well as a [type-level function] mapping the `Length`
/// to the corresponding [`Word`] size.
///
/// The SPI transaction length is represented in the type domain using
/// [`Unsigned`] types from the [`typenum`] crate. The length can be set
/// statically, using a length from `U1` to `U255`, or it can be set
/// dynamically, using the [`DynLength`] marker type. All valid `Length` types
/// are re-exported in this module.
///
/// The SPI transaction length affects the word size for the embedded HAL
/// traits, as well as other aspects of the SPI API. Transaction lengths of 1-4
/// only require a single read/write of the `DATA` register, so they have an
/// [`AtomicSize`] behave differently than longer transaction lengths.
///
/// [type-level enum]: crate::typelevel#type-level-enums
/// [type-level function]: crate::typelevel#type-level-functions
/// [`OpMode`]: super::OpMode
/// [`AtomicSize`]: super::AtomicSize
pub trait Length: Sealed + Unsigned + 'static {
    /// Word size for the transaction length
    ///
    /// For lengths 1-4, this type is `u8`, `u16` or `u32`. For longer
    /// transactions, this type is `[u8, Self::USIZE]`.
    type Word: 'static;
}

/// Type alias to recover the [`Word`](Length::Word) type from an
/// implementation of [`Length`]
pub type Word<L> = <L as Length>::Word;

/// Marker type for a run-time dynamic [`Length`]
pub type DynLength = U0;

impl Sealed for DynLength {}
impl Length for DynLength {
    type Word = ();
}

/// Marker trait for statically known transaction [`Length`]s
pub trait StaticLength: Length {}

impl Sealed for U1 {}
impl StaticLength for U1 {}
impl Length for U1 {
    type Word = u8;
}

impl Sealed for U2 {}
impl StaticLength for U2 {}
impl Length for U2 {
    type Word = u16;
}

impl Sealed for U3 {}
impl StaticLength for U3 {}
impl Length for U3 {
    type Word = u32;
}

impl Sealed for U4 {}
impl StaticLength for U4 {}
impl Length for U4 {
    type Word = u32;
}

/// Marker trait for transaction [`Length`]s greater than four
pub trait GreaterThan4: Length {}

seq!(N in 5..=255 {
    impl Sealed for typenum::U#N {}
    impl StaticLength for typenum::U#N {}
    impl GreaterThan4 for typenum::U#N {}
    impl Length for typenum::U#N {
        type Word = [u8; typenum::U#N::USIZE];
    }
});
