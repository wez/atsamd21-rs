use crate::hal::timer::{CountDown, Periodic};
use embedded_time::duration::*;

/// Trait for timers that can enable & disable an interrupt that fires
/// when the timer expires
pub trait InterruptDrivenTimer: CountDown<Time = Nanoseconds> + Periodic {
    /// Enable the timer interrupt
    fn enable_interrupt(&mut self);

    /// Disable the timer interrupt
    fn disable_interrupt(&mut self);
}
