//! # Xosc32k - External Oscillator 32 kHz
//! TODO
//!
//! Provides 32 kHz outputs for [`gclk`][super::gclk]s, [`rtc`][super::rtc]
//! and [`dpll`][super::dpll].
//! Additionally provides 1 kHz output for the [`rtc`][super::rtc] module.
//!
//! Independently controllable outputs for 32 kHz and 1 kHz
//!
//! See:
//! * [`Enabled<Xosc32k>::activate_32k`]
//! * [`Enabled<Xosc32k>::activate_1k`]

use core::marker::PhantomData;
use typenum::U0;

use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::osc32kctrl::xosc32k::{CGM_A, STARTUP_A};

use crate::pac::osc32kctrl::{RegisterBlock, STATUS, XOSC32K};

use crate::clock::types::{Counter, Enabled, PrivateIncrement};
use crate::clock::v2::{Source, SourceMarker};
use crate::gpio::v2::{AnyPin, FloatingDisabled, Pin, PA00, PA01};
use crate::time::{Hertz, U32Ext};

use super::dpll::{DpllSource, DpllSourceMarker, DpllSrc};
use super::gclk::{GclkSource, GclkSourceMarker, GenNum};
use super::gclkio::NotGclkInput;
use super::rtc::*;
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
    fn set_start_up(&mut self, start_up: StartUp32k) {
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
    fn activate_1k(&mut self, enabled: bool) {
        self.xosc32k().modify(|_, w| w.en1k().bit(enabled));
    }

    #[inline]
    fn activate_32k(&mut self, enabled: bool) {
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
pub type StartUp32k = STARTUP_A;

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
    /// Control external crystal tuning
    control_gain_mode_high_speed: bool,
}
impl Mode for CrystalMode {}
impl Sealed for CrystalMode {}

//==============================================================================
// Xosc32k
//==============================================================================

pub struct Xosc32k<M, X, Y>
where
    M: Mode,
    X: Output32k,
    Y: Output1k,
{
    token: Xosc32kToken,
    xin32: XIn32,
    mode: M,
    run_standby: bool,
    on_demand_mode: bool,
    start_up_masking: StartUp32k,
    output32k: PhantomData<X>,
    output1k: PhantomData<Y>,
}

impl<M, X, Y> Xosc32k<M, X, Y>
where
    M: Mode,
    X: Output32k,
    Y: Output1k,
{
    /// Set for how long the clock output should be masked during startup
    #[inline]
    pub fn set_start_up(mut self, start_up: StartUp32k) -> Self {
        self.start_up_masking = start_up;
        self
    }

    /// Controls how Xosc32k behaves when a peripheral clock request is detected
    #[inline]
    pub fn set_on_demand(mut self, on_demand: bool) -> Self {
        self.on_demand_mode = on_demand;
        self
    }

    /// Controls how Xosc32k should behave during standby
    #[inline]
    pub fn set_run_standby(mut self, run_standby: bool) -> Self {
        self.run_standby = run_standby;
        self
    }
    /// Busy wait until the clock source is ready
    #[inline]
    pub fn wait_ready(&self) {
        self.token.wait_ready();
    }
}

impl Xosc32k<ClockMode, Inactive32k, Inactive1k> {
    /// Construct a Xosc32k from a single pin oscillator clock signal
    #[inline]
    pub fn from_clock(token: Xosc32kToken, xin32: impl AnyPin<Id = PA00>) -> Self {
        // Configure input pin
        let xin32 = xin32.into().into_floating_disabled();
        Self {
            token,
            xin32,
            mode: ClockMode {},
            run_standby: false,
            on_demand_mode: true,
            start_up_masking: StartUp32k::CYCLE2048,
            output32k: PhantomData,
            output1k: PhantomData,
        }
    }

    /// Enable the Xosc32k, allowing it to be used by other peripherals
    ///
    /// To output a 32 kHz clock signal the output must be activated with
    /// the method: [`Enabled<Xosc32k>::activate_32k`]
    ///
    #[inline]
    pub fn enable(mut self) -> Enabled<Self, U0> {
        self.token.from_clock();
        self.token.set_on_demand(self.on_demand_mode);
        self.token.set_run_standby(self.run_standby);
        self.token.set_start_up(self.start_up_masking);
        self.token.enable();
        Enabled::new(self)
    }
}

impl<X, Y> Xosc32k<ClockMode, X, Y>
where
    X: Output32k,
    Y: Output1k,
{
    /// Deconstruct the Xosc32k into a Xosc32kToken and the associated GPIO pin
    #[inline]
    pub fn free(self) -> (Xosc32kToken, XIn32) {
        (self.token, self.xin32)
    }
}

impl Xosc32k<CrystalMode, Inactive32k, Inactive1k> {
    /// Construct a Xosc32k from a two pin crystal oscillator signal
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
            run_standby: false,
            on_demand_mode: true,
            start_up_masking: StartUp32k::CYCLE2048,
            output32k: PhantomData,
            output1k: PhantomData,
        }
    }

    /// Crystal oscillator gain
    ///
    /// Pick between high speed or low power consumption
    #[inline]
    pub fn set_gain_mode(mut self, high_speed: bool) -> Self {
        self.mode.control_gain_mode_high_speed = high_speed;
        self
    }

    /// Enable the Xosc32k, allowing it to be used by other peripherals
    ///
    /// To output a 32 kHz clock signal the output must be activated with
    /// the method: [`Enabled<Xosc32k>::activate_32k`]
    #[inline]
    pub fn enable(mut self) -> Enabled<Self, U0> {
        self.token.from_crystal();
        self.token.set_on_demand(self.on_demand_mode);
        self.token.set_run_standby(self.run_standby);
        self.token.set_start_up(self.start_up_masking);

        self.token
            .set_gain_mode(self.mode.control_gain_mode_high_speed);
        self.token.enable();
        Enabled::new(self)
    }
}

impl<X, Y> Xosc32k<CrystalMode, X, Y>
where
    X: Output32k,
    Y: Output1k,
{
    /// Deconstruct the Xosc32k into a Xosc32kToken and the two associated GPIO pins
    #[inline]
    pub fn free(self) -> (Xosc32kToken, XIn32, XOut32) {
        (self.token, self.xin32, self.mode.xout32)
    }
}

impl<M, Y> Enabled<Xosc32k<M, Inactive32k, Y>, U0>
where
    M: Mode,
    Y: Output1k,
{
    /// Enable the output of 32 kHz clock
    #[inline]
    pub fn activate_32k(mut self) -> Enabled<Xosc32k<M, Active32k, Y>, U0> {
        self.0.token.activate_32k(true);
        let xosc32k = Xosc32k {
            token: self.0.token,
            xin32: self.0.xin32,
            mode: self.0.mode,
            run_standby: self.0.run_standby,
            on_demand_mode: self.0.on_demand_mode,
            start_up_masking: self.0.start_up_masking,
            output32k: PhantomData,
            output1k: self.0.output1k,
        };
        Enabled::new(xosc32k)
    }
}

impl<M, Y> Enabled<Xosc32k<M, Active32k, Y>, U0>
where
    M: Mode,
    Y: Output1k,
{
    /// Disable the output of 32 kHz clock
    #[inline]
    pub fn deactivate_32k(mut self) -> Enabled<Xosc32k<M, Inactive32k, Y>, U0> {
        self.0.token.activate_32k(false);
        let xosc32k = Xosc32k {
            token: self.0.token,
            xin32: self.0.xin32,
            mode: self.0.mode,
            run_standby: self.0.run_standby,
            on_demand_mode: self.0.on_demand_mode,
            start_up_masking: self.0.start_up_masking,
            output32k: PhantomData,
            output1k: self.0.output1k,
        };
        Enabled::new(xosc32k)
    }
}

impl<M, X> Enabled<Xosc32k<M, X, Inactive1k>, U0>
where
    M: Mode,
    X: Output32k,
{
    /// Enable the output of 1 kHz (1024 Hz) clock
    ///
    /// Used by RTC only
    #[inline]
    pub fn activate_1k(mut self) -> Enabled<Xosc32k<M, X, Active1k>, U0> {
        self.0.token.activate_1k(true);
        let xosc32k = Xosc32k {
            token: self.0.token,
            xin32: self.0.xin32,
            mode: self.0.mode,
            run_standby: self.0.run_standby,
            on_demand_mode: self.0.on_demand_mode,
            start_up_masking: self.0.start_up_masking,
            output32k: self.0.output32k,
            output1k: PhantomData,
        };
        Enabled::new(xosc32k)
    }
}

impl<M, X> Enabled<Xosc32k<M, X, Active1k>, U0>
where
    M: Mode,
    X: Output32k,
{
    /// Disable the output of 1 kHz (1024 Hz) clock
    ///
    /// Used by RTC only
    #[inline]
    pub fn deactivate_1k(mut self) -> Enabled<Xosc32k<M, X, Inactive1k>, U0> {
        self.0.token.activate_1k(false);
        let xosc32k = Xosc32k {
            token: self.0.token,
            xin32: self.0.xin32,
            mode: self.0.mode,
            run_standby: self.0.run_standby,
            on_demand_mode: self.0.on_demand_mode,
            start_up_masking: self.0.start_up_masking,
            output32k: self.0.output32k,
            output1k: PhantomData,
        };
        Enabled::new(xosc32k)
    }
}

impl<M, X, Y, N> Enabled<Xosc32k<M, X, Y>, N>
where
    M: Mode,
    X: Output32k,
    Y: Output1k,
    N: Counter + PrivateIncrement,
{
    /// Write lock the Xosc32k
    ///
    /// Locked until a Power-On Reset (POR) is detected.
    ///
    /// TODO, how should we model the hardware write lock?
    /// For now artificially raise the use counter by 1
    #[inline]
    pub fn write_lock(mut self) -> <Self as PrivateIncrement>::Inc {
        self.0.token.wrtlock();
        self.inc()
    }
}

impl<M, X, Y> Enabled<Xosc32k<M, X, Y>, U0>
where
    M: Mode,
    X: Output32k,
    Y: Output1k,
{
    /// Disable the enabled Xosc32k
    ///
    /// Only possible with no users
    ///
    /// This allows changing the configuration or retrieving the Xosc32kToken
    /// and GPIO pin
    #[inline]
    pub fn disable(mut self) -> Xosc32k<M, X, Y> {
        self.0.token.activate_32k(false);
        self.0.token.activate_1k(false);
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

impl<G, M, Y, N> GclkSource<G> for Enabled<Xosc32k<M, Active32k, Y>, N>
where
    G: GenNum,
    M: Mode,
    Y: Output1k,
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

impl<M, Y, N> DpllSource for Enabled<Xosc32k<M, Active32k, Y>, N>
where
    M: Mode,
    Y: Output1k,
    N: Counter,
{
    type Type = Osc32k;
}

//==============================================================================
// Source
//==============================================================================

impl<M, Y, N> Source for Enabled<Xosc32k<M, Active32k, Y>, N>
where
    M: Mode,
    Y: Output1k,
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

impl<M, Y, N> RtcSource32k for Enabled<Xosc32k<M, Active32k, Y>, N>
where
    M: Mode,
    Y: Output1k,
    N: Counter,
{
    const RTC_SRC_32K: RTCSEL_A = RTCSEL_A::XOSC32K;
}

impl<M, X, N> RtcSource1k for Enabled<Xosc32k<M, X, Active1k>, N>
where
    M: Mode,
    X: Output32k,
    N: Counter,
{
    const RTC_SRC_1K: RTCSEL_A = RTCSEL_A::XOSC1K;
}
