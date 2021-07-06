#![no_std]

pub extern crate embedded_hal as hal;

pub use paste;

pub mod typelevel;

#[cfg(feature = "samd11c")]
pub use atsamd11c as target_device;

#[cfg(feature = "samd11d")]
pub use atsamd11d as target_device;

#[cfg(feature = "samd21e")]
pub use atsamd21e as target_device;

#[cfg(feature = "samd21g")]
pub use atsamd21g as target_device;

#[cfg(feature = "samd21j")]
pub use atsamd21j as target_device;

#[cfg(feature = "samd51g")]
pub use atsamd51g as target_device;

#[cfg(feature = "samd51j")]
pub use atsamd51j as target_device;

#[cfg(feature = "samd51n")]
pub use atsamd51n as target_device;

#[cfg(feature = "samd51p")]
pub use atsamd51p as target_device;

#[cfg(feature = "same51g")]
pub use atsame51g as target_device;

#[cfg(feature = "same51j")]
pub use atsame51j as target_device;

#[cfg(feature = "same51n")]
pub use atsame51n as target_device;

#[cfg(feature = "same53j")]
pub use atsame53j as target_device;

#[cfg(feature = "same53n")]
pub use atsame53n as target_device;

#[cfg(feature = "same54n")]
pub use atsame54n as target_device;

#[cfg(feature = "same54p")]
pub use atsame54p as target_device;

#[cfg(any(feature = "samd11", feature = "samd21", feature = "min-samd51g"))]
pub use target_device as pac;

#[cfg(feature = "use_rtt")]
pub use jlink_rtt;

#[cfg(feature = "use_rtt")]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = $crate::jlink_rtt::NonBlockingOutput::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

#[cfg(not(feature = "use_rtt"))]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}

#[cfg(feature = "device")]
pub mod delay;
#[cfg(feature = "device")]
pub mod gpio;
#[cfg(feature = "device")]
pub mod prelude;
#[cfg(feature = "device")]
pub mod rtc;
#[cfg(feature = "device")]
pub mod sercom;
pub mod sleeping_delay;
#[cfg(feature = "device")]
pub mod spi_common;
pub mod time;
pub mod timer_params;
pub mod timer_traits;

#[cfg(all(feature = "unproven", feature = "dma"))]
pub mod dmac;

// The SAMD11 hardware supports USB too, but the SAMD11 HAL currently doesn't
#[cfg(all(feature = "usb", not(feature = "samd21"), not(feature = "min-samd51g")))]
compile_error!("The 'usb' feature is enabled, but not a specific chip that supports USB");

#[cfg(any(feature = "samd11", feature = "samd21"))]
pub mod thumbv6m;
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub use crate::thumbv6m::*;

#[cfg(feature = "min-samd51g")]
pub mod thumbv7em;
#[cfg(feature = "min-samd51g")]
pub use crate::thumbv7em::*;

// This module maintains backwards compatibility for the v1 SERCOM pads API
#[cfg(feature = "device")]
pub mod pad {
    pub use crate::sercom::v1::pads::PadPin;
}

// This module maintains backwards compatibility within this major release
#[macro_use]
pub mod common {
    #[cfg(feature = "device")]
    pub use crate::delay;
    #[cfg(feature = "device")]
    pub use crate::gpio;
    #[cfg(feature = "device")]
    pub use crate::prelude;
    #[cfg(feature = "device")]
    pub use crate::rtc;
    #[cfg(feature = "device")]
    pub use crate::sercom;
    pub use crate::sleeping_delay;
    #[cfg(feature = "device")]
    pub use crate::spi_common;
    pub use crate::time;
    pub use crate::timer_params;
    pub use crate::timer_traits;

    #[cfg(all(feature = "unproven", feature = "dma"))]
    pub use crate::dmac;

    #[cfg(any(feature = "samd11", feature = "samd21"))]
    pub use crate::thumbv6m;
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    pub use crate::thumbv6m::*;

    #[cfg(feature = "min-samd51g")]
    pub use crate::thumbv7em;
    #[cfg(feature = "min-samd51g")]
    pub use crate::thumbv7em::*;

    #[cfg(feature = "device")]
    pub use crate::pad;
}

// The following modules are included purely for backward compatibility reasons.
// Whenever major breaking changes are made to the HAL next, these modules
// should be removed.

#[cfg(feature = "samd51")]
pub mod samd51 {
    #[cfg(feature = "unproven")]
    pub use crate::pwm;
}

#[cfg(feature = "same51")]
pub mod same51 {
    #[cfg(feature = "unproven")]
    pub use crate::pwm;
}

#[cfg(feature = "same53")]
pub mod same53 {
    #[cfg(feature = "unproven")]
    pub use crate::pwm;
}

#[cfg(feature = "same54")]
pub mod same54 {
    #[cfg(feature = "unproven")]
    pub use crate::pwm;
}
