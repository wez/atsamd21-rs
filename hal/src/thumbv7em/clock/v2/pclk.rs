//! TODO

use core::marker::PhantomData;

use paste::paste;
use seq_macro::seq;

use crate::pac;

pub use crate::pac::gclk::pchctrl::GEN_A as PclkSourceEnum;

use crate::clock::v2::{Source, SourceMarker};
use crate::sercom::*;
use crate::time::Hertz;
use crate::typelevel::counted::Counted;
use crate::typelevel::{Counter, Decrement, Increment, Sealed};

use super::gclk::*;
use super::sources::dpll::{Pll0, Pll1};

//==============================================================================
// Registers
//==============================================================================

pub type PclkToken<P> = Registers<P>;

/// TODO
pub struct Registers<P: PclkType> {
    pclk: PhantomData<P>,
}

impl<P: PclkType> Registers<P> {
    /// TODO
    #[inline]
    unsafe fn new() -> Self {
        Registers { pclk: PhantomData }
    }

    #[inline]
    fn gclk(&self) -> &pac::gclk::RegisterBlock {
        unsafe { &*pac::GCLK::ptr() }
    }

    /// TODO
    #[inline]
    fn pchctrl(&self) -> &pac::gclk::PCHCTRL {
        &self.gclk().pchctrl[P::ID as usize]
    }

    /// TODO
    #[inline]
    fn set_source(&mut self, variant: PclkSourceEnum) {
        self.pchctrl().modify(|_, w| w.gen().variant(variant));
    }

    /// TODO
    #[inline]
    fn enable(&mut self) {
        self.pchctrl().modify(|_, w| w.chen().set_bit());
    }

    /// TODO
    #[inline]
    fn disable(&mut self) {
        self.pchctrl().modify(|_, w| w.chen().clear_bit());
    }
}

//==============================================================================
// TODO
//==============================================================================

/// Type-level `enum` for the 48 peripheral clock variants
pub trait PclkType: Sealed {
    const ID: PclkId;
}

// If a suitable type already exists in the HAL, reuse it for the `PclkTypeType`
// The `Sercom` types are a good example.
macro_rules! pclk_type {
    // A type already exists; reuse it
    ( true, $Type:ident, $Id:ident ) => {
        impl PclkType for $Type {
            const ID: PclkId = PclkId::$Id;
        }
    };
    // A type does not exist yet; create one
    ( false, $Type:ident, $Id:ident ) => {
        /// [`PclkType`] variant TODO
        pub enum $Type {}
        impl Sealed for $Type {}
        impl PclkType for $Type {
            const ID: PclkId = PclkId::$Id;
        }
    };
}

//==============================================================================
// PclkSource
//==============================================================================

/// TODO
pub trait PclkSourceType: GenNum + SourceMarker {
    const PCLK_SRC: PclkSourceEnum;
}

seq!(N in 0..=11 {
    impl PclkSourceType for Gen#N {
        const PCLK_SRC: PclkSourceEnum = PclkSourceEnum::GCLK#N;
    }
});

/// TODO
pub trait PclkSource: Source {
    type Type: PclkSourceType;
}

impl<G, T, N> PclkSource for Counted<Gclk<G, T>, N>
where
    G: PclkSourceType,
    T: GclkSourceType,
    N: Counter,
{
    type Type = G;
}

//==============================================================================
// TODO
//==============================================================================

pub struct Pclk<P, T>
where
    P: PclkType,
    T: PclkSourceType,
{
    token: PclkToken<P>,
    src: PhantomData<T>,
    freq: Hertz,
}

impl<P, T> Pclk<P, T>
where
    P: PclkType,
    T: PclkSourceType,
{
    /// TODO
    #[inline]
    pub(super) unsafe fn create(freq: Hertz) -> Self {
        let token = PclkToken::new();
        let src = PhantomData;
        Self { token, src, freq }
    }

    /// TODO
    #[inline]
    pub fn new<S>(mut token: PclkToken<P>, gclk: S) -> (Self, S::Inc)
    where
        S: PclkSource<Type = T> + Increment,
    {
        token.set_source(T::PCLK_SRC);
        token.enable();
        let freq = gclk.freq();
        let pclk = Pclk {
            token,
            src: PhantomData,
            freq,
        };
        (pclk, gclk.inc())
    }

    /// Disable the peripheral channel clock
    #[inline]
    pub fn disable<S>(mut self, gclk: S) -> (PclkToken<P>, S::Dec)
    where
        S: PclkSource<Type = T> + Decrement,
    {
        self.token.disable();
        (self.token, gclk.dec())
    }

    //#[inline]
    //pub unsafe fn copy(&self) -> Self {
    //    EnabledChan { chan: PclkType::new(self.chan.freq, self.chan.div) }
    //}

    #[inline]
    pub fn freq(&self) -> Hertz {
        self.freq
    }
}

impl<P, T> Sealed for Pclk<P, T>
where
    P: PclkType,
    T: PclkSourceType,
{
}

//==============================================================================
// TODO
//==============================================================================

macro_rules! pclks {
    (
        $(
            $( #[$cfg:meta] )?
            ($exists:literal, $Type:ident, $Id:ident)
        ),+
    ) =>
    {
        paste! {
            /// Internal `enum` used to index the correct peripheral channel
            /// register
            pub enum PclkId {
                $(
                    $( #[$cfg] )?
                    [<$Id:camel>],
                )+
            }

            $(
                $( #[$cfg] )?
                pclk_type!( $exists, [<$Type:camel>], [<$Id:camel>] );
            )+

            /// Struct containing all possible peripheral clocks.
            pub struct Tokens {
                $(
                    $( #[$cfg] )?
                    pub [<$Id:lower>]: PclkToken<[<$Type:camel>]>,
                )+
            }

            impl Tokens {
                pub(super) fn new() -> Self {
                    Tokens {
                        $(
                            // TODO
                            $( #[$cfg] )?
                            [<$Id:lower>]: unsafe { PclkToken::new() },
                        )+
                    }
                }
            }
        }
    };
}

// Try to use existing types as tokens, if possible. Otherwise, create new ones.
pclks!(
    (false, dfll48, dfll48),
    (true, Pll0, dpll0),
    (true, Pll1, dpll1),
    (false, slow_32k, slow_32k),
    (false, eic, eic),
    (false, freqm_msr, freqm_msr),
    (false, freqm_ref, freqm_ref),
    (true, sercom0, sercom0),
    (true, sercom1, sercom1),
    (false, tc0_tc1, tc0_tc1),
    (false, usb, usb),
    (false, evsys0, evsys0),
    (false, evsys1, evsys1),
    (false, evsys2, evsys2),
    (false, evsys3, evsys3),
    (false, evsys4, evsys4),
    (false, evsys5, evsys5),
    (false, evsys6, evsys6),
    (false, evsys7, evsys7),
    (false, evsys8, evsys8),
    (false, evsys9, evsys9),
    (false, evsys10, evsys10),
    (false, evsys11, evsys11),
    (true, sercom2, sercom2),
    (true, sercom3, sercom3),
    (false, tcc0_tcc1, tcc0_tcc1),
    (false, tc2_tc3, tc2_tc3),
    (false, can0, can0),
    (false, can1, can1),
    (false, tcc2_tcc3, tcc2_tcc3),
    (false, tc4_tc5, tc4_tc5),
    (false, pdec, pdec),
    (false, ac, ac),
    (false, ccl, ccl),
    (true, sercom4, sercom4),
    (true, sercom5, sercom5),
    #[cfg(feature = "min-samd51n")]
    (true, sercom6, sercom6),
    #[cfg(feature = "min-samd51n")]
    (true, sercom7, sercom7),
    (false, tcc4, tcc4),
    (false, tc6_tc7, tc6_tc7),
    (false, adc0, adc0),
    (false, adc1, adc1),
    (false, dac, dac),
    (false, i2s0, i2s0),
    (false, i2s1, i2s1),
    (false, sdhc0, sdhc0),
    (false, sdhc1, sdhc1),
    (false, cm4_trace, cm4_trace)
);
