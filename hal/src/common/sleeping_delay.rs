//! Delays with WFI sleep while we wait using a timer
use core::sync::atomic;
use cortex_m::asm;

use super::prelude::*;
use crate::timer_traits::InterruptDrivenTimer;
use embedded_time::{duration::Seconds, rate::*};
use hal::blocking::delay::{DelayMs, DelayUs};

/// Delay and sleep while we do (WFI) using a timer
pub struct SleepingDelay<TIM> {
    timer: TIM,
    interrupt_fired: &'static atomic::AtomicBool,
}

impl<TIM> SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
{
    /// Initializes a new SleepingDelay struct
    pub fn new(timer: TIM, interrupt_fired: &'static atomic::AtomicBool) -> SleepingDelay<TIM>
    where
        TIM: InterruptDrivenTimer,
    {
        SleepingDelay {
            timer,
            interrupt_fired,
        }
    }

    /// Releases the timer resource
    pub fn free(self) -> TIM {
        self.timer
    }
}

impl<TIM, TYPE> DelayUs<TYPE> for SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
    TYPE: Into<u32>,
{
    fn delay_us(&mut self, us: TYPE) {
        let us: u32 = us.into();

        // Determine how many cycles we need to run for this delay, if any
        // Avoid timers that run longer than a second because for 48 MHz-based timers,
        //   there is no valid divisor + cycle count greater than ~1.3s, so we'd panic.
        let mut count: u32 = 1 + Seconds::<u32>::from(us.microseconds()).integer();

        // Start the timer and sleep!
        self.timer.start((us / count).nanoseconds());
        self.timer.enable_interrupt();
        loop {
            asm::wfi();
            if self.timer.wait().is_ok() || self.interrupt_fired.load(atomic::Ordering::Relaxed) {
                self.interrupt_fired.store(false, atomic::Ordering::Relaxed);
                count -= 1;
                if count == 0 {
                    break;
                }
            }
        }
        self.timer.disable_interrupt();
    }
}

impl<TIM, TYPE> DelayMs<TYPE> for SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
    TYPE: Into<u32>,
{
    fn delay_ms(&mut self, ms: TYPE) {
        self.delay_us(ms.into() * 1_000_u32);
    }
}
