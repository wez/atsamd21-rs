//! Use a SERCOM peripheral for SPI transactions
//!
//! Using an SPI peripheral occurs in three steps. First, you must supply
//! [`gpio`] [`Pin`]s to create a set of [`Pads`]. Next, you combine the
//! `Pads` with other pieces to form a [`Config`] struct. Finally, after
//! configuring the peripheral, you [`enable`] it to yield a functional
//! [`Spi`] struct. Transactions are performed using traits from the
//! [`embedded_hal`] crate, specifically those from the
//! [`spi`](embedded_hal::spi), [`serial`](embedded_hal::serial), and
//! [`blocking`](embedded_hal::blocking) modules.
//!
//! # Crating a set of [`Pads`]
//!
//! An SPI peripheral can use up to four [`Pin`]s as [`Sercom`] pads. However,
//! only certain `Pin` combinations are acceptable. All `Pin`s must be mapped to
//! the same `Sercom`, and for SAMx5x chips, they must also belong to the same
#![cfg_attr(any(feature = "samd11", feature = "samd21"), doc = "`IoSet`.")]
#![cfg_attr(feature = "min-samd51g", doc = "[`IoSet`].")]
//! This HAL makes it impossible to use invalid `Pin` combinations, and the
//! [`Pads`] struct is responsible for enforcing these constraints.
//!
//! A `Pads` type takes five or six type parameters, depending on the chip. The
//! first type always specifies the `Sercom`. On SAMx5x chips, the second type
//! specifies the `IoSet`. The remaining four type parameters, `DI`, `DO`, `CK`
//! and `SS`, represent the Data In, Data Out, Sclk and SS pads respectively.
//! Each of these type parameters is an [`OptionalPad`] and defaults to
//! [`NoneT`]. A `Pad` is just a `Pin` configured in the correct [`PinMode`]
//! that implements [`IsPad`]. The [`bsp_pins!`](crate::bsp_pins) macro can be
//! used to define convenient type aliases for `Pad` types.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09, AlternateC};
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::typelevel::NoneT;
//!
//! // SAMx5x-specific imports
//! use atsamd_hal::sercom::v2::pad::IoSet1;
//!
//! type Miso = Pin<PA08, AlternateC>;
//! type Sclk = Pin<PA09, AlternateC>;
//!
//! // SAMD11/SAMD21 version
//! type Pads = spi::Pads<Sercom0, Miso, NoneT, Sclk>;
//! // SAMx5x version
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
//! use atsamd_hal::typelevel::NoneT;
//!
//! // SAMx5x-specific imports
//! use atsamd_hal::sercom::v2::pad::IoSet1;
//!
//! // SAMD11/SAMD21 version
//! type Pads = spi::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
//! // SAMx5x version
//! type Pads = spi::PadsFromIds<Sercom0, IoSet1, PA08, NoneT, PA09>;
//! ```
//!
//! Instances of `Pads` are created using the builder pattern. Start by creating
//! an empty set of `Pads` using [`Default`]. Then pass each respective `Pin`
//! using the corresponding methods. For SAMD21 and SAMx5x chips, the builder
//! methods automatically convert each pin to the correct [`PinMode`]. However,
//! due to inherent ambiguities, users must manually configure `PinMode`s for
//! SAMD11 chips.
//!
//! ```
//! use atsamd_hal::target_device::Peripherals;
//! use atsamd_hal::gpio::v2::Pins;
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//!
//! // SAMx5x-specific imports
//! use atsamd_hal::sercom::v2::pad::IoSet1;
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! // SAMD21 version
//! let pads = spi::Pads::<Sercom0>::default()
//!     .sclk(pins.pa09)
//!     .data_in(pins.pa08)
//!     .data_out(pins.pa11);
//! // SAMx5x version
//! let pads = spi::Pads::<Sercom0, IoSet1>::default()
//!     .sclk(pins.pa09)
//!     .data_in(pins.pa08)
//!     .data_out(pins.pa11);
//! ```
//!
//! To be accepted by the [`Config`] struct as a set of [`ValidPads`], the
//! `Pads` must do two things:
//! - Specify [`SomePad`] for `CK` and at least one of `DI` or `DO`
//! - Use a valid combination of [`PadNum`]s, so that the `Pads` implement
//!   [`DipoDopo`]
//!
//! # `Config`uring the peripheral
//!
//! Next, create a [`Config`] struct, which represents the SPI peripheral in its
//! disabled state. A `Config` is specified with three type parameters: the
//! [`Pads`] type; an [`OpMode`], which defaults to [`Master`]; and a
//! [`Size`] type that varies by chip. [`Size`] essentially acts as a trait
//! alias. On SAMD11 and SAMD21 chips, it represents the
#![cfg_attr(
    any(feature = "samd11", feature = "samd21"),
    doc = "[`CharSize`], which can either be [`EightBit`] or [`NineBit`]. "
)]
#![cfg_attr(
    feature = "min-samd51g",
    doc = "`CharSize`, which can either be `EightBit` or `NineBit`. "
)]
//! While on SAMx5x chips, it represents the transaction
#![cfg_attr(any(feature = "samd11", feature = "samd21"), doc = "`Length`")]
#![cfg_attr(feature = "min-samd51g", doc = "[`Length`]")]
//! in bytes, using type-level numbers provided by the [`typenum`] crate. Valid
//! transaction lengths, from `U1` to `U255`, are re-exported in the
#![cfg_attr(any(feature = "samd11", feature = "samd21"), doc = "`lengths`")]
#![cfg_attr(feature = "min-samd51g", doc = "[`lengths`]")]
//! sub-module.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::sercom::v2::spi::Master;
//! use atsamd_hal::typelevel::NoneT;
//!
//! // SAMD11/SAMD21-specific imports
//! use atsamd_hal::sercom::v2::spi::NineBit;
//!
//! // SAMx5x-specific imports
//! use atsamd_hal::sercom::v2::spi::lengths::U2;
//! use atsamd_hal::sercom::v2::pad::IoSet1;
//!
//! // SAMD11/SAMD21 version
//! type Pads = spi::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
//! type Config = spi::Config<Pads, Master, NineBit>;
//!
//! // SAMx5x version
//! type Pads = spi::PadsFromIds<Sercom0, IoSet1, PA08, NoneT, PA09>;
//! type Config = spi::Config<Pads, Master, U2>;
//! ```
//!
//! For simplicity, this module ignores character size on SAMx5x chips. Instead,
//! the SPI peripheral is always configured to use 32-bit extension mode and the
//! hardware `LENGTH` counter. Note that, due to a hardware bug, `ICSPACE` must
//! be at least one when using the length counter. See the silicon errata for
//! more details.
//!
//! Upon creation, the [`Config`] takes ownership of both the [`Pads`] and the
//! PAC [`Sercom`] struct. It takes a reference to the `PM` or `MCLK`, so that
//! it can enable the APB clock, and it takes a frequency to indicate the GCLK
//! configuration. Users are responsible for correctly configuring the GCLK.
//!
//! ```
//! use atsamd_hal::time::U32Ext;
//!
//! // Not shown: configure GCLK for 10 MHz
//!
//! // SAMD11/SAMD21 version
//! let pm = peripherals.PM;
//! let sercom = peripherals.SERCOM0;
//! let freq = 10.mhz();
//! let config = spi::Config::new(&pm, sercom, pads, freq);
//!
//! // SAMx5x version
//! let mclk = peripherals.MCLK;
//! let sercom = peripherals.SERCOM0;
//! let freq = 10.mhz();
//! let config = spi::Config::new(&mclk, sercom, pads, freq);
//! ```
//!
//! The [`Config`] uses two different APIs for configuration. For most
//! parameters, it provides `get_` and `set_` methods that take `&self` and
//! `&mut self` respectively, e.g. [`get_bit_order`](Config::get_bit_order) and
//! [`set_bit_order`](Config::set_bit_order). However, because `Config` tracks
//! the [`OpMode`] and [`Size`] at compile-time, which requires changing the
//! corresponding type parameters, `Config` also provides a builder-pattern API,
//! where methods take and return `self`, e.g. [`bit_order`](Config::bit_order).
//!
//! Once configured, the [`enable`] method consumes the `Config` and returns an
//! enabled [`Spi`] struct that can be used for transactions. Because the
//! `enable` function takes the `Config` as `self`, the builder-pattern API is
//! usually the more ergonomic option.
//!
//! ```
//! use embedded_hal::spi::MODE_1;
//!
//! // SAMD11/SAMD21 version
//! let spi = spi::Config::new(&pm, sercom, pads, freq)
//!     .baud(1.mhz())
//!     .char_size::<NineBit>()
//!     .bit_order(BitOrder::LsbFirst)
//!     .spi_mode(MODE_1)
//!     .enable();
//!
//! // SAMx5x version
//! let spi = spi::Config::new(&mclk, sercom, pads, freq)
//!     .baud(1.mhz())
//!     .length::<U2>()
//!     .bit_order(BitOrder::LsbFirst)
//!     .spi_mode(MODE_1)
//!     .enable();
//! ```
//!
//! To be accepted as a [`ValidConfig`], the `Config` must have a set of
//! [`ValidPads`] that matches its [`OpMode`]. In particular, the `SS` pad must
//! be [`NoneT`] for [`Master`] mode, where the user is expected to handle it
//! manaully. But it must be [`SomePad`] in [`MasterHWSS`] and [`Slave`] modes,
//! where it is controlled by the hardware.
//!
//! # Using a functional `Spi` peripheral
//!
//! An [`Spi`] struct has two type parameters. The first is the corresponding
//! `Config`, while the second represents its [`Capability`], i.e. [`Rx`],
//! [`Tx`] or [`Duplex`]. The [`enable`] function determines the `Capability`
//! automaically from the set of [`ValidPads`].
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, spi};
//! use atsamd_hal::sercom::v2::spi::{Master, Rx};
//! use atsamd_hal::typelevel::NoneT;
//!
//! // SAMD11/SAMD21-specific imports
//! use atsamd_hal::sercom::v2::spi::NineBit;
//!
//! // SAMx5x-specific imports
//! use atsamd_hal::sercom::v2::spi::lengths::U2;
//! use atsamd_hal::sercom::v2::pad::IoSet1;
//!
//! // SAMD11/SAMD21 version
//! type Pads = spi::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
//! type Config = spi::Config<Pads, Master, NineBit>;
//! type Spi = spi::Spi<Config, Rx>;
//!
//! // SAMx5x version
//! type Pads = spi::PadsFromIds<Sercom0, IoSet1, PA08, NoneT, PA09>;
//! type Config = spi::Config<Pads, Master, U2>;
//! type Spi = spi::Spi<Config, Rx>;
//! ```
//!
//! Only `Spi` structs can actually perform transactions. To do so, use the
//! various embedded HAL traits, like
//! [`spi::FullDuplex`](embedded_hal::spi::FullDuplex),
//! [`serial::Read`](embedded_hal::serial::Read) or
//! [`serial::Write`](embedded_hal::serial::Write).
//! See the [`impl_ehal`] module documentation for more details about the
//! specific trait implementations, which vary based on [`Size`] and
//! [`Capability`].
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
//! [`gpio`]: crate::gpio::v2
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

use core::convert::TryFrom;
use core::marker::PhantomData;

use bitflags::bitflags;
use embedded_hal::spi;
pub use embedded_hal::spi::{Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3};

use crate::sercom::v2::*;
use crate::time::Hertz;
use crate::typelevel::{Is, NoneT, Sealed};

mod reg;
use reg::Registers;

//=============================================================================
// Chip-specific imports
//=============================================================================

#[cfg(any(feature = "samd11", feature = "samd21"))]
use crate::pac::sercom0::spi::ctrla::MODE_A;
#[cfg(feature = "min-samd51g")]
use crate::pac::sercom0::spim::ctrla::MODE_A;

#[cfg(any(feature = "samd11", feature = "samd21"))]
#[path = "spi/pads_thumbv6m.rs"]
mod pads;

#[cfg(feature = "min-samd51g")]
#[path = "spi/pads_thumbv7em.rs"]
mod pads;

pub use pads::*;

#[cfg(any(feature = "samd11", feature = "samd21"))]
#[path = "spi/char_size.rs"]
mod size;

#[cfg(feature = "min-samd51g")]
#[path = "spi/length.rs"]
mod size;

pub use size::*;

/// Valid transaction [`Length`]s from the [`typenum`] crate
#[cfg(feature = "min-samd51g")]
pub mod lengths {
    seq_macro::seq!(N in 1..=255 {
        pub use typenum::U#N;
    });
}

#[cfg(any(feature = "samd11", feature = "samd21"))]
#[path = "spi/impl_ehal_thumbv6m.rs"]
pub mod impl_ehal;

#[cfg(feature = "min-samd51g")]
#[path = "spi/impl_ehal_thumbv7em.rs"]
pub mod impl_ehal;

//=============================================================================
// BitOrder
//=============================================================================

/// Define the bit order of transactions
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BitOrder {
    LsbFirst,
    MsbFirst,
}

//=============================================================================
// Flags
//=============================================================================

bitflags! {
    /// Interrupt bit flags for SPI transactions
    ///
    /// The available interrupt flags are `DRE`, `RXC`, `TXC`, `SSL` and
    /// `ERROR`. The binary format of the underlying bits exactly matches the
    /// `INTFLAG` register.
    pub struct Flags: u8 {
        const DRE = 0x01;
        const RXC = 0x02;
        const TXC = 0x04;
        const SSL = 0x08;
        const ERROR = 0x80;
    }
}

//=============================================================================
// Status
//=============================================================================

bitflags! {
    /// Status bit flags for SPI transactions
    ///
    /// The available status flags are `BUFOVF` and `LENERR`. The binary format
    /// of the underlying bits exactly matches the `STATUS` register.
    pub struct Status: u16 {
        const BUFOVF = 0x0004;
        const LENERR = 0x0800;
    }
}

/// Convert [`Status`] flags into the corresponding [`Error`] variants
impl TryFrom<Status> for () {
    type Error = Error;
    #[inline]
    fn try_from(status: Status) -> Result<(), Error> {
        // Buffer overflow has priority
        if status.contains(Status::BUFOVF) {
            Err(Error::Overflow)
        } else if status.contains(Status::LENERR) {
            Err(Error::LengthError)
        } else {
            Ok(())
        }
    }
}

//=============================================================================
// Error
//=============================================================================

/// Error `enum` for SPI transactions
///
/// The SPI peripheral only has two error types, buffer overflow and transaction
/// length error.
#[derive(Debug)]
pub enum Error {
    Overflow,
    LengthError,
}

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
// Size
//=============================================================================

/// Type alias for the width of the `DATA` register
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub type DataWidth = u16;

/// Type alias for the width of the `DATA` register
#[cfg(feature = "min-samd51g")]
pub type DataWidth = u32;

/// Trait alias whose definition varies by chip
///
/// On SAMD11 and SAMD21 chips, this represents the [`CharSize`].
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub trait Size: CharSize {}

#[cfg(any(feature = "samd11", feature = "samd21"))]
impl<C: CharSize> Size for C {}

/// Type alias for the default [`Size`] type, which varies by chip
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub type DefaultSize = EightBit;

/// Trait alias whose definition varies by chip
///
/// On SAMx5x chips, this represents the transaction [`Length`].
#[cfg(feature = "min-samd51g")]
pub trait Size: Length {}

#[cfg(feature = "min-samd51g")]
impl<L: Length> Size for L {}

/// Type alias for the default [`Size`] type, which varies by chip
#[cfg(feature = "min-samd51g")]
pub type DefaultSize = typenum::U1;

//==============================================================================
// AtomicSize
//==============================================================================

/// Marker trait for transaction [`Size`]s that can be completed in a single
/// read or write of the `DATA` register
pub trait AtomicSize: Size {}

#[cfg(any(feature = "samd11", feature = "samd21"))]
impl<C: CharSize> AtomicSize for C {}

#[cfg(feature = "min-samd51g")]
seq!(N in 1..=4 {
    impl AtomicSize for lengths::U#N {}
});

//==============================================================================
// Capability
//==============================================================================

/// Type-level enum representing the simplex or duplex transaction capability
///
/// The available, type-level variants are [`Rx`], [`Tx`] and [`Duplex`]. See
/// the [type-level enum] documentation for more details.
///
/// [type-level enum]: crate::typelevel#type-level-enums
pub trait Capability: Sealed + Default {}

/// Sub-set of [`Capability`] variants that can receive data, i.e. [`Rx`] and
/// [`Duplex`]
pub trait Receive: Capability {}

/// Sub-set of [`Capability`] variants that can transmit dat, i.e. [`Tx`] and
/// [`Duplex`]
pub trait Transmit: Capability {}

/// Type-level variant of the [`Capability`] enum for simplex, [`Receive`]-only
/// transactions
///
/// [`Spi`] structs are `Rx` when the `DO` (Data Out) type is [`NoneT`] in the
/// corresponding [`Pads`] struct.
///
/// While the [`Tx`] and [`Duplex`] structs are zero-sized, this struct is not.
/// Because an SPI master must initiate all transactions, using it in a simplex,
/// [`Receive`]-only context is slightly complicated. In that case, the [`Spi`]
/// struct must track whether a transaction needs to be started or is already in
/// progress. This struct contains a `bool` to track that progress.
#[derive(Default)]
pub struct Rx {
    pub(super) in_progress: bool,
}

impl Sealed for Rx {}
impl Capability for Rx {}
impl Receive for Rx {}

/// Type-level variant of the [`Capability`] enum for simplex, [`Transmit`]-only
/// transactions
///
/// [`Spi`] structs are `Tx` when the `DI` (Data In) type is [`NoneT`] in the
/// corresponding [`Pads`] struct.
#[derive(Default)]
pub struct Tx;

impl Sealed for Tx {}
impl Capability for Tx {}
impl Transmit for Tx {}

/// Type-level variant of the [`Capability`] enum for duplex transactions
///
/// [`Spi`] structs are `Duplex` when both the `DI` and `DO` [`Pads`] are
/// [`SomePad`].
/// corresponding [`Pads`] struct.
#[derive(Default)]
pub struct Duplex;

impl Sealed for Duplex {}
impl Capability for Duplex {}
impl Receive for Duplex {}
impl Transmit for Duplex {}

//=============================================================================
// Config
//=============================================================================

/// A configurable SPI peripheral in its disabled state
///
/// See the [module-level](super) documentation for more details on declaring
/// and instantiating `Pads` types.
pub struct Config<P, M = Master, Z = DefaultSize>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
    regs: Registers<P::Sercom>,
    pads: P,
    mode: PhantomData<M>,
    size: PhantomData<Z>,
    freq: Hertz,
}

impl<P: ValidPads> Config<P> {
    /// Create a new [`Config`] in the default configuration.
    #[inline]
    fn default(sercom: P::Sercom, pads: P, freq: impl Into<Hertz>) -> Self {
        let mut regs = Registers { sercom };
        regs.reset();
        regs.set_op_mode(Master::MODE, Master::MSSEN);
        regs.set_dipo_dopo(P::DIPO_DOPO);
        #[cfg(any(feature = "samd11", feature = "samd21"))]
        regs.set_char_size(EightBit::BITS);
        #[cfg(feature = "min-samd51g")]
        regs.set_length(1);
        Self {
            regs,
            pads,
            mode: PhantomData,
            size: PhantomData,
            freq: freq.into(),
        }
    }

    /// Create a new [`Config`] in the default configuration
    ///
    /// This function will enable the corresponding APB clock, reset the
    /// [`Sercom`] peripheral, and return a [`Config`] in the default
    /// configuration. The default [`OpMode`] is [`Master`], while the default
    /// [`Size`] is an
    #[cfg_attr(
        any(feature = "samd11", feature = "samd21"),
        doc = "[`EightBit`] [`CharSize`]"
    )]
    #[cfg_attr(feature = "min-samd51g", doc = "`EightBit` `CharSize`")]
    /// for SAMD11 and SAMD21 chips or a
    #[cfg_attr(any(feature = "samd11", feature = "samd21"), doc = "`Length` of `U1`")]
    #[cfg_attr(feature = "min-samd51g", doc = "[`Length`] of `U1`")]
    /// for SAMx5x chips. Note that [`Config`] takes ownership of both the
    /// PAC [`Sercom`] struct as well as the [`Pads`].
    ///
    /// Users must configure GCLK manually. The `freq` parameter represents the
    /// GCLK frequency for this [`Sercom`] instance.
    #[inline]
    pub fn new(
        apb_clk_ctrl: &APB_CLK_CTRL,
        mut sercom: P::Sercom,
        pads: P,
        freq: impl Into<Hertz>,
    ) -> Self {
        sercom.enable_apb_clock(apb_clk_ctrl);
        Self::default(sercom, pads, freq)
    }
}

impl<P, M, Z> Config<P, M, Z>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
    /// Change the [`OpMode`] or [`Size`]
    #[inline]
    fn change<M2, Z2>(self) -> Config<P, M2, Z2>
    where
        M2: OpMode,
        Z2: Size,
    {
        Config {
            regs: self.regs,
            pads: self.pads,
            mode: PhantomData,
            size: PhantomData,
            freq: self.freq,
        }
    }

    /// Obtain a reference to the PAC `SERCOM` struct
    ///
    /// Directly accessing the `SERCOM` could break the invariants of the
    /// type-level tracking in this module, so it is unsafe.
    #[inline]
    pub unsafe fn sercom(&self) -> &P::Sercom {
        &self.regs.sercom
    }

    /// Trigger the [`Sercom`]'s SWRST and return a [`Config`] in the
    /// default configuration.
    #[inline]
    pub fn reset(self) -> Config<P> {
        Config::default(self.regs.sercom, self.pads, self.freq)
    }

    /// Consume the [`Config`], reset the peripheral, and return the [`Sercom`]
    /// and [`Pads`]
    #[inline]
    pub fn free(mut self) -> (P::Sercom, P) {
        self.regs.reset();
        (self.regs.sercom, self.pads)
    }

    /// Change the [`OpMode`]
    #[inline]
    pub fn op_mode<M2: OpMode>(mut self) -> Config<P, M2, Z> {
        self.regs.set_op_mode(M2::MODE, M2::MSSEN);
        self.change()
    }

    /// Change the [`CharSize`] using the builder patter
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    #[inline]
    pub fn char_size<C2: CharSize>(mut self) -> Config<P, M, C2> {
        self.regs.set_char_size(C2::BITS);
        self.change()
    }

    /// Change the transaction [`Length`] using the builder patter
    ///
    /// To use a run-time dynamic length, set the [`Length`] type to
    /// [`DynLength`] and then use the [`dyn_length`] method.
    ///
    /// [`dyn_length`]: Config::dyn_length
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn length<L2: Length>(mut self) -> Config<P, M, L2> {
        self.regs.set_length(L2::U8);
        self.change()
    }

    /// Get the clock polarity
    #[inline]
    pub fn get_cpol(&self) -> Polarity {
        self.regs.get_cpol()
    }

    /// Set the clock polarity
    #[inline]
    pub fn set_cpol(&mut self, cpol: Polarity) {
        self.regs.set_cpol(cpol);
    }

    /// Set the clock polarity using the builder pattern
    #[inline]
    pub fn cpol(mut self, cpol: Polarity) -> Self {
        self.set_cpol(cpol);
        self
    }

    /// Get the clock phase
    #[inline]
    pub fn get_cpha(&self) -> Phase {
        self.regs.get_cpha()
    }

    /// Set the clock phase
    #[inline]
    pub fn set_cpha(&mut self, cpha: Phase) {
        self.regs.set_cpha(cpha)
    }

    /// Set the clock phase using the builder pattern
    #[inline]
    pub fn cpha(mut self, cpha: Phase) -> Self {
        self.set_cpha(cpha);
        self
    }

    /// Get the SPI mode (clock polarity & phase)
    #[inline]
    pub fn get_spi_mode(&self) -> spi::Mode {
        self.regs.get_spi_mode()
    }

    /// Set the SPI mode (clock polarity & phase)
    #[inline]
    pub fn set_spi_mode(&mut self, mode: spi::Mode) {
        self.regs.set_spi_mode(mode);
    }

    /// Set the SPI mode (clock polarity & phase) using the builder pattern
    #[inline]
    pub fn spi_mode(mut self, mode: spi::Mode) -> Self {
        self.set_spi_mode(mode);
        self
    }

    /// Get the bit order of transmission (MSB/LSB first)
    ///
    /// This only affects the order of bits within each byte. Bytes are always
    /// transferred in little endian order from the 32-bit DATA register.
    #[inline]
    pub fn get_bit_order(&self) -> BitOrder {
        self.regs.get_bit_order()
    }

    /// Set the bit order of transmission (MSB/LSB first) using the builder
    /// pattern
    ///
    /// This only affects the order of bits within each byte. Bytes are always
    /// transferred in little endian order from the 32-bit DATA register.
    #[inline]
    pub fn set_bit_order(&mut self, order: BitOrder) {
        self.regs.set_bit_order(order);
    }

    /// Set the bit order of transmission (MSB/LSB first) using the builder
    /// pattern
    ///
    /// This only affects the order of bits within each byte. Bytes are always
    /// transferred in little endian order from the 32-bit DATA register.
    #[inline]
    pub fn bit_order(mut self, order: BitOrder) -> Self {
        self.set_bit_order(order);
        self
    }

    /// Get the baud rate
    ///
    /// The returned baud rate may not exactly match what was set.
    #[inline]
    pub fn get_baud(&mut self) -> Hertz {
        self.regs.get_baud(self.freq)
    }

    /// Set the baud rate
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// half the GCLK frequency. The minimum baud rate is the GCLK frequency /
    /// 512. Values outside this range will saturate at the extremes.
    #[inline]
    pub fn set_baud(&mut self, baud: impl Into<Hertz>) {
        self.regs.set_baud(self.freq, baud);
    }

    /// Set the baud rate using the builder API
    ///
    /// This function will calculate the best BAUD register setting based on the
    /// stored GCLK frequency and desired baud rate. The maximum baud rate is
    /// half the GCLK frequency. The minimum baud rate is the GCLK frequency /
    /// 512. Values outside this range will saturate at the extremes.
    #[inline]
    pub fn baud(mut self, baud: impl Into<Hertz>) -> Self {
        self.set_baud(baud);
        self
    }

    /// Read the enabled state of the immediate buffer overflow notification
    ///
    /// If set to true, an [`Error::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    pub fn get_ibon(&self) -> bool {
        self.regs.get_ibon()
    }

    /// Enable or disable the immediate buffer overflow notification
    ///
    /// If set to true, an [`Error::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    pub fn set_ibon(&mut self, enabled: bool) {
        self.regs.set_ibon(enabled);
    }

    /// Enable or disable the immediate buffer overflow notification using the
    /// builder API
    ///
    /// If set to true, an [`Error::Overflow`] will be issued as soon as an
    /// overflow occurs. Otherwise, it will not be issued until its place within
    /// the data stream.
    #[inline]
    pub fn ibon(mut self, enabled: bool) -> Self {
        self.set_ibon(enabled);
        self
    }

    /// Read the enable state of run in standby mode
    #[inline]
    pub fn get_run_in_standby(&self) -> bool {
        self.regs.get_run_in_standby()
    }

    /// Enable or disable run in standby mode
    #[inline]
    pub fn set_run_in_standby(&mut self, enabled: bool) {
        self.regs.set_run_in_standby(enabled);
    }

    /// Enable or disable run in standby mode using the builder API
    #[inline]
    pub fn run_in_standby(mut self, enabled: bool) -> Self {
        self.set_run_in_standby(enabled);
        self
    }

    /// Enable the SPI peripheral
    ///
    /// SPI transactions are not possible until the peripheral is enabled.
    /// This function is limited to [`ValidConfig`]s.
    #[inline]
    pub fn enable(mut self) -> Spi<Self, P::Capability>
    where
        Self: ValidConfig,
    {
        self.regs.rx_enable();
        self.regs.enable();
        Spi {
            config: self,
            capability: P::Capability::default(),
        }
    }
}

#[cfg(feature = "min-samd51g")]
impl<P, M> Config<P, M, DynLength>
where
    P: ValidPads,
    M: OpMode,
{
    /// Get the transaction length
    #[inline]
    pub fn get_dyn_length(&self) -> u8 {
        self.regs.get_length()
    }

    /// Set the transaction length
    ///
    /// Write the LENGTH register to set the transaction length. If the length
    /// is zero, it will be set to 1.
    #[inline]
    pub fn set_dyn_length(&mut self, length: u8) {
        self.regs.set_length(length);
    }

    /// Set the transaction length using the builder API
    ///
    /// Write the LENGTH register to set the transaction length. If the length
    /// is zero, it will be set to 1.
    #[inline]
    pub fn dyn_length(mut self, length: u8) -> Self {
        self.set_dyn_length(length);
        self
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
/// copies the [`Sercom`], [`Capability`] and [`Word`] types, to make it easier
/// to apply bounds to these types at the next level of abstraction.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait AnyConfig: Is<Type = SpecificConfig<Self>> {
    type Sercom: Sercom;
    type Pads: ValidPads<Sercom = Self::Sercom>;
    type Capability: Capability;
    type OpMode: OpMode;
    type Size: Size;
    type Word: 'static;
}

/// Type alias to recover the specific [`Config`] type from an implementation of
/// [`AnyConfig`]
pub type SpecificConfig<C> =
    Config<<C as AnyConfig>::Pads, <C as AnyConfig>::OpMode, <C as AnyConfig>::Size>;

impl<P, M, Z> Sealed for Config<P, M, Z>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
}

impl<P, M, Z> AnyConfig for Config<P, M, Z>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
    type Sercom = P::Sercom;
    type Pads = P;
    type Capability = P::Capability;
    type OpMode = M;
    type Size = Z;
    type Word = Z::Word;
}

impl<P, M, Z> AsRef<Self> for Config<P, M, Z>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<P, M, Z> AsMut<Self> for Config<P, M, Z>
where
    P: ValidPads,
    M: OpMode,
    Z: Size,
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
/// A functional SPI peripheral must have, at a minimum, an SCLK pad and
/// either a Data In or a Data Out pad. Dependeing on the [`OpMode`], an SS
/// pad may also be required.
///
/// The `ValidConfig` trait is implemented only for valid combinations of
/// [`Pads`] and [`OpMode`]. No [`Config`] is valid if the SCK pad is [`NoneT`]
/// or if both the Data In and Data Out pads are `NoneT`. When in [`Master`]
/// `OpMode`, the `SS` pad must be `NoneT`, while in [`MasterHWSS`] or
/// [`Slave`] [`OpMode`], the SS pad must be [`SomePad`].
pub trait ValidConfig: AnyConfig {}

impl<P, Z> ValidConfig for Config<P, Master, Z>
where
    P: ValidPads<SS = NoneT>,
    Z: Size,
{
}

impl<P, Z> ValidConfig for Config<P, MasterHWSS, Z>
where
    P: ValidPads,
    Z: Size,
    P::SS: SomePad,
{
}

impl<P, Z> ValidConfig for Config<P, Slave, Z>
where
    P: ValidPads,
    Z: Size,
    P::SS: SomePad,
{
}

//=============================================================================
// Spi
//=============================================================================

/// An enabled SPI peripheral that can perform transactions
///
/// See the [`impl_ehal`] documentation for details on the implementations of
/// the embedded HAL traits, which vary based on [`Size`] and [`Capability`].
pub struct Spi<C, A>
where
    C: ValidConfig,
    A: Capability,
{
    config: C,
    capability: A,
}

/// Get a shared reference to the underlying [`Config`] struct
///
/// This can be used to call the various `get_*` functions on `Config`
impl<C, A> AsRef<SpecificConfig<C>> for Spi<C, A>
where
    C: ValidConfig,
    A: Capability,
{
    #[inline]
    fn as_ref(&self) -> &SpecificConfig<C> {
        self.config.as_ref()
    }
}

impl<C, A> Spi<C, A>
where
    C: ValidConfig,
    A: Capability,
{
    /// Change the transaction [`Length`]
    ///
    /// Changing the transaction [`Length`] while is enabled is permissible but
    /// `unsafe`. If you have sent or received *any* bytes at the current
    /// [`Length`], you **must** wait for a TXC flag before changing to a new
    /// [`Length`].
    #[inline]
    #[cfg(feature = "min-samd51g")]
    pub unsafe fn length<L: Length>(self) -> Spi<Config<C::Pads, C::OpMode, L>, A>
    where
        Config<C::Pads, C::OpMode, L>: ValidConfig,
    {
        Spi {
            config: self.config.into().length(),
            capability: self.capability,
        }
    }

    /// Update the SPI configuration.
    ///
    /// Calling this method will temporarily disable the SERCOM peripheral, as
    /// some registers are enable-protected. This may interrupt any ongoing
    /// transactions.
    #[inline]
    pub fn reconfigure(&mut self, update: impl FnOnce(&mut SpecificConfig<C>)) {
        self.config.as_mut().regs.disable();
        update(self.config.as_mut());
        self.config.as_mut().regs.enable();
    }

    /// Enable interrupts for the specified flags
    #[inline]
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.config.as_mut().regs.enable_interrupts(flags)
    }

    /// Disable interrupts for the specified flags
    #[inline]
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.config.as_mut().regs.disable_interrupts(flags);
    }

    /// Read the interrupt flags
    #[inline]
    pub fn read_flags(&self) -> Flags {
        self.config.as_ref().regs.read_flags()
    }

    /// Clear the corresponding interrupt flags
    ///
    /// Only the ERROR, SSL and TXC flags can be cleared.
    ///
    /// **Note:** The implementation of of [`serial::Write::flush`] waits on and
    /// clears the `TXC` flag. Manually clearing this flag could cause it to
    /// hang indefinitely.
    ///
    /// [`serial::Write::flush`]: embedded_hal::serial::Write::flush
    #[inline]
    pub fn clear_flags(&mut self, flags: Flags) {
        self.config.as_mut().regs.clear_flags(flags);
    }

    /// Read the error status flags
    #[inline]
    pub fn read_status(&self) -> Status {
        self.config.as_ref().regs.read_status()
    }

    /// Clear the corresponding error status flags
    #[inline]
    pub fn clear_status(&mut self, status: Status) {
        self.config.as_mut().regs.clear_status(status);
    }

    /// Try to read the interrupt flags, but first check the error status flags.
    #[inline]
    pub fn read_flags_errors(&self) -> Result<Flags, Error> {
        self.config.as_ref().regs.read_flags_errors()
    }

    /// Read from the DATA register
    ///
    /// Reading from the data register directly is `unsafe`, because it will
    /// clear the RXC flag, which could break assumptions made elsewhere in
    /// this module.
    #[inline]
    pub unsafe fn read_data(&mut self) -> DataWidth {
        self.config.as_mut().regs.read_data()
    }

    /// Write to the DATA register
    ///
    /// Writing to the data register directly is `unsafe`, because it will clear
    /// the DRE flag, which could break assumptions made elsewhere in this
    /// module.
    #[inline]
    pub unsafe fn write_data(&mut self, data: DataWidth) {
        self.config.as_mut().regs.write_data(data);
    }

    /// Disable the SPI peripheral and return the [`Config`] struct
    #[inline]
    pub fn disable(mut self) -> C {
        self.config.as_mut().regs.rx_disable();
        self.config.as_mut().regs.disable();
        self.config
    }
}

#[cfg(feature = "min-samd51g")]
impl<P, M, A> Spi<Config<P, M, DynLength>, A>
where
    P: ValidPads,
    M: OpMode,
    Config<P, M, DynLength>: ValidConfig,
    A: Capability,
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
        self.config.set_dyn_length(length);
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
/// copies the [`Sercom`], [`Pads`], [`Capability`], [`OpMode`], [`Size`] and
/// [`Word`] types, to make it easier to apply bounds to these types at the next
/// level of abstraction.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait AnySpi: Is<Type = SpecificSpi<Self>> {
    type Sercom: Sercom;
    type Pads: ValidPads;
    type Capability: Capability;
    type OpMode: OpMode;
    type Size: Size;
    type Word: 'static;
    type Config: ValidConfig<Sercom = Self::Sercom>;
}

/// Type alias to recover the specific [`Spi`] type from an implementation of
/// [`AnySpi`]
pub type SpecificSpi<S> = Spi<<S as AnySpi>::Config, <S as AnySpi>::Capability>;

impl<C, A> AsRef<Self> for Spi<C, A>
where
    C: ValidConfig,
    A: Capability,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<C, A> AsMut<Self> for Spi<C, A>
where
    C: ValidConfig,
    A: Capability,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<C, A> Sealed for Spi<C, A>
where
    C: ValidConfig,
    A: Capability,
{
}

impl<C, A> AnySpi for Spi<C, A>
where
    C: ValidConfig,
    A: Capability,
{
    type Sercom = C::Sercom;
    type Pads = C::Pads;
    type Capability = A;
    type OpMode = C::OpMode;
    type Size = C::Size;
    type Word = C::Word;
    type Config = C;
}
