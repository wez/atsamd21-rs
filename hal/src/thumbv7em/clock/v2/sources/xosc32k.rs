use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::osc32kctrl::xosc32k::{CGM_A, STARTUP_A};
use crate::pac::osc32kctrl::{RegisterBlock, STATUS, XOSC32K};

use crate::gpio::v2::{AnyPin, FloatingDisabled, Pin, PA00, PA01};
use crate::time::{Hertz, U32Ext};
use crate::typelevel::{Count, Decrement, Increment, Lockable, Sealed, Unlockable, Zero};

use super::super::gclk::{GclkSource, GclkSourceType, GenNum};
use super::super::RtcClock;
use super::dpll::{DpllSource, DpllSourceType, DpllSrc};

//==============================================================================
// Registers
//==============================================================================

pub type XOsc32kToken = Registers;

pub struct Registers;

impl Registers {
    /// TODO
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self
    }

    #[inline]
    fn osc32kctrl(&self) -> &RegisterBlock {
        unsafe { &*crate::pac::OSC32KCTRL::ptr() }
    }

    #[inline]
    fn status(&self) -> &STATUS {
        &self.osc32kctrl().status
    }

    #[inline]
    fn xosc32k(&self) -> &XOSC32K {
        &self.osc32kctrl().xosc32k
    }

    #[inline]
    fn set_gain_mode(&mut self, high_speed: bool) {
        let variant = match high_speed {
            false => CGM_A::XT,
            true => CGM_A::HS,
        };
        self.xosc32k().modify(|_, w| w.cgm().variant(variant));
    }

    #[inline]
    fn set_start_up(&mut self, start_up: StartUp) {
        self.xosc32k().modify(|_, w| w.startup().variant(start_up));
    }

    #[inline]
    fn set_on_demand(&mut self, on_demand: bool) {
        self.xosc32k().modify(|_, w| w.ondemand().bit(on_demand));
    }

    #[inline]
    fn set_run_standby(&mut self, run_standby: bool) {
        self.xosc32k().modify(|_, w| w.runstdby().bit(run_standby));
    }

    #[inline]
    fn enable_1k(&mut self, enable: bool) {
        self.xosc32k().modify(|_, w| w.en1k().bit(enable));
    }

    #[inline]
    fn enable_32k(&mut self, enable: bool) {
        self.xosc32k().modify(|_, w| w.en32k().bit(enable));
    }

    #[inline]
    fn from_clock(&mut self) {
        self.xosc32k().modify(|_, w| w.xtalen().bit(false));
    }

    #[inline]
    fn from_crystal(&mut self) {
        self.xosc32k().modify(|_, w| w.xtalen().bit(true));
    }

    #[inline]
    fn enable(&mut self) {
        self.xosc32k().modify(|_, w| w.enable().bit(true));
    }

    #[inline]
    fn wrtlock(&mut self) {
        self.xosc32k().modify(|_, w| w.wrtlock().bit(true));
    }

    #[inline]
    fn disable(&mut self) {
        self.xosc32k().modify(|_, w| w.enable().bit(false));
    }

    #[inline]
    fn wait_ready(&self) {
        while self.status().read().xosc32krdy().bit_is_clear() {}
    }
}

//==============================================================================
// Aliases
//==============================================================================

/// TODO
pub type StartUp = STARTUP_A;

/// TODO
pub type XIn32 = Pin<PA00, FloatingDisabled>;

/// TODO
pub type XOut32 = Pin<PA01, FloatingDisabled>;

//==============================================================================
// Mode structure for XOsc32kConfig
//==============================================================================

pub trait Mode: Sealed {}

pub struct ClockInputMode {}
impl Mode for ClockInputMode {}
impl Sealed for ClockInputMode {}

pub struct XOsc32kInputMode {
    xout32: XOut32,
    /// TODO
    control_gain_mode_high_speed: bool,
}
impl Mode for XOsc32kInputMode {}
impl Sealed for XOsc32kInputMode {}

//==============================================================================
// XOsc32kConfig
//==============================================================================

pub struct XOsc32kConfig<SrcMode>
where
    SrcMode: Mode,
{
    token: Registers,
    mode: SrcMode,
    xin32: XIn32,
}

impl<SrcMode: Mode> XOsc32kConfig<SrcMode> {
    /// TODO
    #[inline]
    pub fn set_start_up(mut self, start_up: StartUp) -> Self {
        self.token.set_start_up(start_up);
        self
    }

    /// TODO
    #[inline]
    pub fn set_on_demand(mut self, on_demand: bool) -> Self {
        self.token.set_on_demand(on_demand);
        self
    }

    /// TODO
    #[inline]
    pub fn set_run_standby(mut self, run_standby: bool) -> Self {
        self.token.set_run_standby(run_standby);
        self
    }

    /// TODO
    #[inline]
    pub fn enable_1k(mut self, enable: bool) -> Self {
        self.token.enable_1k(enable);
        self
    }

    /// TODO
    #[inline]
    pub fn enable_32k(mut self, enable: bool) -> Self {
        self.token.enable_32k(enable);
        self
    }

    /// Lock the XOsc32k configuration
    ///
    /// Locked until a Power-On Reset (POR) is detected.
    /// Discard the token and possibility to further
    /// modify the oscillator to model this write lock
    #[inline]
    pub fn wrtlock(mut self) -> XOsc32k<SrcMode> {
        self.token.enable();
        let count = Zero::new();
        let new_token = unsafe {
            Registers::new()
        };
        //XOsc32k { config: self, count }
        XOsc32k {
            config: XOsc32kConfig {
                token: new_token,
                mode: self.mode,
                xin32: self.xin32,
            },
            count,
        }
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> XOsc32k<SrcMode> {
        self.token.enable();
        let count = Zero::new();
        XOsc32k {
            config: self,
            count,
        }
    }
}

impl XOsc32kConfig<ClockInputMode> {
    /// TODO
    #[inline]
    pub fn from_clock(mut token: XOsc32kToken, xin32: impl AnyPin<Id = PA00>) -> Self {
        let xin32 = xin32.into().into_floating_disabled();
        // TODO
        token.from_clock();
        Self {
            token,
            mode: ClockInputMode {},
            xin32,
        }
    }

    /// TODO
    #[inline]
    pub fn free(self) -> (XOsc32kToken, XIn32) {
        (self.token, self.xin32)
    }
}

impl XOsc32kConfig<XOsc32kInputMode> {
    /// TODO
    #[inline]
    pub fn from_crystal(
        mut token: XOsc32kToken,
        xin32: impl AnyPin<Id = PA00>,
        xout32: impl AnyPin<Id = PA01>,
    ) -> Self {
        let xin32 = xin32.into().into_floating_disabled();
        let xout32 = xout32.into().into_floating_disabled();
        let control_gain_mode_high_speed = false;
        // TODO
        token.from_crystal();
        Self {
            token,
            xin32,
            mode: XOsc32kInputMode {
                xout32,
                control_gain_mode_high_speed,
            },
        }
    }

    /// TODO
    #[inline]
    pub fn set_gain_mode(mut self, high_speed: bool) {
        self.mode.control_gain_mode_high_speed = true;
        self.token.set_gain_mode(high_speed);
    }

    /// TODO
    #[inline]
    pub fn free(self) -> (XOsc32kToken, XIn32, XOut32) {
        (self.token, self.xin32, self.mode.xout32)
    }
}

//==============================================================================
// XOsc32k
//==============================================================================

pub struct XOsc32k<SrcMode, N = Zero>
where
    SrcMode: Mode,
    N: Count,
{
    config: XOsc32kConfig<SrcMode>,
    count: N,
}

impl<SrcMode: Mode, N: Count> XOsc32k<SrcMode, N> {
    /// TODO
    #[inline]
    fn create(config: XOsc32kConfig<SrcMode>, count: N) -> Self {
        XOsc32k { config, count }
    }

    /// TODO
    #[inline]
    pub fn disable(mut self) -> XOsc32kConfig<SrcMode> {
        self.config.token.disable();
        self.config
    }

    /// TODO
    #[inline]
    pub fn enable_1k(mut self, enable: bool) -> Self {
        self.config.token.enable_1k(enable);
        self
    }

    /// TODO
    #[inline]
    pub fn enable_32k(mut self, enable: bool) -> Self {
        self.config.token.enable_32k(enable);
        self
    }
}

impl<SrcMode: Mode, N: Count> Sealed for XOsc32k<SrcMode, N> {}

//==============================================================================
// Lockable
//==============================================================================

impl<SrcMode, N> Lockable for XOsc32k<SrcMode, N>
where
    SrcMode: Mode,
    N: Increment,
{
    type Locked = XOsc32k<SrcMode, N::Inc>;
    fn lock(self) -> Self::Locked {
        XOsc32k::create(self.config, self.count.inc())
    }
}

//==============================================================================
// Unlockable
//==============================================================================

impl<SrcMode, N> Unlockable for XOsc32k<SrcMode, N>
where
    SrcMode: Mode,
    N: Decrement,
{
    type Unlocked = XOsc32k<SrcMode, N::Dec>;
    fn unlock(self) -> Self::Unlocked {
        XOsc32k::create(self.config, self.count.dec())
    }
}

//==============================================================================
// GclkSource
//==============================================================================

pub enum Osc32k {}

impl Sealed for Osc32k {}

impl GclkSourceType for Osc32k {
    const GCLK_SRC: SRC_A = SRC_A::XOSC32K;
}

impl<G: GenNum, SrcMode: Mode, N: Count> GclkSource<G> for XOsc32k<SrcMode, N> {
    type Type = Osc32k;

    #[inline]
    fn freq(&self) -> Hertz {
        32768.hz()
    }
}

//==============================================================================
// DpllSource
//==============================================================================

impl DpllSourceType for Osc32k {
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC32;
}

impl<SrcMode, N> DpllSource for XOsc32k<SrcMode, N>
where
    SrcMode: Mode,
    N: Count,
    XOsc32k<SrcMode, N>: Sealed,
{
    type Type = Osc32k;

    #[inline]
    fn freq(&self) -> Hertz {
        32768.hz()
    }
}

//==============================================================================
// RtcClock
//==============================================================================

impl<SrcMode: Mode> RtcClock for XOsc32k<SrcMode> {
    #[inline]
    fn enable_1k(&mut self) -> RTCSEL_A {
        self.config.token.enable_1k(true);
        RTCSEL_A::XOSC1K
    }

    #[inline]
    fn enable_32k(&mut self) -> RTCSEL_A {
        self.config.token.enable_32k(true);
        RTCSEL_A::XOSC32K
    }
}
