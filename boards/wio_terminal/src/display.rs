use atsamd_hal::clock::GenericClockController;
use atsamd_hal::delay::Delay;
use atsamd_hal::gpio::*;
use atsamd_hal::hal::spi;
use atsamd_hal::prelude::*;
use atsamd_hal::sercom::{PadPin, SPIMaster7, Sercom7Pad1, Sercom7Pad2, Sercom7Pad3};
use atsamd_hal::target_device::{MCLK, SERCOM7};
use atsamd_hal::time::Hertz;
use core::convert::TryFrom;
use display_interface_spi::SPIInterface;
use ili9341::{Ili9341, Orientation};

/// ILI9341 LCD display pins (uses `SERCOM7`)
pub struct Display {
    /// LCD MISO pin
    pub miso: Pb18<Input<Floating>>,

    /// LCD MOSI pin
    pub mosi: Pb19<Input<Floating>>,

    /// LCD SCK pin
    pub sck: Pb20<Input<Floating>>,

    /// LCD chip select pin
    pub cs: Pb21<Input<Floating>>,

    /// LCD data/command pin
    pub dc: Pc6<Input<Floating>>,

    /// LCD reset pin
    pub reset: Pc7<Input<Floating>>,

    /// LCD backlight pin
    pub backlight: Pc5<Input<Floating>>,
}

/// Type alias for the ILI9341 LCD display.
pub type LCD = Ili9341<
    SPIInterface<
        SPIMaster7<Sercom7Pad2<Pb18<PfD>>, Sercom7Pad3<Pb19<PfD>>, Sercom7Pad1<Pb20<PfD>>>,
        Pc6<Output<PushPull>>,
        Pb21<Output<PushPull>>,
    >,
    Pc7<Output<PushPull>>,
>;

pub use ili9341::Scroller;

impl Display {
    /// Initialize the display and its corresponding SPI bus peripheral. Return
    /// a tuple containing the configured display driver struct and backlight
    /// pin.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        sercom7: SERCOM7,
        mclk: &mut MCLK,
        port: &mut Port,
        delay: &mut Delay,
    ) -> Result<(LCD, Pc5<Output<PushPull>>), ()> {
        // Initialize the SPI peripherial on the configured pins, using SERCOM7 and
        // running at 20MHz.
        let gclk0 = clocks.gclk0();
        let spi = SPIMaster7::new(
            &clocks.sercom7_core(&gclk0).ok_or(())?,
            Hertz::try_from(Hertz::from(20.MHz())).unwrap(),
            spi::Mode {
                phase: spi::Phase::CaptureOnFirstTransition,
                polarity: spi::Polarity::IdleLow,
            },
            sercom7,
            mclk,
            (
                self.miso.into_pad(port),
                self.mosi.into_pad(port),
                self.sck.into_pad(port),
            ),
        );

        // Configure the chip select, data/command, and reset pins as push-pull outputs.
        let cs = self.cs.into_push_pull_output(port);
        let dc = self.dc.into_push_pull_output(port);
        let reset = self.reset.into_push_pull_output(port);

        // Create a SPIInterface over the peripheral, then create the ILI9341 driver
        // using said interface and set its default orientation.
        let interface = SPIInterface::new(spi, dc, cs);
        let mut ili9341 = Ili9341::new(interface, reset, delay).map_err(|_| ())?;
        ili9341
            .set_orientation(Orientation::LandscapeFlipped)
            .map_err(|_| ())?;

        // Configure the backlight pin as a push-pull output; unfortunately this pin
        // does not appear to support PWM.
        //   HIGH - backlight enabled
        //   LOW  - backlight disabled
        let mut backlight = self.backlight.into_push_pull_output(port);
        backlight.set_high()?;

        // Return a result consisting of a Tuple containing the display driver and
        // backlight pin.
        Ok((ili9341, backlight))
    }
}
