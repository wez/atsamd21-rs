pub mod v1;
pub use v1::*;

pub mod v2;

pub mod types;

#[cfg(feature = "min-samd51n")]
pub fn test() {
    use crate::clock::v2::{
        dpll::Dpll,
        gclk::{Gclk, Gclk1Div, GclkDiv},
        gclkio::{GclkIn, GclkOut},
        pclk::Pclk,
        retrieve_clocks,
    };
    use crate::gpio::v2::Pins;
    use crate::pac::Peripherals;
    use crate::time::U32Ext;

    let mut peripherals = Peripherals::take().unwrap();

    // Get the clocks & tokens
    let (gclk0, dfll, osculp32k, tokens) = retrieve_clocks(
        peripherals.OSCCTRL,
        peripherals.OSC32KCTRL,
        peripherals.GCLK,
        peripherals.MCLK,
        &mut peripherals.NVMCTRL,
    );

    // Get the pins
    let pins = Pins::new(peripherals.PORT);

    // Enable pin PA27 as an external source for Gclk1 at 24 MHz
    let gclk_in1 = GclkIn::enable(tokens.gclk_io.gclk_in1, pins.pa27, 24.mhz());

    // Set Gclk1 to use GclkIn1 divided by 10 = 2.4 MHz
    let (gclk1, _gclk_in1) = Gclk::new(tokens.gclks.gclk1, gclk_in1);
    let gclk1 = gclk1.div(Gclk1Div::Div(10)).enable();

    // Set Dpll0 to use Gclk1 times 80 = 192 MHz
    let (pclk_dpll0, gclk1) = Pclk::enable(tokens.pclks.dpll0, gclk1);
    let dpll0 = Dpll::from_pclk(tokens.dpll0, pclk_dpll0)
        .set_loop_div(80, 0)
        .enable();

    // Change Gclk0 from Dfll to Dpll0 and divide by 2 for 96 MHz
    let (gclk0, _dfll, dpll0) = gclk0.swap(dfll, dpll0);
    let _gclk0 = gclk0.div(GclkDiv::Div(2));

    // Set Gclk2 to use Dpll0 divided by 8 = 24 MHz
    let (gclk2, _dpll0) = Gclk::new(tokens.gclks.gclk2, dpll0);
    let _gclk2 = gclk2.div(GclkDiv::Div(8)).enable();

    // Set Gclk3 to use Gclk1 divided by 10 = 240 kHz
    let (gclk3, _gclk1) = Gclk::new(tokens.gclks.gclk3, gclk1);
    let gclk3 = gclk3.div(GclkDiv::Div(10)).enable();
    // Disable the Gclk, and then enable it again with the improve_duty_cycle bit enabled
    let gclk3 = gclk3.disable();
    let gclk3 = gclk3.improve_duty_cycle(true).enable();

    // Output Gclk3 on pin PB17
    let gclk_out3 = tokens.gclk_io.gclk_out3;
    let (_gclk_out3, _gclk3) = GclkOut::enable(gclk_out3, pins.pb17, gclk3, false);

    let (gclk5, _osculp32k) = Gclk::new(tokens.gclks.gclk5, osculp32k);
    let gclk5 = gclk5.div(GclkDiv::Div(0)).enable();
    let (_gclk_out5, _gclk5) = GclkOut::enable(tokens.gclk_io.gclk_out5, pins.pb11, gclk5, false);
}
