//! Use the SERCOM peripheral for UART communications
//!
//! Configuring an UART peripheral occurs in three steps. First, you must create
//! a set of [`Pads`] for use by the peripheral. Next, you assemble pieces into
//! a [`Config`] struct. After configuring the peripheral, you then [`enable`]
//! it, yielding a functional [`Uart`] struct.
//! Transactions are performed using the [`serial`](embedded_hal::serial) traits
//! from embedded HAL.
//!
//! # [`Pads`]
//!
//! A [`Sercom`] can use up to four [`Pin`]s as peripheral [`Pad`]s, but only
//! certain [`Pin`] combinations are acceptable. In particular, all [`Pin`]s
//! must be mapped to the same `Sercom` (see the datasheet). This HAL makes it
//! impossible to use invalid [`Pin`]/[`Pad`] combinations, and the [`Pads`]
//! struct is responsible for enforcing these constraints.
//!
//!
//! A `Pads` type takes five type parameters. The first specifies the `Sercom`,
//! while the remaining four, `DI`, `DO`, `CK` and `SS`, represent the Data In,
//! Data Out, Sclk and SS pads respectively. Each of the remaining type
//! parameters is an [`OptionalPad`] and defaults to [`NoneT`]. Aliases defining
//! the pad types can be provided by the [`bsp_pins!`] macro.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09, AlternateC};
//! use atsamd_hal::sercom::v2::{Sercom0, uart};
//! use atsamd_hal::typelevel::NoneT;
//!
//! type Rx = Pin<PA08, AlternateC>;
//! type Tx = Pin<PA09, AlternateC>;
//! type Pads = uart::Pads<Sercom0, Rx, Tx>;
//! ```
#![cfg_attr(
    not(feature = "samd11"),
    doc = "
Alternatively, you can use the [`PadsFromIds`] alias to define a set of
`Pads` in terms of [`PinId`]s instead of `Pin`s. This is useful when you
don't have `Pin` aliases pre-defined.

```
use atsamd_hal::gpio::v2::{PA08, PA09};
use atsamd_hal::sercom::v2::{Sercom0, uart};
use atsamd_hal::typelevel::NoneT;

type Pads = uart::PadsFromIds<Sercom0, PA09, PA09>;
```

"
)]
//!
//! Instaces of [`Pads`] are created using the builder pattern. Start by
//! creating an empty set of [`Pads`] using [`Default`]. Then pass each
//! respective [`Pin`] using the corresponding methods. Both `v1::Pin` and
//! `v2::Pin` types are accepted here.
//!
//! On SAMD21 and SAMx5x chips, the builder methods automatically convert each
//! pin to the correct [`PinMode`]. But for SAMD11 chips, users must manually
//! convert each pin before calling the builder methods. This is a consequence
//! of inherent ambiguities in the SAMD11 SERCOM pad definitions. Specifically,
//! the same [`PinId`] can correspond to two different [`PadNum`]s for the
//! *same* `Sercom`.
//!
//! ```
//! use atsamd_hal::target_device::Peripherals;
//! use atsamd_hal::gpio::v2::Pins;
//! use atsamd_hal::sercom::v2::{Sercom0, uart};
//!
//! let mut peripherals = Peripherals::take().unwrap();
//! let pins = Pins::new(peripherals.PORT);
//! let pads = uart::Pads::<Sercom0>::default()
//!     .rx(pins.pa09)
//!     .tx(pins.pa08);
//! ```
//!
//! To be accepted as [`ValidPads`], a set of [`Pads`] must do two things:
//! - Specify a type for at least one of `RX` or `TX`
//! - Satisfy the `RxpoTxpo` trait (SAMD11/SAMD21), or the `Rxpo` and `Txpo`
//!   traits (SAMx5x)
//!
//! # [`Config`]
//!
//! Next, create a [`Config`] struct, which represents the UART peripheral in
//! its disabled state. A [`Config`] is specified with two type parameters: the
//! [`Pads`] type; and a [`CharSize`], which defaults to [`EightBit`].
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, uart};
//! use atsamd_hal::sercom::v2::uart::{NineBit};
//! use atsamd_hal::typelevel::NoneT;
//!
//! type Pads = uart::PadsFromIds<Sercom0, PA08, PA09>;
//! type Config = uart::Config<Pads, NineBit>;
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
//! let config = uart::Config::new(&pm, sercom, pads, freq);
//! ```
//!
//! The [`Config`] struct can configure the peripheral in one of two ways:
//!
//! * A set of methods is provided to use in a builder pattern: for example
//!   [`baud`](Config::baud), [`stop_bits`](Config::stop_bits), etc. These
//!   methods take `self` and return `Self`.
//! * A set of methods is provided to use as setters: for example
//!   [`set_baud`](Config::set_baud), [`set_stop_bits`](Config::set_stop_bits),
//!   etc. These methods take `&mut self` and return nothing.
//!
//! In any case, the peripheral setup ends with a call to [`enable`], which
//! consumes the [`Config`] and returns an enabled [`Uart`] peripheral.
//!
//! ```
//! use atsamd_hal::sercom::v2::uart::{StopBits, NineBit, BitOrder, BaudMode, Oversampling};
//!
//! let uart = uart::Config::new(&mclk, sercom, pads, freq)
//!     .baud(1.mhz(), BaudMode::Arithmetic(Oversampling::Bits16))
//!     .char_size::<NineBit>()
//!     .bit_order(BitOrder::LsbFirst)
//!     .stop_bits(StopBits::TwoBits)
//!     .enable();
//! ```
//!
//! Alternatively,
//!
//! ```
//! use atsamd_hal::sercom::v2::uart::{StopBits, NineBit, BitOrder, BaudMode, Oversampling};
//!
//! let uart = uart::Config::new(&mclk, sercom, pads, freq);
//!     uart.set_baud(1.mhz(), BaudMode::Arithmetic(Oversampling::Bits16));
//!     uart.set_char_size::<NineBit>();
//!     uart.set_bit_order(BitOrder::LsbFirst);
//!     uart.set_stop_bits(StopBits::TwoBits);
//!     let uart = uart.enable();
//! ```
//!
//!
//! To be accepted as a [`ValidConfig`], the [`Config`] must have at least one
//! of `Rx` or `Tx` pads.
//!
//! ## [`CharSize`]
//!
//! The UART peripheral can be configured to use different character sizes. By
//! default, a [`Config`] is configured with an [`EightBit`] character size.
//! This can be changed through the [`char_size`](Config::char_size) method.
//! Changing the character normally also changes the [`Config`]'s type.
//! Alternatively, you can also use a [`DynCharSize`] through the
//! [`dyn_char_size`](Config::dyn_char_size) method. This enables you to
//! dynamically change the character size on the fly through the
//! [`set_dyn_char_size`](Config::set_dyn_char_size) method when calling
//! [`reconfigure`](Uart::reconfigure).
//!
//! ## Reading the current configuration
//!
//! It is possible to read the current configuration by using the getter methods
//! provided: for example [`get_baud`](Config::get_baud),
//! [`get_stop_bits`](Config::get_stop_bits), etc.
//!
//! # [`Uart`] and capabilities
//!
//! [`Uart`] structs can only be created from a [`Config`]. They have two type
//! parameters: the first one represents the underlying [`Config`], while the
//! second represents the [`Uart`]'s capabilities. The second type parameter can
//! be one of:
//!
//! * [`Rx`] or [`RxDuplex`]: Can perform receive transactions
//! * [`Tx`] or [`TxDuplex`]: Can perform transmit transactions
//! * [`Duplex`]: UART configured as duplex that can perform receive and
//!   transmit transactions. Additionally, the [`split`] method can be
//!  called to return a `Uart<C, RxDuplex>, Uart<C, TxDuplex>)` tuple. See the
//! [Splitting](self#Splitting) section for more information.
//!
//! The nature of the underlying [`Pads`] contained inside [`Config`] determines
//! the type returned by a call to [`enable`]. If the pads only have a `TX` pin
//! specified, then [`enable`] will return a `Uart<C, Tx>`. Similarly, If the
//! pads only have a `RX` pin specified, then [`enable`] will return a `Uart<C,
//! Rx>`. Finally, if both `RX` and `TX` pins are specified, then [`enable`]
//! will return a `Uart<C, Duplex>`, which can be further split into a `Uart<C,
//! RxDuplex>` and a `Uart<C, TxDuplex>`.
//!
//! ```
//! use atsamd_hal::gpio::v2::{PA08, PA09};
//! use atsamd_hal::sercom::v2::{Sercom0, uart};
//! use atsamd_hal::sercom::v2::uart::NineBit;
//! use atsamd_hal::typelevel::NoneT;
//!
//! // Assuming SAMD21 or SAMx5x
//! type Pads = uart::PadsFromIds<Sercom0, PA08, NoneT, PA09>;
//! type Config = uart::Config<Pads, NineBit>;
//! type UartRx = uart::Uart<Config, RxDuplex>;
//! type UartTx = uart::UartTx<Config, RxDuples>;
//! ```
//!
//! Only the [`Uart`] struct can actually perform
//! transactions. To do so, use the embedded HAL traits, like
//! [`serial::Read`] and [`serial::Write`].
//!
//! ```
//! use nb::block;
//! use embedded_hal::serial::Write;
//!
//! block!(uart_tx.write(0x0fe));
//! ```
//!
//! # UART flow control (CTS/RTS)
//!
//! This module supports CTS and RTS pins.
//!
//! The `RTS` pin is a fully hardware-controlled output pin that gets deasserted
//! when:
//!
//! * The USART receiver is disabled;
//! * The USART's RX buffer is full.
//!
//! The `CTS` pin is an input pin that provides an interrupt when a change
//! (rising or falling edge) is detected on the corresponding Pad. This
//! interrupt, `CTSIC`, can be enabled with the
//! [`enable_ctsic`](Uart::enable_ctsic) method only when the corresponding
//! [`Config`] has a `CTS` pad specified. The
//! [`disable_ctsic`](Uart::disable_ctsic) and
//! [`clear_ctsic`](Uart::clear_ctsic) methods are also available under the same
//! conditions. [This application note](https://www.silabs.com/documents/public/application-notes/an0059.0-uart-flow-control.pdf)
//! provides more information about UART hardware flow control.
//!
//! # Splitting
//!
//! A `Uart<C, Duplex>` can be split into its [`RxDuplex`] and [`TxDuplex`]
//! constituents:
//!
//! ```
//! use atsamd_hal::sercom::v2::uart::Uart;
//! // Assume uart is a Uart<C, Duplex>
//! let (rx, tx) = uart.split();
//! ```
//!
//! # Joining
//!
//! When a `Uart<C, Duplex>` has been split into its [`RxDuplex`] and
//! [`TxDuplex`] parts, these parts can be joined back into a `Uart<C, Duplex>`
//! by calling the [`join`] function for `Uart<C, Duplex>`. It takes a `Uart<C,
//! RxDuplex>` and a `Uart<C, TxDuplex>` and moves them into a full [`Duplex`]
//! [`Uart`].
//!
//! ```
//! use atsamd_hal::sercom::v2::uart::Uart;
//!
//! // Assume rx is a Uart<C, RxDuplex> and tx is a Uart<C, TxDuplex>
//! let uart = Uart::join(rx, tx);
//! // uart is now a Uart<C, Duplex>
//! ```
//!
//! The [`AsMut<Uart<C, Duplex>>`] trait is also implemented for `(&mut Uart<C,
//! RxDuplex>, &mut Uart<C, TxDuplex>)`. This is useful if you need an `&mut
//! Uart<C, Duplex>` but you only have a pair of `&mut Uart<C, RxDuplex>` and
//! `&mut Uart<C, TxDuplex>`. This can be leveraged to use the [`reconfigure`]
//! method when all you have is a pair of mutable references to the [`RxDuplex`]
//! and [`TxDuplex`] halves.
//!
//! ```
//! use atsamd_hal::sercom::v2::uart::Uart;
//! use atsamd_hal::time::*;
//!
//! // Assume rx is a Uart<C, RxDuplex> and tx is a Uart<C, TxDuplex>
//!
//! // Reconfigure peripheral from mutable references to RxDuplex
//! // and TxDuplex halves
//! (&mut rx, &mut tx).as_mut().reconfigure(|c| c.set_run_in_standby(false));
//! ```
//!
//! # Reading the current configuration
//!
//! The `AsRef<Config<P, C>>` trait is implemented for `Uart<Config<P, C>, D>`.
//! This means you can use the `get_` methods implemented for `Config`, since
//! they take an `&self` argument.
//!
//! ```
//! // Assume uart is a Uart<C, D>
//! let (baud, baud_mode) = uart.as_ref().get_baud();
//! ```
//!
//! # Disabling and reconfiguring
//!
//! Some methods, such as [`disable`] and [`reconfigure`], need to operate on
//! all parts of a UART at once. In practice, this means that these methods
//! operate on the type that was returned by [`enable`]. This can be `Uart<C,
//! Rx>`, `Uart<C, Tx>`, or `Uart<C, Duplex>`, depending on how the
//! peripheral was configured.
//!
//! The [`reconfigure`] method gives out an `&mut Config` reference, which can
//! then use the `set_*` methods.
//!
//! ```
//! use atsamd_hal::sercom::v2::uart::Uart;
//! use atsamd_hal::time::*;
//!
//! // Assume config is a valid Duplex UART Config struct
//! let (rx, tx)= config.enable().split();
//!
//! // Send/receive data with tx/rx halves...
//!
//! // If the UART peripheral is configured in Duplex mode,
//! // the two constituting halves need to be joined back into
//! // a Uart<C, Duplex> before calling disable()
//! let uart = Uart::join(rx, tx);
//!
//! // Reconfigure UART peripheral
//! uart.reconfigure(|c| c.set_run_in_standby(false));
//!
//! // Disable UART peripheral
//! let config = uart.disable();
//! ```
//!
//! # Non-supported advanced features
//!
//! * Synchronous mode (USART) is not supported
//! * LIN mode is not supported (SAMx5x)
//! * 32-bit extension mode is not supported (SAMx5x). If you need to transfer
//!   slices, consider using the DMA methods instead. The `dma` Cargo feature
//!   must be enabled.
//!
//! [`enable`]: Config::enable
//! [`disable`]: Uart::disable
//! [`reconfigure`]: Uart::reconfigure
//! [`bsp_pins`]: crate::bsp_pins
//! [`Pin`]: crate::gpio::v2::pin::Pin
//! [`PinId`]: crate::gpio::v2::pin::PinId
//! [`PinMode`]: crate::gpio::v2::pin::PinMode
//! [`split`]: Uart::split
//! [`join`]: Uart::join
//! [`NoneT`]: crate::typelevel::NoneT
//! [`serial::Write`]: embedded_hal::serial::Write
//! [`serial::Read`]: embedded_hal::serial::Read
#![cfg_attr(
    feature = "dma",
    doc = "
# Using UART with DMA

This HAL includes support for DMA-enabled UART transfers. [`Uart`]
implements the DMAC [`Buffer`]
trait. The provided [`send_with_dma`] and
[`receive_with_dma`] build and begin a
[`dmac::Transfer`], thus starting the UART
in a non-blocking way. Optionally, interrupts can be enabled on the provided
[`Channel`]. Note that the `dma` feature must
be enabled. Please refer to the [`dmac`](crate::dmac) module-level
documentation for more information.

```
/// Assume channel0 and channel1 are configured `dmac::Channel`s,
/// rx is a Uart<C, RxDuplex>, and tx is a Uart<C, TxDuplex>.

/// Create data to send
let tx_buffer: [u8; 50] = [0xff; 50];
let rx_buffer: [u8; 100] = [0xab; 100];

/// Launch transmit transfer
let tx_dma = tx.send_with_dma(&mut tx_buffer, channel0, ());

/// Launch receive transfer
let rx_dma = rx.receive_with_dma(&mut rx_buffer, channel1, ());

/// Wait for transfers to complete and reclaim resources
let (chan0, tx_buffer, tx) = tx_dma.wait();
let (chan1, rx, rx_buffer) = rx_dma.wait();
```

[`Buffer`]: crate::dmac::transfer::Buffer
[`send_with_dma`]: Uart::send_with_dma
[`receive_with_dma`]: Uart::receive_with_dma
[`dmac::Transfer`]: crate::dmac::Transfer
[`Channel`]: crate::dmac::channel::Channel
[`dmac`]: crate::dmac

"
)]

#[cfg(any(feature = "samd11", feature = "samd21"))]
#[path = "pads_thumbv6m.rs"]
mod pads;

#[cfg(feature = "min-samd51g")]
#[path = "pads_thumbv7em.rs"]
mod pads;

pub use pads::*;

mod reg;
use reg::Registers;

mod charsize;
pub use charsize::*;

mod flags;
pub use flags::*;

mod config;
pub use config::*;

pub mod impl_ehal;

use crate::{sercom::v2::*, typelevel::Sealed};
use core::{convert::TryInto, marker::PhantomData};
use num_traits::AsPrimitive;

//=============================================================================
// Stop bits, parity, baud rate, bit order
//=============================================================================

/// Number of stop bits in a UART frame
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum StopBits {
    /// 1 stop bit
    OneBit = 0,
    /// 2 stop bits
    TwoBits = 1,
}

impl From<StopBits> for bool {
    #[inline]
    fn from(item: StopBits) -> bool {
        match item {
            StopBits::OneBit => false,
            StopBits::TwoBits => true,
        }
    }
}

impl From<bool> for StopBits {
    #[inline]
    fn from(item: bool) -> StopBits {
        match item {
            false => StopBits::OneBit,
            true => StopBits::TwoBits,
        }
    }
}

/// Parity setting of a UART frame
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Parity {
    /// No parity
    None,
    /// Even parity
    Even,
    /// Odd parity
    Odd,
}

/// Bit order of a UART frame
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BitOrder {
    /// MSB-first
    MsbFirst = 0,
    /// LSB-first
    LsbFirst = 1,
}

impl From<BitOrder> for bool {
    #[inline]
    fn from(item: BitOrder) -> bool {
        match item {
            BitOrder::MsbFirst => false,
            BitOrder::LsbFirst => true,
        }
    }
}
impl From<bool> for BitOrder {
    #[inline]
    fn from(item: bool) -> BitOrder {
        match item {
            false => BitOrder::MsbFirst,
            true => BitOrder::LsbFirst,
        }
    }
}

/// Baudrate oversampling values
///
/// *NOTE* 3x oversampling has been intentionally left out
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Oversampling {
    // 3 samples per bit
    // Bits3 = 3,
    /// 8 samples per bit
    Bits8 = 8,
    /// 16 samples per bit
    Bits16 = 16,
}

/// Baudrate calculation in asynchronous mode
#[derive(Debug, Clone, Copy)]
pub enum BaudMode {
    /// Asynchronous arithmetic baud calculation
    Arithmetic(Oversampling),
    /// Asynchronous fractional baud calculation
    Fractional(Oversampling),
}

impl BaudMode {
    #[inline]
    fn sampr(&self) -> u8 {
        use BaudMode::*;
        use Oversampling::*;
        match self {
            Arithmetic(n) => match n {
                Bits16 => 0,
                Bits8 => 2,
            },

            Fractional(n) => match n {
                Bits16 => 1,
                Bits8 => 3,
            },
        }
    }

    #[inline]
    fn get_sampr(sampr: u8) -> Self {
        use BaudMode::*;
        use Oversampling::*;
        match sampr {
            0 => Arithmetic(Bits16),
            1 => Fractional(Bits16),
            2 => Arithmetic(Bits8),
            3 => Fractional(Bits8),
            _ => unreachable!(),
        }
    }
}

//=============================================================================
// Capability
//=============================================================================

/// Type-level `enum` representing the capabilities of a UART peripheral
pub trait Capability: Sealed {
    /// Available interrupt flags for the specified capability
    const FLAG_MASK: u8;
    /// Available status flags for the specified capability
    const STATUS_MASK: u16;
}

/// Type-level enum representing a UART that can transmit
pub trait Transmit: Capability {}

/// Type-level enum representing a UART that can receive
pub trait Receive: Capability {}

/// Type-level enum representing a UART that has transmit or receive
/// capability, but not both
pub trait Simplex: Capability {}

/// Marker type representing a UART that has both transmit and receive
/// capability
pub enum Duplex {}
impl Sealed for Duplex {}
impl Capability for Duplex {
    // All flags are valid for a Duplex UART
    const FLAG_MASK: u8 = DUPLEX_FLAG_MASK;

    // All status flags are valid for a Duplex UART
    const STATUS_MASK: u16 = DUPLEX_STATUS_MASK;
}
impl Receive for Duplex {}
impl Transmit for Duplex {}

/// Marker type representing a UART that can only receive
pub enum Rx {}
impl Sealed for Rx {}
impl Capability for Rx {
    // Available interrupt flags for a RX half-UART
    const FLAG_MASK: u8 = RX_FLAG_MASK;

    // Available status flags for a RX half-UART
    const STATUS_MASK: u16 = RX_STATUS_MASK;
}
impl Receive for Rx {}
impl Simplex for Rx {}

/// Marker type representing a UART that can only transmit
pub enum Tx {}
impl Sealed for Tx {}
impl Capability for Tx {
    // Available interrupt flags for a TX half-UART
    const FLAG_MASK: u8 = TX_FLAG_MASK;

    // There are no settable/clearable status flags for TX half-UARTs
    const STATUS_MASK: u16 = 0;
}
impl Transmit for Tx {}
impl Simplex for Tx {}

/// Marker type representing the Rx half of a  [`Duplex`] UART
pub enum RxDuplex {}
impl Sealed for RxDuplex {}
impl Capability for RxDuplex {
    // Available interrupt flags for a RX half-UART
    const FLAG_MASK: u8 = RX_FLAG_MASK;

    // Available status flags for a RX half-UART
    const STATUS_MASK: u16 = RX_STATUS_MASK;
}
impl Receive for RxDuplex {}

/// Marker type representing a the Tx half of a [`Duplex`] UART
pub enum TxDuplex {}
impl Sealed for TxDuplex {}
impl Capability for TxDuplex {
    // Available interrupt flags for a TX half-UART
    const FLAG_MASK: u8 = TX_FLAG_MASK;

    // There are no settable/clearable status flags for TX half-UARTs
    const STATUS_MASK: u16 = 0;
}
impl Transmit for TxDuplex {}

//=============================================================================
// Uart
//=============================================================================

/// Abstraction over a UART peripheral, allowing to perform UART transactions.
/// The second type parameter, `D`, denotes what the struct's [`Capability`] is.
///
/// * [`Rx`] or [`RxDuplex`]: Can perform receive transactions
/// * [`Tx`] or [`TxDuplex`]: Can perform transmit transactions
/// * [`Duplex`]: Can perform receive and transmit transactions. Additionally,
///   you can call [`split`](Uart::split) to return a `(Uart<C, RxDuplex>,
///   Uart<C, TxDuplex>)` tuple.
pub struct Uart<C, D>
where
    C: ValidConfig,
    D: Capability,
{
    config: C,
    capability: PhantomData<D>,
}

impl<C, D> Uart<C, D>
where
    C: ValidConfig,
    D: Capability,
{
    /// Obtain a pointer to the `DATA` register. Necessary for DMA transfers.
    #[cfg(feature = "dma")]
    #[inline]
    pub(crate) fn data_ptr(&self) -> *mut C::Word {
        self.config.as_ref().registers.data_ptr()
    }

    /// Helper method to remove the interrupt flags not pertinent to `Self`'s
    /// `Capability`
    #[inline]
    fn capability_flags(flags: Flags) -> Flags {
        flags & unsafe { Flags::from_bits_unchecked(D::FLAG_MASK) }
    }

    /// Helper method to remove the status flags not pertinent to `Self`'s
    /// `Capability`
    #[inline]
    fn capability_status(status: Status) -> Status {
        status & unsafe { Status::from_bits_unchecked(D::STATUS_MASK) }
    }

    /// Read the interrupt flags
    #[inline]
    pub fn read_flags(&self) -> Flags {
        let bits = self.config.as_ref().registers.read_flags();
        Flags::from_bits_truncate(bits)
    }

    /// Clear interrupt status flags
    ///
    /// Setting the `ERROR`, `RXBRK`, `CTSIC`, `RXS`, or `TXC` flag will clear
    /// the interrupts. This function has no effect on the `DRE` or
    /// `RXC` flags.
    ///
    /// Note that only the flags pertinent to `Self`'s [`Capability`]
    /// will be cleared. The other flags will be **SILENTLY IGNORED**.
    ///
    /// * Available flags for [`Receive`] capability: `RXC`, `RXS`, `RXBRK` and
    ///   `ERROR`
    /// * Available flags for [`Transmit`] capability: `DRE` and `TXC`.
    ///   **Note**: The `CTSIC` flag can only be cleared if a `CTS` Pad was
    ///   specified in the [`Config`] via the [`clear_ctsic`](Uart::clear_ctsic)
    ///   method.
    /// * Since [`Duplex`] [`Uart`]s are [`Receive`] + [`Transmit`] they have
    ///   all flags available.
    ///
    /// **Warning:** The implementation of of
    /// [`Write::flush`](embedded_hal::serial::Write::flush) waits on and
    /// clears the `TXC` flag. Manually clearing this flag could cause it to
    /// hang indefinitely.
    #[inline]
    pub fn clear_flags(&mut self, flags: Flags) {
        // Remove flags not pertinent to Self's Capability
        let bits = Self::capability_flags(flags).bits();
        self.config.as_mut().registers.clear_flags(bits);
    }

    /// Enable interrupts for the specified flags.
    ///
    /// Note that only the flags pertinent to `Self`'s [`Capability`]
    /// will be cleared. The other flags will be **SILENTLY IGNORED**.
    ///
    /// * Available flags for [`Receive`] capability: `RXC`, `RXS`, `RXBRK` and
    ///   `ERROR`
    /// * Available flags for [`Transmit`] capability: `DRE` and `TXC`.
    ///   **Note**: The `CTSIC` interrupt can only be enabled if a `CTS` Pad was
    ///   specified in the [`Config`] via the
    ///   [`enable_ctsic`](Uart::enable_ctsic) method.
    /// * Since [`Duplex`] [`Uart`]s are [`Receive`] + [`Transmit`] they have
    ///   all flags available.
    #[inline]
    pub fn enable_interrupts(&mut self, flags: Flags) {
        // Remove flags not pertinent to Self's Capability
        let bits = Self::capability_flags(flags).bits();
        self.config.as_mut().registers.enable_interrupts(bits);
    }

    /// Disable interrupts for the specified flags.
    ///
    /// Note that only the flags pertinent to `Self`'s [`Capability`]
    /// will be cleared. The other flags will be **SILENTLY IGNORED**    
    ///
    /// * Available flags for [`Receive`] capability: `RXC`, `RXS`, `RXBRK` and
    ///   `ERROR`
    /// * Available flags for [`Transmit`] capability: `DRE` and `TXC`.
    ///   **Note**: The `CTSIC` interrupt can only be disabled if a `CTS` Pad
    ///   was specified in the [`Config`] via the
    ///   [`disable_ctsic`](Uart::disable_ctsic) method.
    /// * Since [`Duplex`] [`Uart`]s are [`Receive`] + [`Transmit`] they have
    ///   all flags available.
    #[inline]
    pub fn disable_interrupts(&mut self, flags: Flags) {
        // Remove flags not pertinent to Self's Capability
        let bits = Self::capability_flags(flags).bits();
        self.config.as_mut().registers.disable_interrupts(bits);
    }

    /// Read the status flags
    #[inline]
    pub fn read_status(&self) -> Status {
        let bits = self.config.as_ref().registers.read_status();
        Status::from_bits_truncate(bits)
    }

    /// Clear the status flags
    ///
    /// Note that only the status flags pertinent to `Self`'s [`Capability`]
    /// will be cleared. The other stattus flags will be **SILENTLY IGNORED**.
    ///
    /// * Available status flags for [`Receive`] capability: `PERR`, `FERR`,
    ///   `BUFOVF`, `ISF` and `COLL`
    /// * [`Transmit`]-only [`Uart`]s have no clearable status flags.
    /// * Since [`Duplex`] [`Uart`]s are [`Receive`] + [`Transmit`] they have
    ///   all status flags available.
    #[inline]
    pub fn clear_status(&mut self, status: Status) {
        // Remove status flags not pertinent to Self's Capability
        let bits = Self::capability_status(status).bits();
        self.config.as_mut().registers.clear_status(bits);
    }

    #[inline]
    pub(super) fn _reconfigure<F>(&mut self, update: F)
    where
        F: FnOnce(&mut SpecificConfig<C>),
    {
        self.config.as_mut().registers.enable_peripheral(false);
        update(&mut self.config.as_mut());
        self.config.as_mut().registers.enable_peripheral(true);
    }
}

impl<C, D> Uart<C, D>
where
    C: ValidConfig,
    <C::Pads as PadSet>::Cts: SomePad,
    D: Transmit,
{
    /// Clear the `CTSIC` interrupt flag
    #[inline]
    pub fn clear_ctsic(&mut self) {
        let bit = CTSIC;
        self.config.as_mut().registers.clear_flags(bit);
    }

    /// Enable the `CTSIC` interrupt
    #[inline]
    pub fn enable_ctsic(&mut self) {
        let bit = CTSIC;
        self.config.as_mut().registers.enable_interrupts(bit);
    }

    /// Disable the `CTSIC` interrupt
    #[inline]
    pub fn disable_ctsic(&mut self) {
        let bit = CTSIC;
        self.config.as_mut().registers.disable_interrupts(bit);
    }
}

impl<C, D> Uart<C, D>
where
    C: ValidConfig,
    D: Simplex,
{
    /// Disable the UART peripheral and return the underlying [`Config`]
    #[inline]
    pub fn disable(self) -> C {
        let mut config = self.config;
        config.as_mut().registers.disable();
        config
    }

    /// Reconfigure the UART.
    ///
    /// Calling this method will temporarily disable the SERCOM peripheral, as
    /// some registers are enable-protected. This may interrupt any ongoing
    /// transactions.
    ///
    /// ```
    /// use atsamd_hal::sercom::v2::uart::{BaudMode, Oversampling, Uart};
    /// uart.reconfigure(|c| c.set_run_in_standby(false));
    /// ```
    #[inline]
    pub fn reconfigure<U>(&mut self, update: U)
    where
        U: FnOnce(&mut SpecificConfig<C>),
    {
        self._reconfigure(update);
    }
}

impl<C> Uart<C, Duplex>
where
    C: ValidConfig,
{
    /// Split the [`Uart`] into [`RxDuplex`] and [`TxDuplex`] halves
    #[inline]
    pub fn split(self) -> (Uart<C, RxDuplex>, Uart<C, TxDuplex>) {
        let config = unsafe { core::ptr::read(&self.config) };
        (
            Uart {
                config: self.config,
                capability: PhantomData,
            },
            Uart {
                config,
                capability: PhantomData,
            },
        )
    }

    /// Disable the UART peripheral and return the underlying [`Config`]
    #[inline]
    pub fn disable(self) -> C {
        let mut config = self.config;
        config.as_mut().registers.disable();
        config
    }

    /// Update the UART [`Config`]uration.
    ///
    /// Calling this method will temporarily disable the SERCOM peripheral, as
    /// some registers are enable-protected. This may interrupt any ongoing
    /// transactions.
    ///
    /// ```
    /// use atsamd_hal::sercom::v2::uart::{BaudMode, Oversampling, Uart};
    /// uart.reconfigure(|c| c.set_run_in_standby(false));
    /// ```
    #[inline]
    pub fn reconfigure<F>(&mut self, update: F)
    where
        F: FnOnce(&mut SpecificConfig<C>),
    {
        self._reconfigure(update);
    }

    /// Join [`RxDuplex`] and [`TxDuplex`] halves back into a full `Uart<C,
    /// Duplex>`
    pub fn join(rx: Uart<C, RxDuplex>, _tx: Uart<C, TxDuplex>) -> Self {
        Self {
            config: rx.config,
            capability: PhantomData,
        }
    }
}

impl<C: ValidConfig> AsMut<Uart<C, Duplex>> for (&mut Uart<C, RxDuplex>, &mut Uart<C, TxDuplex>) {
    #[inline]
    fn as_mut(&mut self) -> &mut Uart<C, Duplex> {
        // SAFETY: Pointer casting &mut Uart<C, RxDuplex> into &mut
        // Uart<C, Duplex> should be safe as long as RxDuplex and Duplex
        // both only have one nonzero-sized field.
        unsafe { &mut *(self.0 as *mut _ as *mut Uart<C, Duplex>) }
    }
}

impl<C, D> AsRef<SpecificConfig<C>> for Uart<C, D>
where
    C: ValidConfig,
    D: Capability,
{
    #[inline]
    fn as_ref(&self) -> &SpecificConfig<C> {
        self.config.as_ref()
    }
}

//=============================================================================
// Rx/Tx specific functionality
//=============================================================================

impl<C, D> Uart<C, D>
where
    C: ValidConfig,
    D: Receive,
    DataSize: AsPrimitive<C::Word>,
{
    /// Read from the DATA register
    ///
    /// # Safety
    ///
    /// Reading from the data register directly is `unsafe`, because it will
    /// clear the RXC flag, which could break assumptions made elsewhere in
    /// this module.
    #[inline]
    pub unsafe fn read_data(&mut self) -> DataSize {
        self.config.as_mut().registers.read_data()
    }

    /// Read the status register and convert into a [`Result`]
    /// containing the corresponding [`Flags`] or [`Error`]
    #[inline]
    fn read_flags_errors(&self) -> Result<Flags, Error> {
        self.read_status().try_into()?;
        Ok(self.read_flags())
    }

    /// Flush the RX buffer and clear RX errors
    #[inline]
    pub fn flush_rx_buffer(&mut self) {
        // TODO
        // The datasheet states that disabling the receiver (RXEN) clears
        // the RX buffer, and clears the BUFOVF, PERR and FERR bits.
        // However, in practice, it seems like BUFOVF errors still pop
        // up after a disable/enable cycle of the receiver, then immediately begin
        // reading bytes from the DATA register.
        //
        // Is this a hardware bug???
        /*
        usart.ctrlb.modify(|_, w| w.rxen().clear_bit());
        while usart.syncbusy.read().ctrlb().bit() || usart.ctrlb.read().rxen().bit_is_set() {}

        usart.ctrlb.modify(|_, w| w.rxen().set_bit());
        while usart.syncbusy.read().ctrlb().bit() || usart.ctrlb.read().rxen().bit_is_clear() {}
        */

        // Workaround is to read a few bytes to clear the RX buffer (3 bytes seems to do
        // the trick), then manually clear the BUFOVF bit Clear rx buffer
        for _ in 0..=2 {
            let _data = unsafe { self.config.as_mut().registers.read_data() };
        }

        // Clear all errors
        self.clear_status(
            Status::BUFOVF | Status::FERR | Status::PERR | Status::ISF | Status::COLL,
        );
    }
}

impl<C, D> Uart<C, D>
where
    C: ValidConfig,
    D: Transmit,
{
    /// Write to the DATA register
    ///
    /// # Safety
    ///
    /// Writing to the data register directly is `unsafe`, because it will clear
    /// the DRE flag, which could break assumptions made elsewhere in this
    /// module.
    #[inline]
    pub unsafe fn write_data(&mut self, data: DataSize) {
        self.config.as_mut().registers.write_data(data);
    }
}
