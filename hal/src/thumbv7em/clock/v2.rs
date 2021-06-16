//! # Clocking API v2
//! TODO
//!
//! Functionality:
//!
//! * Type-safe management of ATSAMD clocking system
//! * Full flexibility: API design allows to represent any logical clocking
//!   configuration forseen by a HW manufacturer

use typenum::{U0, U1};

use crate::clock::types::Enabled;
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::{GCLK, MCLK, NVMCTRL, OSC32KCTRL, OSCCTRL};
use crate::time::Hertz;

mod presets;

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

/// Collection of low-level PAC structs.
///
/// Gathers all clocking related peripherals consumed by [`retrieve_clocks`]
/// function that are then being contained within [`Tokens::pac`] field. PAC
/// structs can be accessed using unsafe [`PacClocks::steal`] function.
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

/// This struct contains instantiated `Token` structs that can be used to
/// construct clocking components.
///
/// E.g. to call [`gclk::Gclk<gclk::Gen5, _>::new`] one needs
/// [`gclk::Tokens::gclk5`] from within [`Tokens::gclks`]. Thus, it is
/// impossible to create multiple instances of [`gclk::Gclk<gclk::Gen5, _>`].
///
/// These do not expose any API externally. Inside of the HAL crate, these
/// expose low-level API to HW register of finer granularity than regular PAC
/// structs.
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

/// Standalone function returning a set of instantiated clocking abstractions
/// representing a default state of a clocking system. For `thumbv7em` based
/// devices it is a chain of:
/// - [`dfll::Dfll<OpenLoop>`] (`48 MHz`)
/// - [`gclk::Gclk0<Dfll>`] (`48 MHz`)
///
/// And also ultra low power internal 32k oscillator:
///
/// - [`osculp32k::OscUlp32k`] (`32 KHz`)
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
    // Safe because registers are instantiated only once
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

/// Marker supertrait unifying family of more specific source marker traits.
///
/// These ones are essential during a construction (`fn ::{new, enable}`) and
/// deconstruction (`fn ::{free, disable}`) of clocking components as they
/// provide information to the constructed/deconstructed type what its source is
/// and which variant of source (associated constant) is applicable while
/// performing a HW register write.
pub trait SourceMarker: crate::typelevel::Sealed {}

/// Supertrait unifying family of more specific source traits.
///
/// These are implemented by specific specialized forms of
/// [`super::types::Enabled`]. They are used during a construction (`fn ::{new,
/// enable}`) and deconstruction (`fn ::{free, disable}`) of clocking components
/// and they express the type of dependency needed by a dependee. For examples,
/// [`gclk::Gclk::new`] will only consume source implementing
/// [`gclk::GclkSource`] trait.
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
