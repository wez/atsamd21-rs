
use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::osc32kctrl::{RegisterBlock, STATUS, XOSC32K};
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::osc32kctrl::xosc32k::{CGM_A, STARTUP_A};

use crate::gpio::v2::{AnyPin, FloatingDisabled, OptionalPin, Pin, PA00, PA01};
use crate::time::{Hertz, U32Ext};
use crate::typelevel::{NoneT, Sealed};

use super::super::RtcClock;
use super::super::gclk::GenNum;
use super::{SourceForGclk, SourceType};

//==============================================================================
// Registers
//==============================================================================

struct Registers;

impl Registers {
    /// TODO
    #[inline]
    unsafe fn new() -> Self {
        Self
    }

    #[inline]
    fn osc32kctrl(&self) -> *const RegisterBlock {
        crate::pac::OSC32KCTRL::ptr()
    }

    #[inline]
    fn status(&self) -> *const STATUS {
        unsafe { &(*self.osc32kctrl()).status as *const _ }
    }

    #[inline]
    fn xosc32k(&self) -> *const XOSC32K {
        unsafe { &(*self.osc32kctrl()).xosc32k as *const _ }
    }

    #[inline]
    fn xosc32k_mut(&mut self) -> *mut XOSC32K {
        self.xosc32k() as *mut _
    }

    #[inline]
    fn set_gain_mode(&mut self, high_speed: bool) {
        let xosc32k = self.xosc32k_mut();
        let variant = match high_speed {
            false => CGM_A::XT,
            true => CGM_A::HS,
        };
        unsafe { (*xosc32k).modify(|_, w| w.cgm().variant(variant)) };
    }

    #[inline]
    fn set_start_up(&mut self, start_up: StartUp) {
        let xosc32k = self.xosc32k_mut();
        unsafe { (*xosc32k).modify(|_, w| w.startup().variant(start_up)) };
    }

    #[inline]
    fn set_on_demand(&mut self, on_demand: bool) {
        let xosc32k = self.xosc32k_mut();
        unsafe { (*xosc32k).modify(|_, w| w.ondemand().bit(on_demand)) };
    }

    #[inline]
    fn set_run_standby(&mut self, run_standby: bool) {
        let xosc32k = self.xosc32k_mut();
        unsafe { (*xosc32k).modify(|_, w| w.runstdby().bit(run_standby)) };
    }

    #[inline]
    fn enable_1k(&mut self, enable: bool) {
        let xosc32k = self.xosc32k_mut();
        unsafe { (*xosc32k).modify(|_, w| w.en1k().bit(enable)) };
    }

    #[inline]
    fn enable_32k(&mut self, enable: bool) {
        let xosc32k = self.xosc32k_mut();
        unsafe { (*xosc32k).modify(|_, w| w.en32k().bit(enable)) };
    }

    #[inline]
    fn from_clock(&mut self) {
        let xosc32k = self.xosc32k_mut();
        unsafe { (*xosc32k).modify(|_, w| w.xtalen().bit(false)) };
    }

    #[inline]
    fn from_crystal(&mut self) {
        let xosc32k = self.xosc32k_mut();
        unsafe { (*xosc32k).modify(|_, w| w.xtalen().bit(true)) };
    }

    #[inline]
    fn enable(&mut self) {
        let xosc32k = self.xosc32k_mut();
        unsafe { (*xosc32k).modify(|_, w| w.enable().bit(true)) };
    }

    #[inline]
    fn disable(&mut self) {
        let xosc32k = self.xosc32k_mut();
        unsafe { (*xosc32k).modify(|_, w| w.enable().bit(false)) };
    }

    #[inline]
    fn wait_ready(&self) {
        let status = self.status();
        unsafe { while (*status).read().xosc32krdy().bit_is_clear() {} }
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
// XOsc32kConfig
//==============================================================================

pub struct XOsc32kConfig<P = NoneT>
where
    P: OptionalPin,
{
    regs: Registers,
    xin32: XIn32,
    xout32: P,
}

impl XOsc32kConfig {
    /// TODO
    #[inline]
    pub fn from_clock(xin32: impl AnyPin<Id = PA00>) -> Self {
        let xin32 = xin32.into().into_floating_disabled();
        // TODO
        let mut regs = unsafe { Registers::new() };
        regs.from_clock();
        Self {
            regs,
            xin32,
            xout32: NoneT,
        }
    }

    /// TODO
    #[inline]
    pub fn free(self) -> XIn32 {
        self.xin32
    }
}

impl XOsc32kConfig<XOut32> {
    /// TODO
    #[inline]
    pub fn from_crystal(xin32: impl AnyPin<Id = PA00>, xout32: impl AnyPin<Id = PA01>) -> Self {
        let xin32 = xin32.into().into_floating_disabled();
        let xout32 = xout32.into().into_floating_disabled();
        // TODO
        let mut regs = unsafe { Registers::new() };
        regs.from_crystal();
        Self {
            regs,
            xin32,
            xout32,
        }
    }

    /// TODO
    #[inline]
    pub fn free(self) -> (XIn32, XOut32) {
        (self.xin32, self.xout32)
    }
}

impl<P: OptionalPin> XOsc32kConfig<P> {
    /// TODO
    #[inline]
    pub fn set_gain_mode(&mut self, high_speed: bool) {
        self.regs.set_gain_mode(high_speed);
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
    pub fn enable_1k(&mut self, enable: bool) {
        self.regs.enable_1k(enable);
    }

    /// TODO
    #[inline]
    pub fn enable_32k(&mut self, enable: bool) {
        self.regs.enable_32k(enable);
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> XOsc32k<P> {
        self.regs.enable();
        self.regs.wait_ready();
        XOsc32k { config: self }
    }
}

//==============================================================================
// XOsc32k
//==============================================================================

pub struct XOsc32k<P = NoneT>
where
    P: OptionalPin,
{
    config: XOsc32kConfig<P>,
}

impl<P: OptionalPin> XOsc32k<P> {
    /// TODO
    #[inline]
    pub fn disable(mut self) -> XOsc32kConfig<P> {
        self.config.regs.disable();
        self.config
    }

    /// TODO
    #[inline]
    pub fn enable_1k(&mut self, enable: bool) {
        self.config.regs.enable_1k(enable);
    }

    /// TODO
    #[inline]
    pub fn enable_32k(&mut self, enable: bool) {
        self.config.regs.enable_32k(enable);
    }

}

//==============================================================================
// SourceType
//==============================================================================

impl<P: OptionalPin> Sealed for XOsc32k<P> {}

impl<P: OptionalPin> SourceType for XOsc32k<P> {
    const GCLK_SRC: SRC_A = SRC_A::XOSC32K;

    #[inline]
    fn freq(&self) -> Hertz {
        32768.hz()
    }
}

impl<P, G> SourceForGclk<G> for XOsc32k<P>
where
    P: OptionalPin,
    G: GenNum,
{
}

//==============================================================================
// RtcClock
//==============================================================================

impl<P: OptionalPin> RtcClock for XOsc32k<P> {
    #[inline]
    fn enable_1k(&mut self) -> RTCSEL_A {
        self.enable_1k(true);
        RTCSEL_A::XOSC1K
    }

    #[inline]
    fn enable_32k(&mut self) -> RTCSEL_A {
        self.enable_32k(true);
        RTCSEL_A::XOSC32K
    }
}