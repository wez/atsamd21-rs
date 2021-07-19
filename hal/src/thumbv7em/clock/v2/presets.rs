#![allow(unused_imports)] // Imports are used in doc strings

use super::dfll::*;
use super::dpll::*;
use super::gclk::*;
use super::osculp32k::*;
use super::pclk::*;
use super::xosc32k::*;

/// TODO: Make sure all these link are valid before integration. Task:
/// Refine `Source` types and `SourceMarker` types and normalize naming.
///
/// This macro provides an opinionated clocking setup that consists of
/// a following chain of configured components:
///
/// - [`Dfll<OpenLoop>`] (`48 MHz`)
/// - [`Gclk5<Dfll>`] (`48 MHz / 24 -> 2 MHz`)
/// - [`Pclk<Pll0, Gen5>`]
/// - [`Dpll0<PclkDriven<Pll0, Gen5>>`] (`2 MHz * 60 -> 120 MHz`)
/// - [`Gclk0<Pll0>`] (`120 MHz`)
///
/// Input arguments for this macro can be retrieved by calling
/// [`crate::clock::v2::retrieve_clocks`] function.
///
/// Example of usage:
///
/// ```rust
/// let mut pac = atsamd_hal::pac::Peripherals::take().unwrap();
///
/// let (gclk0, dfll, _, tokens) = atsamd_hal::clock::v2::retrieve_clocks(
///     pac.OSCCTRL,
///     pac.OSC32KCTRL,
///     pac.GCLK,
///     pac.MCLK,
///     &mut pac.NVMCTRL,
/// );
///
/// let _ = atsamd_hal::clocking_preset_gclk0_120mhz_gclk5_2mhz!(gclk0, dfll, tokens);
/// ```
#[macro_export]
macro_rules! clocking_preset_gclk0_120mhz_gclk5_2mhz {
    (
        $gclk0:expr,
        $dfll:expr,
        $tokens:expr
    ) => {{
        let (gclk5, dfll) = gclk::Gclk::new($tokens.gclks.gclk5, $dfll);
        let gclk5 = gclk5.div(gclk::GclkDiv::Div(24)).enable();
        let (pclk_dpll0, gclk5) = pclk::Pclk::enable($tokens.pclks.dpll0, gclk5);
        let dpll0 = dpll::Dpll0::from_pclk($tokens.dpll0, pclk_dpll0)
            .set_loop_div(60, 0)
            .enable();
        let (gclk0, dfll, dpll0) = $gclk0.swap(dfll, dpll0);
        (gclk0, gclk5, dpll0, dfll)
    }};
}
/// TODO: Make sure all these link are valid before integration. Task:
/// Refine `Source` types and `SourceMarker` types and normalize naming.
///
/// This macro provides an opinionated clocking setup that consists of
/// a following chain of configured components:
///
/// - [`Dfll<OpenLoop>`] (`48 MHz`)
/// - [`Gclk5<Dfll>`] (`48 MHz / 24 -> 2 MHz`)
/// - [`Pclk<Pll0, Gen5>`]
/// - [`Dpll0<PclkDriven<Pll0, Gen5>>`] (`2 MHz * 60 -> 120 MHz`)
/// - [`Gclk0<Pll0>`] (`120 MHz`)
///
/// with configured external 32k crystal oscillator:
///
/// - [`Xosc32k<Xosc32kMode>`] (`32 KHz`)
/// - [`Gclk1<Osc32k>`] (`32 KHz`)
///
/// Input arguments for this macro can be retrieved by calling
/// [`crate::clock::v2::retrieve_clocks`] function.
///
/// Example of usage:
///
/// ```rust
/// let mut pac = atsamd_hal::pac::Peripherals::take().unwrap();
/// let pins = atsamd_hal::gpio::v2::Pins::new(peripherals.PORT);
///
/// let (gclk0, dfll, _, tokens) = atsamd_hal::clock::v2::retrieve_clocks(
///     pac.OSCCTRL,
///     pac.OSC32KCTRL,
///     pac.GCLK,
///     pac.MCLK,
///     &mut pac.NVMCTRL,
/// );
///
/// let _ = atsamd_hal::clocking_preset_gclk0_120mhz_gclk5_2mhz_gclk1_external_32khz!(
///     gclk0, dfll, pins.pa00, pins.pa01, tokens
/// );
/// ```
#[macro_export]
macro_rules! clocking_preset_gclk0_120mhz_gclk5_2mhz_gclk1_external_32khz {
    (
        $gclk0:expr,
        $dfll:expr,
        $xosc32k_in:expr,
        $xosc32k_out:expr,
        $tokens:expr
    ) => {{
        let (gclk0, gclk5, dpll0, dfll) =
            crate::clocking_preset_gclk0_120mhz_gclk5_2mhz!($gclk0, $dfll, $tokens);
        let xosc32k = xosc32k::Xosc32k::from_crystal($tokens.xosc32k, $xosc32k_in, $xosc32k_out)
            .enable_32k(true)
            .enable_1k(true)
            .enable();
        let (gclk1, xosc32k) = gclk::Gclk::new($tokens.gclks.gclk1, xosc32k);
        (gclk0, gclk5, gclk1, xosc32k, dpll0, dfll)
    }};
}
/// TODO: Make sure all these link are valid before integration. Task:
/// Refine `Source` types and `SourceMarker` types and normalize naming.
///
/// This macro provides an opinionated clocking setup that consists of
/// a following chain of configured components:
///
/// - [`Dfll<OpenLoop>`] (`48 MHz`)
/// - [`Gclk5<Dfll>`] (`48 MHz / 24 -> 2 MHz`)
/// - [`Pclk<Pll0, Gen5>`]
/// - [`Dpll0<PclkDriven<Pll0, Gen5>>`] (`2 MHz * 60 -> 120 MHz`)
/// - [`Gclk0<Pll0>`] (`120 MHz`)
///
/// with internal, ultra low power 32k oscillator:
///
/// - [`OscUlp32k`] (`32 KHz`)
/// - [`Gclk1<Ulp32k>`] (`32 KHz`)
///
/// Input arguments for this macro can be retrieved by calling
/// [`crate::clock::v2::retrieve_clocks`] function.
///
/// Example of usage:
///
/// ```rust
/// let mut pac = atsamd_hal::pac::Peripherals::take().unwrap();
///
/// let (gclk0, dfll, osculp32k, tokens) = atsamd_hal::clock::v2::retrieve_clocks(
///     pac.OSCCTRL,
///     pac.OSC32KCTRL,
///     pac.GCLK,
///     pac.MCLK,
///     &mut pac.NVMCTRL,
/// );
///
/// let _ = atsamd_hal::clocking_preset_gclk0_120mhz_gclk5_2mhz_gclk1_internal_32khz!(
///     gclk0, dfll, osculp32k, tokens
/// );
/// ```
#[macro_export]
macro_rules! clocking_preset_gclk0_120mhz_gclk5_2mhz_gclk1_internal_32khz {
    (
        $gclk0:expr,
        $dfll:expr,
        $osculp32k:expr,
        $tokens:expr
    ) => {{
        let (gclk0, gclk5, dpll0, dfll) =
            crate::clocking_preset_gclk0_120mhz_gclk5_2mhz!($gclk0, $dfll, $tokens);
        let (gclk1, osculp32k) = gclk::Gclk::new($tokens.gclks.gclk1, $osculp32k);
        (gclk0, gclk5, gclk1, osculp32k, dpll0, dfll)
    }};
}
