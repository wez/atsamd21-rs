use crate::clock;
use crate::hal::{Pwm, PwmPin};
use crate::time::*;
use crate::timer::TimerParams;

use crate::target_device::{PM, TCC0};
#[cfg(feature = "samd11")]
use crate::target_device::{TC1, TC2};
#[cfg(feature = "samd21")]
use crate::target_device::{TC3, TC4, TC5, TCC1, TCC2};
#[cfg(feature = "samd21j")]
use crate::target_device::{TC6, TC7};

// Timer/Counter (TCx)

macro_rules! pwm {
    ($($TYPE:ident: ($TC:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident),)+) => {
        $(

pub struct $TYPE {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tc: $TC,
}

impl $TYPE {
    pub fn new<F: Into<Hertz>> (
        clock: &clock::$clock,
        freq: F,
        tc: $TC,
        pm: &mut PM,
    ) -> Self {
        {
            let count = tc.count16();
            let params = TimerParams::new(freq, clock.freq().0);
            pm.$apmask.modify(|_, w| w.$apbits().set_bit());
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
            count.ctrla.write(|w| w.wavegen().mpwm());
            count.cc[0].write(|w| unsafe { w.cc().bits(params.cycles as u16) });
            count.cc[1].write(|w| unsafe { w.cc().bits(0) });
            count.ctrla.modify(|_, w| w.enable().set_bit());
        }

        Self {
            clock_freq: clock.freq(),
            tc,
        }
    }

    pub fn set_period<P>(&mut self, period: P)
    where
        P: Into<Hertz>
    {
        let params = TimerParams::new(period, self.clock_freq.0);
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
    }

    pub fn get_period(&self) -> Hertz {
        let count = self.tc.count16();
        let divisor = count.ctrla.read().prescaler().bits();
        let top = count.cc[0].read().cc().bits();
        Hertz(self.clock_freq.0 / divisor as u32 / (top + 1) as u32)
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
        let duty: u16 = count.cc[1].read().cc().bits();
        duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        let count = self.tc.count16();
        let top = count.cc[0].read().cc().bits();
        top
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        let count = self.tc.count16();
        count.cc[1].write(|w| unsafe { w.cc().bits(duty) });
    }
}

)+}}

#[cfg(feature = "samd11")]
pwm! {
    Pwm1: (TC1, Tc1Tc2Clock, apbcmask, tc1_, Pwm1Wrapper),
    Pwm2: (TC2, Tc1Tc2Clock, apbcmask, tc2_, Pwm2Wrapper),
}

#[cfg(feature = "samd21")]
pwm! {
    Pwm3: (TC3, Tcc2Tc3Clock, apbcmask, tc3_, Pwm3Wrapper),
    Pwm4: (TC4, Tc4Tc5Clock, apbcmask, tc4_, Pwm4Wrapper),
    Pwm5: (TC5, Tc4Tc5Clock, apbcmask, tc5_, Pwm5Wrapper),
}

#[cfg(feature = "samd21j")]
pwm! {
    Pwm6: (TC6, Tc6Tc7Clock, apbcmask, tc6_, Pwm6Wrapper),
    Pwm7: (TC7, Tc6Tc7Clock, apbcmask, tc7_, Pwm7Wrapper),
}

// Timer/Counter for Control Applications (TCCx)

#[derive(Copy, Clone)]
pub enum Channel {
    _0,
    _1,
    _2,
    _3,
}

macro_rules! pwm_tcc {
    ($($TYPE:ident: ($TCC:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident),)+) => {
        $(

pub struct $TYPE {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tcc: $TCC,
}

impl $TYPE {
    pub fn new<F: Into<Hertz>> (
        clock: &clock::$clock,
        freq: F,
        tcc: $TCC,
        pm: &mut PM,
    ) -> Self {
        {
            let params = TimerParams::new(freq, clock.freq().0);
            pm.$apmask.modify(|_, w| w.$apbits().set_bit());
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
        }
    }
}

impl Pwm for $TYPE {
    type Channel = Channel;
    type Time = Hertz;
    type Duty = u32;

    fn disable(&mut self, _channel: Self::Channel) {
        self.tcc.ctrla.modify(|_, w| w.enable().clear_bit());
    }

    fn enable(&mut self, _channel: Self::Channel) {
        self.tcc.ctrla.modify(|_, w| w.enable().set_bit());
    }

    fn get_period(&self) -> Self::Time {
        let divisor = self.tcc.ctrla.read().prescaler().bits();
        let top = self.tcc.per().read().bits();
        Hertz(self.clock_freq.0 / divisor as u32 / (top + 1) as u32)
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        let cc = self.tcc.cc();
        let duty: u32 = cc[channel as usize].read().cc().bits();
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
        let params = TimerParams::new(period, self.clock_freq.0);
        self.tcc.ctrla.modify(|_, w| w.enable().clear_bit());
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
        self.tcc.per().write(|w| unsafe { w.bits(params.cycles as u32) });
        while self.tcc.syncbusy.read().per().bit() {}
    }
}

)+}}

#[cfg(feature = "samd11")]
pwm_tcc! {
    Pwm0: (TCC0, Tcc0Clock, apbcmask, tcc0_, Pwm0Wrapper),
}

#[cfg(feature = "samd21")]
pwm_tcc! {
    Pwm0: (TCC0, Tcc0Tcc1Clock, apbcmask, tcc0_, Pwm0Wrapper),
    Pwm1: (TCC1, Tcc0Tcc1Clock, apbcmask, tcc1_, Pwm1Wrapper),
    Pwm2: (TCC2, Tcc2Tc3Clock, apbcmask, tcc2_, Pwm2Wrapper),
}
