pub mod v1;
pub use v1::*;

pub mod v2;

#[cfg(feature = "min-samd51n")]
pub fn test() {
    use crate::clock::v2::{gclk, Clocks, Source};
    use crate::gpio::v2::Pins;
    use crate::pac::Peripherals;
    use crate::time::U32Ext;
    let mut peripherals = Peripherals::take().unwrap();
    // Get the clocks
    let clocks = Clocks::new(
        peripherals.OSCCTRL,
        peripherals.OSC32KCTRL,
        peripherals.GCLK,
        peripherals.MCLK,
        &mut peripherals.NVMCTRL,
    );
    // Get the pins
    let pins = Pins::new(peripherals.PORT);
    // Pin PA27 is a clock input for GCLK 1, and it has a 24 MHz oscillator.
    // Get the corresponding `GclkInToken` from the `Clocks` struct
    let gclk_in1 = clocks.sources.gclk_io.gclk_in1;
    // Enable the `GclkInConfig` using pin PA27 to produce a `GclkIn`.
    let gclk_in1 = gclk_in1.enable(pins.pa27, 24.mhz());
    // Package the `GclkIn` as a clock `Source`
    let gclk_in1 = Source::new(gclk_in1);
    // Take the `GclkConfig` for GCLK 1 from the `Clocks` struct
    let gclk1 = clocks.gclks.gclk1;
    // Use the `GclkIn` as the `Source` for the `GclkConfig`. Doing so locks
    // the `GclkIn` `Source`. It can't be safely modified until that lock is
    // released.
    let (gclk1, _gclk_in1) = gclk1.set_source(gclk_in1);
    // Set the `GclkConfig` divider to 10, which yields a 2.4 MHz output.
    // Enable the `GclkConfig` to produce a `Gclk`.
    let gclk1 = gclk1.div(gclk::Div::Div(10)).enable();
    // Take the peripheral clock token (`PclkToken`) for DPLL0 from the `Clocks`
    // struct.
    let pclk_dpll0 = clocks.pclks.dpll0;
    // Enable the `PclkToken` using `Gclk` 1 to produce a `Pclk`. Doing so also
    // locks `Gclk` 1, which can't be safely disabled until the lock is
    // released.
    let (dpll0_src, gclk1) = pclk_dpll0.enable(gclk1);
    // Get the `DpllConfig` for DPLL 0 from the `Clocks` struct
    let dpll0 = clocks.sources.dpll0;
    // Use the DPLL0 `Pclk` to set the source for DPLL0. The `Pclk` is consumed.
    // Set the loop divider to 80, which will yield a 192 MHz clock. Enable the
    // `DpllConfig` to produce a `Dpll`.
    let dpll0 = dpll0
        .set_gclk_source(dpll0_src)
        .set_loop_div(80, 0)
        .enable();
    // Package the `Dpll` as a clock `Source`
    let dpll0 = Source::new(dpll0);
    // Get the `Dfll` from the `Clocks` struct. It is already wrapped in a
    // `Source` and has one lock, because it is used as the main clock at
    // reset.
    let dfll = clocks.sources.dfll;
    // Get `Gclk` 0 from the `Clocks` struct. It is already enabled, using the
    // `Dfll` as its source.
    let gclk0 = clocks.gclks.gclk0;
    // Swap the clock `Source` for `Gclk` 0 from the `Dfll` to `Dpll` 0. Doing
    // so is inherently unsafe, because you are altering a running clock.
    // The `Dfll` lock is released, so it can now be modified. The `Dpll` 0 is
    // lock, so it cannot.
    let (mut gclk0, _dfll, dpll0) = unsafe { gclk0.swap_source(dfll, dpll0) };
    // Set the `Gclk` 0 divider to 2, which makes the main clock 96 MHz. Doing
    // so is again unsafe, because it is modifying a running clock.
    unsafe { gclk0.div(gclk::Div::Div(2)) };
    // Use `Dpll` 0 as the `Source` for `GclkConfig` 2
    let (gclk2, _dpll0) = clocks.gclks.gclk2.set_source(dpll0);
    // Set the divider to 8 for a 24 MHz clock and enable `Gclk` 2
    let _gclk2 = gclk2.div(gclk::Div::Div(8)).enable();
    // Package `Gclk` 1 as a clock `Source`
    let gclk1 = Source::new(gclk1);
    // Use `Gclk` 1 as the `Source` for `GclkConfig` 3
    let (gclk3, _gclk1) = clocks.gclks.gclk3.set_source(gclk1);
    // Set the divider to 10 for a 240 kHz clock and enable `Gclk` 3
    let gclk3 = gclk3.div(gclk::Div::Div(10)).enable();
    // Get the `GclkOutToken` for `Gclk` 3 from the `Clocks` struct
    let gclk_out3 = clocks.sources.gclk_io.gclk_out3;
    // Enable the `GclkOutToken` using pin PB17 to produce a `GclkOut`. Set the
    // off value of the output clock to low. This locks `Gclk` 3, so it can't
    // be safely disabled.
    let (_gclk_out3, _gclk3) = gclk_out3.enable(gclk3, pins.pb17, false);
}
