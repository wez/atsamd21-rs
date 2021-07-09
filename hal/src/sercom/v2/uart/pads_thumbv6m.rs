//! UART pad definitions for thumbv6m targets

use core::marker::PhantomData;

use super::{AnyConfig, Capability, CharSize, Config, Duplex, Rx, Tx};
use crate::sercom::v2::*;
use crate::typelevel::{NoneT, Sealed};

#[cfg(not(feature = "samd11"))]
use crate::gpio::v2::AnyPin;

//=============================================================================
// RxpoTxpo
//=============================================================================

/// Configure the `RXPO` and `TXPO` fields based on a set of [`Pads`]
///
/// According to the datasheet, the `RXPO` and `TXPO` values specify which
/// SERCOM pads are used for various functions. Moreover, depending on which
/// pads are actually in use, only certain combinations of these values make
/// sense and are valid.
///
/// This trait is implemented for valid, four-tuple combinations of
/// [`OptionalPadNum`]s. Those implementations are then lifted to the
/// corresponding [`Pads`] types.
///
/// To satisfy this trait, the combination of [`OptionalPadNum`]s must specify
/// [`PadNum`] for at least one of `RX` and `TX`. Furthermore, no
/// two [`PadNum`]s can conflict.
pub trait RxpoTxpo {
    /// `RXPO` field value
    const RXPO: u8;

    /// `RXPO` field value
    const TXPO: u8;
}

/// Lift the implementations of [`RxpoTxpo`] from four-tuples of
/// [`OptionalPadNum`]s to the corresponding [`Pads`] types.
impl<S, RX, TX, RTS, CTS> RxpoTxpo for Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
    (RX::PadNum, TX::PadNum, RTS::PadNum, CTS::PadNum): RxpoTxpo,
{
    const RXPO: u8 = <(RX::PadNum, TX::PadNum, RTS::PadNum, CTS::PadNum)>::RXPO;
    const TXPO: u8 = <(RX::PadNum, TX::PadNum, RTS::PadNum, CTS::PadNum)>::TXPO;
}

//=============================================================================
// Implement RxpoTxpo
//=============================================================================

/// Filter [`PadNum`] permutations and implement [`RxpoTxpo`]
    #[rustfmt::skip]
    macro_rules! impl_rxpotxpo {
	// This is the entry pattern. Start by checking RTS and CTS.
	($RX:ident, $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@check_rts_cts, $RX, $TX, $RTS, $CTS); };
    
	// Check whether RTS and CTS form a valid pair.
	// They both must be the correct pad or absent.
	(@check_rts_cts, $RX:ident, $TX:ident, NoneT, NoneT) => { impl_rxpotxpo!(@assign_rxpo, $RX, $TX, NoneT, NoneT); };
	(@check_rts_cts, $RX:ident, $TX:ident, Pad2, NoneT) => { impl_rxpotxpo!(@assign_rxpo, $RX, $TX, Pad2, NoneT); };
	(@check_rts_cts, $RX:ident, $TX:ident, NoneT, Pad3) => { impl_rxpotxpo!(@assign_rxpo, $RX, $TX, NoneT, Pad3); };
	(@check_rts_cts, $RX:ident, $TX:ident, Pad2, Pad3) => { impl_rxpotxpo!(@assign_rxpo, $RX, $TX, Pad2, Pad3); };
    
	// If RTS and CTS are not valid, fall through to this pattern.
	(@check_rts_cts, $RX:ident, $TX:ident, $RTS:ident, $CTS:ident) => { };
    
	// Assign RXPO based on RX.
	// Our options are exhaustive, so no fall through pattern is needed.
	(@assign_rxpo, NoneT, $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@assign_txpo, NoneT, $TX, $RTS, $CTS, 0); };
	(@assign_rxpo, Pad0,  $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@assign_txpo, Pad0,  $TX, $RTS, $CTS, 0); };
	(@assign_rxpo, Pad1,  $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@assign_txpo, Pad1,  $TX, $RTS, $CTS, 1); };
	(@assign_rxpo, Pad2,  $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@assign_txpo, Pad2,  $TX, $RTS, $CTS, 2); };
	(@assign_rxpo, Pad3,  $TX:ident, $RTS:ident, $CTS:ident) => { impl_rxpotxpo!(@assign_txpo, Pad3,  $TX, $RTS, $CTS, 3); };
    
	// Assign TXPO based on RX and RTS
	(@assign_txpo, $RX:ident, NoneT, NoneT, $CTS:ident, $RXPO:literal) => { impl_rxpotxpo!(@filter_conflicts, $RX, NoneT, NoneT, $CTS, $RXPO, 0); };
	(@assign_txpo, $RX:ident, NoneT, Pad2, $CTS:ident, $RXPO:literal) => { impl_rxpotxpo!(@filter_conflicts, $RX, NoneT, Pad2, $CTS, $RXPO, 2); };
	(@assign_txpo, $RX:ident, Pad0, NoneT, $CTS:ident, $RXPO:literal) => { impl_rxpotxpo!(@filter_conflicts, $RX, Pad0, NoneT, $CTS, $RXPO, 0); };
	(@assign_txpo, $RX:ident, Pad2, NoneT, $CTS:ident, $RXPO:literal) => { impl_rxpotxpo!(@filter_conflicts, $RX, Pad2, NoneT, $CTS, $RXPO, 1); };
	(@assign_txpo, $RX:ident, Pad0, Pad2, $CTS:ident, $RXPO:literal) => { impl_rxpotxpo!(@filter_conflicts, $RX, Pad0, Pad2, $CTS, $RXPO, 2); };
    
	// If TX is not valid, fall through to this pattern.
	(@assign_txpo, $RX:ident, $TX:ident, $RTS:ident, $CTS:ident, $RXPO:literal) => { };
    
	// Filter any remaining permutations that conflict.
	(@filter_conflicts, NoneT, NoneT, $RTS:ident, $CTS:ident, $RXPO:literal, $TXPO:literal) => { };
	(@filter_conflicts, Pad0, Pad0, $RTS:ident, $CTS:ident, $RXPO:literal, $TXPO:literal) => { };
	(@filter_conflicts, Pad2, Pad2, $RTS:ident, $CTS:ident, $RXPO:literal, $TXPO:literal) => { };
	(@filter_conflicts, Pad2, $TX:ident, Pad2, $CTS:ident, $RXPO:literal, $TXPO:literal) => { };
	(@filter_conflicts, Pad3, $TX:ident, $RTS:ident, Pad3, $RXPO:literal, $TXPO:literal) => { };
	(@filter_conflicts, $RX:ident, Pad2, Pad2, $CTS:ident, $RXPO:literal, $TXPO:literal) => { };
    
	// If there are no conflicts, fall through to this pattern and implement RxpoTxpo
	(@filter_conflicts, $RX:ident, $TX:ident, $RTS:ident, $CTS:ident, $RXPO:literal, $TXPO:literal) => {
	    impl RxpoTxpo for ($RX, $TX, $RTS, $CTS) {
		const RXPO: u8 = $RXPO;
		const TXPO: u8 = $TXPO;
	    }
	};
    }

/// Try to implement [`RxpoTxpo`] on all possible 4-tuple permutations of
/// [`OptionalPadNum`]s.
///
/// The leading `()` token tree stores a growing permutation of [`PadNum`]s.
/// When it reaches four [`PadNum`]s, try to implement [`RxpoTxpo`].
///
/// The next `[]` token tree is a list of possible [`PadNum`]s to append to the
/// growing permutation. Loop through this list and append each option to the
/// permutation.
///
/// The final, optional `[]` token tree exists to temporarily store the entire
/// list before pushing it down for the next permutation element.
macro_rules! padnum_permutations {
	// If we have built up four [`PadNum`]s, try to implement [`RxpoTxpo`].
	// Ignore the remaining list of [`PadNum`]s.
	(
	    ( $RX:ident $TX:ident $RTS:ident $CTS:ident ) [ $( $Pads:ident )* ]
	) => {
	    impl_rxpotxpo!($RX, $TX, $RTS, $CTS);
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

/// Container for a set of SERCOM [`Pad`]s
///
/// See the [module-level](crate::sercom::v2::uart) documentation for more
/// details on specifying a `Pads` type and creating instances.
pub struct Pads<S, RX = NoneT, TX = NoneT, RTS = NoneT, CTS = NoneT>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    sercom: PhantomData<S>,
    receive: RX,
    transmit: TX,
    ready_to_send: RTS,
    clear_to_send: CTS,
}

impl<S: Sercom> Default for Pads<S> {
    fn default() -> Self {
        Self {
            sercom: PhantomData,
            receive: NoneT,
            transmit: NoneT,
            ready_to_send: NoneT,
            clear_to_send: NoneT,
        }
    }
}

impl<S, RX, TX, RTS, CTS> Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
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

#[cfg(feature = "samd11")]
impl<S, RX, TX, RTS, CTS> Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    /// Set the `RX` [`Pad`]
    #[inline]
    pub fn rx<P: IsPad>(self, pin: P) -> Pads<S, P, TX, RTS, CTS> {
        Pads {
            sercom: self.sercom,
            receive: pin,
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `TX` [`Pad`]
    #[inline]
    pub fn tx<P: IsPad>(self, pin: P) -> Pads<S, RX, P, RTS, CTS> {
        Pads {
            sercom: self.sercom,
            receive: self.receive,
            transmit: pin,
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `RTS` [`Pad`]
    #[inline]
    pub fn rts<P: IsPad>(self, pin: P) -> Pads<S, RX, TX, P, CTS> {
        Pads {
            sercom: self.sercom,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: pin,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `CTS` [`Pad`]
    #[inline]
    pub fn cts<P: IsPad>(self, pin: P) -> Pads<S, RX, TX, RTS, P> {
        Pads {
            sercom: self.sercom,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: pin,
        }
    }
}

#[cfg(feature = "samd21")]
impl<S, RX, TX, RTS, CTS> Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    /// Set the `RX` [`Pad`]
    #[inline]
    pub fn rx<I>(self, pin: impl AnyPin<Id = I>) -> Pads<S, Pad<S, I>, TX, RTS, CTS>
    where
        I: GetPad<S>,
        Pad<S, I>: IsPad,
    {
        Pads {
            sercom: self.sercom,
            receive: pin.into().into_mode(),
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `TX` [`Pad`]
    #[inline]
    pub fn tx<I>(self, pin: impl AnyPin<Id = I>) -> Pads<S, RX, Pad<S, I>, RTS, CTS>
    where
        I: GetPad<S>,
        Pad<S, I>: IsPad,
    {
        Pads {
            sercom: self.sercom,
            receive: self.receive,
            transmit: pin.into().into_mode(),
            ready_to_send: self.ready_to_send,
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `RTS` [`Pad`]
    #[inline]
    pub fn rts<I>(self, pin: impl AnyPin<Id = I>) -> Pads<S, RX, TX, Pad<S, I>, CTS>
    where
        I: GetPad<S>,
        Pad<S, I>: IsPad,
    {
        Pads {
            sercom: self.sercom,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: pin.into().into_mode(),
            clear_to_send: self.clear_to_send,
        }
    }

    /// Set the `CTS` [`Pad`]
    #[inline]
    pub fn cts<I>(self, pin: impl AnyPin<Id = I>) -> Pads<S, RX, TX, RTS, Pad<S, I>>
    where
        I: GetPad<S>,
        Pad<S, I>: IsPad,
    {
        Pads {
            sercom: self.sercom,
            receive: self.receive,
            transmit: self.transmit,
            ready_to_send: self.ready_to_send,
            clear_to_send: pin.into().into_mode(),
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
/// parameters of [`Pads`], i.e. `RX`, `TX`, `RTS` & `CTS`. Each of the
/// remaining type parameters defaults to [`NoneT`].
///
/// ```
/// use atsamd_hal::pac::Peripherals;
/// use atsamd_hal::gpio::v2::{PA08, PA09, Pins};
/// use atsamd_hal::sercom::v2::{Sercom0, uart};
/// use atsamd_hal::typelevel::NoneT;
///
/// pub type Pads = uart::PadsFromIds<Sercom0, PA08, PA09>;
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

#[cfg(feature = "samd21")]
pub type PadsFromIds<S, RX = NoneT, TX = NoneT, RTS = NoneT, CTS = NoneT> = Pads<
    S,
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
/// corresponding [`Sercom`] and [`OptionalPad`] types. It serves to cut down on
/// the total number of type parameters needed in the [`Config`] struct. The
/// [`Config`] struct doesn't need access to the [`Pin`]s directly.  Rather, it
/// only needs to apply the [`SomePad`] trait bound when a `Pin` is required.
/// The [`PadSet`] trait allows each [`Config`] struct to store an instance of
/// [`Pads`] without itself being generic over all six type parameters of the
/// [`Pads`] type.
///
/// [`Pin`]: crate::gpio::v2::Pin
/// [type-level function]: crate::typelevel#type-level-functions
/// [`Config`]: crate::sercom::v2::uart::Config
pub trait PadSet: Sealed {
    type Sercom: Sercom;
    type Rx: OptionalPad;
    type Tx: OptionalPad;
    type Rts: OptionalPad;
    type Cts: OptionalPad;
}

impl<S, RX, TX, RTS, CTS> Sealed for Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
}

impl<S, RX, TX, RTS, CTS> PadSet for Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: OptionalPad,
    TX: OptionalPad,
    RTS: OptionalPad,
    CTS: OptionalPad,
{
    type Sercom = S;
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
/// This trait labels sets of [`Pads`] that satisfy the [`RxpoTxpo`] trait. It
/// guarantees to the [`Config`] struct that this set of `Pads` can be
/// configured through that trait.
///
/// [`Config`]: crate::sercom::v2::uart::Config
pub trait ValidPads: PadSet + RxpoTxpo {
    type Capability: Capability;
}

impl<S, RX, RTS> ValidPads for Pads<S, RX, NoneT, RTS, NoneT>
where
    S: Sercom,
    RX: SomePad,
    RTS: OptionalPad,
    Self: PadSet + RxpoTxpo,
{
    type Capability = Rx;
}

impl<S, TX, CTS> ValidPads for Pads<S, NoneT, TX, NoneT, CTS>
where
    S: Sercom,
    TX: SomePad,
    CTS: OptionalPad,
    Self: PadSet + RxpoTxpo,
{
    type Capability = Tx;
}

impl<S, RX, TX, RTS, CTS> ValidPads for Pads<S, RX, TX, RTS, CTS>
where
    S: Sercom,
    RX: SomePad,
    TX: SomePad,
    RTS: OptionalPad,
    CTS: OptionalPad,
    Self: PadSet + RxpoTxpo,
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
