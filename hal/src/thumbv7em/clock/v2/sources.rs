//! TODO

pub mod gclkio;
pub use gclkio::*;

pub mod dpll;
pub use dpll::*;

pub mod xosc;
pub use xosc::*;

pub mod dfll;
pub use dfll::*;

pub mod xosc32k;
pub use xosc32k::*;

pub mod osculp32k;
pub use osculp32k::*;

/// TODO
pub struct Sources {
    pub gclk_io: gclkio::Tokens,
    pub dpll0: DpllToken<Pll0>,
    pub dpll1: DpllToken<Pll1>,
    pub osc_ulp_32k: OscUlp32k,
    pub xosc0: XOscToken<Osc0>,
    pub xosc1: XOscToken<Osc1>,
}

impl Sources {
    /// TODO
    pub(super) unsafe fn new() -> Sources {
        Sources {
            gclk_io: gclkio::Tokens::new(),
            dpll0: DpllToken::new(),
            dpll1: DpllToken::new(),
            osc_ulp_32k: OscUlp32k::new(),
            xosc0: XOscToken::new(),
            xosc1: XOscToken::new(),
        }
    }
}
