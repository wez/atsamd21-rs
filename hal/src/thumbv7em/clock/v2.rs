//! TODO

use crate::pac::{GCLK, MCLK, NVMCTRL, OSC32KCTRL, OSCCTRL};
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;

use crate::typelevel::One;

pub mod sources;
pub use sources::*;

pub mod gclk;
pub use gclk::*;

pub mod pclk;
pub use pclk::*;

pub mod ahb;
pub use ahb::*;

pub mod apb;
pub use apb::*;

/// TODO
/// Collection of PAC structs. Users can get access to this as an escape hatch
/// to handle situations outside the scope of the HAL.
pub struct PacClocks {
    pub oscctrl: OSCCTRL,
    pub osc32kctrl: OSC32KCTRL,
    pub gclk: GCLK,
    pub mclk: MCLK,
}

/// TODO
/// This is the main entry point for users
pub struct Tokens {
    pac: Option<PacClocks>,
    pub sources: sources::Sources,
    pub gclks: gclk::Tokens,
    pub pclks: pclk::Tokens,
    pub ahbs: ahb::AhbClks,
    pub apbs: apb::ApbClks,
}

impl Tokens {
    /// TODO
    /// Creating this is safe, because it takes ownership of the singleton
    /// PAC structs. But all other `new` functions below are `unsafe`, because
    /// they could allow duplicate clocks if used incorrectly.
    pub fn new(
        oscctrl: OSCCTRL,
        osc32kctrl: OSC32KCTRL,
        gclk: GCLK,
        mclk: MCLK,
        nvmctrl: &mut NVMCTRL,
    ) -> (Gclk0<Fll, One>, Dfll<One>, Tokens) {
        // TODO
        unsafe {
            let tokens = Tokens {
                pac: Some(PacClocks {
                    oscctrl,
                    osc32kctrl,
                    gclk,
                    mclk,
                }),
                sources: sources::Sources::new(),
                gclks: gclk::Tokens::new(nvmctrl),
                pclks: pclk::Tokens::new(),
                ahbs: ahb::AhbClks::new(),
                apbs: apb::ApbClks::new(),
            };
            let dfll = Dfll::init();
            let freq = dfll.freq();
            let gclk0 = Gclk0::init(freq);
            (gclk0, dfll, tokens)
        }
    }

    /// TODO
    /// Escape hatch for access to PAC structs
    pub unsafe fn pac(&mut self) -> Option<PacClocks> {
        self.pac.take()
    }
}

/// TODO
/// This is a bit of a hack right now. I think it might be best if the RTC
/// migrates into the `clock` module, since it's so integrated with OSC32KCTRL.
pub trait RtcClock {
    fn enable_1k(&mut self) -> RTCSEL_A;
    fn enable_32k(&mut self) -> RTCSEL_A;
}

/// TODO
pub fn set_rtc_clock<C: RtcClock>(clock: &mut C, enable_32k: bool) {
    use crate::pac::osc32kctrl::RegisterBlock;
    let rtc_sel = if enable_32k {
        clock.enable_32k()
    } else {
        clock.enable_1k()
    };
    unsafe {
        let osc32kctrl = OSC32KCTRL::ptr() as *mut RegisterBlock;
        (*osc32kctrl).rtcctrl.write(|w| w.rtcsel().variant(rtc_sel));
    }
}
