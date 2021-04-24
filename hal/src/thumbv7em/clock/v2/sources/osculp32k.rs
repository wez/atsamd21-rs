use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::osc32kctrl::OSCULP32K;
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;

use crate::time::{Hertz, U32Ext};
use crate::typelevel::Sealed;

use super::super::RtcClock;
use super::super::gclk::{GenNum, GclkSource, GclkSourceType};

//==============================================================================
// Registers
//==============================================================================

pub struct Registers {
    __: (),
}

impl Registers {
    /// TODO
    #[inline]
    unsafe fn new() -> Self {
        Self { __: () }
    }

    #[inline]
    fn osculp32k(&self) -> &OSCULP32K {
        unsafe { &(*crate::pac::OSC32KCTRL::ptr()).osculp32k }
    }

    #[inline]
    fn set_calib(&mut self, calib: u8) {
        self.osculp32k().modify(|_, w| unsafe { w.calib().bits(calib & 0x3F) });
    }

    #[inline]
    fn enable_1k(&mut self, enable: bool) {
        self.osculp32k().modify(|_, w| w.en1k().bit(enable));
    }

    #[inline]
    fn enable_32k(&mut self, enable: bool) {
        self.osculp32k().modify(|_, w| w.en32k().bit(enable));
    }
}

//==============================================================================
// OscUlp32k
//==============================================================================

pub struct OscUlp32k {
    regs: Registers,
}

impl OscUlp32k {
    /// TODO
    #[inline]
    pub(super) unsafe fn new() -> Self {
        let regs = Registers::new();
        Self { regs }
    }
}

impl OscUlp32k {
    /// TODO
    #[inline]
    pub fn set_calib(&mut self, calib: u8) {
        self.regs.set_calib(calib);
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
}

impl Sealed for OscUlp32k {}

//==============================================================================
// GclkSource
//==============================================================================

pub enum Ulp32k {}

impl Sealed for Ulp32k {}

impl GclkSourceType for Ulp32k {
    const GCLK_SRC: SRC_A = SRC_A::OSCULP32K;
}

impl<G: GenNum> GclkSource<G> for OscUlp32k {
    type Type = Ulp32k;

    #[inline]
    fn freq(&self) -> Hertz {
        32768.hz()
    }
}

//==============================================================================
// RtcClock
//==============================================================================

impl RtcClock for OscUlp32k {
    #[inline]
    fn enable_1k(&mut self) -> RTCSEL_A {
        self.enable_1k(true);
        RTCSEL_A::ULP1K
    }

    #[inline]
    fn enable_32k(&mut self) -> RTCSEL_A {
        self.enable_32k(true);
        RTCSEL_A::ULP32K
    }
}