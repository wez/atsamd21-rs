//! Use the SERCOM peripheral for SPI transactions
//!
//! Configuring an SPI peripheral occurs in three steps. First, you must create
//! a set of [`Pads`] for use by the peripheral. Next, you assemble pieces into
//! a [`Config`] struct. After configuring the peripheral, you then [`enable`]
//! it, yielding a functional [`Spi`] struct. Transactions are performed using
//! the [`spi`](embedded_hal::spi) and [`serial`](embedded_hal::serial) traits
//! from embedded HAL.
//!
//! # [`Pads`]
//!
//! A [`Sercom`] can use up to four [`Pin`]s as peripheral pads, but only
//! certain `Pin` combinations are acceptable. In particular, all `Pin`s must be
//! mapped to the same `Sercom` and [`IoSet`] (see section 6.2.8.1 of the
//! datasheet). This HAL makes it impossible to use invalid `Pin` combinations,
//! and the [`Pads`] struct is responsible for enforcing these constraints.
//!
//! A `Pads` type takes up to six type parameters. The first two specify the
//! `Sercom` and `IoSet`, while the remaining four, `DI`, `DO`, `CK` and `SS`,
//! represent the Data In, Data Out, Sclk and SS pads respectively. Each of the
//! remaining type parameters is an [`OptionalPad`] and defaults to [`NoneT`].
//! Aliases defining the pad types can be provided by the
//! [`bsp_pins!`](crate::bsp_pins) macro.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09, AlternateC};
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::sercom::v2::pad::IoSet1;
//! use atsamd_hal::typelevel::NoneT;
//!
//! type Miso = Pin<PA08, AlternateC>;
//! type Sclk = Pin<PA09, AlternateC>;
//! type Pads = spi::Pads<Sercom0, IoSet1, Miso, NoneT, Sclk>;
//! ```
//!
//! Alternatively, you can use the [`PadsFromIds`] alias to define a set of
//! `Pads` in terms of [`PinId`]s instead of `Pin`s. This is useful when you
//! don't have `Pin` aliases pre-defined.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::sercom::v2::pad::IoSet1;
//! use atsamd_hal::typelevel::NoneT;
//!
//! type Pads = spi::PadsFromIds<Sercom0, IoSet1, PA08, NoneT, PA09>;
//! ```
//!
//! Instances of `Pads` are created using the builder pattern. Start by creating
//! an empty set of `Pads` using [`Default`]. Then pass each respective `Pin`
//! using the corresponding methods. Both `v1::Pin` and `v2::Pin` types are
//! accepted here. The builder methods automatically convert each
//! pin to the correct [`PinMode`].
//!
//! Note that the `CK` `Pin` must map to [`Pad1`], and if specified, the `SS`
//! `Pin` must map to [`Pad2`]. The `DI` and `DO` `Pin`s can vary in [`PadNum`]
//! based on the [`Dipo`] and [`Dopo`] values.
//!
//! ```
//! use atsamd_hal::target_device::Peripherals;
//! use atsamd_hal::gpio::v2::Pins;
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::sercom::v2::pad::IoSet1;
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! let pads = spi::Pads::<Sercom0, IoSet1>::default()
//!     .sclk(pins.pa09)
//!     .data_in(pins.pa08)
//!     .data_out(pins.pa11);
//! ```
//!
//! To be accepted as [`ValidPads`], a set of `Pads` must do two things:
//! - Specify a type for `CK` and at least one of `DI` or `DO`
//! - Satisfy the [`Dipo`] and [`Dopo`] traits
//!
//! # [`Config`]
//!
//! Next, create a [`Config`] struct, which represents the SPI peripheral in its
//! disabled state. A `Config` is specified with three type parameters: the
//! [`Pads`] type; an [`OpMode`], which defaults to [`Master`]; and a
//! transaction [`Length`], in bytes, represented at the type level using the
//! [`typenum`] crate. Valid transaction lengths are provided in the [`lengths`]
//! sub-module. The default `Length` is [`U1`].
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::sercom::v2::spi::{Master, lengths::U2};
//! use atsamd_hal::sercom::v2::pad::IoSet1;
//! use atsamd_hal::typelevel::NoneT;
//!
//! type Pads = spi::PadsFromIds<Sercom0, IoSet1, PA08, NoneT, PA09>;
//! type Config = spi::Config<Pads, Master, U2>;
//! ```
//!
//! The SPI peripheral has two different ways to control the transaction length,
//! the character size and the length counter. The character size can be set to
//! 8-bit or 9-bit transactions. The length counter can be set to produce
//! transactions of any length from 1-255 *bytes*. For simplicity, this module
//! ignores character size. Instead, the SPI peripheral is always configured to
//! use 32-bit extension mode and the length counter.
//!
//! Upon creation, the [`Config`] takes ownership of both the [`Pads`] and the
//! PAC [`Sercom`] struct. It takes a reference to the MCLK, so that it can
//! enable the APB clock, and it takes a frequency to indicate the GCLK
//! configuration. Users are responsible for correctly configuring the GCLK.
//!
//! ```
//! use atsamd_hal::time::U32Ext;
//!
//! let mclk = peripherals.MCLK;
//! let sercom = peripherals.SERCOM0;
//! // Configure GCLK for 10 MHz
//! let freq = 10.mhz();
//! let config = spi::Config::new(&mclk, sercom, pads, freq);
//! ```
//!
//! The [`Config`] struct uses the builder pattern to configure the peripheral,
//! ending with a call to [`enable`], which consumes the [`Config`] and returns
//! an enabled [`Spi`] peripheral.
//!
//! ```
//! use embedded_hal::spi::MODE_1;
//!
//! let spi = spi::Config::new(&mclk, sercom, pads, freq)
//!     .baud(1.mhz())
//!     .length::<U2>()
//!     .msb_first(false)
//!     .spi_mode(MODE_1)
//!     .enable();
//! ```
//!
//! To be accepted as a [`ValidConfig`], the `Config` must have all the
//! necessary pads for its [`OpMode`].
//!
//! # [`Spi`]
//!
//! An [`Spi`] struct can only be created from a [`Config`], and it has only one
//! type parameter, the corresponding config.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::sercom::v2::spi::{Master, lengths::U2};
//! use atsamd_hal::sercom::v2::pad::IoSet1;
//! use atsamd_hal::typelevel::NoneT;
//!
//! type Pads = spi::PadsFromIds<Sercom0, IoSet1, PA08, NoneT, PA09>;
//! type Config = spi::Config<Pads, Master, U2>;
//! type Spi = spi::Spi<Config>;
//! ```
//!
//! Only [`Spi`]s struct can actually perform transactions. To do so, use the
//! embedded HAL traits, like [`spi::FullDuplex`](FullDuplex),
//! [`serial::Read`](Read) and [`serial::Write`](Write). See the [`Spi`]
//! documentation for more information about the trait implementations, which
//! vary based on the transaction [`Length`] and [`Pads`]. For instance,
//! [`FullDuplex`] is only implemented if the [`Pads`] are both [`Tx`] and
//! [`Rx`] and if the transaction [`Length`] is less than [`U4`].
//!
//! ```
//! use nb::block;
//! use embedded_hal::spi::FullDuplex;
//!
//! block!(spi.send(0xAA55));
//! let rcvd: u16 = block!(spi.read());
//! ```
//!
//! [`enable`]: Config::enable
//! [`bsp_pins`]: crate::bsp_pins
//! [`Pin`]: crate::gpio::v2::pin::Pin
//! [`PinId`]: crate::gpio::v2::pin::PinId
//! [`PinMode`]: crate::gpio::v2::pin::PinMode
#![cfg_attr(
    feature = "dma",
    doc = "
# Using SPI with DMA

This HAL includes support for DMA-enabled SPI transfers. An enabled [`Spi`]
struct implements the DMAC [`Buffer`] trait. The provided [`send_with_dma`]
and [`receive_with_dma`] methods will build and begin a [`dmac::Transfer`]
to create a non-blocking SPI transfer.

Optionally, interrupts can be enabled on the provided [`Channel`]. Note that
the `dma` feature must be enabled. Refer to the [`dmac`] module-level
documentation for more information.

```
// Assume channel is a configured `dmac::Channel`, and spi a
// fully-configured `Spi`

// Create data to send
let buffer: [u8; 50] = [0xff; 50]

// Launch the transfer
let dma_transfer = spi.send_with_dma(&mut buffer, channel, ());

// Wait for the transfer to complete and reclaim resources
let (chan0, _, spi, _) = dma_transfer.wait();
```

[`Buffer`]: crate::dmac::transfer::Buffer
[`send_with_dma`]: Spi::send_with_dma
[`receive_with_dma`]: Spi::receive_with_dma
[`dmac::Transfer`]: crate::dmac::Transfer
[`Channel`]: crate::dmac::channel::Channel
[`dmac`]: crate::dmac
"
)]

use core::convert::{TryFrom, TryInto};
use core::marker::PhantomData;

use bitflags::bitflags;
use embedded_hal::blocking;
use embedded_hal::serial::{Read, Write};
use embedded_hal::spi::{self, FullDuplex};
pub use embedded_hal::spi::{Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3};
use nb::Error::WouldBlock;
use num_traits::{AsPrimitive, PrimInt};
use seq_macro::seq;
use typenum::{Unsigned, U0, U1, U2, U3, U4};

use crate::target_device as pac;
use pac::sercom0::spim::ctrla::{CPHA_A, CPOL_A, DIPO_A, DOPO_A, DORD_A, MODE_A};
use pac::sercom0::RegisterBlock;
use pac::MCLK;

use crate::gpio::v2::AnyPin;
use crate::sercom::v2::*;
use crate::time::Hertz;
use crate::typelevel::{Is, NoneT, Sealed};

/// Re-export [`typenum`] constants for use as [`Length`] type parameters
///
/// Only the values `U1` - `U255` are valid [`Length`]s
pub mod lengths {
    use seq_macro::seq;

    seq!(N in 1..=255 {
        pub use typenum::U#N;
    });
}

//=============================================================================
// Dipo
//=============================================================================

/// Control the `DIPO` field as a function of the [`PadNum`] type
pub trait Dipo: Sealed {
    /// Corresponding variant from the PAC `enum`
    const DIPO: DIPO_A;

    /// Configure the pad according to [`Self::DIPO`]
    #[inline]
    fn configure(sercom: &RegisterBlock) {
        sercom
            .spim()
            .ctrla
            .modify(|_, w| w.dipo().variant(Self::DIPO));
    }
}

impl Dipo for Pad0 {
    const DIPO: DIPO_A = DIPO_A::PAD0;
}
impl Dipo for Pad1 {
    const DIPO: DIPO_A = DIPO_A::PAD1;
}
impl Dipo for Pad2 {
    const DIPO: DIPO_A = DIPO_A::PAD2;
}
impl Dipo for Pad3 {
    const DIPO: DIPO_A = DIPO_A::PAD3;
}

impl Dipo for NoneT {
    /// This value is arbitrary and meaningless for [`NoneT`]
    const DIPO: DIPO_A = DIPO_A::PAD0;

    /// Override the default implementation to do nothing
    fn configure(_: &RegisterBlock) {}
}

/// Lift the implementations of [`Dipo`] from [`OptionalPadNum`]s to the
/// corresponding [`Pads`] types.
impl<S, I, DI, DO, CK, SS> Dipo for Pads<S, I, DI, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
    DI::PadNum: Dipo,
{
    const DIPO: DIPO_A = DI::PadNum::DIPO;
}

//=============================================================================
// Dopo
//=============================================================================

/// Control the `DOPO` field as a function of the [`PadNum`] type
pub trait Dopo: Sealed {
    /// Corresponding variant from the PAC `enum`
    const DOPO: DOPO_A;

    /// Configure the pad according to [`Self::DOPO`]
    #[inline]
    fn configure(sercom: &RegisterBlock) {
        sercom
            .spim()
            .ctrla
            .modify(|_, w| w.dopo().variant(Self::DOPO));
    }
}

impl Dopo for Pad0 {
    const DOPO: DOPO_A = DOPO_A::PAD0;
}
impl Dopo for Pad3 {
    const DOPO: DOPO_A = DOPO_A::PAD2;
}

impl Dopo for NoneT {
    /// This value is arbitrary and meaningless for [`NoneT`]
    const DOPO: DOPO_A = DOPO_A::PAD0;

    /// Override the default implementation to do nothing
    fn configure(_: &RegisterBlock) {}
}

/// Lift the implementations of [`Dopo`] from [`OptionalPadNum`]s to the
/// corresponding [`Pads`] types.
impl<S, I, DI, DO, CK, SS> Dopo for Pads<S, I, DI, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DI: OptionalPad,
    DO: OptionalPad,
    CK: OptionalPad,
    SS: OptionalPad,
    DO::PadNum: Dopo,
{
    const DOPO: DOPO_A = DO::PadNum::DOPO;
}

//=============================================================================
// Pads
//=============================================================================

/// Container for a set of SERCOM pads
///
/// See the [module-level](self) documentation for more details on specifying
/// a `Pads` type and creating instances.
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
/// the total number of type parameters needed in the [`Config`] struct. The
/// `Config` struct doesn't need access to the [`Pin`]s directly.  Rather, it
/// only needs to apply the [`SomePad`] trait bound when a `Pin` is required.
/// The `PadSet` trait allows each `Config` struct to store an instance of
/// `Pads` without itself being generic over all six type parameters of the
/// `Pads` type.
///
/// [`Pin`]: crate::gpio::v2::Pin
/// [type-level function]: crate::typelevel#type-level-functions
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
/// This trait labels sets of [`Pads`] that satisfy the [`Dipo`] and [`Dopo`]
/// traits. It guarantees to the [`Config`] struct that this set of `Pads` can
/// be configured through those traits.
pub trait ValidPads: PadSet + Dipo + Dopo {}

impl<P: PadSet + Dipo + Dopo> ValidPads for P {}

//=============================================================================
// Tx/Rx
//=============================================================================

/// Marker trait for a set of [`Pads`] that can transmit
///
/// To transmit, both SCLK and Data Out must be [`SomePad`].
pub trait Tx: ValidPads {}

impl<P> Tx for P
where
    P: ValidPads,
    P::DataOut: SomePad,
    P::Sclk: SomePad,
{
}

/// Marker trait for a set of [`Pads`] that can receive
///
/// To receive, both SCLK and Data In must be [`SomePad`].
pub trait Rx: ValidPads {}

impl<P> Rx for P
where
    P: ValidPads,
    P::DataIn: SomePad,
    P::Sclk: SomePad,
{
}

/// Marker trait for a set of [`Pads`] that cannot transmit
///
/// A set of [`Pads`] cannot be used to transmit when the Data Out pad is
/// [`NoneT`].
pub trait NotTx: ValidPads {}

impl<P> NotTx for P where P: ValidPads<DataOut = NoneT> {}

/// Marker trait for a set of [`Pads`] that cannot receive
///
/// A set of [`Pads`] cannot be used to receive when the Data In pad is
/// [`NoneT`].
pub trait NotRx: ValidPads {}

impl<P> NotRx for P where P: ValidPads<DataIn = NoneT> {}

/// Marker trait for a set of [`Pads`] that can transmit OR receive
///
/// To satisfy this trait, SCLK must always be [`SomePad`] and one or both of
/// Data In and Data Out must also be [`SomePad`].
pub trait TxOrRx: ValidPads {}

impl<S, I, DI, CK, SS> TxOrRx for Pads<S, I, DI, NoneT, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DI: SomePad,
    CK: SomePad,
    SS: OptionalPad,
    DI::PadNum: Dipo,
{
}

impl<S, I, DO, CK, SS> TxOrRx for Pads<S, I, NoneT, DO, CK, SS>
where
    S: Sercom,
    I: IoSet,
    DO: SomePad,
    CK: SomePad,
    SS: OptionalPad,
    DO::PadNum: Dopo,
{
}

impl<P: Tx + Rx> TxOrRx for P {}

//=============================================================================
// Operating mode
//=============================================================================

/// Type-level enum representing the SPI operating mode
///
/// See the documentation on [type-level enums] for a discussion of the pattern.
///
/// The available operating modes are [`Master`], [`MasterHWSS`] and [`Slave`].
/// In [`Master`] mode, the `SS` signal must be handled by the user, so `SS`
/// must be [`NoneT`]. In [`MasterHWSS`] mode, the hardware drives the `SS`
/// line, so [`SomePad`] is required. In [`Slave`] mode, the `SS` pad is
/// required as well, to indicate when data is valid.
///
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait OpMode: Sealed {
    /// Corresponding variant from the PAC enum
    const MODE: MODE_A;

    /// Bit indicating whether hardware `SS` control is enabled
    const MSSEN: bool;

    /// Configure the SPI operating mode
    ///
    /// For maximum flexibility, this module chooses to always operate in 32-bit
    /// extension mode. The LENGTH counter is used to control the number of byes
    /// in each SPI transaction. Due to a hardware bug, ICSPACE must be at least
    /// one. See the silicon errata for more details.
    #[inline]
    fn configure(sercom: &RegisterBlock) -> () {
        sercom
            .spim()
            .ctrla
            .modify(|_, w| w.mode().variant(Self::MODE));
        sercom
            .spim()
            .ctrlb
            .modify(|_, w| w.mssen().bit(Self::MSSEN));
        sercom.spim().ctrlc.write(|w| unsafe {
            w.data32b().data_trans_32bit();
            w.icspace().bits(1)
        });
        while sercom.spim().syncbusy.read().ctrlb().bit_is_set() {}
    }
}

/// [`OpMode`] variant for Master mode
pub enum Master {}

/// [`OpMode`] variant for Master mode with hardware-controlled slave select
pub enum MasterHWSS {}

/// [`OpMode`] variant for Slave mode
pub enum Slave {}

impl Sealed for Master {}
impl Sealed for MasterHWSS {}
impl Sealed for Slave {}

impl OpMode for Master {
    const MODE: MODE_A = MODE_A::SPI_MASTER;
    const MSSEN: bool = false;
}

impl OpMode for MasterHWSS {
    const MODE: MODE_A = MODE_A::SPI_MASTER;
    const MSSEN: bool = true;
}

impl OpMode for Slave {
    const MODE: MODE_A = MODE_A::SPI_SLAVE;
    const MSSEN: bool = false;
}

/// Marker trait for Master operating modes
///
/// This trait is implemented for [`Master`] and [`MasterHWSS`] but not for
/// [`Slave`].
pub trait MasterMode: OpMode {}

impl MasterMode for Master {}
impl MasterMode for MasterHWSS {}

//=============================================================================
// Transaction length
//=============================================================================

/// Type-level enum representing the SPI transaction length, in bytes
///
/// This trait acts as both a [type-level enum], forming a type class for
/// transaction lengths, as well as a [type-level function] mapping the
/// corresponding word size.
///
/// As mentioned in the [`OpMode`] documentation, this module chooses to always
/// operate in 32-bit extension mode. The LENGTH counter is used to control the
/// number of byes in each SPI transaction.
///
/// The SPI transaction length is represented in the type domain using
/// [`Unsigned`] types from the [`typenum`] crate. The length can be set
/// statically, using a length from `U1` to `U255`, or it can be set
/// dynamically, using the [`DynLength`] marker type. The static [`Length`]
/// types can be imported from the [`lengths`] submodule.
///
/// The SPI transaction length affects the word size for the embedded HAL
/// traits, as well as other aspects of the SPI API. Transaction lengths of 1-4
/// only require a single read/write of the DATA register, so they behave
/// differently than longer transaction lengths.
///
/// [type-level enum]: crate::typelevel#type-level-enums
/// [type-level function]: crate::typelevel#type-level-functions
pub trait Length: Sealed + Unsigned + 'static {
    /// Word size for the transaction length
    ///
    /// For lengths 1-4, this type is `u8`, `u16` or `u32`. For longer
    /// transactions, this type is `[u8, Self::USIZE]`.
    type Word: 'static;

    /// Configure the `LENGTH` register and enable the `LENGTH` counter
    #[inline]
    fn configure(sercom: &RegisterBlock) -> () {
        sercom.spim().length.write(|w| unsafe {
            w.len().bits(Self::U8);
            w.lenen().set_bit()
        });
        while sercom.spim().syncbusy.read().length().bit_is_set() {}
    }
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

/// Marker trait for transactions that are performed atomically
pub trait AtomicLength: Length {}

impl Sealed for U1 {}
impl StaticLength for U1 {}
impl AtomicLength for U1 {}
impl Length for U1 {
    type Word = u8;
}

impl Sealed for U2 {}
impl StaticLength for U2 {}
impl AtomicLength for U2 {}
impl Length for U2 {
    type Word = u16;
}

impl Sealed for U3 {}
impl StaticLength for U3 {}
impl AtomicLength for U3 {}
impl Length for U3 {
    type Word = u32;
}

impl Sealed for U4 {}
impl StaticLength for U4 {}
impl AtomicLength for U4 {}
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

//=============================================================================
// Flags
//=============================================================================

bitflags! {
    /// Interrupt bit flags for SPI transactions
    ///
    /// The available interrupt flags are `DRE`, `RXC`, `TXC`, `SSL` and
    /// `ERROR`. The binary format of the underlying bits exactly matches the
    /// INTFLAG register.
    pub struct Flags: u8 {
        const DRE = 0x01;
        const RXC = 0x02;
        const TXC = 0x04;
        const SSL = 0x08;
        const ERROR = 0x80;
    }
}

//=============================================================================
// Errors
//=============================================================================

bitflags! {
    /// Error bit flags for SPI transactions
    ///
    /// The available error flags are `BUFOVF` and `LENERR`. The binary format
    /// of the underlying bits exactly matches the STATUS register.
    pub struct Errors: u16 {
        const BUFOVF = 0x0004;
        const LENERR = 0x0800;
    }
}

/// Error `enum` for SPI transactions
///
/// The SPI peripheral only has two error types, buffer overflow and transaction
/// length error.
#[derive(Debug)]
pub enum Error {
    Overflow,
    LengthError,
}

impl TryFrom<Errors> for () {
    type Error = Error;
    #[inline]
    fn try_from(errors: Errors) -> Result<(), Error> {
        // Buffer overflow has priority
        if errors.contains(Errors::BUFOVF) {
            Err(Error::Overflow)
        } else if errors.contains(Errors::LENERR) {
            Err(Error::LengthError)
        } else {
            Ok(())
        }
    }
}

//=============================================================================
// Config
//=============================================================================

/// A configurable, disabled SPI peripheral
///
/// This `struct` represents a configurable SPI peripheral in its disabled
/// state. It is generic over the set of [`Pads`], [`OpMode`] and transaction
/// [`Length`]. Upon creation, the [`Config`] takes ownership of the [`Sercom`]
/// and resets it, returning it configured as an SPI peripheral in [`Master`]
/// [`OpMode`] with [`Length`] [`U1`].
///
/// [`Config`] uses a builder-pattern API to configure the peripheral,
/// culminating in a call to [`enable`], which consumes the [`Config`] and
/// returns an enabled [`Spi`] peripheral. The [`enable`] function is restricted
/// to [`ValidConfig`]s.
///
/// [`enable`]: Config::enable
pub struct Config<P, M = Master, L = U1>
where
    P: ValidPads,
    M: OpMode,
    L: Length,
{
    sercom: P::Sercom,
    pad_map: P,
    mode: PhantomData<M>,
    len: PhantomData<L>,
    freq: Hertz,
}

impl<P: ValidPads> Config<P> {
    /// Create a new [`Config`] in the default configuration.
    #[inline]
    fn create(sercom: P::Sercom, pad_map: P, freq: impl Into<Hertz>) -> Self {
        Self::swrst(&sercom);
        Master::configure(&sercom);
        <P as Dipo>::configure(&sercom);
        <P as Dopo>::configure(&sercom);
        U1::configure(&sercom);
        Self {
            sercom,
            pad_map,
            mode: PhantomData,
            len: PhantomData,
            freq: freq.into(),
        }
    }

    /// Create a new [`Config`] in the default configuration
    ///
    /// This function will enable the corresponding APB clock, reset the
    /// [`Sercom`] peripheral, and return a [`Config`] in the default
    /// configuration, [`Master`] [`OpMode`] with [`Length`] [`U1`]. [`Config`]
    /// takes ownership of the [`Sercom`] and [`Pads`].
    ///
    /// Users must configure GCLK manually. The `freq` parameter represents the
    /// GCLK frequency for this [`Sercom`] instance.
    #[inline]
    pub fn new(mclk: &MCLK, mut sercom: P::Sercom, pad_map: P, freq: impl Into<Hertz>) -> Self {
        sercom.enable_apb_clock(mclk);
        Self::create(sercom, pad_map, freq)
    }
}

impl<P, M, L> Config<P, M, L>
where
    P: ValidPads,
    M: OpMode,
    L: Length,
{
    /// Reset the SERCOM peripheral
    #[inline]
    fn swrst(sercom: &P::Sercom) {
        sercom.spim().ctrla.write(|w| w.swrst().set_bit());
        while sercom.spim().syncbusy.read().swrst().bit_is_set() {}
        unsafe { Self::reset_serial_read_state() };
    }

    /// Reset the `static` state used by the implementation of [`Read`]
    ///
    /// The state should be reset whenever the peripheral is reset or disabled.
    /// It should not be possible for the state to become out of sync from safe
    /// code.
    #[inline]
    unsafe fn reset_serial_read_state() {
        SERIAL_READ_STATE[P::Sercom::NUM] = false;
    }

    /// Change the [`Config`] [`OpMode`] or [`Length`]
    #[inline]
    fn change<M2, L2>(self) -> Config<P, M2, L2>
    where
        M2: OpMode,
        L2: Length,
    {
        Config {
            sercom: self.sercom,
            pad_map: self.pad_map,
            mode: PhantomData,
            len: PhantomData,
            freq: self.freq,
        }
    }

    /// Trigger the [`Sercom`]'s SWRST and return a [`Config`] in the
    /// default configuration.
    #[inline]
    pub fn reset(self) -> Config<P> {
        Config::create(self.sercom, self.pad_map, self.freq)
    }

    /// Obtain a reference to the PAC `SERCOM` struct
    ///
    /// Directly accessing the `SERCOM` could break the invariants of the
    /// type-level tracking in this module, so it is unsafe.
    #[inline]
    pub unsafe fn sercom(&self) -> &P::Sercom {
        &self.sercom
    }

    /// Consume the [`Config`], reset the peripheral, and return the [`Sercom`]
    /// and [`Pads`]
    #[inline]
    pub fn free(self) -> (P::Sercom, P) {
        Self::swrst(&self.sercom);
        (self.sercom, self.pad_map)
    }

    /// Change the [`OpMode`]
    #[inline]
    pub fn op_mode<M2: OpMode>(self) -> Config<P, M2, L> {
        M2::configure(&self.sercom);
        self.change()
    }

    /// Change the transaction [`Length`]
    ///
    /// To use a run-time dynamic length, set the [`Length`] type to
    /// [`DynLength`] and then use the [`get_dyn_length`] and [`set_dyn_length`]
    /// methods.
    ///
    /// [`get_dyn_length`]: Config::get_dyn_length
    /// [`set_dyn_length`]: Config::set_dyn_length
    #[inline]
    pub fn length<L2: Length>(self) -> Config<P, M, L2> {
        L2::configure(&self.sercom);
        self.change()
    }

    /// Change the clock polarity
    #[inline]
    pub fn cpol(self, cpol: Polarity) -> Self {
        let cpol = match cpol {
            Polarity::IdleLow => CPOL_A::IDLE_LOW,
            Polarity::IdleHigh => CPOL_A::IDLE_HIGH,
        };
        self.sercom
            .spim()
            .ctrla
            .modify(|_, w| w.cpol().variant(cpol));
        self
    }

    /// Change the clock phase
    #[inline]
    pub fn cpha(self, cpha: Phase) -> Self {
        let cpha = match cpha {
            Phase::CaptureOnFirstTransition => CPHA_A::LEADING_EDGE,
            Phase::CaptureOnSecondTransition => CPHA_A::TRAILING_EDGE,
        };
        self.sercom
            .spim()
            .ctrla
            .modify(|_, w| w.cpha().variant(cpha));
        self
    }

    /// Change the SPI mode (clock polarity & phase)
    #[inline]
    pub fn spi_mode(self, mode: spi::Mode) -> Self {
        let cpol = match mode.polarity {
            Polarity::IdleLow => CPOL_A::IDLE_LOW,
            Polarity::IdleHigh => CPOL_A::IDLE_HIGH,
        };
        let cpha = match mode.phase {
            Phase::CaptureOnFirstTransition => CPHA_A::LEADING_EDGE,
            Phase::CaptureOnSecondTransition => CPHA_A::TRAILING_EDGE,
        };
        self.sercom.spim().ctrla.modify(|_, w| {
            w.cpol().variant(cpol);
            w.cpha().variant(cpha)
        });
        self
    }

    /// Change the bit order of transmission (MSB/LSB first)
    ///
    /// This only affects the order of bits within each byte. Bytes are always
    /// transferred in little endian order from the 32-bit DATA register.
    #[inline]
    pub fn msb_first(self, msb_first: bool) -> Self {
        let dord = match msb_first {
            true => DORD_A::MSB,
            false => DORD_A::LSB,
        };
        self.sercom
            .spim()
            .ctrla
            .modify(|_, w| w.dord().variant(dord));
        self
    }

    /// Set the baud rate
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// half the GCLK frequency. The minimum baud rate is the GCLK frequency /
    /// 512. Values outside this range will saturate at the extremes.
    #[inline]
    pub fn baud<B: Into<Hertz>>(self, baud: B) -> Self {
        let baud: Hertz = baud.into();
        let baud = (self.freq.0 / 2 / baud.0).saturating_sub(1);
        let baud = if baud <= u8::MAX as u32 {
            baud as u8
        } else {
            u8::MAX
        };
        self.sercom
            .spim()
            .baud
            .modify(|_, w| unsafe { w.baud().bits(baud) });
        self
    }

    /// Control the buffer overflow notification
    ///
    /// If set to true, an [`Error::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    pub fn immediate_overflow_notification(&mut self, set: bool) {
        self.sercom.spim().ctrla.modify(|_, w| w.ibon().bit(set));
    }

    /// Run in standby mode
    ///
    /// When set, the SPI peripheral will run in standby mode. See the datasheet
    /// for more details.
    #[inline]
    pub fn run_in_standby(&mut self, set: bool) {
        self.sercom
            .spim()
            .ctrla
            .modify(|_, w| w.runstdby().bit(set));
    }

    /// Enable interrupts for the specified flags
    #[inline]
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.sercom
            .spim()
            .intenset
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Disable interrupts for the specified flags
    #[inline]
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.sercom
            .spim()
            .intenclr
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Enable the SPI peripheral
    ///
    /// SPI transactions are not possible until the peripheral is enabled.
    /// This function is limited to [`ValidConfig`]s.
    #[inline]
    pub fn enable(mut self) -> Spi<Self>
    where
        Self: ValidConfig,
    {
        self.sercom.spim().ctrlb.modify(|_, w| w.rxen().set_bit());
        while self.sercom.spim().syncbusy.read().ctrlb().bit_is_set() {}
        self.enable_peripheral(true);
        Spi { config: self }
    }

    /// Enable or disable the SERCOM peripheral, and wait for the ENABLE bit to
    /// synchronize.
    fn enable_peripheral(&mut self, enable: bool) {
        self.sercom
            .spim()
            .ctrla
            .modify(|_, w| w.enable().bit(enable));
        while self.sercom.spim().syncbusy.read().enable().bit_is_set() {}
    }
}

impl<P, M> Config<P, M, DynLength>
where
    P: ValidPads,
    M: OpMode,
{
    /// Return the current transaction length
    ///
    /// Read the LENGTH register to determine the current transaction length
    #[inline]
    pub fn get_dyn_length(&self) -> u8 {
        self.sercom.spim().length.read().len().bits()
    }

    /// Set the transaction length
    ///
    /// Write the LENGTH register to set the transaction length. Panics if the
    /// length is zero.
    #[inline]
    pub fn set_dyn_length(&mut self, length: u8) {
        if length == 0 {
            panic!("Cannot set SPI LENGTH field to zero")
        }
        self.sercom.spim().length.write(|w| unsafe {
            w.lenen().set_bit();
            w.len().bits(length)
        });
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
/// copies the [`Sercom`] and [`Word`] types, to make it easier to apply
/// bounds to these types at the next level of abstraction.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait AnyConfig: Is<Type = SpecificConfig<Self>> {
    type Sercom: Sercom;
    type Pads: ValidPads<Sercom = Self::Sercom>;
    type OpMode: OpMode;
    type Length: Length<Word = Self::Word>;
    type Word: 'static;
}

/// Type alias to recover the specific [`Config`] type from an implementation of
/// [`AnyConfig`]
pub type SpecificConfig<C> =
    Config<<C as AnyConfig>::Pads, <C as AnyConfig>::OpMode, <C as AnyConfig>::Length>;

impl<P, M, L> Sealed for Config<P, M, L>
where
    P: ValidPads,
    M: OpMode,
    L: Length,
{
}

impl<P, M, L> AnyConfig for Config<P, M, L>
where
    P: ValidPads,
    M: OpMode,
    L: Length,
{
    type Sercom = P::Sercom;
    type Pads = P;
    type OpMode = M;
    type Length = L;
    type Word = L::Word;
}

impl<P, M, L> AsRef<Self> for Config<P, M, L>
where
    P: ValidPads,
    M: OpMode,
    L: Length,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<P, M, L> AsMut<Self> for Config<P, M, L>
where
    P: ValidPads,
    M: OpMode,
    L: Length,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

//=============================================================================
// ValidConfig
//=============================================================================

/// Marker trait for valid SPI [`Config`]urations
///
/// A functional SPI peripheral must have, at a minimum, an SCK pad and
/// either a Data In or a Data Out pad. Dependeing on the [`OpMode`], an SS
/// pad may also be required.
///
/// The `ValidConfig` trait is implemented only for valid combinations of
/// [`Pads`] and [`OpMode`]. No [`Config`] is valid if the SCK pad is [`NoneT`]
/// or if both the Data In and Data Out pads are `NoneT`. When in [`Master`]
/// `OpMode`, the `SS` pad must be `NoneT`, while in [`MasterHWSS`] or
/// [`Slave`] [`OpMode`], the SS pad must be [`SomePad`].
pub trait ValidConfig: AnyConfig {}

impl<P, L> ValidConfig for Config<P, Master, L>
where
    P: TxOrRx<SS = NoneT>,
    L: Length,
{
}

impl<P, L> ValidConfig for Config<P, MasterHWSS, L>
where
    P: TxOrRx,
    L: Length,
    P::SS: SomePad,
{
}

impl<P, L> ValidConfig for Config<P, Slave, L>
where
    P: TxOrRx,
    L: Length,
    P::SS: SomePad,
{
}

//=============================================================================
// Spi
//=============================================================================

/// An enabled SPI peripheral that can perform transactions
///
/// As noted in the [`OpMode`] and [`Length`] traits, this module chooses to
/// always operate in 32-bit extension mode and uses the LENGTH counter to set
/// the number of bytes in each transaction, from 1 to 255. In 32-bit extension
/// mode, transaction [`Length`]s of four bytes or fewer can be completed in a
/// single read or write of the DATA register. Longer transactions require
/// multiple reads or writes of the DATA register.
///
/// # Embedded HAL traits
///
/// The embedded HAL [`FullDuplex`] trait is non-blocking, which has
/// consequences for different transaction [`Length`]s. Transaction [`Length`]s
/// of 1-4 bytes fit the [`FullDuplex`] model well, because they can return
/// [`WouldBlock`] until it is possible to perform a single read or write that
/// completes the transaction. The [`FullDuplex`] word type is [`u8`] for 1-byte
/// transactions, [`u16`] for 2-byte transactions and [`u32`] for 3- and 4-byte
/// transactions.
///
/// For half-duplex, non-blocking transactions, this type implements the
/// `serial` [`Read`] and [`Write`] traits. They are only implemented for
/// transaction [`Length`]s `<= 4` and only when the [`Pads`] are not
/// [`Tx`]` + `[`Rx`], so they don't compete with [`FullDuplex`].
///
/// Longer [`Length`]s require multiple reads or writes, which does not fit the
/// [`FullDuplex`] model very well. When returning [`WouldBlock`], the
/// implementation would have to store some sort of internal state to track the
/// progress of an on-going transaction. As a consequence, [`FullDuplex`] is
/// only implemented for [`Length`]s of 1-4.
///
/// However, the [`blocking::spi`] traits are different, because they can always
/// execute a transaction to completion before returning. For [`Length`]s of
/// 1-4 bytes, these traits are implemented using an approach very similar to
/// the `Default` implementations. Longer [`Length`]s use a custom
/// implementation of [`Transfer`] and [`Write`](blocking::spi::Write) and
/// accept slices of `u8` with a length matching the transaction [`Length`].
///
/// When using a [`DynLength`], [`Spi`] only implements [`Transfer`] and
/// [`Write`](blocking::spi::Write), using the same approach as long transaction
/// [`Length`]s.
///
/// For a non-blocking alternative that can be used to transfer slices, see the
/// [`SpiFuture`] type.
///
/// # Further documentation
///
/// The individual implementations of the embedded HAL traits are well
/// documented. Unfortunately, however, rustdoc does not produce the most
/// visually scannable documentation, especially when using `typenum`. Here are
/// some tips for finding more information in the `impl` blocks below:
///
/// - [`FullDuplex`] uses a single blanket implementation for [`Length`]s of 1-4
/// - [`Read`] has two different implementations, one for [`Slave`] [`OpMode`]
///   and another for [`MasterMode`]s
/// - [`Write`] uses a single blanket implementation, like [`FullDuplex`].
///   However, watch out for ambiguity between [`serial::Write`](Write),
///   [`blocking::serial::Write`] and [`blocking::spi::Write`].
/// - [`blocking::serial::Write`] uses the [`blocking::serial::write::Default`]
///   implementation
/// - [`blocking::spi::Transfer`] uses custom implementations. The
///   implementations for [`Length`]s of 1-4 are generated by macro, so there
///   are four different implementations for `U1` - `U4`. This is responsible
///   for most of the visual noise in the docs. [`Length`]s greater than four
///   use a blanket implementation. [`DynLength`] uses its own implementation
///   with an added run-time check of the transaction length.
/// - [`blocking::spi::Write`] takes the same approach as [`Transfer`].
/// - [`blocking::spi::WriteIter`] is implemented via macro for [`Length`]s 1-4,
///   but it is not implemented for longer [`Length`]s or [`DynLength`].
///
/// [`SpiFuture`]: crate::sercom::v2::spi_future::SpiFuture
/// [`Transfer`]: blocking::spi::Transfer
pub struct Spi<C: ValidConfig> {
    config: C,
}

impl<C: ValidConfig> Spi<C> {
    /// Obtain a reference to the PAC `SERCOM` struct
    ///
    /// Directly accessing the `SERCOM` could break the invariants of the
    /// type-level tracking in this module, so it is unsafe.
    #[inline]
    pub unsafe fn sercom(&self) -> &C::Sercom {
        self.config.as_ref().sercom()
    }

    /// Update the SPI configuration.
    ///
    /// Calling this method will temporarily disable the SERCOM peripheral, as
    /// some registers are enable-protected. This may interrupt any ongoing
    /// transactions.
    #[inline]
    pub fn reconfigure<F>(&mut self, update: F)
    where
        F: FnOnce(SpecificConfig<C>) -> SpecificConfig<C>,
    {
        self.config.as_mut().enable_peripheral(false);

        // Perform a bitwise copy of the old configuration. This will be used as default
        // in case the call to update(self.config) panics. This should be safe
        // as either one of self.config or old_config will be used, and Config
        // does not deallocate when dropped.
        let old_config = unsafe { core::ptr::read(&mut self.config as *const _) };
        replace_with::replace_with(&mut self.config, || old_config, |c| update(c.into()).into());

        self.config.as_mut().enable_peripheral(true);
    }

    /// Change the transaction [`Length`]
    ///
    /// Changing the transaction [`Length`] while is enabled is permissible but
    /// `unsafe`. If you have sent or received *any* bytes at the current
    /// [`Length`], you **must** wait for a TXC flag before changing to a new
    /// [`Length`].
    #[inline]
    pub unsafe fn length<L: Length>(self) -> Spi<Config<C::Pads, C::OpMode, L>>
    where
        Config<C::Pads, C::OpMode, L>: ValidConfig,
    {
        Spi {
            config: self.config.into().length(),
        }
    }

    /// Enable interrupts for the specified flags
    #[inline]
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.config.as_mut().enable_interrupts(flags)
    }

    /// Disable interrupts for the specified flags
    #[inline]
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.config.as_mut().disable_interrupts(flags);
    }

    /// Read the interrupt status flags
    #[inline]
    pub fn read_flags(&self) -> Flags {
        let bits = unsafe { self.sercom().spim().intflag.read().bits() };
        Flags::from_bits_truncate(bits)
    }

    /// Clear interrupt status flags
    ///
    /// Setting the ERROR, SSL or TXC flag will clear the interrupt. Clearing
    /// any flag will have no effect. This function has no effect on the DRE or
    /// RXC flags.
    ///
    /// **Warning:** The implementation of of [`Write::flush`] waits on and
    /// clears the `TXC` flag. Manually clearing this flag could cause it to
    /// hang indefinitely.
    #[inline]
    pub fn clear_flags(&mut self, flags: Flags) {
        unsafe { self.sercom().spim().intflag.write(|w| w.bits(flags.bits())) };
    }

    /// Read the error status flags
    #[inline]
    pub fn read_errors(&self) -> Errors {
        let bits = unsafe { self.sercom().spim().status.read().bits() };
        Errors::from_bits_truncate(bits)
    }

    /// Clear error status flags
    ///
    /// Setting a flag will clear the error. Clearing any flag will have no
    /// effect.
    #[inline]
    pub fn clear_errors(&mut self, errors: Errors) {
        unsafe { self.sercom().spim().status.write(|w| w.bits(errors.bits())) };
    }

    #[inline]
    pub fn read_flags_errors(&self) -> Result<Flags, Error> {
        self.read_errors().try_into()?;
        Ok(self.read_flags())
    }

    /// Read from the DATA register
    ///
    /// Reading from the data register directly is `unsafe`, because it will
    /// clear the RXC flag, which could break assumptions made elsewhere in
    /// this module.
    #[inline]
    pub unsafe fn read_data(&mut self) -> u32 {
        self.sercom().spim().data.read().bits()
    }

    /// Write to the DATA register
    ///
    /// Writing to the data register directly is `unsafe`, because it will clear
    /// the DRE flag, which could break assumptions made elsewhere in this
    /// module.
    #[inline]
    pub unsafe fn write_data(&mut self, data: u32) {
        self.sercom().spim().data.write(|w| w.bits(data))
    }

    /// Disable the SPI peripheral and return the [`Config`] struct
    #[inline]
    pub fn disable(self) -> C {
        // SAFETY: The read state must be reset when disabling the peripheral
        unsafe { Config::<C::Pads>::reset_serial_read_state() };
        let spim = unsafe { self.sercom().spim() };
        spim.ctrlb.modify(|_, w| w.rxen().clear_bit());
        while spim.syncbusy.read().ctrlb().bit_is_set() {}
        spim.ctrla.modify(|_, w| w.enable().clear_bit());
        while spim.syncbusy.read().enable().bit_is_set() {}
        self.config
    }
}

impl<P, M> Spi<Config<P, M, DynLength>>
where
    P: ValidPads,
    M: OpMode,
    Config<P, M, DynLength>: ValidConfig,
{
    /// Return the current transaction length
    ///
    /// Read the LENGTH register to determine the current transaction length
    #[inline]
    pub fn get_dyn_length(&self) -> u8 {
        self.config.get_dyn_length()
    }

    /// Set the transaction length
    ///
    /// Write the LENGTH register to set the transaction length. Panics if the
    /// length is zero.
    ///
    /// # Safety
    ///
    /// If you have sent any data at the current transaction length, you
    /// **must** wait for `TXC` before changing the length.
    #[inline]
    pub unsafe fn set_dyn_length(&mut self, length: u8) {
        self.config.set_dyn_length(length)
    }
}

impl<C> Spi<C>
where
    C: ValidConfig,
    C::Pads: Rx + NotTx,
    C::OpMode: MasterMode,
    C::Word: PrimInt,
{
    /// Reset the internal state tracking `serial` [`Read`] transactions
    ///
    /// See the implementation of [`Read`] for more details.
    #[inline]
    pub unsafe fn reset_serial_read_state(&mut self) {
        SpecificConfig::<C>::reset_serial_read_state();
    }
}

//=============================================================================
// AnySpi
//=============================================================================

/// Type class for all possible [`Spi`] types
///
/// This trait uses the [`AnyKind`] trait pattern to create a [type class] for
/// [`Spi`] types. See the `AnyKind` documentation for more details on the
/// pattern.
///
/// In addition to the normal, `AnyKind` associated types. This trait also
/// copies the [`Sercom`], [`Pads`], [`OpMode`], [`Length`] and [`Word`] types,
/// to make it easier to apply bounds to these types at the next level of
/// abstraction.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait AnySpi: Is<Type = SpecificSpi<Self>> {
    type Sercom: Sercom;
    type Pads: ValidPads<Sercom = Self::Sercom>;
    type OpMode: OpMode;
    type Length: Length<Word = Self::Word>;
    type Word: 'static;
    type Config: ValidConfig<
        Sercom = Self::Sercom,
        Pads = Self::Pads,
        OpMode = Self::OpMode,
        Length = Self::Length,
        Word = Self::Word,
    >;
}

/// Type alias to recover the specific [`Spi`] type from an implementation of
/// [`AnySpi`]
pub type SpecificSpi<S> = Spi<<S as AnySpi>::Config>;

impl<C: ValidConfig> AsRef<Self> for Spi<C> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<C: ValidConfig> AsMut<Self> for Spi<C> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<C: ValidConfig> Sealed for Spi<C> {}

impl<C: ValidConfig> AnySpi for Spi<C> {
    type Sercom = C::Sercom;
    type Pads = C::Pads;
    type OpMode = C::OpMode;
    type Length = C::Length;
    type Word = C::Word;
    type Config = C;
}

//=============================================================================
// Embedded HAL traits
//=============================================================================

#[cfg(not(feature = "min-samd51n"))]
static mut SERIAL_READ_STATE: [bool; 6] = [false; 6];
#[cfg(feature = "min-samd51n")]
static mut SERIAL_READ_STATE: [bool; 8] = [false; 8];

/// Implement [`Read`] for [`MasterMode`]s
///
/// [`Read`] is only implemented when the [`Pads`] are [`Rx`] but [`NotTx`].
/// If the [`Pads`] are both [`Rx`] and [`Tx`], then use [`FullDuplex`].
///
/// In a [`MasterMode`], [`Read`] has to initiate transactions and receive the
/// responses. To do so, it uses a `static` allocation to keep track of the
/// transaction state. If a transaction is in progress, it will wait on `RXC`.
/// If not, it will wait on `DRE` and then send `0`.
///
/// It should not be possible for the tracked state to become invalid using only
/// safe code. However, if using `unsafe`, the state could fall out of sync. In
/// that case, the [`Spi::reset_serial_read_state`] method can be used to reset
/// the state. After reset, a transaction is assumed to NOT be in progress.
impl<P, M, L> Read<L::Word> for Spi<Config<P, M, L>>
where
    Config<P, M, L>: ValidConfig,
    P: Rx + NotTx,
    M: MasterMode,
    L: Length,
    L::Word: PrimInt,
    u32: AsPrimitive<L::Word>,
{
    type Error = Error;

    /// If a transaction is already in progress, wait for an `RXC` flag, then
    /// read the word. If not, wait for a `DRE` flag, then write `0`.
    ///
    /// Track the transaction state with an internal `static`. It should not be
    /// possible for the state to get out of sync with the hardware using only
    /// safe code. If using `unsafe` and the state does get out of sync, use
    /// [`Spi::reset_serial_read_state`] to reset it.
    #[inline]
    fn read(&mut self) -> nb::Result<L::Word, Error> {
        let index = <P::Sercom as Sercom>::NUM;
        let in_progress = unsafe { &mut SERIAL_READ_STATE[index] };
        let flags = self.read_flags_errors()?;
        if !*in_progress && flags.contains(Flags::DRE) {
            unsafe { self.write_data(0) };
            *in_progress = true;
            Err(WouldBlock)
        } else if *in_progress && flags.contains(Flags::RXC) {
            *in_progress = false;
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }
}

/// Implement [`Read`] for [`Slave`] [`OpMode`]
///
/// [`Read`] is only implemented when the [`Pads`] are [`Rx`] but [`NotTx`].
/// If the [`Pads`] are both [`Rx`] and [`Tx`], then use [`FullDuplex`].
///
/// In [`Slave`] [`OpMode`], [`Read`] does not have to initiate transactions, so
/// it does not have to store any internal state. It only has to wait on `RXC`.
impl<P, L> Read<L::Word> for Spi<Config<P, Slave, L>>
where
    Config<P, Slave, L>: ValidConfig,
    P: Rx + NotTx,
    L: Length,
    L::Word: PrimInt,
    u32: AsPrimitive<L::Word>,
{
    type Error = Error;

    /// Wait for an `RXC` flag, then read the word
    #[inline]
    fn read(&mut self) -> nb::Result<L::Word, Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::RXC) {
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }
}

/// Implement [`Write`] for [`Spi`]
///
/// [`Write`] is only implemented when the [`Pads`] are [`Tx`] but [`NotRx`].
/// If the [`Pads`] are both [`Tx`] and [`Rx`], then use [`FullDuplex`].
///
/// Because [`Write`] is only implemented when the [`Pads`] are [`NotRx`], this
/// implementation never reads the DATA register and ignores all buffer overflow
/// errors.
impl<C> Write<C::Word> for Spi<C>
where
    C: ValidConfig,
    C::Pads: Tx + NotRx,
    C::Word: PrimInt + AsPrimitive<u32>,
{
    type Error = Error;

    /// Wait for a `DRE` flag, then write a word
    #[inline]
    fn write(&mut self, word: C::Word) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_errors().contains(Errors::LENERR) {
            Err(Error::LengthError.into())
        } else if self.read_flags().contains(Flags::DRE) {
            unsafe { self.write_data(word.as_()) };
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }

    /// Wait for a `TXC` flag
    #[inline]
    fn flush(&mut self) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_errors().contains(Errors::LENERR) {
            Err(Error::LengthError.into())
        } else if self.read_flags().contains(Flags::TXC) {
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

impl<C> blocking::serial::write::Default<C::Word> for Spi<C>
where
    C: ValidConfig,
    Spi<C>: Write<C::Word>,
{
}

/// Implement [`FullDuplex`] for short [`Spi`] transaction [`Length`]s
///
/// [`FullDuplex` is only implemented when [`Pads`] is both [`Tx`] and [`Rx`],
/// the [`OpMode`] is a [`MasterMode`], and the transaction [`Length`] is `<= 4`
/// bytes. When the [`Length`] is `<= 4`, the [`Word`] is a primitive
/// integer, with a size that depends on the [`Length`] (`u8`, `u16` or `u32`).
impl<C> FullDuplex<C::Word> for Spi<C>
where
    C: ValidConfig,
    C::Pads: Tx + Rx,
    C::OpMode: MasterMode,
    C::Word: PrimInt + AsPrimitive<u32>,
    u32: AsPrimitive<C::Word>,
{
    type Error = Error;

    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::RXC) {
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }

    #[inline]
    fn send(&mut self, word: C::Word) -> nb::Result<(), Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::DRE) {
            unsafe { self.write_data(word.as_()) };
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

macro_rules! impl_blocking_traits {
    ( $($Length:ident),+ ) => {
        $(

            /// Implement [`Transfer`] for short [`Spi`] transaction [`Length`]s
            ///
            /// The [`Spi`] [`Pads`] must be [`Rx`] and the transaction
            /// [`Length`] must be `<= 4`. The transfer accepts a slice of
            /// primitive integers, depending on the [`Length`] (`u8`, `u16` or
            /// `u32`).
            ///
            /// [`Transfer`]: blocking::spi::Transfer
            impl<P, M> blocking::spi::Transfer<Word<$Length>> for Spi<Config<P, M, $Length>>
            where
                P: Rx,
                M: OpMode,
                Config<P, M, $Length>: ValidConfig,
            {
                type Error = Error;

                #[inline]
                fn transfer<'w>(&mut self, words: &'w mut [Word<$Length>]) -> Result<&'w [Word<$Length>], Error> {
                    for word in words.iter_mut() {
                        loop {
                            let flags = self.read_flags_errors()?;
                            if flags.contains(Flags::DRE) {
                                unsafe { self.write_data(*word as u32) };
                                break
                            }
                        }
                        loop {
                            let flags = self.read_flags_errors()?;
                            if flags.contains(Flags::RXC) {
                                *word = unsafe { self.read_data() as Word<$Length> };
                                break
                            }
                        }
                    }
                    Ok(words)
                }
            }

            /// Implement [`Write`] for short [`Spi`] transaction [`Length`]s
            ///
            /// The [`Spi`] [`Pads`] must be [`Tx`] but [`NotRx`] and the
            /// transaction [`Length`] must be `<= 4`. The transfer accepts a
            /// slice of primitive integers, depending on the [`Length`] (`u8`,
            /// `u16` or `u32`).
            ///
            /// Because [`Write`] is only implemented when the [`Pads`] are
            /// [`NotRx`], this implementation never reads the DATA register and
            /// ignores all buffer overflow errors.
            ///
            /// [`Write`]: blocking::spi::Write
            impl<P, M> blocking::spi::Write<Word<$Length>> for Spi<Config<P, M, $Length>>
            where
                P: Tx + NotRx,
                M: OpMode,
                Config<P, M, $Length>: ValidConfig,
            {
                type Error = Error;

                #[inline]
                fn write(&mut self, words: &[Word<$Length>]) -> Result<(), Error> {
                    for word in words {
                        loop {
                            // Ignore buffer overflow errors
                            if self.read_errors().contains(Errors::LENERR) {
                                return Err(Error::LengthError)
                            } else if self.read_flags().contains(Flags::DRE) {
                                unsafe { self.write_data(*word as u32) };
                                break
                            }
                        }
                    }
                    Ok(())
                }
            }

            /// Implement [`WriteIter`] for short [`Spi`] transaction [`Length`]s
            ///
            /// The [`Spi`] [`Pads`] must be [`Tx`] but [`NotRx`] and the
            /// transaction [`Length`] must be `<= 4`. The transfer accepts an
            /// iterator that yields primitive integers, depending on the
            /// [`Length`] (`u8`, `u16` or `u32`).
            ///
            /// Because [`WriteIter`] is only implemented when the [`Pads`] are
            /// [`NotRx`], this implementation never reads the DATA register and
            /// ignores all buffer overflow errors.
            ///
            /// [`WriteIter`]: blocking::spi::WriteIter
            #[cfg(feature = "unproven")]
            impl<P, M> blocking::spi::WriteIter<Word<$Length>> for Spi<Config<P, M, $Length>>
            where
                P: Tx + NotRx,
                M: OpMode,
                Config<P, M, $Length>: ValidConfig,
            {
                type Error = Error;

                #[inline]
                fn write_iter<WI>(&mut self, words: WI) -> Result<(), Error>
                where
                    WI: IntoIterator<Item = Word<$Length>>,
                {
                    for word in words.into_iter() {
                        loop {
                            // Ignore buffer overflow errors
                            if self.read_errors().contains(Errors::LENERR) {
                                return Err(Error::LengthError)
                            } else if self.read_flags().contains(Flags::DRE) {
                                unsafe { self.write_data(word as u32) };
                                break
                            }
                        }
                    }
                    Ok(())
                }
            }
        )+
    };
}

impl_blocking_traits!(U1, U2, U3, U4);

/// Implement [`Transfer`] for longer [`Spi`] transaction [`Length`]s
///
/// The [`Spi`] [`Pads`] must be [`Rx`] and the transaction [`Length`] must be
/// `> 4`. The transfer accepts a slice of `u8` with a length equal to the
/// transaction [`Length`]. If the slice length is incorrect, it will panic.
///
/// [`Transfer`]: blocking::spi::Transfer
impl<'a, P, M, L> blocking::spi::Transfer<u8> for Spi<Config<P, M, L>>
where
    Config<P, M, L>: ValidConfig,
    P: Rx,
    M: OpMode,
    L: GreaterThan4,
{
    type Error = Error;

    #[inline]
    fn transfer<'w>(&mut self, buf: &'w mut [u8]) -> Result<&'w [u8], Error> {
        assert_eq!(buf.len(), L::USIZE);
        let sercom = unsafe { self.sercom() };
        transfer_slice(sercom, buf)
    }
}

/// Implement [`Transfer`] for [`Spi`] types with [`DynLength`]
///
/// The [`Spi`] [`Pads`] must be [`Rx`]. The transfer accepts a slice of `u8`
/// with a length equal to the run-time dynamic transaction length. If the slice
/// length does not match the result of [`Spi::get_dyn_length`], it will panic.
///
/// [`Transfer`]: blocking::spi::Transfer
impl<P, M> blocking::spi::Transfer<u8> for Spi<Config<P, M, DynLength>>
where
    P: Rx,
    M: OpMode,
    Config<P, M, DynLength>: ValidConfig<Length = DynLength>,
{
    type Error = Error;

    #[inline]
    fn transfer<'w>(&mut self, buf: &'w mut [u8]) -> Result<&'w [u8], Error> {
        assert_eq!(buf.len(), self.get_dyn_length() as usize);
        let sercom = unsafe { self.sercom() };
        transfer_slice(sercom, buf)
    }
}

/// Transfer a slice over SPI
///
/// This function exists to avoid monomorphization code bloat
fn transfer_slice<'w>(sercom: &RegisterBlock, buf: &'w mut [u8]) -> Result<&'w [u8], Error> {
    let cells = core::cell::Cell::from_mut(buf).as_slice_of_cells();
    let mut to_send = cells.iter();
    let mut to_recv = cells.iter();
    while to_recv.len() > 0 {
        let errors = sercom.spim().status.read();
        if errors.bufovf().bit_is_set() {
            return Err(Error::Overflow);
        } else if errors.lenerr().bit_is_set() {
            return Err(Error::LengthError);
        }
        let flags = sercom.spim().intflag.read();
        if to_send.len() > 0 && flags.dre().bit_is_set() {
            let mut bytes = [0; 4];
            for byte in bytes.iter_mut() {
                match to_send.next() {
                    Some(cell) => *byte = cell.get(),
                    None => break,
                }
            }
            let word = u32::from_le_bytes(bytes);
            sercom.spim().data.write(|w| unsafe { w.data().bits(word) });
        }
        if to_recv.len() > to_send.len() && flags.rxc().bit_is_set() {
            let word = sercom.spim().data.read().data().bits();
            let bytes = word.to_le_bytes();
            for byte in bytes.iter() {
                match to_recv.next() {
                    Some(cell) => cell.set(*byte),
                    None => break,
                }
            }
        }
    }
    Ok(buf)
}

/// Implement [`Write`] for longer [`Spi`] transaction [`Length`]s
///
/// The [`Spi`] [`Pads`] must be [`Tx`] but [`NotRx`] and the transaction
/// [`Length`] must be `> 4`. The transfer accepts a slice of `u8` with a length
/// equal to the transfer [`Length`]. If the slice length is incorrect, it will
/// panic.
///
/// Because [`Write`] is only implemented when the [`Pads`] are [`NotRx`], this
/// implementation never reads the DATA register and ignores all buffer overflow
/// errors.
///
/// [`Write`]: blocking::spi::Write
impl<P, M, L> blocking::spi::Write<u8> for Spi<Config<P, M, L>>
where
    Config<P, M, L>: ValidConfig,
    P: Tx,
    M: OpMode,
    L: GreaterThan4,
{
    type Error = Error;

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        assert_eq!(buf.len(), L::USIZE);
        let sercom = unsafe { self.sercom() };
        write_slice(sercom, buf)
    }
}

/// Implement [`Write`] for [`Spi`] types with [`DynLength`]
///
/// The [`Spi`] [`Pads`] must be [`Tx`]. The transfer accepts a slice of `u8`
/// with a length equal to the run-time dynamic transaction length. If the slice
/// length does not match the result of [`Spi::get_dyn_length`], it will panic.
///
/// Because [`Write`] is only implemented when the [`Pads`] are [`NotRx`], this
/// implementation never reads the DATA register and ignores all buffer overflow
/// errors.
///
/// [`Write`]: blocking::spi::Write
impl<P, M> blocking::spi::Write<u8> for Spi<Config<P, M, DynLength>>
where
    P: Tx,
    M: OpMode,
    Config<P, M, DynLength>: ValidConfig<Length = DynLength>,
{
    type Error = Error;

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        assert_eq!(buf.len(), self.get_dyn_length() as usize);
        let sercom = unsafe { self.sercom() };
        write_slice(sercom, buf)
    }
}

/// Write a slice over SPI
///
/// This function exists to avoid monomorphization code bloat
fn write_slice(sercom: &RegisterBlock, buf: &[u8]) -> Result<(), Error> {
    let mut data = buf.iter();
    while data.len() > 0 {
        let errors = sercom.spim().status.read();
        if errors.lenerr().bit_is_set() {
            return Err(Error::LengthError);
        }
        if sercom.spim().intflag.read().dre().bit_is_set() {
            let mut bytes = [0; 4];
            for byte in bytes.iter_mut() {
                match data.next() {
                    Some(d) => *byte = *d,
                    None => break,
                }
            }
            let word = u32::from_le_bytes(bytes);
            sercom.spim().data.write(|w| unsafe { w.data().bits(word) });
        }
    }
    Ok(())
}
