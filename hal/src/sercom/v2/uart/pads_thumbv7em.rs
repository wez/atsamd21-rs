//! UART pad definitions for thumbv7em targets

use core::marker::PhantomData;

use crate::target_device::sercom0::usart_int::ctrla::{RXPO_A, TXPO_A};

use super::{AnyConfig, Capability, CharSize, Config, Duplex, Rx, Tx};
use crate::sercom::v2::*;
use crate::typelevel::{NoneT, Sealed};

use crate::gpio::v2::AnyPin;

//=============================================================================
// Rxpo
//=============================================================================

/// Control the `RXPO` field as a function of the [`PadNum`] type
pub trait Rxpo: Sealed {
    /// Corresponding variant from the PAC `enum`
    const RXPO: RXPO_A;
}

impl Rxpo for Pad0 {
    const RXPO: RXPO_A = RXPO_A::PAD0;
}
impl Rxpo for Pad1 {
    const RXPO: RXPO_A = RXPO_A::PAD1;
}
impl Rxpo for Pad2 {
    const RXPO: RXPO_A = RXPO_A::PAD2;
}
impl Rxpo for Pad3 {
    const RXPO: RXPO_A = RXPO_A::PAD3;
}

impl Rxpo for NoneT {
    /// This value is arbitrary and meaningless for [`NoneT`]
    const RXPO: RXPO_A = RXPO_A::PAD0;
}

/// Lift the implementations of [`Rxpo`] from [`OptionalPadNum`]s to the
/// corresponding [`Pads`] types.
impl<S, I, RX, TX, RTS, CTS> Rxpo for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
    RX::PadNum: Rxpo,
{
    const RXPO: RXPO_A = RX::PadNum::RXPO;
}

//=============================================================================
// Txpo
//=============================================================================

/// Control the `TXPO` field as a function of the [`PadNum`] type
pub trait Txpo: Sealed {
    /// Corresponding variant from the PAC `enum`
    const TXPO: TXPO_A;
}

impl Txpo for Pad0 {
    const TXPO: TXPO_A = TXPO_A::TXPO_2;
}

impl Txpo for NoneT {
    /// This value is arbitrary and meaningless for [`NoneT`]
    const TXPO: TXPO_A = TXPO_A::TXPO_2;
}

/// Lift the implementations of [`Txpo`] from [`OptionalPadNum`]s to the
/// corresponding [`Pads`] types.
impl<S, I, RX, TX, RTS, CTS> Txpo for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
    TX::PadNum: Txpo,
{
    const TXPO: TXPO_A = TX::PadNum::TXPO;
}

//=============================================================================
// Pads
//=============================================================================

/// Container for a set of SERCOM [`Pad`]s
///
/// See the [module-level](crate::sercom::v2::uart) documentation for more
/// details on specifying a `Pads` type and creating instances.
pub struct Pads<S, I, RX = NoneT, TX = NoneT, RTS = NoneT, CTS = NoneT>
where
    S: Sercom,
    I: IoSet,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    sercom: PhantomData<S>,
    ioset: PhantomData<I>,
    receive: RX,
    transmit: TX,
    ready_to_send: RTS,
    clear_to_send: CTS,
}

impl<S: Sercom, I: IoSet> Default for Pads<S, I> {
    fn default() -> Self {
        Self {
            sercom: PhantomData,
            ioset: PhantomData,
            receive: NoneT,
            transmit: NoneT,
            ready_to_send: NoneT,
            clear_to_send: NoneT,
        }
    }
}

impl<S, I, RX, TX, RTS, CTS> Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    /// Set the `RX` [`Pad`]
    #[inline]
    pub fn rx<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, Pad<S, Id>, TX, RTS, CTS>
    where
        Id: GetPad<S>,
        Id::PadNum: Rxpo,
        Pad<S, Id>: InIoSet<I>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            receive: pin.into().into_mode(),
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `TX` [`Pad`]
    #[inline]
    pub fn tx<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, RX, Pad<S, Id>, RTS, CTS>
    where
        Id: GetPad<S>,
        Id::PadNum: Txpo,
        Pad<S, Id>: InIoSet<I>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            receive: self.receive,
            transmit: pin.into().into_mode(),
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `RTS` [`Pad`], which is always [`Pad2`]
    #[inline]
    pub fn rts<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, RX, TX, Pad<S, Id>, CTS>
    where
        Id: GetPad<S>,
        Pad<S, Id>: InIoSet<I>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: pin.into().into_mode(),
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `CTS` [`Pad`], which is always [`Pad3`]
    #[inline]
    pub fn cts<Id>(self, pin: impl AnyPin<Id = Id>) -> Pads<S, I, RX, TX, RTS, Pad<S, Id>>
    where
        Id: GetPad<S>,
        Pad<S, Id>: InIoSet<I>,
    {
        Pads {
            sercom: self.sercom,
            ioset: self.ioset,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: pin.into().into_mode(),
        }
    }

    /// Consume the [`Pads`] and return each individual [`Pad`]
    #[inline]
    pub fn free(self) -> (RX, TX, RTS, CTS) {
        (
            self.receive,
            self.transmit,
            self.ready_to_send,
            self.clear_to_send,
        )
    }
}

/// Define a set of [`Pads`] using [`PinId`]s instead of [`Pin`]s
///
/// In some cases, it is more convenient to specify a set of `Pads` using
/// `PinId`s rather than `Pin`s. This alias makes it easier to do so.
///
/// The first two type parameters are the [`Sercom`] and [`IoSet`], while the
/// remaining four are effectively [`OptionalPinId`]s representing the
/// corresponding type parameters of [`Pads`], i.e. `RX`, `TX`, `RTS` & `CTS`.
/// Each of the remaining type parameters defaults to [`NoneT`].
///
/// ```
/// use atsamd_hal::pac::Peripherals;
/// use atsamd_hal::gpio::v2::{PA08, PA09, Pins};
/// use atsamd_hal::sercom::v2::{Sercom0, uart};
/// use atsamd_hal::sercom::v2::pad::IoSet1;
/// use atsamd_hal::typelevel::NoneT;
///
/// pub type Pads = uart::PadsFromIds<Sercom0, IoSet1, PA09T, PA08>;
///
/// pub fn create_pads() -> Pads {
///     let peripherals = Peripherals::take().unwrap();
///     let pins = Pins::new(peripherals.PORT);
///     uart::Pads::default().rx(pins.pa09).tx(pins.pa08)
/// }
/// ```
///
/// [`Pin`]: crate::gpio::v2::Pin
/// [`PinId`]: crate::gpio::v2::PinId
/// [`OptionalPinId`]: crate::gpio::v2::OptionalPinId
pub type PadsFromIds<S, I, RX = NoneT, TX = NoneT, RTS = NoneT, CTS = NoneT> = Pads<
    S,
    I,
    <RX as GetOptionalPad<S>>::Pad,
    <TX as GetOptionalPad<S>>::Pad,
    <RTS as GetOptionalPad<S>>::Pad,
    <CTS as GetOptionalPad<S>>::Pad,
>;

//=============================================================================
// PadSet
//=============================================================================

/// Type-level function to recover the [`OptionalPad`] types from a generic set
/// of [`Pads`]
///
/// This trait is used as an interface between the [`Pads`] type and other
/// types in this module. It acts as a [type-level function], returning the
/// corresponding [`Sercom`], and [`OptionalPad`] types. It serves to
/// cut down on the total number of type parameters needed in the [`Config`]
/// struct. The [`Config`] struct doesn't need access to the [`Pad`]s directly.
/// Rather, it only needs to apply the [`SomePad`] trait bound when a `Pin` is
/// required. The [`PadSet`] trait allows each [`Config`] struct to store an
/// instance of [`Pads`] without itself being generic over all six type
/// parameters of the [`Pads`] type.
///
/// [`Pin`]: crate::gpio::v2::Pin
/// [`Config`]: crate::sercom::v2::uart::Config
/// [type-level function]: crate::typelevel#type-level-functions
pub trait PadSet: Sealed {
    type Sercom: Sercom;
    type IoSet: IoSet;
    type Rx: OptionalPad;
    type Tx: OptionalPad;
    type Rts: OptionalPad;
    type Cts: OptionalPad;
}

impl<S, I, RX, TX, RTS, CTS> Sealed for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
}

impl<S, I, RX, TX, RTS, CTS> PadSet for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    type Sercom = S;
    type IoSet = I;
    type Rx = RX;
    type Tx = TX;
    type Rts = RTS;
    type Cts = CTS;
}

//=============================================================================
// ValidPads
//=============================================================================

/// Marker trait for valid sets of [`Pads`]
///
/// This trait labels sets of [`Pads`] that satisfy the [`Rxpo`] and [`Txpo`]
/// traits. It guarantees to the [`Config`] struct that this set of `Pads` can
/// be configured through those traits.
///
/// [`Config`]: crate::sercom::v2::uart::Config
pub trait ValidPads: PadSet + Rxpo + Txpo {
    type Capability: Capability;
}

impl<S, I, RX, RTS> ValidPads for Pads<S, I, RX, NoneT, RTS, NoneT>
where
    S: Sercom,
    I: IoSet,
    RX: SomePad,
    RTS: OptionalPad,
    Self: PadSet + Rxpo + Txpo,
{
    type Capability = Rx;
}

impl<S, I, TX, CTS> ValidPads for Pads<S, I, NoneT, TX, NoneT, CTS>
where
    S: Sercom,
    I: IoSet,
    TX: SomePad,
    CTS: OptionalPad,
    Self: PadSet + Rxpo + Txpo,
{
    type Capability = Tx;
}

impl<S, I, RX, TX, RTS, CTS> ValidPads for Pads<S, I, RX, TX, RTS, CTS>
where
    S: Sercom,
    I: IoSet,
    RX: SomePad,
    TX: SomePad,
    RTS: OptionalPad,
    CTS: OptionalPad,
    Self: PadSet + Rxpo + Txpo,
{
    type Capability = Duplex;
}

//=============================================================================
// ValidConfig
//=============================================================================

/// Marker trait for valid UART [`Config`]urations
///
/// A functional UART peripheral must have, at a minimum either a Rx or a Tx
/// [`Pad`].
pub trait ValidConfig: AnyConfig {}

impl<P: ValidPads, C: CharSize> ValidConfig for Config<P, C> {}
