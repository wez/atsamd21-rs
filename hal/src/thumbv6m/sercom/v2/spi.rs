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
//! A [`Sercom`] can use up to four [`Pin`]s as configured as SERCOM pads, but
//! only certain `Pin` combinations are acceptable. In particular, all `Pin`s
//! must be mapped to the same `Sercom` (see the datasheet). This HAL makes it
//! impossible to use invalid `Pin` combinations, and the [`Pads`] struct is
//! responsible for enforcing these constraints.
//!
//! A `Pads` type takes five type parameters. The first specifies the `Sercom`,
//! while the remaining four, `DI`, `DO`, `CK` and `SS`, represent the Data In,
//! Data Out, Sclk and SS pads respectively. Each of the remaining type
//! parameters is an [`OptionalPad`] and defaults to [`NoneT`]. Aliases defining
//! the pad types can be provided by the [`bsp_pins!`](crate::bsp_pins) macro.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09, AlternateC, AlternateD};
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::typelevel::NoneT;
//!
//! type Miso = Pin<PA08, AlternateC>;
//! type Sclk = Pin<PA09, AlternateC>;
//! type Pads = spi::Pads<Sercom0, Miso, NoneT, Sclk>;
//! ```
#![cfg_attr(
    feature = "samd21",
    doc = "
Alternatively, you can use the [`PadsFromIds`] alias to define a set of
`Pads` in terms of [`PinId`]s instead of `Pin`s. This is useful when you
don't have `Pin` aliases pre-defined.

```
use atsamd_hal::gpio::v2::{PA08, PA09};
use atsamd_hal::sercom::v2::{Sercom0, spi};
use atsamd_hal::typelevel::NoneT;

type Pads = spi::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
```
"
)]
//! Instaces of `Pads` are created using the builder pattern. Start by creating
//! an empty set of `Pads` using [`Default`]. Then pass each respective `Pin`
//! using the corresponding methods. Both `v1::Pin` and `v2::Pin` types are
//! accepted here.
//!
//! On SAMD21 chips, the builder methods automatically convert each
//! pin to the correct [`PinMode`]. But for SAMD11 chips, users must manually
//! convert each pin before calling the builder methods. This is a consequence
//! of inherent ambiguities in the SAMD11 SERCOM pad definitions. Specifically,
//! the same `PinId` can correspond to two different `PadNum`s for the *same*
//! `Sercom`.
//!
//! ```
//! use atsamd_hal::target_device::Peripherals;
//! use atsamd_hal::gpio::v2::Pins;
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! let pads = spi::Pads::<Sercom0>::default()
//!     .sclk(pins.pa09)
//!     .data_in(pins.pa08)
//!     .data_out(pins.pa11);
//! ```
//!
//! To be accepted as [`ValidPads`], a set of `Pads` must do two things:
//! - Specify a type for `CK` and at least one of `DI` or `DO`
//! - Satisfy the [`DipoDopo`] trait
//!
//! # [`Config`]
//!
//! Next, create a [`Config`] struct, which represents the SPI peripheral in its
//! disabled state. A `Config` is specified with three type parameters: the
//! [`Pads`] type; an [`OpMode`], which defaults to [`Master`]; and a
//! [`CharSize`], which defaults to [`EightBit`].
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::sercom::v2::spi::{Master, NineBit};
//! use atsamd_hal::typelevel::NoneT;
//!
//! type Pads = spi::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
//! type Config = spi::Config<Pads, Master, NineBit>;
//! ```
//!
//! Upon creation, the [`Config`] takes ownership of both the [`Pads`] struct
//! and the PAC [`Sercom`] struct. It takes a reference to the PM, so that it
//! can enable the APB clock, and it takes a frequency to indicate the GCLK
//! configuration. Users are responsible for correctly configuring the GCLK.
//!
//! ```
//! use atsamd_hal::time::U32Ext;
//!
//! let pm = peripherals.PM;
//! let sercom = peripherals.SERCOM0;
//! // Configure GCLK for 10 MHz
//! let freq = 10.mhz();
//! let config = spi::Config::new(&pm, sercom, pads, freq);
//! ```
//!
//! The [`Config`] struct uses the builder pattern to configure the peripheral,
//! ending with a call to [`enable`], which consumes the [`Config`] and returns
//! an enabled [`Spi`] peripheral.
//!
//! ```
//! use embedded_hal::spi::MODE_1;
//! use atsamd_hal::sercom::v2::spi::NineBit;
//!
//! let spi = spi::Config::new(&mclk, sercom, pads, freq)
//!     .baud(1.mhz())
//!     .char_size::<NineBit>()
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
//! An [`Spi`] struct can only be created from a [`ValidConfig`], and it has
//! only one type parameter, the corresponding `Config`uration.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::sercom::v2::spi::{Master, NineBit};
//! use atsamd_hal::typelevel::NoneT;
//!
//! // Assuming SAMD21
//! type Pads = spi::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
//! type Config = spi::Config<Pads, Master, NineBit>;
//! type Spi = spi::Spi<Config>;
//! ```
//!
//! Only the [`Spi`] struct can actually perform transactions. To do so, use the
//! embedded HAL traits, like [`spi::FullDuplex`](FullDuplex),
//! [`serial::Read`](Read) and [`serial::Write`](Write). See the [`Spi`]
//! documentation for more information about the trait implementations, which
//! vary based on the [`CharSize`] and [`Pads`]. For instance, [`FullDuplex`] is
//! only implemented if the [`Pads`] are both [`Tx`] and [`Rx`], and its word
//! size varies between `u8` and `u16`, depending on [`CharSize`].
//!
//! ```
//! use nb::block;
//! use embedded_hal::spi::FullDuplex;
//!
//! block!(spi.send(0x0155));
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

use crate::target_device as pac;
use pac::sercom0::spi::ctrla::MODE_A;
use pac::sercom0::RegisterBlock;
use pac::PM;

#[cfg(not(feature = "samd11"))]
use crate::gpio::v2::AnyPin;
use crate::sercom::v2::*;
use crate::time::Hertz;
use crate::typelevel::{Is, NoneT, Sealed};

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
    /// `DIPO` field value
    const DIPO: u8;

    /// `DIPO` field value
    const DOPO: u8;

    /// Configure the pad according to [`Self::DIPO`] and [`Self::DOPO`]
    #[inline]
    fn configure(sercom: &RegisterBlock) {
        sercom.spi().ctrla.modify(|_, w| unsafe {
            w.dipo().bits(Self::DIPO);
            w.dopo().bits(Self::DOPO)
        });
    }
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
    const DIPO: u8 = <(DI::PadNum, DO::PadNum, CK::PadNum, SS::PadNum)>::DIPO;
    const DOPO: u8 = <(DI::PadNum, DO::PadNum, CK::PadNum, SS::PadNum)>::DOPO;
}

//=============================================================================
// Implement DipoDopo
//=============================================================================

/// Filter [`PadNum`] permutations and implement [`DipoDopo`]
#[rustfmt::skip]
macro_rules! impl_dipodopo {
    // This is the entry pattern. Start by checking CK and SS.
    ($DI:ident, $DO:ident, $CK:ident, $SS:ident) => { impl_dipodopo!(@check_ck_ss, $DI, $DO, $CK, $SS); };

    // Check whether CK and SS form a valid pair.
    // CK must be present, while SS must be the correct pad or absent.
    (@check_ck_ss, $DI:ident, $DO:ident, Pad1, NoneT) => { impl_dipodopo!(@assign_dipo, $DI, $DO, Pad1, NoneT); };
    (@check_ck_ss, $DI:ident, $DO:ident, Pad1, Pad2)  => { impl_dipodopo!(@assign_dipo, $DI, $DO, Pad1, Pad2); };
    (@check_ck_ss, $DI:ident, $DO:ident, Pad3, NoneT) => { impl_dipodopo!(@assign_dipo, $DI, $DO, Pad3, NoneT); };
    (@check_ck_ss, $DI:ident, $DO:ident, Pad3, Pad1)  => { impl_dipodopo!(@assign_dipo, $DI, $DO, Pad3, Pad1); };

    // If CK and SS are not valid, fall through to this pattern.
    (@check_ck_ss, $DI:ident, $DO:ident, $CK:ident, $SS:ident) => { };

    // Assign DIPO based on DI.
    // Our options are exhaustive, so no fall through pattern is needed.
    (@assign_dipo, NoneT, $DO:ident, $CK:ident, $SS:ident) => { impl_dipodopo!(@assign_dopo, NoneT, $DO, $CK, $SS, 0); };
    (@assign_dipo, Pad0,  $DO:ident, $CK:ident, $SS:ident) => { impl_dipodopo!(@assign_dopo, Pad0,  $DO, $CK, $SS, 0); };
    (@assign_dipo, Pad1,  $DO:ident, $CK:ident, $SS:ident) => { impl_dipodopo!(@assign_dopo, Pad1,  $DO, $CK, $SS, 1); };
    (@assign_dipo, Pad2,  $DO:ident, $CK:ident, $SS:ident) => { impl_dipodopo!(@assign_dopo, Pad2,  $DO, $CK, $SS, 2); };
    (@assign_dipo, Pad3,  $DO:ident, $CK:ident, $SS:ident) => { impl_dipodopo!(@assign_dopo, Pad3,  $DO, $CK, $SS, 3); };

    // Assign DOPO based on DO and CK.
    (@assign_dopo, $DI:ident, NoneT, Pad1, $SS:ident, $DIPO:literal) => { impl_dipodopo!(@filter_conflicts, $DI, NoneT, Pad1, $SS, $DIPO, 0); };
    (@assign_dopo, $DI:ident, NoneT, Pad3, $SS:ident, $DIPO:literal) => { impl_dipodopo!(@filter_conflicts, $DI, NoneT, Pad3, $SS, $DIPO, 1); };
    (@assign_dopo, $DI:ident, Pad0,  Pad1, $SS:ident, $DIPO:literal) => { impl_dipodopo!(@filter_conflicts, $DI, Pad0,  Pad1, $SS, $DIPO, 0); };
    (@assign_dopo, $DI:ident, Pad2,  Pad3, $SS:ident, $DIPO:literal) => { impl_dipodopo!(@filter_conflicts, $DI, Pad2,  Pad3, $SS, $DIPO, 1); };
    (@assign_dopo, $DI:ident, Pad3,  Pad1, $SS:ident, $DIPO:literal) => { impl_dipodopo!(@filter_conflicts, $DI, Pad3,  Pad1, $SS, $DIPO, 2); };
    (@assign_dopo, $DI:ident, Pad0,  Pad3, $SS:ident, $DIPO:literal) => { impl_dipodopo!(@filter_conflicts, $DI, Pad0,  Pad3, $SS, $DIPO, 3); };

    // If DO is not valid, fall through to this pattern.
    (@assign_dopo, $DI:ident, $DO:ident, $CK:ident, $SS:ident, $DIPO:literal) => { };

    // Filter any remaining permutations that conflict.
    (@filter_conflicts, NoneT, NoneT, $CK:ident, $SS:ident, $DIPO:literal, $DOPO:literal) => { };
    (@filter_conflicts, Pad0, Pad0, $CK:ident, $SS:ident, $DIPO:literal, $DOPO:literal) => { };
    (@filter_conflicts, Pad1, $DO:ident, Pad1, $SS:ident, $DIPO:literal, $DOPO:literal) => { };
    (@filter_conflicts, Pad1, $DO:ident, $CK:ident, Pad1, $DIPO:literal, $DOPO:literal) => { };
    (@filter_conflicts, Pad2, Pad2, $CK:ident, $SS:ident, $DIPO:literal, $DOPO:literal) => { };
    (@filter_conflicts, Pad2, $DO:ident, $CK:ident, Pad2, $DIPO:literal, $DOPO:literal) => { };
    (@filter_conflicts, Pad3, Pad3, $CK:ident, $SS:ident, $DIPO:literal, $DOPO:literal) => { };
    (@filter_conflicts, Pad3, $DO:ident, Pad3, $SS:ident, $DIPO:literal, $DOPO:literal) => { };
    (@filter_conflicts, $DI:ident, Pad2, $CK:ident, Pad2, $DIPO:literal, $DOPO:literal) => { };

    // If there are no conflicts, fall through to this pattern and implement DipoDopo
    (@filter_conflicts, $DI:ident, $DO:ident, $CK:ident, $SS:ident, $DIPO:literal, $DOPO:literal) => {
        impl DipoDopo for ($DI, $DO, $CK, $SS) {
            const DIPO: u8 = $DIPO;
            const DOPO: u8 = $DOPO;
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
/// See the [module-level](self) documentation for more details on specifying
/// a `Pads` type and creating instances.
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
/// [`Pin`]: crate::gpio::v2::Pin
/// [type-level function]: crate::typelevel#type-level-functions
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
/// This trait labels sets of [`Pads`] that satisfy the [`DipoDopo`] trait. It
/// guarantees to the [`Config`] struct that this set of `Pads` can be
/// configured through that trait.
pub trait ValidPads: PadSet + DipoDopo {}

impl<P: PadSet + DipoDopo> ValidPads for P {}

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

impl<S, DI, CK, SS> TxOrRx for Pads<S, DI, NoneT, CK, SS>
where
    S: Sercom,
    DI: SomePad,
    CK: SomePad,
    SS: OptionalPad,
    Self: DipoDopo,
{
}

impl<S, DO, CK, SS> TxOrRx for Pads<S, NoneT, DO, CK, SS>
where
    S: Sercom,
    DO: SomePad,
    CK: SomePad,
    SS: OptionalPad,
    Self: DipoDopo,
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
    #[inline]
    fn configure(sercom: &RegisterBlock) -> () {
        sercom
            .spi()
            .ctrla
            .modify(|_, w| w.mode().variant(Self::MODE));
        sercom.spi().ctrlb.modify(|_, w| w.mssen().bit(Self::MSSEN));
        while sercom.spi().syncbusy.read().ctrlb().bit_is_set() {}
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
// Character size
//=============================================================================

/// Type-level enum representing the SPI character size
///
/// This trait acts as both a [type-level enum], forming a type class for
/// character sizes, as well as a [type-level function] mapping the
/// corresponding word size.
///
/// The SPI character size affects the word size for the embedded HAL traits.
/// Eight-bit transactions use a `u8` word, while nine-bit transactions use a
/// `u16` word.
///
/// [type-level enum]: crate::typelevel#type-level-enums
/// [type-level function]: crate::typelevel#type-level-functions
pub trait CharSize: Sealed {
    /// Word size for the character size
    type Word: 'static;

    /// Register it pattern for the corresponding `CharSize`
    const BITS: u8;

    /// Configure the `LENGTH` register and enable the `LENGTH` counter
    #[inline]
    fn configure(sercom: &RegisterBlock) -> () {
        sercom
            .spi()
            .ctrlb
            .modify(|_, w| unsafe { w.chsize().bits(Self::BITS) });
    }
}

/// Type alias to recover the [`Word`](CharSize::Word) type from an
/// implementation of [`CharSize`]
pub type Word<C> = <C as CharSize>::Word;

/// [`CharSize`] variant for 8-bit transactions
pub enum EightBit {}

/// [`CharSize`] variant for 9-bit transactions
pub enum NineBit {}

impl Sealed for EightBit {}
impl CharSize for EightBit {
    type Word = u8;
    const BITS: u8 = 0;
}

impl Sealed for NineBit {}
impl CharSize for NineBit {
    type Word = u16;
    const BITS: u8 = 1;
}

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
    /// `BUFOVF` is the only available error flag. The binary format of the
    /// underlying bits exactly matches the STATUS register.
    pub struct Errors: u16 {
        const BUFOVF = 0x0004;
    }
}

/// Error `enum` for SPI transactions
///
/// The SPI peripheral only has one error type: buffer overflow.
#[derive(Debug)]
pub enum Error {
    Overflow,
}

impl TryFrom<Errors> for () {
    type Error = Error;
    fn try_from(errors: Errors) -> Result<(), Error> {
        if errors.contains(Errors::BUFOVF) {
            Err(Error::Overflow)
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
/// state. It is generic over the set of [`Pads`], [`OpMode`] and
/// [`CharSize`]. Upon creation, the [`Config`] takes ownership of the
/// [`Sercom`] and resets it, returning it configured as an SPI peripheral in
/// [`Master`] [`OpMode`] with an [`EightBit`] [`CharSize`].
///
/// [`Config`] uses a builder-pattern API to configure the peripheral,
/// culminating in a call to [`enable`], which consumes the [`Config`] and
/// returns an enabled [`Spi`] peripheral. The [`enable`] function is restricted
/// to [`ValidConfig`]s.
///
/// [`enable`]: Config::enable
pub struct Config<P, M = Master, C = EightBit>
where
    P: ValidPads,
    M: OpMode,
    C: CharSize,
{
    sercom: P::Sercom,
    pad_map: P,
    mode: PhantomData<M>,
    chsize: PhantomData<C>,
    freq: Hertz,
}

impl<P: ValidPads> Config<P> {
    /// Create a new [`Config`] in the default configuration
    fn create(sercom: P::Sercom, pad_map: P, freq: impl Into<Hertz>) -> Self {
        Self::swrst(&sercom);
        Master::configure(&sercom);
        P::configure(&sercom);
        EightBit::configure(&sercom);
        Self {
            sercom,
            pad_map,
            mode: PhantomData,
            chsize: PhantomData,
            freq: freq.into(),
        }
    }

    /// Create a new [`Config`] in the default configuration
    ///
    /// This function will enable the corresponding APB clock, reset the
    /// [`Sercom`] peripheral, and return a [`Config`] in the default
    /// configuration, [`Master`] [`OpMode`] with an [`EightBit`] [`CharSize`].
    /// [`Config`] takes ownership of the [`Sercom`] and [`Pads`].
    ///
    /// Users must configure GCLK manually. The `freq` parameter represents the
    /// GCLK frequency for this [`Sercom`] instance.
    #[inline]
    pub fn new(pm: &PM, mut sercom: P::Sercom, pad_map: P, freq: impl Into<Hertz>) -> Self {
        sercom.enable_apb_clock(pm);
        Self::create(sercom, pad_map, freq)
    }
}

impl<P, M, C> Config<P, M, C>
where
    P: ValidPads,
    M: OpMode,
    C: CharSize,
{
    /// Reset the SERCOM peripheral
    #[inline]
    fn swrst(sercom: &P::Sercom) {
        sercom.spi().ctrla.write(|w| w.swrst().set_bit());
        while sercom.spi().syncbusy.read().swrst().bit_is_set() {}
        unsafe { Self::reset_serial_read_state() };
    }

    /// Reset the `static` state used by the implementation of [`Read`]
    ///
    /// The state should be reset whenever the peripheral is reset or disabled.
    /// It should not be possible for the state to become out of sync from safe
    /// code.
    unsafe fn reset_serial_read_state() {
        SERIAL_READ_STATE[P::Sercom::NUM] = false;
    }

    /// Change the [`Config`] [`OpMode`] or [`CharSize`]
    #[inline]
    fn change<M2, C2>(self) -> Config<P, M2, C2>
    where
        M2: OpMode,
        C2: CharSize,
    {
        Config {
            sercom: self.sercom,
            pad_map: self.pad_map,
            mode: PhantomData,
            chsize: PhantomData,
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
    pub fn op_mode<M2: OpMode>(self) -> Config<P, M2, C> {
        M2::configure(&self.sercom);
        self.change()
    }

    /// Change the [`CharSize`]
    #[inline]
    pub fn char_size<C2: CharSize>(self) -> Config<P, M, C2> {
        C2::configure(&self.sercom);
        self.change()
    }

    /// Change the clock polarity
    #[inline]
    pub fn cpol(self, cpol: Polarity) -> Self {
        let cpol = match cpol {
            Polarity::IdleLow => false,
            Polarity::IdleHigh => true,
        };
        self.sercom.spi().ctrla.modify(|_, w| w.cpol().bit(cpol));
        self
    }

    /// Change the clock phase
    #[inline]
    pub fn cpha(self, cpha: Phase) -> Self {
        let cpha = match cpha {
            Phase::CaptureOnFirstTransition => false,
            Phase::CaptureOnSecondTransition => true,
        };
        self.sercom.spi().ctrla.modify(|_, w| w.cpha().bit(cpha));
        self
    }

    /// Change the SPI mode (clock polarity & phase)
    #[inline]
    pub fn spi_mode(self, mode: spi::Mode) -> Self {
        let cpol = match mode.polarity {
            Polarity::IdleLow => false,
            Polarity::IdleHigh => true,
        };
        let cpha = match mode.phase {
            Phase::CaptureOnFirstTransition => false,
            Phase::CaptureOnSecondTransition => true,
        };
        self.sercom.spi().ctrla.modify(|_, w| {
            w.cpol().bit(cpol);
            w.cpha().bit(cpha)
        });
        self
    }

    /// Change the bit order of transmission (MSB/LSB first)
    #[inline]
    pub fn msb_first(self, msb_first: bool) -> Self {
        self.sercom
            .spi()
            .ctrla
            .modify(|_, w| w.dord().bit(!msb_first));
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
            .spi()
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
        self.sercom.spi().ctrla.modify(|_, w| w.ibon().bit(set));
    }

    /// Run in standby mode
    ///
    /// When set, the SPI peripheral will run in standby mode. See the datasheet
    /// for more details.
    #[inline]
    pub fn run_in_standby(&mut self, set: bool) {
        self.sercom.spi().ctrla.modify(|_, w| w.runstdby().bit(set));
    }

    /// Enable interrupts for the specified flags
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.sercom
            .spi()
            .intenset
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Disable interrupts for the specified flags
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.sercom
            .spi()
            .intenclr
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Enable the SPI peripheral
    ///
    /// SPI transactions are not possible until the peripheral is enabled.
    /// This function is limited to [`ValidConfig`]s.
    #[inline]
    pub fn enable(self) -> Spi<Self>
    where
        Self: ValidConfig,
    {
        self.sercom.spi().ctrlb.modify(|_, w| w.rxen().set_bit());
        while self.sercom.spi().syncbusy.read().ctrlb().bit_is_set() {}
        self.sercom.spi().ctrla.modify(|_, w| w.enable().set_bit());
        while self.sercom.spi().syncbusy.read().enable().bit_is_set() {}
        Spi { config: self }
    }

    /// Enable or disable the SERCOM peripheral, and wait for the ENABLE bit to
    /// synchronize.
    fn enable_peripheral(&mut self, enable: bool) {
        self.sercom
            .spi()
            .ctrla
            .modify(|_, w| w.enable().bit(enable));
        while self.sercom.spi().syncbusy.read().enable().bit_is_set() {}
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
    type CharSize: CharSize<Word = Self::Word>;
    type Word: 'static;
}

/// Type alias to recover the specific [`Config`] type from an implementation of
/// [`AnyConfig`]
pub type SpecificConfig<C> =
    Config<<C as AnyConfig>::Pads, <C as AnyConfig>::OpMode, <C as AnyConfig>::CharSize>;

impl<P, M, C> AsRef<Self> for Config<P, M, C>
where
    P: ValidPads,
    M: OpMode,
    C: CharSize,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<P, M, C> AsMut<Self> for Config<P, M, C>
where
    P: ValidPads,
    M: OpMode,
    C: CharSize,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<P, M, C> Sealed for Config<P, M, C>
where
    P: ValidPads,
    M: OpMode,
    C: CharSize,
{
}

impl<P, M, C> AnyConfig for Config<P, M, C>
where
    P: ValidPads,
    M: OpMode,
    C: CharSize,
{
    type Sercom = P::Sercom;
    type Pads = P;
    type OpMode = M;
    type CharSize = C;
    type Word = C::Word;
}

//=============================================================================
// ValidConfig
//=============================================================================

/// Marker trait for valid SPI [`Config`]urations
///
/// A functional SPI peripheral must have, at a minimum, an SCK pad and
/// either a Data In or a Data Out pad. Dependeing on the
/// [`OpMode`], an SS pad may also be required.
///
/// The [`ValidConfig`] trait is implemented only for valid combinations of
/// [`Pads`] and [`OpMode`]. No [`Config`] is valid if the SCK pad is
/// [`NoneT`] or if both the Data In and Data Out pads are [`NoneT`]. And when
/// [`OpMode`] is [`MasterHWSS`] or [`Slave`], the SS pad must not be [`NoneT`]
/// either.
pub trait ValidConfig: AnyConfig {}

impl<P, C> ValidConfig for Config<P, Master, C>
where
    P: ValidPads<SS = NoneT> + TxOrRx,
    C: CharSize,
{
}

impl<P, C> ValidConfig for Config<P, MasterHWSS, C>
where
    P: ValidPads + TxOrRx,
    C: CharSize,
    P::SS: SomePad,
{
}

impl<P, C> ValidConfig for Config<P, Slave, C>
where
    P: ValidPads + TxOrRx,
    C: CharSize,
    P::SS: SomePad,
{
}

//=============================================================================
// Spi
//=============================================================================

/// An enabled SPI peripheral that can perform transactions using the embedded
/// HAL traits
///
/// When an [`Spi`] is [`Tx`]` + `[`Rx`], it implements [`FullDuplex`], with a
/// word type that depends on [`CharSize`]. The word type is [`u8`] for an
/// [`EightBit`] [`CharSize`] and [`u16`] for a [`NineBit`] [`CharSize`].
///
/// For half-duplex transactions, [`Spi`] implements the `serial` [`Read`] and
/// [`Write`] traits. They are only implemented when the [`Pads`] are not
/// [`Tx`]` + `[`Rx`], so they don't compete with [`FullDuplex`].
///
/// [`Spi`] uses the default implementations for the [`blocking::spi`] and
/// [`blocking::serial`] traits.
///
/// For a non-blocking alternative that can be used to transfer slices, see the
/// [`SpiFuture`] type.
///
/// [`SpiFuture`]: crate::sercom::v2::spi_future::SpiFuture
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
        &self.config.as_ref().sercom()
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
        let bits = unsafe { self.sercom().spi().intflag.read().bits() };
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
        unsafe { self.sercom().spi().intflag.write(|w| w.bits(flags.bits())) };
    }

    /// Read the error status flags
    #[inline]
    pub fn read_errors(&self) -> Errors {
        let bits = unsafe { self.sercom().spi().status.read().bits() };
        Errors::from_bits_truncate(bits)
    }

    /// Clear error status flags
    ///
    /// Setting a flag will clear the error. Clearing any flag will have no
    /// effect.
    #[inline]
    pub fn clear_errors(&mut self, errors: Errors) {
        unsafe { self.sercom().spi().status.write(|w| w.bits(errors.bits())) };
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
    pub unsafe fn read_data(&mut self) -> u16 {
        self.sercom().spi().data.read().data().bits()
    }

    /// Write to the DATA register
    ///
    /// Writing to the data register directly is `unsafe`, because it will clear
    /// the DRE flag, which could break assumptions made elsewhere in this
    /// module.
    #[inline]
    pub unsafe fn write_data(&mut self, data: u16) {
        self.sercom().spi().data.write(|w| w.data().bits(data))
    }

    /// Disable the SPI peripheral and return the [`Config`] struct
    #[inline]
    pub fn disable(mut self) -> C {
        // SAFETY: The read state must be reset when disabling the peripheral
        unsafe { Config::<C::Pads>::reset_serial_read_state() };
        let spim = unsafe { self.sercom().spi() };
        spim.ctrlb.modify(|_, w| w.rxen().clear_bit());
        while spim.syncbusy.read().ctrlb().bit_is_set() {}
        self.config.as_mut().enable_peripheral(false);
        self.config
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
    pub unsafe fn reset_serial_read_state(&mut self) {
        Config::<C::Pads>::reset_serial_read_state();
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
/// copies the [`Sercom`], [`Pads`], [`OpMode`], [`CharSize`] and [`Word`]
/// types, to make it easier to apply bounds to these types at the next level of
/// abstraction.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait AnySpi: Is<Type = SpecificSpi<Self>> {
    type Sercom: Sercom;
    type Pads: ValidPads<Sercom = Self::Sercom>;
    type OpMode: OpMode;
    type CharSize: CharSize<Word = Self::Word>;
    type Word: 'static;
    type Config: ValidConfig<
        Sercom = Self::Sercom,
        Pads = Self::Pads,
        OpMode = Self::OpMode,
        CharSize = Self::CharSize,
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
    type CharSize = C::CharSize;
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
impl<P, M, C> Read<C::Word> for Spi<Config<P, M, C>>
where
    Config<P, M, C>: ValidConfig,
    P: Rx + NotTx,
    M: MasterMode,
    C: CharSize,
    C::Word: PrimInt,
    u16: AsPrimitive<C::Word>,
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
    fn read(&mut self) -> nb::Result<C::Word, Error> {
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
impl<P, C> Read<C::Word> for Spi<Config<P, Slave, C>>
where
    Config<P, Slave, C>: ValidConfig,
    P: Rx + NotTx,
    C: CharSize,
    C::Word: PrimInt,
    u16: AsPrimitive<C::Word>,
{
    type Error = Error;

    /// Wait for an `RXC` flag, then read the word
    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Error> {
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
    C::Word: PrimInt + AsPrimitive<u16>,
{
    type Error = Error;

    /// Wait for a `DRE` flag, then write a word
    #[inline]
    fn write(&mut self, word: C::Word) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_flags().contains(Flags::DRE) {
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
        if self.read_flags().contains(Flags::TXC) {
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

/// Perform a non-blocking, [`FullDuplex`] trasaction
///
/// [`FullDuplex`] is only implemented when [`Pads`] is both [`Tx`] and [`Rx`]
/// and the [`OpMode`] is a [`MasterMode`]. The word type is dependent on
/// [`CharSize`].
impl<C> FullDuplex<C::Word> for Spi<C>
where
    C: ValidConfig,
    C::Pads: Tx + Rx,
    C::OpMode: MasterMode,
    C::Word: PrimInt + AsPrimitive<u16>,
    u16: AsPrimitive<C::Word>,
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

impl<C> blocking::spi::transfer::Default<C::Word> for Spi<C>
where
    C: ValidConfig,
    Self: FullDuplex<C::Word>,
{
}

impl<C> blocking::spi::write::Default<C::Word> for Spi<C>
where
    C: ValidConfig,
    Self: FullDuplex<C::Word>,
{
}

#[cfg(feature = "unproven")]
impl<C> blocking::spi::write_iter::Default<C::Word> for Spi<C>
where
    C: ValidConfig,
    Self: FullDuplex<C::Word>,
{
}
