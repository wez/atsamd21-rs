//! RTC - Real Time Clock
//! TODO
//!
//! This is a bit of a hack right now. I think it might be best if the RTC
//! migrates into the `clock` module, since it's so integrated with OSC32KCTRL.

use crate::clock::types::{Increment, Decrement};
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::OSC32KCTRL;
use crate::typelevel::Sealed;

/// Set the RTC clock source using a 32 kHz clock
///
/// TODO
pub fn set_rtc_clock_32k<S>(source: S) -> S::Inc
where
    S: RtcSource32k + Increment,
{
    use crate::pac::osc32kctrl::RegisterBlock;
    unsafe {
        let osc32kctrl = OSC32KCTRL::ptr() as *mut RegisterBlock;
        (*osc32kctrl)
            .rtcctrl
            .write(|w| w.rtcsel().variant(S::RTC_SRC_32K));
    }
    source.inc()
}

/// Set the RTC clock source using a 1 kHz clock
///
/// TODO
pub fn set_rtc_clock_1k<S>(source: S) -> S::Inc
where
    S: RtcSource1k + Increment,
{
    use crate::pac::osc32kctrl::RegisterBlock;
    unsafe {
        let osc32kctrl = OSC32KCTRL::ptr() as *mut RegisterBlock;
        (*osc32kctrl)
            .rtcctrl
            .write(|w| w.rtcsel().variant(S::RTC_SRC_1K));
    }
    source.inc()
}

/// TODO This is only a workaround
/// FIXME
pub fn unset_rtc_clock_32k<S>(source: S) -> S::Dec
where
    S: RtcSource32k + Decrement,
{
    source.dec()
}

/// TODO This is only a workaround
/// FIXME
pub fn unset_rtc_clock_1k<S>(source: S) -> S::Dec
where
    S: RtcSource1k + Decrement,
{
    source.dec()
}

//==============================================================================
// RtcSource
//==============================================================================

pub trait RtcSource {}

pub trait RtcSource32k {
    const RTC_SRC_32K: RTCSEL_A;
}

pub trait RtcSource1k {
    const RTC_SRC_1K: RTCSEL_A;
}

pub trait Output32k: Sealed {}
pub trait Output1k: Sealed {}

pub enum Output32kOn {}

impl Sealed for Output32kOn {}
impl Output32k for Output32kOn {}

pub enum Output1kOn {}

impl Sealed for Output1kOn {}
impl Output1k for Output1kOn {}

pub enum Output32kOff {}

impl Sealed for Output32kOff {}
impl Output32k for Output32kOff {}

pub enum Output1kOff {}

impl Sealed for Output1kOff {}
impl Output1k for Output1kOff {}
