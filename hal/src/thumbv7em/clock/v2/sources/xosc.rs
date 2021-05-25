use core::marker::PhantomData;

use crate::pac::oscctrl::xoscctrl::{CFDPRESC_A, STARTUP_A};
use crate::pac::oscctrl::{RegisterBlock, XOSCCTRL};

use crate::gpio::v2::{AnyPin, FloatingDisabled, Pin, PinId, PA14, PA15, PB22, PB23};
use crate::time::{Hertz, U32Ext};
use crate::typelevel::{Count, Decrement, Increment, Lockable, Sealed, Unlockable, Zero};

use super::super::gclk::{GclkSource, GclkSourceEnum, GclkSourceMarker, GenNum};
use super::dpll::{DpllSource, DpllSourceMarker, DpllSrc};

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

/// Oscillator Source 0
pub enum Osc0 {}

impl Sealed for Osc0 {}

impl XOscNum for Osc0 {
    const NUM: usize = 0;
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC0;
    type XIn = PA14;
    type XOut = PA15;
}

/// Oscillator Source 1
pub enum Osc1 {}

impl Sealed for Osc1 {}

impl XOscNum for Osc1 {
    const NUM: usize = 1;
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC1;
    type XIn = PB22;
    type XOut = PB23;
}

#[derive(Debug, Clone, PartialEq)]
/// Current mutliplier/reference pair
pub enum CrystalCurrent {
    /// 8MHz
    BaseFreq,
    /// 8 to 16MHz
    LowFreq,
    /// 16 to 24MHz
    MedFreq,
    /// 24 to 48MHz
    HighFreq,
}

impl CrystalCurrent {
    /// Get the current multiplier
    pub fn imult(&self) -> u8 {
        match &self {
            Self::BaseFreq => 3,
            Self::LowFreq => 4,
            Self::MedFreq => 5,
            Self::HighFreq => 6,
        }
    }

    /// Get the current reference
    pub fn iptat(&self) -> u8 {
        match &self {
            Self::BaseFreq => 2,
            Self::LowFreq => 3,
            Self::MedFreq => 3,
            Self::HighFreq => 3,
        }
    }
}

//==============================================================================
// Registers
//==============================================================================

pub type XOscToken<X> = Registers<X>;

pub struct Registers<X: XOscNum> {
    osc: PhantomData<X>,
}

impl<X: XOscNum> Registers<X> {
    /// TODO
    #[inline]
    pub(super) unsafe fn new() -> Self {
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

    #[inline]
    #[allow(dead_code)]
    fn set_clock_failure_detection(&mut self, cfden: bool) {
        self.xoscctrl().modify(|_, w| w.cfden().bit(cfden));
    }

    #[inline]
    fn set_clock_failure_detection_prescaler(&mut self, prescale: CFDPRESC_A) {
        self.xoscctrl()
            .modify(|_, w| w.cfdpresc().variant(prescale));
    }
    #[inline]
    fn set_crystal_current(&mut self, cc: &CrystalCurrent) {
        self.xoscctrl().modify(|_, w| unsafe {
            w.imult().bits(cc.imult());
            w.iptat().bits(cc.iptat())
        });
    }

    #[inline]
    fn set_amplitude_loop_control(&mut self, enalc: bool) {
        self.xoscctrl().modify(|_, w| w.enalc().bit(enalc));
    }

    #[inline]
    fn set_clock_switch(&mut self, swben: bool) {
        self.xoscctrl().modify(|_, w| w.swben().bit(swben));
    }

    #[inline]
    fn set_low_buf_gain(&mut self, lowbufgain: bool) {
        self.xoscctrl()
            .modify(|_, w| w.lowbufgain().bit(lowbufgain));
    }
}

//==============================================================================
// Aliases
//==============================================================================

/// When Clock Failure Detection (CFD) is active, sets how many clock cycles
/// before monitoring of the clock begins when starting the oscillator.
pub type StartUp = STARTUP_A;

/// Type alias for XOsc Input pin
pub type XIn<X> = Pin<<X as XOscNum>::XIn, FloatingDisabled>;

/// Type alias for XOsc Output pin
pub type XOut<X> = Pin<<X as XOscNum>::XOut, FloatingDisabled>;

//==============================================================================
// Mode structure for XOscConfig
//==============================================================================

pub trait Mode: Sealed {}

pub struct ClockInputMode {}
impl Mode for ClockInputMode {}
impl Sealed for ClockInputMode {}

pub struct XOscInputMode<X: XOscNum> {
    xout: XOut<X>,
    crystal_current: CrystalCurrent,
}
impl<X: XOscNum> Mode for XOscInputMode<X> {}
impl<X: XOscNum> Sealed for XOscInputMode<X> {}

//==============================================================================
// XOscConfig
//==============================================================================

pub struct XOscConfig<X, SrcMode>
where
    X: XOscNum,
    SrcMode: Mode,
{
    token: XOscToken<X>,
    mode: SrcMode,
    xin: XIn<X>,
    freq: Hertz,
}

impl<X, SrcMode> XOscConfig<X, SrcMode>
where
    X: XOscNum,
    SrcMode: Mode,
{
    /// Returns the frequency of the oscillator
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.freq
    }

    /// Sets the number of cycles allowed to pass before Clock Failure Detection (CFD)
    /// starts monitoring the external oscillator. Only valid together with [
    #[inline]
    pub fn set_start_up(mut self, start_up: StartUp) -> Self {
        self.token.set_start_up(start_up);
        self
    }
    /// Controls the on demand functionality of the clock source
    ///
    /// Only starts the clock source when a peripheral uses it
    ///
    /// If cleared the clock will be always active
    /// See Datasheet c. 13.5 for details
    #[inline]
    pub fn set_on_demand(mut self, on_demand: bool) -> Self {
        self.token.set_on_demand(on_demand);
        self
    }

    /// Controls the clock source behaviour during standby
    ///
    /// See Datasheet c. 28.6.2
    #[inline]
    pub fn set_run_standby(mut self, run_standby: bool) -> Self {
        self.token.set_run_standby(run_standby);
        self
    }

    /// XOsc lock Switch Enable
    ///
    /// Controls if XOSCn switches back to external clock or crystal in case of clock recovery
    #[inline]
    pub fn set_clock_switch(mut self, swben: bool) -> Self {
        self.token.set_clock_switch(swben);
        self
    }

    /// If `LOWBUFGAIN`is set when `ENALC` is enabled,
    /// the oscillators amplitude is increased by approximately a factor 2.
    ///
    /// Default value (0) should be used together with low amplitude oscillators.
    /// Can be used to solve stability issues.
    #[inline]
    pub fn set_low_buf_gain(mut self, lowbufgain: bool) -> Self {
        self.token.set_low_buf_gain(lowbufgain);
        self
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> XOsc<X, SrcMode> {
        self.token.enable();
        XOsc::new(self)
    }
}

impl<X: XOscNum> XOscConfig<X, ClockInputMode> {
    /// TODO
    #[inline]
    pub fn from_clock(
        mut token: XOscToken<X>,
        xin: impl AnyPin<Id = X::XIn>,
        freq: impl Into<Hertz>,
    ) -> Self {
        let xin = xin.into().into_floating_disabled();
        token.from_clock();
        // TODO
        Self {
            token,
            mode: ClockInputMode {},
            xin,
            freq: freq.into(),
        }
    }

    /// TODO
    #[inline]
    pub fn free(self) -> (XOscToken<X>, XIn<X>) {
        (self.token, self.xin)
    }
}

impl<X: XOscNum> XOscConfig<X, XOscInputMode<X>> {
    /// Creates an XOsc input fed from a crystal oscillator.
    ///
    /// The crystal oscillator frequency must be supported,
    /// for valid frequencies see [CrystalCurrent].
    ///
    /// By default `Amplitude Loop Control` is set,
    /// see [XOscConfig::set_amplitude_loop_control()]
    #[inline]
    pub fn from_crystal(
        mut token: XOscToken<X>,
        xin: impl AnyPin<Id = X::XIn>,
        xout: impl AnyPin<Id = X::XOut>,
        freq: impl Into<Hertz>,
    ) -> Self {
        let xin = xin.into().into_floating_disabled();
        let xout = xout.into().into_floating_disabled();

        // Calculate the CrystalCurrent from the supplied
        // crystal frequency
        let (crystal_current, frequency) = match freq.into().0 {
            freq @ 8_000_000 => (CrystalCurrent::BaseFreq, freq),
            freq @ 8_000_001..=16_000_000 => (CrystalCurrent::LowFreq, freq),
            freq @ 16_000_001..=24_000_000 => (CrystalCurrent::MedFreq, freq),
            freq @ 24_000_001..=48_000_000 => (CrystalCurrent::HighFreq, freq),
            _ => panic!("XOsc fed with invalid frequency"),
        };

        // Set the crystal drive current
        token.set_crystal_current(&crystal_current);

        // Lowers power usage and protects the crystal
        token.set_amplitude_loop_control(true);

        // Change from default clock input mode to crystal mode
        token.from_crystal();
        Self {
            token,
            mode: XOscInputMode {
                xout,
                crystal_current,
            },
            xin,
            freq: frequency.hz(),
        }
    }

    /// Sets the current drive strength for the crystal
    ///
    /// See [CrystalCurrent] for possible values
    #[inline]
    pub fn set_crystal_current(mut self, crystal_current: CrystalCurrent) -> Self {
        self.token.set_crystal_current(&crystal_current);
        self.mode.crystal_current = crystal_current;
        self
    }

    /// Controls the automatic amplitude loop control
    ///
    /// Recommended option, ensures the crystal is not overdriven,
    /// and lowers power consumption. See datasheet c. 54.13 p. 1811
    #[inline]
    pub fn set_amplitude_loop_control(mut self, enalc: bool) -> Self {
        self.token.set_amplitude_loop_control(enalc);
        self
    }

    /// TODO
    #[inline]
    pub fn free(self) -> (XOscToken<X>, XIn<X>, XOut<X>) {
        (self.token, self.xin, self.mode.xout)
    }
}

//==============================================================================
// XOsc
//==============================================================================

pub struct XOsc<X, SrcMode, N = Zero>
where
    X: XOscNum,
    SrcMode: Mode,
    N: Count,
{
    config: XOscConfig<X, SrcMode>,
    count: N,
}
///
/// TODO
pub type XOsc0<SrcMode> = XOsc<Osc0, SrcMode>;

/// TODO
pub type XOsc1<SrcMode> = XOsc<Osc1, SrcMode>;

impl<X, SrcMode, N> Sealed for XOsc<X, SrcMode, N>
where
    X: XOscNum,
    SrcMode: Mode,
    N: Count,
{
}

impl<X, SrcMode> XOsc<X, SrcMode>
where
    X: XOscNum,
    SrcMode: Mode,
{
    /// TODO
    #[inline]
    fn new(config: XOscConfig<X, SrcMode>) -> Self {
        let count = Zero::new();
        XOsc { config, count }
    }

    /// TODO
    #[inline]
    pub fn disable(mut self) -> XOscConfig<X, SrcMode> {
        self.config.token.disable();
        self.config
    }
}

impl<X, SrcMode, N> XOsc<X, SrcMode, N>
where
    X: XOscNum,
    SrcMode: Mode,
    N: Count,
{
    /// TODO
    #[inline]
    fn create(config: XOscConfig<X, SrcMode>, count: N) -> Self {
        XOsc { config, count }
    }

    /// TODO
    #[inline]
    pub fn wait_ready(&self) {
        self.config.token.wait_ready();
    }

    /// TODO
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.config.freq()
    }
}

//==============================================================================
// Lockable
//==============================================================================

impl<X, SrcMode, N> Lockable for XOsc<X, SrcMode, N>
where
    X: XOscNum,
    SrcMode: Mode,
    N: Increment,
{
    type Locked = XOsc<X, SrcMode, N::Inc>;
    fn lock(self) -> Self::Locked {
        XOsc::create(self.config, self.count.inc())
    }
}

//==============================================================================
// Unlockable
//==============================================================================

impl<X, SrcMode, N> Unlockable for XOsc<X, SrcMode, N>
where
    X: XOscNum,
    SrcMode: Mode,
    N: Decrement,
{
    type Unlocked = XOsc<X, SrcMode, N::Dec>;
    fn unlock(self) -> Self::Unlocked {
        XOsc::create(self.config, self.count.dec())
    }
}

//==============================================================================
// GclkSource
//==============================================================================

impl GclkSourceMarker for Osc0 {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::XOSC0;
}

impl GclkSourceMarker for Osc1 {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::XOSC0;
}

impl<G, X, SrcMode, N> GclkSource<G> for XOsc<X, SrcMode, N>
where
    G: GenNum,
    X: XOscNum + GclkSourceMarker,
    SrcMode: Mode,
    N: Count,
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

impl DpllSourceMarker for Osc0 {
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC0;
}

impl DpllSourceMarker for Osc1 {
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC1;
}

impl<X, SrcMode, N> DpllSource for XOsc<X, SrcMode, N>
where
    X: XOscNum + DpllSourceMarker,
    SrcMode: Mode,
    N: Count,
{
    type Type = X;

    #[inline]
    fn freq(&self) -> Hertz {
        self.config.freq
    }
}
