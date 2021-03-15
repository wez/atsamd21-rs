use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::osc32kctrl::OSCULP32K;
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;

use crate::time::{Hertz, U32Ext};
use crate::typelevel::Sealed;

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
    fn osculp32k(&self) -> *const OSCULP32K {
        unsafe { &(*crate::pac::OSC32KCTRL::ptr()).osculp32k }
    }

    #[inline]
    fn osculp32k_mut(&mut self) -> *mut OSCULP32K {
        self.osculp32k() as *mut _
    }

    #[inline]
    fn set_calib(&mut self, calib: u8) {
        let osculp32k = self.osculp32k_mut();
        unsafe { (*osculp32k).modify(|_, w| w.calib().bits(calib & 0x3F)) };
    }

    #[inline]
    fn enable_1k(&mut self, enable: bool) {
        let osculp32k = self.osculp32k_mut();
        unsafe { (*osculp32k).modify(|_, w| w.en1k().bit(enable)) };
    }

    #[inline]
    fn enable_32k(&mut self, enable: bool) {
        let osculp32k = self.osculp32k_mut();
        unsafe { (*osculp32k).modify(|_, w| w.en32k().bit(enable)) };
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

//==============================================================================
// SourceType
//==============================================================================

impl Sealed for OscUlp32k {}

impl SourceType for OscUlp32k {
    const GCLK_SRC: SRC_A = SRC_A::OSCULP32K;

    #[inline]
    fn freq(&self) -> Hertz {
        32768.hz()
    }
}

impl<G: GenNum> SourceForGclk<G> for OscUlp32k {}

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