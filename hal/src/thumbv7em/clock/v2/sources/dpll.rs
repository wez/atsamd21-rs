//! Digital Phase Locked Loop (DPLL)
//!
//! TODO

use core::marker::PhantomData;

use typenum::U0;

use crate::pac::oscctrl::dpll::{dpllstatus, dpllsyncbusy, DPLLCTRLA, DPLLCTRLB, DPLLRATIO};
use crate::pac::oscctrl::DPLL;

pub use crate::pac::oscctrl::dpll::dpllctrlb::REFCLK_A as DpllSrc;

use crate::clock::types::{Counter, Decrement, Enabled, Increment};
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

/// All sources for DPLL0/DPLL1
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

/// DpllSource is a type of Source
pub trait DpllSource: Source {
    type Type: DpllSourceMarker;
}

//==============================================================================
// DpllToken
//==============================================================================

pub struct DpllToken<D: DpllNum> {
    dpll: PhantomData<D>,
}

impl<D: DpllNum> DpllToken<D> {
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

    // Set the loop division, see page 701 in the Datasheet
    //
    // Formula for calculating the frequency:
    // f_clk_dpll = clk_src * (LDR + 1 + (LDRFRAC / 32))
    //
    // `int` is including the `+ 1`,
    // 'frac` is the same as `LDRFRAC`
    //
    // Write to the divider must be write synchronized
    #[inline]
    fn set_loop_div(&mut self, int: u16, frac: u8) {
        self.ratio().write(|w| unsafe {
            w.ldr().bits((int - 1) & 0x1FFF);
            w.ldrfrac().bits(frac & 0x1F)
        });
        while self.syncbusy().dpllratio().bit_is_set() {}
    }

    // Set the clock source.
    #[inline]
    fn set_source_clock(&mut self, variant: DpllSrc) {
        self.ctrlb().modify(|_, w| w.refclk().variant(variant));
    }

    // When source is a XOSC this has effect, ignored otherwise.
    #[inline]
    fn set_source_div(&mut self, div: u16) {
        self.ctrlb()
            .modify(|_, w| unsafe { w.div().bits(div & 0x7FF) });
    }

    // Ignore the lock, CLK_DPLLn is always running.
    #[inline]
    fn set_lock_bypass(&mut self, bypass: bool) {
        self.ctrlb().modify(|_, w| w.lbypass().bit(bypass));
    }

    // Wake up fast, output the clock directly without waiting for lock.
    #[inline]
    fn set_wake_up_fast(&mut self, wuf: bool) {
        self.ctrlb().modify(|_, w| w.wuf().bit(wuf));
    }

    // Wait until the DPLL clock is ready.
    #[inline]
    fn wait_until_ready(&self) {
        while self.status().clkrdy().bit_is_clear() {}
    }

    // Wait until the DPLL clock is locked.
    #[inline]
    fn wait_until_locked(&self) {
        while self.status().lock().bit_is_clear() {}
    }

    // Wait until register has been synchronized.
    #[inline]
    fn wait_until_enable_synced(&self) {
        while self.syncbusy().enable().bit_is_set() {}
    }

    // Enable the DPLL, ensure register write is synchronized.
    #[inline]
    fn enable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().set_bit());
        self.wait_until_enable_synced();
    }

    // Disable the DPLL, ensure register write is synchronized.
    #[inline]
    fn disable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().clear_bit());
        self.wait_until_enable_synced();
    }
}

//==============================================================================
// Mode structure for Dpll
//==============================================================================

/// 10 bits available for predivision
///
/// ```
/// f_Div = f_xosc / (2 * (divider + 1))
/// ```
///
/// * Default division factor: 2 (register all 0)
/// * Maxumum division factor: 2048 (register all 1)
///
pub type DpllPredivider = u16;

pub trait SrcMode: Sealed {
    /// Return value of an effective predivider that can be applied on a
    /// source frequency
    fn predivider(&self) -> DpllPredivider;
}

pub struct PclkDriven<D, T>
where
    D: DpllNum + PclkType,
    T: PclkSourceMarker,
{
    reference_clk: Pclk<D, T>,
}
impl<D: DpllNum + PclkType, T: PclkSourceMarker> SrcMode for PclkDriven<D, T> {
    fn predivider(&self) -> DpllPredivider {
        1_u16
    }
}
impl<D: DpllNum + PclkType, T: PclkSourceMarker> Sealed for PclkDriven<D, T> {}

pub struct XoscDriven<T: DpllSourceMarker> {
    src: PhantomData<T>,
    raw_predivider: DpllPredivider,
}
impl<T: DpllSourceMarker> SrcMode for XoscDriven<T> {
    fn predivider(&self) -> DpllPredivider {
        2 * (1 + self.raw_predivider)
    }
}
impl<T: DpllSourceMarker> Sealed for XoscDriven<T> {}

pub struct Xosc32kDriven<T: DpllSourceMarker> {
    src: PhantomData<T>,
}
impl<T: DpllSourceMarker> SrcMode for Xosc32kDriven<T> {
    fn predivider(&self) -> DpllPredivider {
        1_u16
    }
}
impl<T: DpllSourceMarker> Sealed for Xosc32kDriven<T> {}

//==============================================================================
// Dpll
//==============================================================================

/// Digital Phase Locked Loop
///
/// Can be fed from:
///
/// * [Pclk][super::super::pclk::Pclk]
/// * [Xosc32k][super::xosc32k::Xosc32k]
/// * [Xosc0][super::xosc::Xosc0]
/// * [Xosc1][super::xosc::Xosc1]
///
/// As indicated in [DpllSrc]
///
pub struct Dpll<D, M: SrcMode>
where
    D: DpllNum,
    M: SrcMode,
{
    token: DpllToken<D>,
    src: PhantomData<D>,
    src_freq: Hertz,
    mult: u16,
    frac: u8,
    mode: M,
}

impl<D, T> Dpll<D, PclkDriven<D, T>>
where
    D: DpllNum + PclkType,
    T: PclkSourceMarker,
{
    /// Create a DPLL from Peripheral Channel (Pclk) fed from Gclk
    ///
    /// Input frequency must be between 32 kHz and 3.2 MHz
    ///
    /// Hold the Pclk until released on deconstruction
    #[inline]
    pub fn from_pclk(token: DpllToken<D>, reference_clk: Pclk<D, T>) -> Dpll<D, PclkDriven<D, T>> {
        let src_freq = reference_clk.freq();
        assert!(src_freq.0 >= 32_000);
        assert!(src_freq.0 <= 3_200_000);
        let (mult, frac) = (1, 0);
        Self {
            token,
            src: PhantomData,
            src_freq,
            mult,
            frac,
            mode: PclkDriven { reference_clk },
        }
    }
    /// Enable the DPLL, do all hardware writes
    pub fn enable(mut self) -> Enabled<Self, U0> {
        assert!(self.freq().0 >= 96_000_000);
        assert!(self.freq().0 <= 200_000_000);

        // Set the source
        self.token.set_source_clock(DpllSrc::GCLK);
        // Set the loop divider ratio
        self.token.set_loop_div(self.mult, self.frac);
        // Enable the DPLL
        self.token.enable();
        Enabled::new(self)
    }

    /// Deconstruct the DPLL, returns the held Pclk
    #[inline]
    pub fn free(self) -> (DpllToken<D>, Pclk<D, T>) {
        (self.token, self.mode.reference_clk)
    }
}

impl<D, T> Dpll<D, Xosc32kDriven<T>>
where
    D: DpllNum,
    T: DpllSourceMarker,
{
    /// Create a DPLL from external 32k oscillator ([Xosc32k][super::xosc32k::Xosc32k])
    ///
    /// Input frequency must be between 32 kHz and 3.2 MHz
    ///
    /// Increases the count, decreased on deconstruction
    #[inline]
    pub fn from_xosc32k<S>(
        token: DpllToken<D>,
        reference_clk: S,
    ) -> (Dpll<D, Xosc32kDriven<T>>, S::Inc)
    where
        S: DpllSource<Type = T> + Increment,
    {
        let src_freq = reference_clk.freq();
        assert!(src_freq.0 >= 32_000);
        assert!(src_freq.0 <= 3_200_000);
        let (mult, frac) = (1, 0);

        let dpll = Dpll {
            token,
            src: PhantomData,
            src_freq,
            mult,
            frac,
            mode: Xosc32kDriven { src: PhantomData },
        };
        (dpll, reference_clk.inc())
    }

    pub fn enable(mut self) -> Enabled<Self, U0> {
        assert!(self.freq().0 >= 96_000_000);
        assert!(self.freq().0 <= 200_000_000);

        // Set the source
        self.token.set_source_clock(DpllSrc::XOSC32);
        // Set the loop divider ratio
        self.token.set_loop_div(self.mult, self.frac);
        // Enable the DPLL
        self.token.enable();
        Enabled::new(self)
    }

    /// Decrease the count, return the disabled DPLL
    #[inline]
    pub fn free<S>(self, source: S) -> (DpllToken<D>, S::Dec)
    where
        S: DpllSource<Type = T> + Decrement,
    {
        (self.token, source.dec())
    }
}

impl<D, T> Dpll<D, XoscDriven<T>>
where
    D: DpllNum,
    T: DpllSourceMarker,
{
    /// Create a DPLL from external oscillator
    /// ([Xosc0][super::xosc::Xosc0]/[Xosc1][super::xosc::Xosc1])
    ///
    /// Input frequency must be between 32 kHz and 3.2 MHz
    ///
    /// Provides additional input pre-divider, see [DpllPredivider]
    ///
    /// Increases the count, decreased on deconstruction
    #[inline]
    pub fn from_xosc<S>(
        token: DpllToken<D>,
        reference_clk: S,
        predivider: DpllPredivider,
    ) -> (Dpll<D, XoscDriven<T>>, S::Inc)
    where
        S: DpllSource<Type = T> + Increment,
    {
        let raw_predivider = predivider;
        let src_freq = reference_clk.freq();
        let (mult, frac) = (1, 0);

        // Assert that the raw_predivider is valid!
        // 2 to 2048

        // Calculate the Dpll input frequency taking into consideration the
        // raw_predivider, but store the actual input source frequency
        let mode = XoscDriven {
            src: PhantomData,
            raw_predivider,
        };
        let frequency = src_freq.0 / mode.predivider() as u32;
        assert!(frequency >= 32_000);
        assert!(frequency <= 3_200_000);

        let dpll = Dpll {
            token,
            src: PhantomData,
            src_freq,
            mult,
            frac,
            mode,
        };
        (dpll, reference_clk.inc())
    }

    pub fn enable(mut self) -> Enabled<Self, U0> {
        assert!(self.freq().0 >= 96_000_000);
        assert!(self.freq().0 <= 200_000_000);
        // Set the source
        self.token.set_source_clock(T::DPLL_SRC);

        // Set the predivider
        self.token.set_source_div(self.mode.raw_predivider);
        // Set the loop divider ratio
        self.token.set_loop_div(self.mult, self.frac);
        // Enable the DPLL
        self.token.enable();
        Enabled::new(self)
    }

    /// Decrease the count, return the disabled DPLL
    #[inline]
    pub fn free<S>(self, source: S) -> (DpllToken<D>, S::Dec)
    where
        S: DpllSource<Type = D> + Decrement,
    {
        (self.token, source.dec())
    }
}

impl<D, M> Dpll<D, M>
where
    D: DpllNum,
    M: SrcMode,
{
    /// Set the DPLL divider
    ///
    /// Calculated as
    ///
    /// ```
    /// f_clk_dpll = clk_src * (int + (frac / 32))
    /// ```
    ///
    /// The `+ 1` in the datasheet is not forgotten, it is handled by the underlying
    /// register write function
    ///
    /// Example 1:
    /// ```
    /// clk_src = 2 MHz
    /// int = 50
    /// frac = 0
    ///
    /// 2 * 50 = 100 MHz
    /// ```
    /// Example 2:
    /// ```
    /// clk_src = 32 kHz
    /// int = 3000
    /// frac = 24
    ///
    /// 0.032 * (3000 +  24/32) = 96.024 MHz
    /// ```
    #[inline]
    pub fn set_loop_div(mut self, int: u16, frac: u8) -> Self {
        self.token.set_loop_div(int, frac);
        self.mult = int;
        self.frac = frac;
        self
    }

    /// Set to ignore the phase-lock, CLK_DPLL is always running regardless of lock status
    #[inline]
    pub fn set_lock_bypass(mut self, bypass: bool) -> Self {
        self.token.set_lock_bypass(bypass);
        self
    }

    /// Set to skip waiting for DPLL lock before outputting clock
    #[inline]
    pub fn set_wake_up_fast(mut self, wuf: bool) -> Self {
        self.token.set_wake_up_fast(wuf);
        self
    }
    /// Busy-wait until DPLL has achieved lock
    #[inline]
    pub fn wait_until_locked(self) -> Self {
        self.token.wait_until_locked();
        self
    }
    /// Busy-wait until DPLL is ready
    #[inline]
    pub fn wait_until_ready(self) -> Self {
        self.token.wait_until_ready();
        self
    }
    /// Return the frequency of the DPLL
    #[inline]
    pub fn freq(&self) -> Hertz {
        Hertz(
            self.src_freq.0 / self.mode.predivider() as u32
                * (self.mult as u32 + 1 + self.frac as u32 / 32),
        )
    }
}

impl<D, T> Dpll<D, XoscDriven<T>>
where
    D: DpllNum,
    T: DpllSourceMarker,
{
    /// Set the predivider, see [DpllPredivider]
    #[inline]
    pub fn set_source_div(mut self, predivider: DpllPredivider) -> Self {
        // Assert the source pre-divider does not go outside input frequency specifications
        let raw_predivider = predivider;
        self.mode.raw_predivider = raw_predivider;
        let frequency = self.src_freq.0 / self.mode.predivider() as u32;
        assert!(frequency >= 32_000);
        assert!(frequency <= 3_200_000);
        self
    }
}

/// Encapsulation for Dpll0
pub type Dpll0<M> = Dpll<Pll0, M>;

/// Encapsulation for Dpll1
pub type Dpll1<M> = Dpll<Pll1, M>;

impl<D, M> Enabled<Dpll<D, M>, U0>
where
    D: DpllNum,
    M: SrcMode,
{
    #[inline]
    pub fn disable(mut self) -> Dpll<D, M> {
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

impl<G, D, M, N> GclkSource<G> for Enabled<Dpll<D, M>, N>
where
    G: GenNum,
    D: DpllNum + GclkSourceMarker,
    M: SrcMode,
    N: Counter,
{
    type Type = D;
}

impl<D, M, N> Source for Enabled<Dpll<D, M>, N>
where
    D: DpllNum + GclkSourceMarker,
    M: SrcMode,
    N: Counter,
{
    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
