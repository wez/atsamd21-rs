//! TODO

use typenum::{U0, U1};

use crate::clock::types::Enabled;
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::{GCLK, MCLK, NVMCTRL, OSC32KCTRL, OSCCTRL};
use crate::time::Hertz;

pub mod ahb;
pub mod apb;
pub mod dfll;
pub mod dpll;
pub mod gclk;
pub mod gclkio;
pub mod osculp32k;
pub mod pclk;
pub mod xosc;
pub mod xosc32k;

/// TODO
/// Collection of PAC structs. Users can get access to this as an escape hatch
/// to handle situations outside the scope of the HAL.
pub struct PacClocks {
    oscctrl: OSCCTRL,
    osc32kctrl: OSC32KCTRL,
    gclk: GCLK,
    mclk: MCLK,
}

impl PacClocks {
    /// Escape hatch allowing to access low-level PAC structs.
    /// This is especially useful when V2 clocking API must interact with
    /// legacy V1 clocking API based peripherals; E.g. access to [`MCLK`] is
    /// necessary in most circumstances.
    pub unsafe fn steal(self) -> (OSCCTRL, OSC32KCTRL, GCLK, MCLK) {
        (self.oscctrl, self.osc32kctrl, self.gclk, self.mclk)
    }
}

/// TODO
/// This is the main entry point for users
pub struct Tokens {
    pub pac: PacClocks,
    pub ahbs: ahb::AhbClks,
    pub apbs: apb::ApbClks,
    pub dpll0: dpll::DpllToken<dpll::Pll0>,
    pub dpll1: dpll::DpllToken<dpll::Pll1>,
    pub gclk_io: gclkio::Tokens,
    pub gclks: gclk::Tokens,
    pub osc_ulp_32k: osculp32k::OscUlp32kToken,
    pub pclks: pclk::Tokens,
    pub xosc0: xosc::XoscToken<xosc::Osc0>,
    pub xosc1: xosc::XoscToken<xosc::Osc1>,
    pub xosc32k: xosc32k::Xosc32kToken,
}

/// TODO
/// Creating this is safe, because it takes ownership of the singleton
/// PAC structs. But all other `new` functions below are `unsafe`, because
/// they could allow duplicate clocks if used incorrectly.
pub fn retrieve_clocks(
    oscctrl: OSCCTRL,
    osc32kctrl: OSC32KCTRL,
    gclk: GCLK,
    mclk: MCLK,
    nvmctrl: &mut NVMCTRL,
) -> (
    Enabled<gclk::Gclk0<dfll::marker::Dfll>, U1>,
    Enabled<dfll::Dfll<dfll::OpenLoop>, U1>,
    Enabled<osculp32k::OscUlp32k, U0>,
    Tokens,
) {
    // TODO
    unsafe {
        let tokens = Tokens {
            pac: PacClocks {
                oscctrl,
                osc32kctrl,
                gclk,
                mclk,
            },
            ahbs: ahb::AhbClks::new(),
            apbs: apb::ApbClks::new(),
            dpll0: dpll::DpllToken::new(),
            dpll1: dpll::DpllToken::new(),
            gclk_io: gclkio::Tokens::new(),
            gclks: gclk::Tokens::new(nvmctrl),
            osc_ulp_32k: osculp32k::OscUlp32kToken::new(),
            pclks: pclk::Tokens::new(),
            xosc0: xosc::XoscToken::new(),
            xosc1: xosc::XoscToken::new(),
            xosc32k: xosc32k::Xosc32kToken::new(),
        };
        let dfll = Enabled::<_, U0>::new(dfll::Dfll::in_open_mode(dfll::DfllToken::new()));
        let (gclk0, dfll) = gclk::Gclk0::new(gclk::GclkToken::new(), dfll);
        let gclk0 = Enabled::new(gclk0);
        let osculp32k = Enabled::new(osculp32k::OscUlp32k::new(osculp32k::OscUlp32kToken::new()));
        (gclk0, dfll, osculp32k, tokens)
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
