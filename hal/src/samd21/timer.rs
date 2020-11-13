//! Working with timer counter hardware
use crate::target_device::tc3::COUNT16;
#[allow(unused)]
use crate::target_device::{PM, TC3, TC4, TC5};
use hal::timer::{CountDown, Periodic};

use crate::clock;
use crate::timer_traits::InterruptDrivenTimer;
use embedded_time::{duration::Nanoseconds, rate::Hertz};
use void::Void;

use cortex_m::asm::delay as cycle_delay;

// Note:
// TC3 + TC4 can be paired to make a 32-bit counter
// TC5 + TC6 can be paired to make a 32-bit counter

/// A generic hardware timer counter.
/// The counters are exposed in 16-bit mode only.
/// The hardware allows configuring the 8-bit mode
/// and pairing up some instances to run in 32-bit
/// mode, but that functionality is not currently
/// exposed by this hal implementation.
/// TimerCounter implements both the `Periodic` and
/// the `CountDown` embedded_hal timer traits.
/// Before a hardware timer can be used, it must first
/// have a clock configured.
pub struct TimerCounter<TC> {
    freq: Hertz,
    tc: TC,
}

/// This is a helper trait to make it easier to make most of the
/// TimerCounter impl generic.  It doesn't make too much sense to
/// to try to implement this trait outside of this module.
pub trait Count16 {
    fn count_16(&self) -> &COUNT16;
}

impl<TC> Periodic for TimerCounter<TC> {}
impl<TC> CountDown for TimerCounter<TC>
where
    TC: Count16,
{
    type Time = Nanoseconds;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>,
    {
        let params = TimerParams::new_us(Nanoseconds(timeout.into().0 as u64), self.freq);
        let divider = params.divider;
        let cycles = params.cycles;

        let count = self.tc.count_16();

        // Disable the timer while we reconfigure it
        count.ctrla.modify(|_, w| w.enable().clear_bit());
        while count.status.read().syncbusy().bit_is_set() {}

        // Now that we have a clock routed to the peripheral, we
        // can ask it to perform a reset.
        count.ctrla.write(|w| w.swrst().set_bit());
        while count.status.read().syncbusy().bit_is_set() {}
        // the SVD erroneously marks swrst as write-only, so we
        // need to manually read the bit here
        while count.ctrla.read().bits() & 1 != 0 {}

        count.ctrlbset.write(|w| {
            // Count up when the direction bit is zero
            w.dir().clear_bit();
            // Periodic
            w.oneshot().clear_bit()
        });

        // Set TOP value for mfrq mode
        count.cc[0].write(|w| unsafe { w.cc().bits(cycles as u16) });

        count.ctrla.modify(|_, w| {
            match divider {
                1 => w.prescaler().div1(),
                2 => w.prescaler().div2(),
                4 => w.prescaler().div4(),
                8 => w.prescaler().div8(),
                16 => w.prescaler().div16(),
                64 => w.prescaler().div64(),
                256 => w.prescaler().div256(),
                1024 => w.prescaler().div1024(),
                _ => unreachable!(),
            };
            // Enable Match Frequency Waveform generation
            w.wavegen().mfrq();
            w.enable().set_bit();
            w.runstdby().set_bit()
        });
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        let count = self.tc.count_16();
        if count.intflag.read().ovf().bit_is_set() {
            // Writing a 1 clears the flag
            count.intflag.modify(|_, w| w.ovf().set_bit());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<TC> InterruptDrivenTimer for TimerCounter<TC>
where
    TC: Count16,
{
    /// Enable the interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to trigger
    /// the interrupt; it does not configure the interrupt controller
    /// or define an interrupt handler.
    fn enable_interrupt(&mut self) {
        self.tc.count_16().intenset.write(|w| w.ovf().set_bit());
    }

    /// Disables interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to prevent
    /// triggering the interrupt; it does not configure the interrupt
    /// controller.
    fn disable_interrupt(&mut self) {
        self.tc.count_16().intenclr.write(|w| w.ovf().set_bit());
    }
}

macro_rules! tc {
    ($($TYPE:ident: ($TC:ident, $pm:ident, $clock:ident),)+) => {
        $(
pub type $TYPE = TimerCounter<$TC>;

impl Count16 for $TC {
    fn count_16(&self) -> &COUNT16 {
        self.count16()
    }
}

impl TimerCounter<$TC>
{
    /// Configure this timer counter instance.
    /// The clock is obtained from the `GenericClockController` instance
    /// and its frequency impacts the resolution and maximum range of
    /// the timeout values that can be passed to the `start` method.
    /// Note that some hardware timer instances share the same clock
    /// generator instance and thus will be clocked at the same rate.
    pub fn $pm(clock: &clock::$clock, tc: $TC, pm: &mut PM) -> Self {
        // this is safe because we're constrained to just the tc3 bit
        pm.apbcmask.modify(|_, w| w.$pm().set_bit());
        {
            let count = tc.count_16();

            // Disable the timer while we reconfigure it
            count.ctrla.modify(|_, w| w.enable().clear_bit());
            while count.status.read().syncbusy().bit_is_set() {}
        }
        Self {
            freq: clock.freq(),
            tc,
        }
    }
}
        )+
    }
}

/// Helper type for computing cycles and divider given frequency
#[derive(Debug, Clone, Copy)]
pub struct TimerParams {
    pub divider: u16,
    pub cycles: u32,
}

impl TimerParams {
    pub fn new<T, F>(timeout: T, src_freq: F) -> Self
    where
        T: Into<Hertz>,
        F: Into<Hertz>,
    {
        let timeout = timeout.into();
        let ticks: u32 = src_freq.into().0 / timeout.0.max(1);
        Self::new_from_ticks(ticks)
    }

    pub fn new_us<T, F>(timeout: T, src_freq: F) -> Self
    where
        T: Into<Nanoseconds<u64>>,
        F: Into<Hertz<u64>>,
    {
        let timeout = timeout.into();
        let ticks: u32 = (timeout.0 * src_freq.into().0 / 1_000_000_000_u64) as u32;
        Self::new_from_ticks(ticks)
    }

    fn new_from_ticks(ticks: u32) -> Self {
        let divider = ((ticks >> 16) + 1).next_power_of_two();
        let divider = match divider {
            1 | 2 | 4 | 8 | 16 | 64 | 256 | 1024 => divider,
            // There are a couple of gaps, so we round up to the next largest
            // divider; we'll need to count twice as many but it will work.
            32 => 64,
            128 => 256,
            512 => 1024,
            // Catch all case; this is lame.  Would be great to detect this
            // and fail at compile time.
            _ => 1024,
        };

        let cycles: u32 = ticks / divider as u32;

        if cycles > u16::max_value() as u32 {
            panic!("cycles {} is out of range for a 16 bit counter", cycles);
        }

        TimerParams {
            divider: divider as u16,
            cycles,
        }
    }
}

tc! {
    TimerCounter3: (TC3, tc3_, Tcc2Tc3Clock),
    TimerCounter4: (TC4, tc4_, Tc4Tc5Clock),
    TimerCounter5: (TC5, tc5_, Tc4Tc5Clock),
}

#[cfg(test)]
mod tests {
    use crate::samd21::timer::TimerParams;
    use embedded_time::duration::*;
    use embedded_time::rate::*;

    #[test]
    fn timer_params_hz_and_us_same_1hz() {
        let tp_from_hz = TimerParams::new(1_u32.Hz(), 48_000_000_u32.Hz());
        let tp_from_us = TimerParams::new_us(1_000_000_u32.microseconds(), 48u32.MHz());

        assert_eq!(tp_from_hz.divider, tp_from_us.divider);
        assert_eq!(tp_from_hz.cycles, tp_from_us.cycles);
    }

    #[test]
    fn timer_params_hz_and_us_same_3hz() {
        let tp_from_hz = TimerParams::new(3_u32.Hz(), 48_000_000_u32.Hz());
        let tp_from_us = TimerParams::new_us(333_333_u32.microseconds(), 48u32.MHz());

        // There's some rounding error here, but it is extremely small (1 cycle
        // difference)
        assert_eq!(tp_from_hz.divider, tp_from_us.divider);
        assert!((tp_from_hz.cycles as i32 - tp_from_us.cycles as i32).abs() <= 1);
    }
}

#[derive(Clone, Copy)]
pub struct SpinTimer {
    cycles: u32,
}

impl SpinTimer {
    pub fn new(cycles: u32) -> SpinTimer {
        SpinTimer { cycles }
    }
}

impl Periodic for SpinTimer {}

impl CountDown for SpinTimer {
    type Time = u32;

    fn start<T>(&mut self, cycles: T)
    where
        T: Into<Self::Time>,
    {
        self.cycles = cycles.into();
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        cycle_delay(self.cycles);
        Ok(())
    }
}
