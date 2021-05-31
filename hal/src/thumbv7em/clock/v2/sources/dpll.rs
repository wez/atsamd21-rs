//! TODO

use core::marker::PhantomData;

use typenum::U0;

use crate::pac::oscctrl::dpll::{dpllstatus, dpllsyncbusy, DPLLCTRLA, DPLLCTRLB, DPLLRATIO};
use crate::pac::oscctrl::DPLL;

pub use crate::pac::oscctrl::dpll::dpllctrlb::REFCLK_A as DpllSrc;

use crate::clock::types::{Enabled, Counter, Decrement, Increment};
use crate::clock::v2::{Source, SourceMarker};
use crate::time::Hertz;
use crate::typelevel::Sealed;

use super::super::gclk::{GclkSource, GclkSourceEnum, GclkSourceMarker, GenNum};
use super::super::pclk::{Pclk, PclkSourceMarker, PclkType};
use super::gclkio::NotGclkInput;

//==============================================================================
// DpllNum
//==============================================================================

pub trait DpllNum: Sealed {
    const NUM: usize;
}

pub enum Pll0 {}

impl Sealed for Pll0 {}

impl DpllNum for Pll0 {
    const NUM: usize = 0;
}

pub enum Pll1 {}

impl Sealed for Pll1 {}

impl DpllNum for Pll1 {
    const NUM: usize = 1;
}

//==============================================================================
// DpllSource
//==============================================================================

/// TODO
pub trait DpllSourceMarker: SourceMarker {
    const DPLL_SRC: DpllSrc;
}

impl<D, T> DpllSourceMarker for Pclk<D, T>
where
    D: DpllNum + PclkType,
    T: PclkSourceMarker,
{
    const DPLL_SRC: DpllSrc = DpllSrc::GCLK;
}

impl<D, T> SourceMarker for Pclk<D, T>
where
    D: DpllNum + PclkType,
    T: PclkSourceMarker,
{
}

/// TODO
/// TODO add Source here
pub trait DpllSource: Source {
    type Type: DpllSourceMarker;
}

//==============================================================================
// Registers
//==============================================================================

pub type DpllToken<D> = Registers<D>;

pub struct Registers<D: DpllNum> {
    dpll: PhantomData<D>,
}

impl<D: DpllNum> Registers<D> {
    /// TODO
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self { dpll: PhantomData }
    }

    #[inline]
    fn dpll(&self) -> &DPLL {
        // TODO
        unsafe { &(*crate::pac::OSCCTRL::ptr()).dpll[D::NUM] }
    }

    #[inline]
    fn ctrla(&self) -> &DPLLCTRLA {
        &self.dpll().dpllctrla
    }

    #[inline]
    fn ctrlb(&self) -> &DPLLCTRLB {
        &self.dpll().dpllctrlb
    }

    #[inline]
    fn ratio(&self) -> &DPLLRATIO {
        &self.dpll().dpllratio
    }

    #[inline]
    fn syncbusy(&self) -> dpllsyncbusy::R {
        self.dpll().dpllsyncbusy.read()
    }

    #[inline]
    fn status(&self) -> dpllstatus::R {
        self.dpll().dpllstatus.read()
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
    fn set_source_clock(&mut self, variant: DpllSrc) {
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

    /// TODO
    #[inline]
    fn wait_until_enable_synced(&self) {
        // TODO
        while self.syncbusy().enable().bit_is_set() {}
    }

    // TODO
    #[inline]
    fn enable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().set_bit());
        self.wait_until_enable_synced();
    }

    // TODO
    #[inline]
    fn disable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().clear_bit());
        self.wait_until_enable_synced();
    }
}

//==============================================================================
// Dpll
//==============================================================================

/// TODO
pub struct Dpll<D, T>
where
    D: DpllNum,
    T: DpllSourceMarker,
{
    token: DpllToken<D>,
    src: PhantomData<T>,
    freq: Hertz,
    mult: u16,
    frac: u8,
    div: u16,
}

impl<D, T> Dpll<D, Pclk<D, T>>
where
    D: DpllNum + PclkType,
    T: PclkSourceMarker,
    Pclk<D, T>: DpllSourceMarker,
{
    /// TODO
    #[inline]
    pub fn from_pclk(mut token: DpllToken<D>, pclk: Pclk<D, T>) -> Self {
        let freq = pclk.freq();
        assert!(freq.0 >= 32_000);
        assert!(freq.0 <= 3_200_000);
        let (mult, frac, div) = (1, 0, 1);
        // TODO: Store the mode type in Dpll and Pclk in it
        // It will allow to move HW calls into `enable()`
        // Also, `free_pclk` won't have to be a hack
        token.set_source_clock(Pclk::<D, T>::DPLL_SRC);
        Dpll {
            token,
            src: PhantomData,
            freq,
            mult,
            frac,
            div,
        }
    }

    /// TODO
    #[inline]
    pub fn free_pclk(self) -> (DpllToken<D>, Pclk<D, T>) {
        // TODO: Store the mode type in Dpll and Pclk in it
        // It will allow to move HW calls into `enable()`
        // Also, `free_pclk` won't have to be a hack
        let pclk = unsafe { Pclk::hack(self.freq) };
        (self.token, pclk)
    }
}

impl<D, T> Dpll<D, T>
where
    D: DpllNum,
    T: DpllSourceMarker,
{
    /// TODO
    #[inline]
    pub fn from_xosc<S>(mut token: DpllToken<D>, source: S) -> (Dpll<D, T>, S::Inc)
    where
        S: DpllSource<Type = T> + Increment,
    {
        let freq = source.freq();
        let (mult, frac, div) = (1, 0, 1);

        let frequency = freq.0 / (2 * (1 + div)) as u32;
        assert!(frequency >= 32_000);
        assert!(frequency <= 3_200_000);
        token.set_source_clock(T::DPLL_SRC);
        let dpll = Dpll {
            token,
            src: PhantomData,
            freq,
            mult,
            frac,
            div,
        };
        // TODO
        (dpll, source.inc())
    }

    /// TODO
    #[inline]
    pub fn free_xosc<S>(self, source: S) -> (DpllToken<D>, S::Dec)
    where
        S: DpllSource<Type = T> + Decrement,
    {
        (self.token, source.dec())
    }
}

impl<D, T> Dpll<D, T>
where
    D: DpllNum,
    T: DpllSourceMarker,
{
    // TODO
    #[inline]
    pub fn set_source_div(mut self, div: u16) -> Self {
        self.token.set_source_div(div);
        self.div = div;
        self
    }

    // TODO
    #[inline]
    pub fn set_loop_div(mut self, int: u16, frac: u8) -> Self {
        self.token.set_loop_div(int, frac);
        self.mult = int;
        self.frac = frac;
        self
    }

    // TODO
    #[inline]
    pub fn set_lock_bypass(mut self, bypass: bool) -> Self {
        self.token.set_lock_bypass(bypass);
        self
    }

    // TODO
    #[inline]
    pub fn set_wake_up_fast(mut self, wuf: bool) -> Self {
        self.token.set_wake_up_fast(wuf);
        self
    }

    /// TODO
    #[inline]
    pub fn freq(&self) -> Hertz {
        Hertz(
            self.freq.0 / (2 * (1 + self.div as u32))
                * (self.mult as u32 + 1 + self.frac as u32 / 32),
        )
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> Enabled<Dpll<D, T>, U0> {
        assert!(self.freq().0 >= 96_000_000);
        assert!(self.freq().0 <= 200_000_000);
        self.token.enable();
        Enabled::new(self)
    }
}

/// TODO
pub type Dpll0<T> = Dpll<Pll0, T>;

/// TODO
pub type Dpll1<T> = Dpll<Pll1, T>;

impl<D, T> Enabled<Dpll<D, T>, U0>
where
    D: DpllNum,
    T: DpllSourceMarker,
{
    #[inline]
    pub fn disable(mut self) -> Dpll<D, T> {
        self.0.token.disable();
        self.0
    }
}

//==============================================================================
// GclkSource
//==============================================================================

impl GclkSourceMarker for Pll0 {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::DPLL0;
}

impl NotGclkInput for Pll0 {}

impl SourceMarker for Pll0 {}

impl GclkSourceMarker for Pll1 {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::DPLL1;
}

impl NotGclkInput for Pll1 {}

impl SourceMarker for Pll1 {}

impl<G, D, T, N> GclkSource<G> for Enabled<Dpll<D, T>, N>
where
    G: GenNum,
    D: DpllNum + GclkSourceMarker,
    T: DpllSourceMarker,
    N: Counter,
{
    type Type = D;
}

impl<D, T, N> Source for Enabled<Dpll<D, T>, N>
where
    D: DpllNum + GclkSourceMarker,
    T: DpllSourceMarker,
    N: Counter,
{
    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
