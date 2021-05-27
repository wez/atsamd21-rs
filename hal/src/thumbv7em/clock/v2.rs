//! TODO

use typenum::{U0, U1};

use crate::clock::types::Enabled;
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::{GCLK, MCLK, NVMCTRL, OSC32KCTRL, OSCCTRL};
use crate::time::Hertz;

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
    ) -> (
        Enabled<Gclk0<marker::Dfll>, U1>,
        Enabled<Dfll<OpenLoop>, U1>,
        Enabled<OscUlp32k, U0>,
        Tokens,
    ) {
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
            let dfll = Enabled::<_, U0>::new(Dfll::in_open_mode(DfllToken::new()));
            let (gclk0, dfll) = Gclk0::new(GclkToken::new(), dfll);
            let gclk0 = Enabled::<_, U1>::new(gclk0);
            let osculp32k = OscUlp32k::new(OscUlp32kToken::new()).enable();
            (gclk0, dfll, osculp32k, tokens)
        }
    }

    /// TODO
    /// Escape hatch for access to PAC structs
    pub unsafe fn pac(&mut self) -> Option<PacClocks> {
        self.pac.take()
    }
}

/// TODO: Super trait of more specific SourceMarker traits
pub trait SourceMarker: crate::typelevel::Sealed {}

/// TODO: Super trait of more specific Source traits
pub trait Source: crate::typelevel::Sealed {
    fn freq(&self) -> Hertz;
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
