//! TODO

use core::marker::PhantomData;

use paste::paste;

use crate::pac;
use crate::pac::gclk::pchctrl::GEN_A;

use crate::sercom::*;
use crate::time::Hertz;
use crate::typelevel::Sealed;

use super::gclk::{AnyGclk, GenNum};
use super::sources::dpll::{Pll0, Pll1};

//==============================================================================
// Registers
//==============================================================================

/// TODO
struct Registers<P: PclkType> {
    pclk: PhantomData<P>,
}

impl<P: PclkType> Registers<P> {
    /// TODO
    #[inline]
    unsafe fn new() -> Self {
        Registers { pclk: PhantomData }
    }

    #[inline]
    fn gclk(&self) -> *const pac::gclk::RegisterBlock {
        pac::GCLK::ptr()
    }

    #[inline]
    fn gclk_mut(&mut self) -> *mut pac::gclk::RegisterBlock {
        self.gclk() as *mut _
    }

    /// TODO
    #[inline]
    fn pchctrl(&mut self) -> &mut pac::gclk::PCHCTRL {
        let gclk = self.gclk_mut();
        // TODO
        unsafe { &mut (*gclk).pchctrl[P::ID as usize] }
    }

    /// TODO
    #[inline]
    fn set_source(&mut self, variant: GEN_A) {
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
// TODO
//==============================================================================

/// TODO
pub struct PclkToken<P>
where
    P: PclkType,
{
    regs: Registers<P>,
}

impl<P> PclkToken<P>
where
    P: PclkType,
{
    /// TODO
    #[inline]
    pub(super) unsafe fn new() -> Self {
        PclkToken {
            regs: Registers::new(),
        }
    }

    /// TODO
    #[inline]
    pub fn enable<G: AnyGclk>(mut self, gclk: G) -> (Pclk<P, G::GenNum>, G::Lock) {
        self.regs.set_source(G::GenNum::GCLK);
        self.regs.enable();
        // TODO
        (Pclk::new(self, gclk.freq()), unsafe { gclk.lock() })
    }
}

//==============================================================================
// TODO
//==============================================================================

pub struct Pclk<P, G>
where
    P: PclkType,
    G: GenNum,
{
    token: PclkToken<P>,
    gclk: PhantomData<G>,
    freq: Hertz,
}

impl<P, G> Pclk<P, G>
where
    P: PclkType,
    G: GenNum,
{
    pub(super) fn new(token: PclkToken<P>, freq: Hertz) -> Self {
        Pclk {
            token,
            gclk: PhantomData,
            freq,
        }
    }

    /// Disable the peripheral channel clock
    #[inline]
    pub fn disable<H>(mut self, gclk: H) -> (PclkToken<P>, H::Unlock)
    where
        H: AnyGclk<GenNum = G>,
    {
        self.token.regs.disable();
        // TODO
        (self.token, unsafe { gclk.unlock() })
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
