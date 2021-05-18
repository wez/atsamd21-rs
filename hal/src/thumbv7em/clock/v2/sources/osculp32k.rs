use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::osc32kctrl::OSCULP32K;

use crate::time::{Hertz, U32Ext};
use crate::typelevel::{Count, Decrement, Increment, Lockable, One, Sealed, Unlockable, Zero};

use super::super::gclk::{GclkSource, GclkSourceType, GenNum};
use super::super::RtcClock;

//==============================================================================
// Registers
//==============================================================================

pub type OscUlp32kToken = Registers;

pub struct Registers;

impl Registers {
    /// TODO
    #[inline]
    pub(super) unsafe fn new() -> Self {
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
// OscUlp32kConfig
//==============================================================================

pub struct OscUlp32kConfig {
    token: Registers,
}

impl OscUlp32kConfig {
    /// TODO
    #[inline]
    pub fn new(token: OscUlp32kToken) -> Self {
        Self { token }
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> OscUlp32k {
        // Enable both outputs to recreate the default reset state
        self.token.enable_1k(true);
        self.token.enable_32k(true);

        let count = Zero::new();
        OscUlp32k {
            config: self,
            count,
        }
    }
}

//==============================================================================
// OscUlp32k
//==============================================================================

pub struct OscUlp32k<N = Zero>
where
    N: Count,
{
    config: OscUlp32kConfig,
    count: N,
}

impl OscUlp32k<One> {
    /// TODO
    #[inline]
    pub(crate) unsafe fn init() -> Self {
        let token = OscUlp32kToken::new();
        let config = OscUlp32kConfig::new(token);
        let count = One::new();
        Self { config, count }
    }
}

impl<N: Count> OscUlp32k<N> {
    /// TODO
    #[inline]
    fn create(config: OscUlp32kConfig, count: N) -> Self {
        OscUlp32k { config, count }
    }

    /// TODO
    #[inline]
    pub fn set_calib(mut self, calib: u8) -> Self {
        self.config.token.set_calib(calib);
        self
    }

    /// Control the output of 1 kHz (1024 Hz) clock
    ///
    /// Output enabled at reset
    #[inline]
    pub fn enable_1k(mut self, enable: bool) -> Self {
        self.config.token.enable_1k(enable);
        self
    }

    /// Control the output of 32 kHz (32.768 kHz) clock
    ///
    /// Output enabled at reset
    #[inline]
    pub fn enable_32k(mut self, enable: bool) -> Self {
        self.config.token.enable_32k(enable);
        self
    }

    /// TODO
    #[inline]
    pub fn disable(mut self) -> OscUlp32kConfig {
        self.config.token.enable_1k(false);
        self.config.token.enable_32k(false);
        self.config
    }
}

//==============================================================================
// Lockable
//==============================================================================

impl<N> Lockable for OscUlp32k<N>
where
    N: Increment,
{
    type Locked = OscUlp32k<N::Inc>;
    fn lock(self) -> Self::Locked {
        OscUlp32k::create(self.config, self.count.inc())
    }
}

//==============================================================================
// Unlockable
//==============================================================================

impl<N> Unlockable for OscUlp32k<N>
where
    N: Decrement,
{
    type Unlocked = OscUlp32k<N::Dec>;
    fn unlock(self) -> Self::Unlocked {
        OscUlp32k::create(self.config, self.count.dec())
    }
}

impl<N: Count> Sealed for OscUlp32k<N> {}

//==============================================================================
// GclkSource
//==============================================================================

pub enum Ulp32k {}

impl Sealed for Ulp32k {}

impl GclkSourceType for Ulp32k {
    const GCLK_SRC: SRC_A = SRC_A::OSCULP32K;
}

impl<G: GenNum, N: Count> GclkSource<G> for OscUlp32k<N> {
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
        self.config.token.enable_1k(true);
        RTCSEL_A::ULP1K
    }

    #[inline]
    fn enable_32k(&mut self) -> RTCSEL_A {
        self.config.token.enable_32k(true);
        RTCSEL_A::ULP32K
    }
}
