//! TODO

use typenum::U0;

use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::osc32kctrl::OSCULP32K;

use crate::clock::types::{Counter, Enabled};
use crate::clock::v2::{Source, SourceMarker};
use crate::time::{Hertz, U32Ext};
use crate::typelevel::Sealed;

use super::gclk::{GclkSource, GclkSourceMarker, GenNum};
use super::gclkio::NotGclkInput;
use super::RtcClock;

//==============================================================================
// OscUlp32kToken
//==============================================================================

pub struct OscUlp32kToken;

impl OscUlp32kToken {
    /// TODO
    #[inline]
    pub(crate) unsafe fn new() -> Self {
        Self
    }

    #[inline]
    fn osculp32k(&self) -> &OSCULP32K {
        unsafe { &(*crate::pac::OSC32KCTRL::ptr()).osculp32k }
    }

    #[inline]
    fn set_calib(&mut self, calib: u8) {
        self.osculp32k()
            .modify(|_, w| unsafe { w.calib().bits(calib & 0x3F) });
    }

    #[inline]
    fn enable_1k(&mut self, enable: bool) {
        self.osculp32k().modify(|_, w| w.en1k().bit(enable));
    }

    #[inline]
    fn enable_32k(&mut self, enable: bool) {
        self.osculp32k().modify(|_, w| w.en32k().bit(enable));
    }

    #[inline]
    fn wrtlock(&mut self) {
        self.osculp32k().modify(|_, w| w.wrtlock().bit(true));
    }
}

//==============================================================================
// OscUlp32k
//==============================================================================

pub struct OscUlp32k {
    token: OscUlp32kToken,
}

impl OscUlp32k {
    /// TODO
    #[inline]
    pub(crate) fn new(token: OscUlp32kToken) -> Self {
        Self { token }
    }

    /// TODO
    #[inline]
    pub fn enable(self) -> Enabled<OscUlp32k, U0> {
        Enabled::new(self)
    }

    /// TODO
    #[inline]
    pub fn set_calib(mut self, calib: u8) -> Self {
        self.token.set_calib(calib);
        self
    }

    /// Control the output of 1 kHz (1024 Hz) clock
    ///
    /// Output enabled at reset
    #[inline]
    pub fn enable_1k(mut self, enable: bool) -> Self {
        self.token.enable_1k(enable);
        self
    }

    /// Control the output of 32 kHz (32.768 kHz) clock
    ///
    /// Output enabled at reset
    #[inline]
    pub fn enable_32k(mut self, enable: bool) -> Self {
        self.token.enable_32k(enable);
        self
    }
    /// Write lock the OscUlp32k
    #[inline]
    pub fn write_lock(mut self) -> Enabled<Self, U0> {
        self.token.wrtlock();
        Enabled::new(self)
    }
}

//==============================================================================
// GclkSource
//==============================================================================

pub enum Ulp32k {}

impl Sealed for Ulp32k {}

impl SourceMarker for Ulp32k {}

impl GclkSourceMarker for Ulp32k {
    const GCLK_SRC: SRC_A = SRC_A::OSCULP32K;
}

impl NotGclkInput for Ulp32k {}

impl<G: GenNum, N: Counter> GclkSource<G> for Enabled<OscUlp32k, N> {
    type Type = Ulp32k;
}

impl<N: Counter> Source for Enabled<OscUlp32k, N> {
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
        self.token.enable_1k(true);
        RTCSEL_A::ULP1K
    }

    #[inline]
    fn enable_32k(&mut self) -> RTCSEL_A {
        self.token.enable_32k(true);
        RTCSEL_A::ULP32K
    }
}
