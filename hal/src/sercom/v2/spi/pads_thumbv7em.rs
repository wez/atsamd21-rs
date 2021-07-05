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
// Dipo
//=============================================================================

/// Map an [`OptionalPadNum`] to its corresponding `DIPO` value
pub trait Dipo: OptionalPadNum {
    const DIPO: Option<u8>;
}

impl Dipo for NoneT {
    const DIPO: Option<u8> = None;
}

impl Dipo for Pad0 {
    const DIPO: Option<u8> = Some(0);
}
impl Dipo for Pad1 {
    const DIPO: Option<u8> = Some(1);
}
impl Dipo for Pad2 {
    const DIPO: Option<u8> = Some(2);
}
impl Dipo for Pad3 {
    const DIPO: Option<u8> = Some(3);
}

//=============================================================================
// Dopo
//=============================================================================

/// Map an [`OptionalPadNum`] to its corresponding `DOPO` value
pub trait Dopo: OptionalPadNum {
    const DOPO: Option<u8>;
}

impl Dopo for NoneT {
    const DOPO: Option<u8> = None;
}

impl Dopo for Pad0 {
    const DOPO: Option<u8> = Some(0);
}
impl Dopo for Pad3 {
    const DOPO: Option<u8> = Some(2);
}

//=============================================================================
// DipoDopo
//=============================================================================

/// Configure the `DIPO` and `DOPO` fields based on a set of [`Pads`]
pub trait DipoDopo: Sealed {
    const DIPO_DOPO: (u8, u8);
}

/// Lift the implementations of [`DipoDopo`] from implementations on
/// [`OptionalPadNum`]s to the corresponding [`Pads`] types.
impl<S, I, DI, DO, CK, SS> DipoDopo for Pads<S, I, DI, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
    DI::PadNum: Dipo,
    DO::PadNum: Dopo,
{
    const DIPO_DOPO: (u8, u8) = match (DI::PadNum::DIPO, DO::PadNum::DOPO) {
        (None, None) => (0, 2),
        (Some(dipo), None) => {
            let dopo = if dipo == 0 { 2 } else { 0 };
            (dipo, dopo)
        }
        (None, Some(dopo)) => {
            let dipo = if dopo == 0 { 3 } else { 0 };
            (dipo, dopo)
        }
        (Some(dipo), Some(dopo)) => (dipo, dopo),
    };
}

//=============================================================================
// Pads
//=============================================================================

/// Container for a set of SERCOM pads
///
/// See the [spi module](super) documentation for more details on declaring and
/// instantiating a [`Pads`] type.
pub struct Pads<S, I, DI = NoneT, DO = NoneT, CK = NoneT, SS = NoneT>
where
    S: Sercom,
    I: IoSet,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
    sercom: PhantomData<S>,
    ioset: PhantomData<I>,
    data_in: DI,
    data_out: DO,
    sclk: CK,
    ss: SS,
}

impl<S: Sercom, I: IoSet> Default for Pads<S, I> {
    fn default() -> Self {
        Self {
            sercom: PhantomData,
            ioset: PhantomData,
            data_in: NoneT,
            data_out: NoneT,
            sclk: NoneT,
            ss: NoneT,
        }
    }
}

impl<S, I, DI, DO, CK, SS> Pads<S, I, DI, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
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
    pub fn data_in<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, Pad<S, Id>, DO, CK, SS>
    where
        Id: GetPad<S>,
        Id::PadNum: Dipo,
        Pad<S, Id>: InIoSet<I>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
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
    pub fn data_out<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, DI, Pad<S, Id>, CK, SS>
    where
        Id: GetPad<S>,
        Id::PadNum: Dopo,
        Pad<S, Id>: InIoSet<I>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            data_in: self.data_in,
            data_out: pin.into().into_mode(),
            sclk: self.sclk,
            ss: self.ss,
        }
    }

    /// Set the `SCK` pad, which is always [`Pad1`]
    #[inline]
    pub fn sclk<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, DI, DO, Pad<S, Id>, SS>
    where
        Id: GetPad<S, PadNum = Pad1>,
        Pad<S, Id>: InIoSet<I>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            data_in: self.data_in,
            data_out: self.data_out,
            sclk: pin.into().into_mode(),
            ss: self.ss,
        }
    }

    /// Set the `SS` pad, which is always [`Pad2`]
    #[inline]
    pub fn ss<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, DI, DO, CK, Pad<S, Id>>
    where
        Id: GetPad<S, PadNum = Pad2>,
        Pad<S, Id>: InIoSet<I>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            data_in: self.data_in,
            data_out: self.data_out,
            sclk: self.sclk,
            ss: pin.into().into_mode(),
        }
    }

    /// Consume the [`Pads`] and return each individual
    /// [`Pin`](crate::gpio::v2::Pin)
    #[inline]
    pub fn free(self) -> (DI, DO, CK, SS) {
        (self.data_in, self.data_out, self.sclk, self.ss)
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
/// The first two type parameters are the [`Sercom`] and [`IoSet`], while the
/// remaining four are effectively [`OptionalPinId`]s representing the
/// corresponding type parameters of [`Pads`], i.e. `DI`, `DO`, `CK` & `SS`.
/// Each of the remaining type parameters defaults to [`NoneT`].
///
/// ```
/// use atsamd_hal::pac::Peripherals;
/// use atsamd_hal::gpio::v2::{PA08, PA09, Pins};
/// use atsamd_hal::sercom::v2::{Sercom0, spi};
/// use atsamd_hal::sercom::v2::pad::IoSet1;
/// use atsamd_hal::typelevel::NoneT;
///
/// pub type Pads = spi::PadsFromIds<Sercom0, IoSet1, PA08, NoneT, PA09>;
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
pub type PadsFromIds<S, I, DI = NoneT, DO = NoneT, CK = NoneT, SS = NoneT> = Pads<
    S,
    I,
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
/// the total number of type parameters needed in the [`Config`](super::Config)
/// struct. The `Config` struct doesn't need access to the [`Pin`]s directly.
/// Rather, it only needs to apply the [`SomePad`] trait bound when a `Pin` is
/// required. The `PadSet` trait allows each `Config` struct to store an
/// instance of `Pads` without itself being generic over all six type parameters
/// of the `Pads` type.
///
/// This trait is a simplified version of the [`AnyKind`] trait pattern.
///
/// [`Pin`]: crate::gpio::v2::Pin
/// [type-level function]: crate::typelevel#type-level-functions
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
pub trait PadSet: Sealed {
    type Sercom: Sercom;
    type IoSet: IoSet;
    type DataIn: OptionalPad;
    type DataOut: OptionalPad;
    type Sclk: OptionalPad;
    type SS: OptionalPad;
}

impl<S, I, DI, DO, CK, SS> Sealed for Pads<S, I, DI, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
}

impl<S, I, DI, DO, CK, SS> PadSet for Pads<S, I, DI, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
{
    type Sercom = S;
    type IoSet = I;
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

impl<S, I, DI, CK, SS> ValidPads for Pads<S, I, DI, NoneT, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DI: SomePad,
    CK: SomePad,
    SS: OptionalPad,
    Pads<S, I, DI, NoneT, CK, SS>: DipoDopo,
{
    type Capability = Rx;
}

impl<S, I, DO, CK, SS> ValidPads for Pads<S, I, NoneT, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DO: SomePad,
    CK: SomePad,
    SS: OptionalPad,
    Pads<S, I, NoneT, DO, CK, SS>: DipoDopo,
{
    type Capability = Tx;
}

impl<S, I, DI, DO, CK, SS> ValidPads for Pads<S, I, DI, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DI: SomePad,
    DO: SomePad,
    CK: SomePad,
    SS: OptionalPad,
    Pads<S, I, DI, DO, CK, SS>: DipoDopo,
{
    type Capability = Duplex;
}
