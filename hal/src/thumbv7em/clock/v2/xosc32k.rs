//! # Xosc32k - External Oscillator 32kHz
//! TODO
//!
//! Provides 32kHz outputs for ['Gclk`]s, [`RTC`] and [`Dpll`].
//! Additionally provides 1kHz output for the ['RTC`] module.

use typenum::U0;

use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::osc32kctrl::xosc32k::{CGM_A, STARTUP_A};
use crate::pac::osc32kctrl::{RegisterBlock, STATUS, XOSC32K};

use crate::clock::types::{Counter, Enabled};
use crate::clock::v2::{Source, SourceMarker};
use crate::gpio::v2::{AnyPin, FloatingDisabled, Pin, PA00, PA01};
use crate::time::{Hertz, U32Ext};

use super::dpll::{DpllSource, DpllSourceMarker, DpllSrc};
use super::gclk::{GclkSource, GclkSourceMarker, GenNum};
use super::gclkio::NotGclkInput;
use super::RtcClock;
use crate::typelevel::Sealed;

//==============================================================================
// Xosc32kToken
//==============================================================================

pub struct Xosc32kToken;

impl Xosc32kToken {
    /// Create a new instance of [`Xosc32kToken`]
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
    fn set_1k_output(&mut self, enabled: bool) {
        self.xosc32k().modify(|_, w| w.en1k().bit(enabled));
    }

    #[inline]
    fn set_32k_output(&mut self, enabled: bool) {
        self.xosc32k().modify(|_, w| w.en32k().bit(enabled));
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

/// For how long should the clock output be masked to prevent
/// unstable clocking output
pub type StartUp = STARTUP_A;

/// Pin alias type to ensure proper GPIO is used
pub type XIn32 = Pin<PA00, FloatingDisabled>;

/// Pin alias type to ensure proper GPIO is used
pub type XOut32 = Pin<PA01, FloatingDisabled>;

//==============================================================================
// Mode structure for Xosc32k
//==============================================================================

pub trait Mode: Sealed {}

pub struct ClockMode {}
impl Mode for ClockMode {}
impl Sealed for ClockMode {}

pub struct CrystalMode {
    xout32: XOut32,
    /// Control  external crystal tuning
    control_gain_mode_high_speed: bool,
}
impl Mode for CrystalMode {}
impl Sealed for CrystalMode {}

//==============================================================================
// Xosc32k
//==============================================================================

pub struct Xosc32k<M>
where
    M: Mode,
{
    token: Xosc32kToken,
    mode: M,
    xin32: XIn32,
}

impl<M: Mode> Xosc32k<M> {
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
    pub fn set_1k_output(mut self, enabled: bool) -> Self {
        self.token.set_1k_output(enabled);
        self
    }

    /// TODO
    #[inline]
    pub fn set_32k_output(mut self, enabled: bool) -> Self {
        self.token.set_32k_output(enabled);
        self
    }

    /// TODO
    #[inline]
    pub fn wait_ready(&self) {
        self.token.wait_ready();
    }

    /// Lock the Xosc32k configuration
    ///
    /// Locked until a Power-On Reset (POR) is detected.
    ///
    /// TODO
    #[inline]
    pub fn write_lock(mut self) -> Xosc32k<M> {
        self.token.wrtlock();
        self
    }

    /// Return the output frequency
    #[inline]
    pub fn freq(&self) -> Hertz {
        32768.hz()
    }
}

impl Xosc32k<ClockMode> {
    /// TODO
    #[inline]
    pub fn from_clock(token: Xosc32kToken, xin32: impl AnyPin<Id = PA00>) -> Self {
        // Configure input pin
        let xin32 = xin32.into().into_floating_disabled();
        Self {
            token,
            mode: ClockMode {},
            xin32,
        }
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> Enabled<Xosc32k<ClockMode>, U0> {
        self.token.from_clock();
        // TODO
        // When a Xosc32k is enabled, the 32k output should also be enabled,
        // otherwise the freq() function is invalid
        self.token.set_32k_output(true);
        self.token.enable();
        Enabled::new(self)
    }

    /// TODO
    #[inline]
    pub fn free(self) -> (Xosc32kToken, XIn32) {
        (self.token, self.xin32)
    }
}

impl Xosc32k<CrystalMode> {
    /// TODO
    #[inline]
    pub fn from_crystal(
        token: Xosc32kToken,
        xin32: impl AnyPin<Id = PA00>,
        xout32: impl AnyPin<Id = PA01>,
    ) -> Self {
        // Configure input pins
        let xin32 = xin32.into().into_floating_disabled();
        let xout32 = xout32.into().into_floating_disabled();
        // Set to default reset value
        let control_gain_mode_high_speed = false;
        Self {
            token,
            xin32,
            mode: CrystalMode {
                xout32,
                control_gain_mode_high_speed,
            },
        }
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> Enabled<Xosc32k<CrystalMode>, U0> {
        self.token.from_crystal();
        self.token
            .set_gain_mode(self.mode.control_gain_mode_high_speed);
        // TODO
        // When a Xosc32k is enabled, the 32k output should also be enabled,
        // otherwise the freq() function is invalid
        self.token.set_32k_output(true);
        self.token.enable();
        Enabled::new(self)
    }

    /// TODO
    #[inline]
    pub fn set_gain_mode(mut self, high_speed: bool) {
        self.mode.control_gain_mode_high_speed = high_speed;
    }

    /// TODO
    #[inline]
    pub fn free(self) -> (Xosc32kToken, XIn32, XOut32) {
        (self.token, self.xin32, self.mode.xout32)
    }
}

impl<M: Mode> Enabled<Xosc32k<M>, U0> {
    /// TODO
    #[inline]
    pub fn disable(mut self) -> Xosc32k<M> {
        self.0.token.disable();
        self.0
    }
}

//==============================================================================
// GclkSource
//==============================================================================

pub enum Osc32k {}

impl Sealed for Osc32k {}

impl SourceMarker for Osc32k {}

impl GclkSourceMarker for Osc32k {
    const GCLK_SRC: SRC_A = SRC_A::XOSC32K;
}

impl NotGclkInput for Osc32k {}

impl<G, M, N> GclkSource<G> for Enabled<Xosc32k<M>, N>
where
    G: GenNum,
    M: Mode,
    N: Counter,
{
    type Type = Osc32k;
}

//==============================================================================
// DpllSource
//==============================================================================

impl DpllSourceMarker for Osc32k {
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC32;
}

impl<M, N> DpllSource for Enabled<Xosc32k<M>, N>
where
    M: Mode,
    N: Counter,
{
    type Type = Osc32k;
}

//==============================================================================
// Source
//==============================================================================

impl<M, N> Source for Enabled<Xosc32k<M>, N>
where
    M: Mode,
    N: Counter,
{
    #[inline]
    fn freq(&self) -> Hertz {
        32768.hz()
    }
}

//==============================================================================
// RtcClock
//==============================================================================

impl<M: Mode> RtcClock for Xosc32k<M> {
    #[inline]
    fn enable_1k(&mut self) -> RTCSEL_A {
        self.token.set_1k_output(true);
        RTCSEL_A::XOSC1K
    }

    #[inline]
    fn enable_32k(&mut self) -> RTCSEL_A {
        self.token.set_32k_output(true);
        RTCSEL_A::XOSC32K
    }
}
