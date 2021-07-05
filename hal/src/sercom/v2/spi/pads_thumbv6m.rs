//! Define a container for a set of SERCOM pads
//!
//! See the [spi module](super) documentation for more details on declaring and
//! instantiating a [`Pads`] type.

use core::marker::PhantomData;

use crate::gpio::v2::AnyPin;
use crate::sercom::v2::*;
use crate::typelevel::{NoneT, Sealed};

use super::{Capability, Duplex, Rx, Tx};

//=============================================================================
// DipoDopo
//=============================================================================

/// Configure the `DIPO` and `DOPO` fields based on a set of [`Pads`]
///
/// According to the datasheet, the `DIPO` and `DOPO` values specify which
/// SERCOM pads are used for various functions. Moreover, depending on which
/// pads are actually in use, only certain combinations of these values make
/// sense and are valid.
///
/// This trait is implemented for valid, four-tuple combinations of
/// [`OptionalPadNum`]s. Those implementations are then lifted to the
/// corresponding [`Pads`] types.
///
/// To satisfy this trait, the combination of `OptionalPadNum`s must specify a
/// [`PadNum`] for `CK` and at least one of `DI` and `DO`. Furthermore, no two
/// `PadNum`s can conflict.
pub trait DipoDopo {
    const DIPO_DOPO: (u8, u8);
}

/// Lift the implementations of [`DipoDopo`] from four-tuples of
/// [`OptionalPadNum`]s to the corresponding [`Pads`] types.
impl<S, DI, DO, CK, SS> DipoDopo for Pads<S, DI, DO, CK, SS>
where
    S: Sercom,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
    (DI::PadNum, DO::PadNum, CK::PadNum, SS::PadNum): DipoDopo,
{
    const DIPO_DOPO: (u8, u8) = <(DI::PadNum, DO::PadNum, CK::PadNum, SS::PadNum)>::DIPO_DOPO;
}

//=============================================================================
// Implement DipoDopo
//=============================================================================

/// Filter [`PadNum`] permutations and implement [`DipoDopo`]
#[rustfmt::skip]
macro_rules! impl_dipodopo {
    // This is the entry pattern. Start by checking CK and SS.
    ($DI:ident, $DO:ident, $CK:ident, $SS:ident) => { impl_dipodopo!(@ck_ss, $DI, $DO, $CK, $SS); };

    // Check whether CK and SS form a valid pair.
    // CK must be present, while SS must be the correct pad or absent.
    (@ck_ss, $DI:ident, $DO:ident, Pad1, NoneT) => { impl_dipodopo!(@dipo, $DI, $DO, Pad1, NoneT); };
    (@ck_ss, $DI:ident, $DO:ident, Pad1, Pad2)  => { impl_dipodopo!(@dipo, $DI, $DO, Pad1, Pad2); };
    (@ck_ss, $DI:ident, $DO:ident, Pad3, NoneT) => { impl_dipodopo!(@dipo, $DI, $DO, Pad3, NoneT); };
    (@ck_ss, $DI:ident, $DO:ident, Pad3, Pad1)  => { impl_dipodopo!(@dipo, $DI, $DO, Pad3, Pad1); };

    // If CK and SS are not valid, fall through to this pattern.
    (@ck_ss, $DI:ident, $DO:ident, $CK:ident, $SS:ident) => { };

    // Assign DIPO based on DI.
    // Our options are exhaustive, so no fall through pattern is needed.
    (@dipo, NoneT, $DO:ident, $CK:ident, $SS:ident) => { impl_dipodopo!(@dopo, NoneT, $DO, $CK, $SS, 0); };
    (@dipo, Pad0,  $DO:ident, $CK:ident, $SS:ident) => { impl_dipodopo!(@dopo, Pad0,  $DO, $CK, $SS, 0); };
    (@dipo, Pad1,  $DO:ident, $CK:ident, $SS:ident) => { impl_dipodopo!(@dopo, Pad1,  $DO, $CK, $SS, 1); };
    (@dipo, Pad2,  $DO:ident, $CK:ident, $SS:ident) => { impl_dipodopo!(@dopo, Pad2,  $DO, $CK, $SS, 2); };
    (@dipo, Pad3,  $DO:ident, $CK:ident, $SS:ident) => { impl_dipodopo!(@dopo, Pad3,  $DO, $CK, $SS, 3); };

    // Assign DOPO based on DO and CK.
    (@dopo, $DI:ident, NoneT, Pad1, $SS:ident, $DIPO:literal) => { impl_dipodopo!(@filter, $DI, NoneT, Pad1, $SS, $DIPO, 0); };
    (@dopo, $DI:ident, NoneT, Pad3, $SS:ident, $DIPO:literal) => { impl_dipodopo!(@filter, $DI, NoneT, Pad3, $SS, $DIPO, 1); };
    (@dopo, $DI:ident, Pad0,  Pad1, $SS:ident, $DIPO:literal) => { impl_dipodopo!(@filter, $DI, Pad0,  Pad1, $SS, $DIPO, 0); };
    (@dopo, $DI:ident, Pad2,  Pad3, $SS:ident, $DIPO:literal) => { impl_dipodopo!(@filter, $DI, Pad2,  Pad3, $SS, $DIPO, 1); };
    (@dopo, $DI:ident, Pad3,  Pad1, $SS:ident, $DIPO:literal) => { impl_dipodopo!(@filter, $DI, Pad3,  Pad1, $SS, $DIPO, 2); };
    (@dopo, $DI:ident, Pad0,  Pad3, $SS:ident, $DIPO:literal) => { impl_dipodopo!(@filter, $DI, Pad0,  Pad3, $SS, $DIPO, 3); };

    // If DO is not valid, fall through to this pattern.
    (@dopo, $DI:ident, $DO:ident, $CK:ident, $SS:ident, $DIPO:literal) => { };

    // Filter any remaining permutations that conflict.
    (@filter, NoneT, NoneT, $CK:ident, $SS:ident, $DIPO:literal, $DOPO:literal) => { };
    (@filter, Pad0, Pad0, $CK:ident, $SS:ident, $DIPO:literal, $DOPO:literal) => { };
    (@filter, Pad1, $DO:ident, Pad1, $SS:ident, $DIPO:literal, $DOPO:literal) => { };
    (@filter, Pad1, $DO:ident, $CK:ident, Pad1, $DIPO:literal, $DOPO:literal) => { };
    (@filter, Pad2, Pad2, $CK:ident, $SS:ident, $DIPO:literal, $DOPO:literal) => { };
    (@filter, Pad2, $DO:ident, $CK:ident, Pad2, $DIPO:literal, $DOPO:literal) => { };
    (@filter, Pad3, Pad3, $CK:ident, $SS:ident, $DIPO:literal, $DOPO:literal) => { };
    (@filter, Pad3, $DO:ident, Pad3, $SS:ident, $DIPO:literal, $DOPO:literal) => { };
    (@filter, $DI:ident, Pad2, $CK:ident, Pad2, $DIPO:literal, $DOPO:literal) => { };

    // If there are no conflicts, fall through to this pattern
    (@filter, $DI:ident, $DO:ident, $CK:ident, $SS:ident, $DIPO:literal, $DOPO:literal) => { impl_dipodopo!(@revise, $DI, $DO, $CK, $SS, $DIPO, $DOPO); };

    // Revise DIPO and DOPO conflicts created by NoneT
    (@revise, NoneT, Pad0, Pad1, $SS:ident, $DIPO:literal, $DOPO:literal) => { impl_dipodopo!(@implement, NoneT, Pad0, Pad1, $SS, 3, $DOPO); };
    (@revise, NoneT, Pad0, Pad3, $SS:ident, $DIPO:literal, $DOPO:literal) => { impl_dipodopo!(@implement, NoneT, Pad0, Pad3, $SS, 2, $DOPO); };
    (@revise, Pad0, NoneT, Pad1, $SS:ident, $DIPO:literal, $DOPO:literal) => { impl_dipodopo!(@implement, Pad0, NoneT, Pad1, $SS, $DIPO, 2); };
    (@revise, Pad2, NoneT, Pad3, $SS:ident, $DIPO:literal, $DOPO:literal) => { impl_dipodopo!(@implement, Pad2, NoneT, Pad3, $SS, $DIPO, 3); };

    // If there are no revisions, fall through to this pattern
    (@revise, $DI:ident, $DO:ident, $CK:ident, $SS:ident, $DIPO:literal, $DOPO:literal) => { impl_dipodopo!(@implement, $DI, $DO, $CK, $SS, $DIPO, $DOPO); };

    // Implement DipoDopo
    (@implement, $DI:ident, $DO:ident, $CK:ident, $SS:ident, $DIPO:literal, $DOPO:literal) => {
        impl DipoDopo for ($DI, $DO, $CK, $SS) {
            const DIPO_DOPO: (u8, u8) = ($DIPO, $DOPO);
        }
    };
}

/// Try to implement [`DipoDopo`] on all possible 4-tuple permutations of
/// [`OptionalPadNum`]s.
///
/// The leading `()` token tree stores a growing permutation of [`PadNum`]s.
/// When it reaches four [`PadNum`]s, try to implement [`DipoDopo`].
///
/// The next `[]` token tree is a list of possible [`PadNum`]s to append to the
/// growing permutation. Loop through this list and append each option to the
/// permutation.
///
/// The final, optional `[]` token tree exists to temporarily store the entire
/// list before pushing it down for the next permutation element.
macro_rules! padnum_permutations {
    // If we have built up four [`PadNum`]s, try to implement [`DipoDopo`].
    // Ignore the remaining list of [`PadNum`]s.
    (
        ( $DI:ident $DO:ident $CK:ident $SS:ident ) [ $( $Pads:ident )* ]
    ) => {
        impl_dipodopo!($DI, $DO, $CK, $SS);
    };
    // If we only have one list of [`PadNum`]s, duplicate it, to save it for the
    // next permutation element.
    (
        ( $($Perm:ident)* ) [ $($Pads:ident)+ ]
    ) => {
        padnum_permutations!( ( $($Perm)* ) [ $($Pads)+ ] [ $($Pads)+ ] );
    };
    (
        ( $($Perm:ident)* ) [ $Head:ident $($Tail:ident)* ] [ $($Pads:ident)+ ]
    ) => {
        // Append the first [`PadNum`] from the list, then push down to the next
        // permutation element.
        padnum_permutations!( ( $($Perm)* $Head ) [ $($Pads)+ ] );

        // Loop through the remaining [`PadNum`]s to do the same thing for each.
        padnum_permutations!( ( $($Perm)* ) [ $($Tail)* ] [ $($Pads)+ ] );
    };
    // Once the list of [`PadNum`]s is empty, we're done with this element.
    ( ( $($Perm:ident)* ) [ ] [ $($Pads:ident)+ ] ) => { };
}

padnum_permutations!( () [NoneT Pad0 Pad1 Pad2 Pad3] );

//=============================================================================
// Pads
//=============================================================================

/// Container for a set of SERCOM pads
///
/// See the [module-level](super) documentation for more details on declaring
/// and instantiating `Pads` types.
pub struct Pads<S, DI = NoneT, DO = NoneT, CK = NoneT, SS = NoneT>
where
    S: Sercom,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
    sercom: PhantomData<S>,
    data_in: DI,
    data_out: DO,
    sclk: CK,
    ss: SS,
}

impl<S: Sercom> Default for Pads<S> {
    fn default() -> Self {
        Self {
            sercom: PhantomData,
            data_in: NoneT,
            data_out: NoneT,
            sclk: NoneT,
            ss: NoneT,
        }
    }
}

impl<S, DI, DO, CK, SS> Pads<S, DI, DO, CK, SS>
where
    S: Sercom,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
    /// Consume the [`Pads`] and return each individual
    /// [`Pin`](crate::gpio::v2::Pin)
    #[inline]
    pub fn free(self) -> (DI, DO, CK, SS) {
        (self.data_in, self.data_out, self.sclk, self.ss)
    }
}

#[cfg(feature = "samd11")]
impl<S, DI, DO, CK, SS> Pads<S, DI, DO, CK, SS>
where
    S: Sercom,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
    /// Set the `DI` pad
    ///
    /// In a [`MasterMode`], this is MISO. In [`Slave`] [`OpMode`], this is
    /// MOSI.
    ///
    /// [`MasterMode`]: super::MasterMode
    /// [`Slave`]: super::Slave
    /// [`OpMode`]: super::OpMode
    #[inline]
    pub fn data_in<P: IsPad>(self, pin: P) -> Pads<S, P, DO, CK, SS> {
        Pads {
            sercom: self.sercom,
            data_in: pin,
            data_out: self.data_out,
            sclk: self.sclk,
            ss: self.ss,
        }
    }

    /// Set the `DO` pad
    ///
    /// In a [`MasterMode`], this is MOSI. In [`Slave`] [`OpMode`], this is
    /// MISO.
    ///
    /// [`MasterMode`]: super::MasterMode
    /// [`Slave`]: super::Slave
    /// [`OpMode`]: super::OpMode
    #[inline]
    pub fn data_out<P: IsPad>(self, pin: P) -> Pads<S, DI, P, CK, SS> {
        Pads {
            sercom: self.sercom,
            data_in: self.data_in,
            data_out: pin,
            sclk: self.sclk,
            ss: self.ss,
        }
    }

    /// Set the `SCK` pad
    #[inline]
    pub fn sclk<P: IsPad>(self, pin: P) -> Pads<S, DI, DO, P, SS> {
        Pads {
            sercom: self.sercom,
            data_in: self.data_in,
            data_out: self.data_out,
            sclk: pin,
            ss: self.ss,
        }
    }

    /// Set the `SS` pad
    #[inline]
    pub fn ss<P: IsPad>(self, pin: P) -> Pads<S, DI, DO, CK, P> {
        Pads {
            sercom: self.sercom,
            data_in: self.data_in,
            data_out: self.data_out,
            sclk: self.sclk,
            ss: pin,
        }
    }
}

#[cfg(feature = "samd21")]
impl<S, DI, DO, CK, SS> Pads<S, DI, DO, CK, SS>
where
    S: Sercom,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
    /// Set the `DI` pad
    ///
    /// In a [`MasterMode`], this is MISO. In [`Slave`] [`OpMode`], this is
    /// MOSI.
    ///
    /// [`MasterMode`]: super::MasterMode
    /// [`Slave`]: super::Slave
    /// [`OpMode`]: super::OpMode
    #[inline]
    pub fn data_in<I>(self, pin: impl AnyPin<Id = I>) -> Pads<S, Pad<S, I>, DO, CK, SS>
    where
        I: GetPad<S>,
        Pad<S, I>: IsPad,
    {
        Pads {
            sercom: self.sercom,
            data_in: pin.into().into_mode(),
            data_out: self.data_out,
            sclk: self.sclk,
            ss: self.ss,
        }
    }

    /// Set the `DO` pad
    ///
    /// In a [`MasterMode`], this is MOSI. In [`Slave`] [`OpMode`], this is
    /// MISO.
    ///
    /// [`MasterMode`]: super::MasterMode
    /// [`Slave`]: super::Slave
    /// [`OpMode`]: super::OpMode
    #[inline]
    pub fn data_out<I>(self, pin: impl AnyPin<Id = I>) -> Pads<S, DI, Pad<S, I>, CK, SS>
    where
        I: GetPad<S>,
        Pad<S, I>: IsPad,
    {
        Pads {
            sercom: self.sercom,
            data_in: self.data_in,
            data_out: pin.into().into_mode(),
            sclk: self.sclk,
            ss: self.ss,
        }
    }

    /// Set the `SCK` pad
    #[inline]
    pub fn sclk<I>(self, pin: impl AnyPin<Id = I>) -> Pads<S, DI, DO, Pad<S, I>, SS>
    where
        I: GetPad<S>,
        Pad<S, I>: IsPad,
    {
        Pads {
            sercom: self.sercom,
            data_in: self.data_in,
            data_out: self.data_out,
            sclk: pin.into().into_mode(),
            ss: self.ss,
        }
    }

    /// Set the `SS` pad
    #[inline]
    pub fn ss<I>(self, pin: impl AnyPin<Id = I>) -> Pads<S, DI, DO, CK, Pad<S, I>>
    where
        I: GetPad<S>,
        Pad<S, I>: IsPad,
    {
        Pads {
            sercom: self.sercom,
            data_in: self.data_in,
            data_out: self.data_out,
            sclk: self.sclk,
            ss: pin.into().into_mode(),
        }
    }
}

//=============================================================================
// PadsFromIds
//=============================================================================

/// Define a set of [`Pads`] using [`PinId`]s instead of [`Pin`]s
///
/// In some cases, it is more convenient to specify a set of `Pads` using
/// `PinId`s rather than `Pin`s. This alias makes it easier to do so.
///
/// The first type parameter is the [`Sercom`], while the remaining four are
/// effectively [`OptionalPinId`]s representing the corresponding type
/// parameters of [`Pads`], i.e. `DI`, `DO`, `CK` & `SS`. Each of the
/// remaining type parameters defaults to [`NoneT`].
///
/// ```
/// use atsamd_hal::pac::Peripherals;
/// use atsamd_hal::gpio::v2::{PA08, PA09, Pins};
/// use atsamd_hal::sercom::v2::{Sercom0, spi};
/// use atsamd_hal::typelevel::NoneT;
///
/// pub type Pads = spi::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
///
/// pub fn create_pads() -> Pads {
///     let peripherals = Peripherals::take().unwrap();
///     let pins = Pins::new(peripherals.PORT);
///     spi::Pads::default().sclk(pins.pa09).data_in(pins.pa08)
/// }
/// ```
///
/// [`Pin`]: crate::gpio::v2::Pin
/// [`PinId`]: crate::gpio::v2::PinId
/// [`OptionalPinId`]: crate::gpio::v2::OptionalPinId
#[cfg(feature = "samd21")]
pub type PadsFromIds<S, DI = NoneT, DO = NoneT, CK = NoneT, SS = NoneT> = Pads<
    S,
    <DI as GetOptionalPad<S>>::Pad,
    <DO as GetOptionalPad<S>>::Pad,
    <CK as GetOptionalPad<S>>::Pad,
    <SS as GetOptionalPad<S>>::Pad,
>;

//=============================================================================
// PadSet
//=============================================================================

/// Type-level function to recover the [`OptionalPad`] types from a generic set
/// of [`Pads`]
///
/// This trait is used as an interface between the [`Pads`] type and other
/// types in this module. It acts as a [type-level function], returning the
/// corresponding [`Sercom`] and [`OptionalPad`] types. It serves to cut down on
/// the total number of type parameters needed in the [`Config`] struct. The
/// `Config` struct doesn't need access to the [`Pin`]s directly.  Rather, it
/// only needs to apply the [`SomePad`] trait bound when a `Pin` is required.
/// The `PadSet` trait allows each `Config` struct to store an instance of
/// `Pads` without itself being generic over all six type parameters of the
/// `Pads` type.
///
/// This trait is a simplified version of the [`AnyKind`] trait pattern.
///
/// [`Pin`]: crate::gpio::v2::Pin
/// [`Config`]: super::Config
/// [type-level function]: crate::typelevel#type-level-functions
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
pub trait PadSet: Sealed {
    type Sercom: Sercom;
    type DataIn: OptionalPad;
    type DataOut: OptionalPad;
    type Sclk: OptionalPad;
    type SS: OptionalPad;
}

impl<S, DI, DO, CK, SS> Sealed for Pads<S, DI, DO, CK, SS>
where
    S: Sercom,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
}

impl<S, DI, DO, CK, SS> PadSet for Pads<S, DI, DO, CK, SS>
where
    S: Sercom,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
    type Sercom = S;
    type DataIn = DI;
    type DataOut = DO;
    type Sclk = CK;
    type SS = SS;
}

//=============================================================================
// ValidPads
//=============================================================================

/// Marker trait for valid sets of [`Pads`]
///
/// This trait labels sets of `Pads` that:
/// - Specify [`SomePad`] for `CK` and at least one of `DI` or `DO`
/// - Use a valid combination of [`PadNum`]s, so that the `Pads` implement
///   [`DipoDopo`]
pub trait ValidPads: PadSet + DipoDopo {
    type Capability: Capability;
}

impl<S, DI, CK, SS> ValidPads for Pads<S, DI, NoneT, CK, SS>
where
    S: Sercom,
    DI: SomePad,
    CK: SomePad,
    SS: OptionalPad,
    Pads<S, DI, NoneT, CK, SS>: DipoDopo,
{
    type Capability = Rx;
}

impl<S, DO, CK, SS> ValidPads for Pads<S, NoneT, DO, CK, SS>
where
    S: Sercom,
    DO: SomePad,
    CK: SomePad,
    SS: OptionalPad,
    Pads<S, NoneT, DO, CK, SS>: DipoDopo,
{
    type Capability = Tx;
}

impl<S, DI, DO, CK, SS> ValidPads for Pads<S, DI, DO, CK, SS>
where
    S: Sercom,
    DI: SomePad,
    DO: SomePad,
    CK: SomePad,
    SS: OptionalPad,
    Pads<S, DI, DO, CK, SS>: DipoDopo,
{
    type Capability = Duplex;
}
