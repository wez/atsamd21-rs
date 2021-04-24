use core::marker::PhantomData;

use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::oscctrl::xoscctrl::STARTUP_A;
use crate::pac::oscctrl::{RegisterBlock, XOSCCTRL};

use crate::gpio::v2::{AnyPin, FloatingDisabled, OptionalPin, Pin, PinId, PA14, PA15, PB22, PB23};
use crate::time::Hertz;
use crate::typelevel::{NoneT, Sealed};

use super::super::gclk::{GenNum, GclkSource, GclkSourceType};
use super::dpll::{DpllSrc, DpllSource, DpllSourceType};

//==============================================================================
// XOscNum
//==============================================================================

/// TODO
pub trait XOscNum: Sealed {
    const NUM: usize;
    const DPLL_SRC: DpllSrc;
    type XIn: PinId;
    type XOut: PinId;
}

/// TODO
pub enum Osc0 {}

impl Sealed for Osc0 {}

impl XOscNum for Osc0 {
    const NUM: usize = 0;
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC0;
    type XIn = PA14;
    type XOut = PA15;
}

/// TODO
pub enum Osc1 {}

impl Sealed for Osc1 {}

impl XOscNum for Osc1 {
    const NUM: usize = 1;
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC1;
    type XIn = PB22;
    type XOut = PB23;
}

//==============================================================================
// Registers
//==============================================================================

struct Registers<X: XOscNum> {
    osc: PhantomData<X>,
}

impl<X: XOscNum> Registers<X> {
    /// TODO
    #[inline]
    unsafe fn new() -> Self {
        Self { osc: PhantomData }
    }

    #[inline]
    fn oscctrl(&self) -> &RegisterBlock {
        unsafe { &*crate::pac::OSCCTRL::ptr() }
    }

    #[inline]
    fn xoscctrl(&self) -> &XOSCCTRL {
        &self.oscctrl().xoscctrl[X::NUM]
    }

    #[inline]
    fn set_start_up(&mut self, start_up: StartUp) {
        self.xoscctrl().modify(|_, w| w.startup().variant(start_up));
    }

    #[inline]
    fn set_on_demand(&mut self, on_demand: bool) {
        self.xoscctrl().modify(|_, w| w.ondemand().bit(on_demand));
    }

    #[inline]
    fn set_run_standby(&mut self, run_standby: bool) {
        self.xoscctrl().modify(|_, w| w.runstdby().bit(run_standby));
    }

    #[inline]
    fn from_clock(&mut self) {
        self.xoscctrl().modify(|_, w| w.xtalen().bit(false));
    }

    #[inline]
    fn from_crystal(&mut self) {
        self.xoscctrl().modify(|_, w| w.xtalen().bit(true));
    }

    #[inline]
    fn enable(&mut self) {
        self.xoscctrl().modify(|_, w| w.enable().bit(true));
    }

    #[inline]
    fn disable(&mut self) {
        self.xoscctrl().modify(|_, w| w.enable().bit(false));
    }

    #[inline]
    fn wait_ready(&self) {
        let mask = 1 << X::NUM;
        while self.oscctrl().status.read().bits() & mask == 0 {}
    }
}

//==============================================================================
// Aliases
//==============================================================================

/// TODO
pub type StartUp = STARTUP_A;

/// TODO
pub type XIn<X> = Pin<<X as XOscNum>::XIn, FloatingDisabled>;

/// TODO
pub type XOut<X> = Pin<<X as XOscNum>::XOut, FloatingDisabled>;

//==============================================================================
// XOscConfig
//==============================================================================

pub struct XOscConfig<X, P = NoneT>
where
    X: XOscNum,
    P: OptionalPin,
{
    regs: Registers<X>,
    xin: XIn<X>,
    xout: P,
    freq: Hertz,
}

impl<X: XOscNum> XOscConfig<X> {
    /// TODO
    #[inline]
    pub fn from_clock(xin: impl AnyPin<Id = X::XIn>, freq: impl Into<Hertz>) -> Self {
        let xin = xin.into().into_floating_disabled();
        let mut regs = unsafe { Registers::new() };
        regs.from_clock();
        // TODO
        Self {
            regs,
            xin,
            xout: NoneT,
            freq: freq.into(),
        }
    }

    /// TODO
    #[inline]
    pub fn free(self) -> XIn<X> {
        self.xin
    }
}

impl<X: XOscNum> XOscConfig<X, XOut<X>> {
    /// TODO
    #[inline]
    pub fn from_crystal(
        xin: impl AnyPin<Id = X::XIn>,
        xout: impl AnyPin<Id = X::XOut>,
        freq: impl Into<Hertz>,
    ) -> Self {
        let xin = xin.into().into_floating_disabled();
        let xout = xout.into().into_floating_disabled();
        let mut regs = unsafe { Registers::new() };
        // TODO
        regs.from_crystal();
        Self {
            regs,
            xin,
            xout,
            freq: freq.into(),
        }
    }

    /// TODO
    #[inline]
    pub fn free(self) -> (XIn<X>, XOut<X>) {
        (self.xin, self.xout)
    }
}

impl<X, P> XOscConfig<X, P>
where
    X: XOscNum,
    P: OptionalPin,
{
    /// TODO
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.freq
    }

    /// TODO
    #[inline]
    pub fn set_start_up(&mut self, start_up: StartUp) {
        self.regs.set_start_up(start_up);
    }

    /// TODO
    #[inline]
    pub fn set_on_demand(&mut self, on_demand: bool) {
        self.regs.set_on_demand(on_demand);
    }

    /// TODO
    #[inline]
    pub fn set_run_standby(&mut self, run_standby: bool) {
        self.regs.set_run_standby(run_standby);
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> XOsc<X, P> {
        self.regs.enable();
        self.regs.wait_ready();
        XOsc { config: self }
    }
}

//==============================================================================
// XOsc
//==============================================================================

pub struct XOsc<X, P = NoneT>
where
    X: XOscNum,
    P: OptionalPin,
{
    config: XOscConfig<X, P>,
}

impl<X, P> XOsc<X, P>
where
    X: XOscNum,
    P: OptionalPin,
{
    /// TODO
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.config.freq()
    }

    /// TODO
    #[inline]
    pub fn disable(mut self) -> XOscConfig<X, P> {
        self.config.regs.disable();
        self.config
    }
}

/// TODO
pub type XOsc0<P = NoneT> = XOsc<Osc0, P>;

/// TODO
pub type XOsc1<P = NoneT> = XOsc<Osc1, P>;

//==============================================================================
// GclkSource
//==============================================================================

impl GclkSourceType for Osc0 {
    const GCLK_SRC: SRC_A = SRC_A::XOSC0;
}

impl GclkSourceType for Osc1 {
    const GCLK_SRC: SRC_A = SRC_A::XOSC1;
}

impl<X, P> Sealed for XOsc<X, P>
where
    X: XOscNum,
    P: OptionalPin,
{
}

impl<G, X, P> GclkSource<G> for XOsc<X, P>
where
    G: GenNum,
    X: XOscNum + GclkSourceType,
    P: OptionalPin,
{
    type Type = X;

    #[inline]
    fn freq(&self) -> Hertz {
        self.config.freq
    }
}

//==============================================================================
// DpllSource
//==============================================================================

impl DpllSourceType for Osc0 {
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC0;
}

impl DpllSourceType for Osc1 {
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC1;
}

impl<X, P> DpllSource for XOsc<X, P>
where
    X: XOscNum + DpllSourceType,
    P: OptionalPin,
{
    type Type = X;

    #[inline]
    fn freq(&self) -> Hertz {
        self.config.freq
    }
}
