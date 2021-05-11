pub mod v1;
pub use v1::*;

pub mod v2;

#[cfg(feature = "min-samd51n")]
pub fn test() {
    use crate::clock::v2::{gclk, Dpll, GclkIn, GclkOut, Pclk, Tokens};
    use crate::gpio::v2::Pins;
    use crate::pac::Peripherals;
    use crate::time::U32Ext;

    let mut peripherals = Peripherals::take().unwrap();

    // Get the clocks & tokens
    let (gclk0, dfll, /*osculp32k,*/ tokens) = Tokens::new(
        peripherals.OSCCTRL,
        peripherals.OSC32KCTRL,
        peripherals.GCLK,
        peripherals.MCLK,
        &mut peripherals.NVMCTRL,
    );

    // Get the pins
    let pins = Pins::new(peripherals.PORT);

    // Enable pin PA27 as an external source for Gclk1 at 24 MHz
    let gclk_in1 = GclkIn::enable(tokens.sources.gclk_io.gclk_in1, pins.pa27, 24.mhz());

    // Set Gclk1 to use GclkIn1 divided by 10 = 2.4 MHz
    let (gclk1, _gclk_in1) = gclk::Gclk::new(tokens.gclks.gclk1, gclk_in1);
    let gclk1 = gclk1.div(gclk::Gclk1Div::Div(10)).enable();

    // Set Dpll0 to use Gclk1 times 80 = 192 MHz
    let (pclk_dpll0, gclk1) = Pclk::new(tokens.pclks.dpll0, gclk1);
    let dpll0 = Dpll::from_pclk(tokens.sources.dpll0, pclk_dpll0)
        .set_loop_div(80, 0)
        .enable();

    // Change Gclk0 from Dfll to Dpll0 and divide by 2 for 96 MHz
    let (mut gclk0, _dfll, dpll0) = unsafe { gclk0.swap(dfll, dpll0) };
    unsafe { gclk0.div(gclk::GclkDiv::Div(2)) };

    // Set Gclk2 to use Dpll0 divided by 8 = 24 MHz
    let (gclk2, _dpll0) = gclk::Gclk::new(tokens.gclks.gclk2, dpll0);
    let _gclk2 = gclk2.div(gclk::GclkDiv::Div(8)).enable();

    // Set Gclk2 to use Gclk1 divided by 10 = 240 kHz
    let (gclk3, _gclk1) = gclk::Gclk::new(tokens.gclks.gclk3, gclk1);
    let gclk3 = gclk3.div(gclk::GclkDiv::Div(10)).enable();

    // Output Gclk3 on pin PB17
    let gclk_out3 = tokens.sources.gclk_io.gclk_out3;
    let (_gclk_out3, _gclk3) = GclkOut::new(gclk_out3, pins.pb17, gclk3, false);
}
