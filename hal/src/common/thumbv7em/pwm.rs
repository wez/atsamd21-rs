use crate::clock;
use crate::gpio::*;
use crate::hal::{Pwm, PwmPin};
use crate::timer::TimerParams;
use embedded_time::rate::Hertz;

use crate::target_device::{MCLK, TC0, TC1, TC2, TC3, TCC0, TCC1, TCC2};
#[cfg(feature = "min-samd51j")]
use crate::target_device::{TC4, TC5, TCC3, TCC4};
#[cfg(feature = "min-samd51p")]
use crate::target_device::{TC6, TC7};

// Timer/Counter (TCx)

pub enum TC0Pinout {
    Pa5(Pa5<PfE>),
    Pa9(Pa9<PfE>),
    Pb31(Pb31<PfE>),
}

pub enum TC1Pinout {
    Pa7(Pa7<PfE>),
    Pa11(Pa11<PfE>),
}

pub enum TC2Pinout {
    Pa1(Pa1<PfE>),
    Pa13(Pa13<PfE>),
    Pa17(Pa17<PfE>),
}

pub enum TC3Pinout {
    Pa15(Pa15<PfE>),
    Pa19(Pa19<PfE>),
}

#[cfg(feature = "min-samd51j")]
pub enum TC4Pinout {
    Pa23(Pa23<PfE>),
    Pb9(Pb9<PfE>),
    Pb13(Pb13<PfE>),
}

#[cfg(feature = "min-samd51j")]
pub enum TC5Pinout {
    Pa25(Pa25<PfE>),
    Pb11(Pb11<PfE>),
    Pb15(Pb15<PfE>),
}

#[cfg(feature = "min-samd51n")]
pub enum TC6Pinout {
    Pb3(Pb3<PfE>),
    Pb17(Pb17<PfE>),
    Pa31(Pa31<PfE>),
}

#[cfg(feature = "min-samd51n")]
pub enum TC7Pinout {
    Pa21(Pa21<PfE>),
    Pb23(Pb23<PfE>),
    Pb1(Pb1<PfE>),
}

macro_rules! pwm {
    ($($TYPE:ident: ($TC:ident, $pinout:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident),)+) => {
        $(

pub struct $TYPE {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tc: $TC,
    #[allow(dead_code)]
    pinout: $pinout,
}

impl $TYPE {
    pub fn new<F: Into<Hertz>> (
        clock: &clock::$clock,
        freq: F,
        tc: $TC,
        pinout: $pinout,
        mclk: &mut MCLK,
    ) -> Self {
        {
            let count = tc.count16();

            let params = TimerParams::new(freq, clock.freq());

            mclk.$apmask.modify(|_, w| w.$apbits().set_bit());
            count.ctrla.write(|w| w.swrst().set_bit());
            while count.ctrla.read().bits() & 1 != 0 {}
            count.ctrla.modify(|_, w| w.enable().clear_bit());
            count.ctrla.modify(|_, w| {
                match params.divider {
                    1 => w.prescaler().div1(),
                    2 => w.prescaler().div2(),
                    4 => w.prescaler().div4(),
                    8 => w.prescaler().div8(),
                    16 => w.prescaler().div16(),
                    64 => w.prescaler().div64(),
                    256 => w.prescaler().div256(),
                    1024 => w.prescaler().div1024(),
                    _ => unreachable!(),
                }
            });
            count.wave.write(|w| w.wavegen().mpwm());
            count.cc[0].write(|w| unsafe { w.cc().bits(params.cycles as u16) });
            while count.syncbusy.read().cc0().bit_is_set() {}
            count.cc[1].write(|w| unsafe { w.cc().bits(0) });
            while count.syncbusy.read().cc1().bit_is_set() {}
            count.ctrla.modify(|_, w| w.enable().set_bit());
        }

        Self {
            clock_freq: clock.freq(),
            tc,
            pinout,
        }
    }

    pub fn get_period(&self) -> Hertz {
        let count = self.tc.count16();
        let divisor = count.ctrla.read().prescaler().bits();
        let top = count.cc[0].read().cc().bits();
        Hertz(self.clock_freq.0 / divisor as u32 / (top + 1) as u32)
    }

    pub fn set_period<P>(&mut self, period: P)
    where
        P: Into<Hertz>
    {
        let params = TimerParams::new(period, self.clock_freq);

        let count = self.tc.count16();
        count.ctrla.modify(|_, w| w.enable().clear_bit());
        count.ctrla.modify(|_, w| {
                match params.divider {
                    1 => w.prescaler().div1(),
                    2 => w.prescaler().div2(),
                    4 => w.prescaler().div4(),
                    8 => w.prescaler().div8(),
                    16 => w.prescaler().div16(),
                    64 => w.prescaler().div64(),
                    256 => w.prescaler().div256(),
                    1024 => w.prescaler().div1024(),
                    _ => unreachable!(),
                }
            });
        count.ctrla.modify(|_, w| w.enable().set_bit());
        count.cc[0].write(|w| unsafe { w.cc().bits(params.cycles as u16) });
        while count.syncbusy.read().cc0().bit_is_set() {}
    }
}

impl PwmPin for $TYPE {
    type Duty = u16;

    fn disable(&mut self) {
        let count = self.tc.count16();
        count.ctrla.modify(|_, w| w.enable().clear_bit());
    }

    fn enable(&mut self) {
        let count = self.tc.count16();
        count.ctrla.modify(|_, w| w.enable().set_bit());
    }


    fn get_duty(&self) -> Self::Duty {
        let count = self.tc.count16();
        let duty: u16 = count.ccbuf[1].read().ccbuf().bits();
        duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        let count = self.tc.count16();
        let top = count.cc[0].read().cc().bits();
        top
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        let count = self.tc.count16();
        count.ccbuf[1].write(|w| unsafe {w.ccbuf().bits(duty)});
    }
}

)+}}

pwm! {
    Pwm0: (TC0, TC0Pinout, Tc0Tc1Clock, apbamask, tc0_, Pwm0Wrapper),
    Pwm1: (TC1, TC1Pinout, Tc0Tc1Clock, apbamask, tc1_, Pwm1Wrapper),
    Pwm2: (TC2, TC2Pinout, Tc2Tc3Clock, apbbmask, tc2_, Pwm2Wrapper),
    Pwm3: (TC3, TC3Pinout, Tc2Tc3Clock, apbbmask, tc3_, Pwm3Wrapper),
}

#[cfg(feature = "min-samd51j")]
pwm! {
    Pwm4: (TC4, TC4Pinout, Tc4Tc5Clock, apbcmask, tc4_, Pwm4Wrapper),
    Pwm5: (TC5, TC5Pinout, Tc4Tc5Clock, apbcmask, tc5_, Pwm5Wrapper),
}

#[cfg(feature = "min-samd51n")]
pwm! {
    Pwm6: (TC6, TC6Pinout, Tc6Tc7Clock, apbdmask, tc6_, Pwm6Wrapper),
    Pwm7: (TC7, TC7Pinout, Tc6Tc7Clock, apbdmask, tc7_, Pwm7Wrapper),
}

// Timer/Counter for Control Applications (TCCx)

#[derive(Copy, Clone)]
pub enum Channel {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

pub enum TCC0Pinout {
    Pa8(Pa8<PfF>),
    Pa9(Pa9<PfF>),
    Pa10(Pa10<PfF>),
    Pa11(Pa11<PfF>),
    Pa12(Pa12<PfF>),
    Pa13(Pa13<PfF>),
    Pa16(Pa16<PfG>),
    Pa17(Pa17<PfG>),
    Pa18(Pa18<PfG>),
    Pa19(Pa19<PfG>),
    Pa20(Pa20<PfG>),
    Pa21(Pa21<PfG>),
    Pa22(Pa22<PfG>),
    Pa23(Pa23<PfG>),
    Pb10(Pb10<PfF>),
    Pb11(Pb11<PfF>),
    #[cfg(feature = "min-samd51j")]
    Pb12(Pb12<PfG>),
    #[cfg(feature = "min-samd51j")]
    Pb13(Pb13<PfG>),
    #[cfg(feature = "min-samd51j")]
    Pb14(Pb14<PfG>),
    #[cfg(feature = "min-samd51j")]
    Pb15(Pb15<PfG>),
    #[cfg(feature = "min-samd51j")]
    Pb16(Pb16<PfG>),
    #[cfg(feature = "min-samd51j")]
    Pb17(Pb17<PfG>),
    #[cfg(feature = "min-samd51j")]
    Pb30(Pb30<PfG>),
    #[cfg(feature = "min-samd51j")]
    Pb31(Pb31<PfG>),
    #[cfg(feature = "min-samd51n")]
    Pc10(Pc10<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pc11(Pc11<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pc12(Pc12<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pc13(Pc13<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pc14(Pc14<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pc15(Pc15<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pc16(Pc16<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pc17(Pc17<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pc18(Pc18<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pc19(Pc19<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pc20(Pc20<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pc21(Pc21<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pc4(Pc4<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pc22(Pc22<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pc23(Pc23<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pd8(Pd8<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pd9(Pd9<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pd10(Pd10<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pd11(Pd11<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pd12(Pd12<PfF>),
}

pub enum TCC1Pinout {
    Pa8(Pa8<PfG>),
    Pa9(Pa9<PfG>),
    Pa10(Pa10<PfG>),
    Pa11(Pa11<PfG>),
    Pa12(Pa12<PfG>),
    Pa13(Pa13<PfG>),
    Pa14(Pa14<PfG>),
    Pa15(Pa15<PfG>),
    Pa16(Pa16<PfF>),
    Pa17(Pa17<PfF>),
    Pa18(Pa18<PfF>),
    Pa19(Pa19<PfF>),
    Pa20(Pa20<PfF>),
    Pa21(Pa21<PfF>),
    Pa22(Pa22<PfF>),
    Pa23(Pa23<PfF>),
    Pb10(Pb10<PfG>),
    Pb11(Pb11<PfG>),
    #[cfg(feature = "min-samd51n")]
    Pb18(Pb18<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pb19(Pb19<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pb20(Pb20<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pb21(Pb21<PfF>),
    #[cfg(feature = "min-samd51n")]
    Pc10(Pc10<PfG>),
    #[cfg(feature = "min-samd51n")]
    Pc11(Pc11<PfG>),
    #[cfg(feature = "min-samd51n")]
    Pc12(Pc12<PfG>),
    #[cfg(feature = "min-samd51n")]
    Pc13(Pc13<PfG>),
    #[cfg(feature = "min-samd51n")]
    Pc14(Pc14<PfG>),
    #[cfg(feature = "min-samd51n")]
    Pc15(Pc15<PfG>),
    #[cfg(feature = "min-samd51p")]
    Pb26(Pb26<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pb27(Pb27<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pb28(Pb28<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pb29(Pb29<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pd20(Pd20<PfF>),
    #[cfg(feature = "min-samd51p")]
    Pd21(Pd21<PfF>),
}

pub enum TCC2Pinout {
    Pa14(Pa14<PfF>),
    Pa15(Pa15<PfF>),
    Pa24(Pa24<PfF>),
    Pa30(Pa30<PfF>),
    Pa31(Pa31<PfF>),
    Pb2(Pb2<PfF>),
}

#[cfg(feature = "min-samd51j")]
pub enum TCC3Pinout {
    Pb12(Pb12<PfF>),
    Pb13(Pb13<PfF>),
    Pb16(Pb16<PfF>),
    Pb17(Pb17<PfF>),
}

#[cfg(feature = "min-samd51j")]
pub enum TCC4Pinout {
    Pb14(Pb14<PfF>),
    Pb15(Pb15<PfF>),
    Pb30(Pb30<PfF>),
    Pb31(Pb31<PfF>),
}

macro_rules! pwm_tcc {
    ($($TYPE:ident: ($TCC:ident, $pinout:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident),)+) => {
        $(

pub struct $TYPE {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tcc: $TCC,
    #[allow(dead_code)]
    pinout: $pinout,
}

impl $TYPE {
    pub fn new<F: Into<Hertz>> (
        clock: &clock::$clock,
        freq: F,
        tcc: $TCC,
        pinout: $pinout,
        mclk: &mut MCLK,
    ) -> Self {
        {
            let params = TimerParams::new(freq, clock.freq());

            mclk.$apmask.modify(|_, w| w.$apbits().set_bit());
            tcc.ctrla.write(|w| w.swrst().set_bit());
            while tcc.syncbusy.read().swrst().bit_is_set() {}
            tcc.ctrlbclr.write(|w| w.dir().set_bit() );
            while tcc.syncbusy.read().ctrlb().bit_is_set() {}
            tcc.ctrla.modify(|_, w| w.enable().clear_bit());
            tcc.ctrla.modify(|_, w| {
                match params.divider {
                    1 => w.prescaler().div1(),
                    2 => w.prescaler().div2(),
                    4 => w.prescaler().div4(),
                    8 => w.prescaler().div8(),
                    16 => w.prescaler().div16(),
                    64 => w.prescaler().div64(),
                    256 => w.prescaler().div256(),
                    1024 => w.prescaler().div1024(),
                    _ => unreachable!(),
                }
            });
            tcc.wave.write(|w| w.wavegen().npwm());
            while tcc.syncbusy.read().wave().bit_is_set() {}
            tcc.per().write(|w| unsafe { w.bits(params.cycles as u32) });
            while tcc.syncbusy.read().per().bit_is_set() {}
            tcc.ctrla.modify(|_, w| w.enable().set_bit());
        }

        Self {
            clock_freq: clock.freq(),
            tcc,
            pinout,
        }
    }
}

impl Pwm for $TYPE {
    type Channel = Channel;
    type Time = Hertz;
    type Duty = u32;

    fn disable(&mut self, _channel: Self::Channel) {
        self.tcc.ctrla.modify(|_, w| w.enable().clear_bit());
        while self.tcc.syncbusy.read().enable().bit_is_set() {}
    }

    fn enable(&mut self, _channel: Self::Channel) {
        self.tcc.ctrla.modify(|_, w| w.enable().set_bit());
        while self.tcc.syncbusy.read().enable().bit_is_set() {}
    }

    fn get_period(&self) -> Self::Time {
        let divisor = self.tcc.ctrla.read().prescaler().bits();
        let top = self.tcc.per().read().bits();
        Hertz(self.clock_freq.0 / divisor as u32 / (top + 1) as u32)
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        let cc = self.tcc.cc();
        let duty = cc[channel as usize].read().cc().bits();
        duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        let top = self.tcc.per().read().bits();
        top
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        let cc = self.tcc.cc();
        cc[channel as usize].write(|w| unsafe { w.cc().bits(duty) });
    }

    fn set_period<P>(&mut self, period: P)
    where
        P: Into<Self::Time>,
    {
        let params = TimerParams::new(period, self.clock_freq);

        self.tcc.ctrla.modify(|_, w| w.enable().clear_bit());
        while self.tcc.syncbusy.read().enable().bit_is_set() {}
        self.tcc.ctrla.modify(|_, w| {
            match params.divider {
                1 => w.prescaler().div1(),
                2 => w.prescaler().div2(),
                4 => w.prescaler().div4(),
                8 => w.prescaler().div8(),
                16 => w.prescaler().div16(),
                64 => w.prescaler().div64(),
                256 => w.prescaler().div256(),
                1024 => w.prescaler().div1024(),
                _ => unreachable!(),
            }
        });
        self.tcc.ctrla.modify(|_, w| w.enable().set_bit());
        while self.tcc.syncbusy.read().enable().bit_is_set() {}
        self.tcc.per().write(|w| unsafe { w.bits(params.cycles as u32) });
        while self.tcc.syncbusy.read().per().bit() {}
    }
}

        )+
    };
}

pwm_tcc! {
    Tcc0Pwm: (TCC0, TCC0Pinout, Tcc0Tcc1Clock, apbbmask, tcc0_, TccPwm0Wrapper),
    Tcc1Pwm: (TCC1, TCC1Pinout, Tcc0Tcc1Clock, apbbmask, tcc1_, TccPwm1Wrapper),
    Tcc2Pwm: (TCC2, TCC2Pinout, Tcc2Tcc3Clock, apbcmask, tcc2_, TccPwm2Wrapper),
}

#[cfg(feature = "min-samd51j")]
pwm_tcc! {
    Tcc3Pwm: (TCC3, TCC3Pinout, Tcc2Tcc3Clock, apbcmask, tcc3_, TccPwm3Wrapper),
    Tcc4Pwm: (TCC4, TCC4Pinout, Tcc4Clock,     apbdmask, tcc4_, TccPwm4Wrapper),
}
