#![no_std]
#![recursion_limit = "1024"]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use hal::pac;

#[cfg(feature = "unproven")]
pub mod buttons;
pub mod pins;

pub use buttons::*;
pub use pins::*;

pub mod util {
    /// Analogous to Arduinos map function
    pub fn map_from(input: i16, from_range: (i16, i16), to_range: (i16, i16)) -> i16 {
        debug_assert!(from_range.0 < from_range.1);
        debug_assert!(to_range.0 < to_range.1);
        debug_assert!(input >= from_range.0);
        debug_assert!(input <= from_range.1);

        let from: f32 = (from_range.1 - from_range.0).into();
        let to: f32 = (to_range.1 - to_range.0).into();
        ((input - from_range.0) as f32 / from * to + to_range.0 as f32) as i16
    }
}

#[cfg(feature = "panic_led")]
#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    use embedded_hal::digital::v2::OutputPin;

    let peripherals = unsafe { crate::pac::Peripherals::steal() };
    let mut pins = Pins::new(peripherals.PORT);
    pins.d13.into_push_pull_output().set_high().ok();

    loop {
        cortex_m::asm::udf()
    }
}
