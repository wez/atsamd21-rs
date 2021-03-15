//! TODO

use core::marker::PhantomData;

use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::oscctrl::dpll::dpllctrlb::REFCLK_A;
use crate::pac::oscctrl::dpll::{dpllstatus, dpllsyncbusy, DPLLCTRLA, DPLLCTRLB, DPLLRATIO};
use crate::pac::oscctrl::DPLL;

use crate::gpio::v2::OptionalPin;
use crate::time::{Hertz, U32Ext};
use crate::typelevel::{Count, NoneT, Sealed};

use super::super::gclk::GenNum;
use super::super::pclk::{Pclk, PclkToken, PclkType};
use super::xosc::{XOsc, XOscNum};
use super::xosc32k::XOsc32k;
use super::{AnySource, Source, SourceForGclk, SourceType};

//==============================================================================
// DpllNum
//==============================================================================

pub trait DpllNum: Sealed {
    const NUM: usize;
    const GCLK_SRC: SRC_A;
}

pub enum Pll0 {}

impl Sealed for Pll0 {}

impl DpllNum for Pll0 {
    const NUM: usize = 0;
    const GCLK_SRC: SRC_A = SRC_A::DPLL0;
}

pub enum Pll1 {}

impl Sealed for Pll1 {}

impl DpllNum for Pll1 {
    const NUM: usize = 1;
    const GCLK_SRC: SRC_A = SRC_A::DPLL1;
}

//==============================================================================
// DpllSource
//==============================================================================

/// TODO
pub trait DpllSource {
    const DPLL_SRC: REFCLK_A;
}

impl<D, G> DpllSource for Pclk<D, G>
where
    D: DpllNum + PclkType,
    G: GenNum,
{
    const DPLL_SRC: REFCLK_A = REFCLK_A::GCLK;
}

impl<P, N> DpllSource for Source<XOsc32k<P>, N>
where
    P: OptionalPin,
    N: Count,
{
    const DPLL_SRC: REFCLK_A = REFCLK_A::XOSC32;
}

impl<X, P, N> DpllSource for Source<XOsc<X, P>, N>
where
    X: XOscNum,
    P: OptionalPin,
    N: Count,
{
    const DPLL_SRC: REFCLK_A = X::DPLL_SRC;
}

//==============================================================================
// OptionalDpllSource
//==============================================================================

/// TODO
pub trait OptionalDpllSource {}

impl OptionalDpllSource for NoneT {}

impl<S: DpllSource> OptionalDpllSource for S {}

/// TODO
pub trait SomeDpllSource: OptionalDpllSource + DpllSource {}

impl<S: DpllSource> SomeDpllSource for S {}

//==============================================================================
// Registers
//==============================================================================

struct Registers<D: DpllNum> {
    dpll: PhantomData<D>,
}

impl<D: DpllNum> Registers<D> {
    /// TODO
    #[inline]
    unsafe fn new() -> Self {
        Self { dpll: PhantomData }
    }

    #[inline]
    fn dpll(&self) -> *const DPLL {
        // TODO
        unsafe { &(*crate::pac::OSCCTRL::ptr()).dpll[D::NUM] as *const _ }
    }

    #[inline]
    fn dpll_mut(&mut self) -> *mut DPLL {
        self.dpll() as *mut _
    }

    #[inline]
    fn ctrla(&mut self) -> &mut DPLLCTRLA {
        // TODO
        unsafe { &mut (*self.dpll_mut()).dpllctrla }
    }

    #[inline]
    fn ctrlb(&mut self) -> &mut DPLLCTRLB {
        // TODO
        unsafe { &mut (*self.dpll_mut()).dpllctrlb }
    }

    #[inline]
    fn ratio(&mut self) -> &mut DPLLRATIO {
        // TODO
        unsafe { &mut (*self.dpll_mut()).dpllratio }
    }

    #[inline]
    fn syncbusy(&self) -> dpllsyncbusy::R {
        // TODO
        unsafe { (*self.dpll()).dpllsyncbusy.read() }
    }

    #[inline]
    fn status(&self) -> dpllstatus::R {
        // TODO
        unsafe { (*self.dpll()).dpllstatus.read() }
    }

    // TODO
    #[inline]
    fn set_loop_div(&mut self, int: u16, frac: u8) {
        // TODO
        self.ratio().write(|w| unsafe {
            w.ldr().bits((int - 1) & 0x1FFF);
            w.ldrfrac().bits(frac & 0x1F)
        });
        while self.syncbusy().dpllratio().bit_is_set() {}
    }

    // TODO
    #[inline]
    fn set_source_clock(&mut self, variant: REFCLK_A) {
        self.ctrlb().modify(|_, w| w.refclk().variant(variant));
    }

    // TODO
    #[inline]
    fn set_source_div(&mut self, div: u16) {
        // TODO
        self.ctrlb()
            .modify(|_, w| unsafe { w.div().bits(div & 0x7FF) });
    }

    // TODO
    #[inline]
    fn set_lock_bypass(&mut self, bypass: bool) {
        self.ctrlb().modify(|_, w| w.lbypass().bit(bypass));
    }

    // TODO
    #[inline]
    fn set_wake_up_fast(&mut self, wuf: bool) {
        self.ctrlb().modify(|_, w| w.wuf().bit(wuf));
    }

    // TODO
    #[inline]
    fn wait_until_ready(&self) {
        while self.status().clkrdy().bit_is_clear() {}
    }

    // TODO
    #[inline]
    fn wait_until_locked(&self) {
        while self.status().lock().bit_is_clear() {}
    }

    // TODO
    #[inline]
    fn enable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().set_bit());
        while self.syncbusy().enable().bit_is_set() {}
    }

    // TODO
    #[inline]
    fn disable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().clear_bit());
    }
}

//==============================================================================
// DpllConfig
//==============================================================================

/// TODO
pub struct DpllConfig<D, S = NoneT>
where
    D: DpllNum,
    S: OptionalDpllSource,
{
    regs: Registers<D>,
    src: PhantomData<S>,
    freq: Hertz,
    mult: u16,
    frac: u8,
    div: u16,
}

impl<D, S> DpllConfig<D, S>
where
    D: DpllNum,
    S: OptionalDpllSource,
{
    /// TODO
    #[inline]
    fn change_source<T>(self) -> DpllConfig<D, T>
    where
        T: OptionalDpllSource,
    {
        DpllConfig {
            regs: self.regs,
            src: PhantomData,
            freq: self.freq,
            mult: self.mult,
            frac: self.frac,
            div: self.div,
        }
    }

    // TODO
    #[inline]
    pub fn set_source_div(mut self, div: u16) -> Self {
        self.regs.set_source_div(div);
        self.div = div;
        self
    }

    // TODO
    #[inline]
    pub fn set_loop_div(mut self, int: u16, frac: u8) -> Self {
        self.regs.set_loop_div(int, frac);
        self.mult = int;
        self.frac = frac;
        self
    }

    // TODO
    #[inline]
    pub fn set_lock_bypass(mut self, bypass: bool) -> Self {
        self.regs.set_lock_bypass(bypass);
        self
    }

    // TODO
    #[inline]
    pub fn set_wake_up_fast(mut self, wuf: bool) -> Self {
        self.regs.set_wake_up_fast(wuf);
        self
    }
}

impl<D: DpllNum> DpllConfig<D> {
    /// TODO
    #[inline]
    pub(super) unsafe fn default() -> Self {
        DpllConfig {
            regs: Registers::new(),
            src: PhantomData,
            freq: 0.hz(),
            mult: 1,
            frac: 0,
            div: 1,
        }
    }

    /// TODO
    #[inline]
    pub fn set_gclk_source<G>(self, pclk: Pclk<D, G>) -> DpllConfig<D, Pclk<D, G>>
    where
        D: PclkType,
        G: GenNum,
        Pclk<D, G>: DpllSource,
    {
        let freq = pclk.freq();
        assert!(freq.0 >= 32_000);
        assert!(freq.0 <= 3_200_000);
        let mut dpll = self.change_source();
        dpll.regs.set_source_clock(Pclk::<D, G>::DPLL_SRC);
        dpll.freq = freq;
        dpll
        // If the DpllSource is a Pclk, we would prefer to store it and return
        // it when the Dpll is dropped. However, if the DpllSource is an
        // instance of AnySource, we can't store it. The easy solution is to
        // drop the Pclk here and recreate it later.
    }

    /// TODO
    #[inline]
    pub fn set_source<S>(self, source: S) -> (DpllConfig<D, S>, S::Lock)
    where
        S: AnySource + DpllSource,
    {
        let freq = source.freq();
        assert!(freq.0 >= 32_000);
        assert!(freq.0 <= 3_200_000);
        let mut dpll = self.change_source();
        dpll.regs.set_source_clock(S::DPLL_SRC);
        dpll.freq = freq;
        // TODO
        (dpll, unsafe { source.lock() })
    }
}

impl<D, S> DpllConfig<D, S>
where
    D: DpllNum,
    S: DpllSource,
{
    /// TODO
    #[inline]
    pub fn freq(&self) -> Hertz {
        Hertz(self.freq.0 * self.mult as u32 * self.frac as u32 / self.div as u32 / 32)
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> Dpll<D, S> {
        assert!(self.freq().0 >= 96_000_000);
        assert!(self.freq().0 <= 200_000_000);
        self.regs.enable();
        Dpll::new(self)
    }

    /// TODO
    #[inline]
    pub fn unset_source(self, source: S) -> (DpllConfig<D>, S::Unlock)
    where
        S: AnySource + DpllSource,
    {
        // TODO
        (self.change_source(), unsafe { source.unlock() })
    }
}

impl<D, G> DpllConfig<D, Pclk<D, G>>
where
    D: DpllNum + PclkType,
    G: GenNum,
    Pclk<D, G>: DpllSource,
{
    /// TODO
    #[inline]
    pub fn unset_gclk_source(self) -> (DpllConfig<D>, Pclk<D, G>) {
        // TODO
        let pclk = unsafe { PclkToken::new() };
        // If the DpllSource is a Pclk, we would prefer to store it and return
        // it when the Dpll is dropped. However, if the DpllSource is an
        // instance of AnySource, we can't store it. The easy solution is to
        // drop the Pclk and recreate it here.
        let pclk = Pclk::new(pclk, self.freq);
        (self.change_source(), pclk)
    }
}

//==============================================================================
// Dpll
//==============================================================================

/// TODO
pub struct Dpll<D, S>
where
    D: DpllNum,
    S: DpllSource,
{
    config: DpllConfig<D, S>,
}

/// TODO
pub type Dpll0<S> = Dpll<Pll0, S>;

/// TODO
pub type Dpll1<S> = Dpll<Pll1, S>;

impl<D, S> Sealed for Dpll<D, S>
where
    D: DpllNum,
    S: DpllSource,
{
}

impl<D, S> Dpll<D, S>
where
    D: DpllNum,
    S: DpllSource,
{
    /// TODO
    #[inline]
    fn new(config: DpllConfig<D, S>) -> Self {
        Dpll { config }
    }

    /// TODO
    #[inline]
    pub fn wait_until_ready(&self) {
        self.config.regs.wait_until_ready();
    }

    /// TODO
    #[inline]
    pub fn wait_until_locked(&self) {
        self.config.regs.wait_until_locked();
    }

    /// TODO
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.config.freq()
    }

    /// TODO
    #[inline]
    pub fn disable(mut self) -> DpllConfig<D, S> {
        self.config.regs.disable();
        self.config
    }
}

//==============================================================================
// SourceType & SourceForGclk
//==============================================================================

impl<D, S> SourceType for Dpll<D, S>
where
    D: DpllNum,
    S: DpllSource,
{
    const GCLK_SRC: SRC_A = D::GCLK_SRC;

    #[inline]
    fn freq(&self) -> Hertz {
        self.freq()
    }
}

impl<G, D, S> SourceForGclk<G> for Dpll<D, S>
where
    G: GenNum,
    D: DpllNum,
    S: DpllSource,
{
}
