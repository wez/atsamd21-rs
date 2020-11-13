#[rustfmt::skip]
use atsamd_hal::clock::{GenericClockController, Sercom6CoreClock};
use atsamd_hal::gpio::{
    Floating, Input, Output, Pa10, Pa11, Pa8, Pa9, Pb10, Pb11, Pc16, Pc17, Pc18, Pc19, Pd21, PfC,
    Port, PushPull,
};
use atsamd_hal::hal::spi;
use atsamd_hal::prelude::*;
use atsamd_hal::qspi;
use atsamd_hal::sercom::{PadPin, SPIMaster6, Sercom6Pad0, Sercom6Pad1, Sercom6Pad2};
use atsamd_hal::target_device::{MCLK, QSPI, SERCOM6};
use atsamd_hal::time::Hertz;
use core::convert::TryFrom;

use embedded_sdmmc::{SdMmcSpi, TimeSource};

/// QSPI Flash pins (uses `SERCOM4`)
pub struct QSPIFlash {
    /// QSPI Flash `sck` pin
    pub sck: Pb10<Input<Floating>>,

    /// QSPI Flash chip select pin
    pub cs: Pb11<Input<Floating>>,

    /// QSPI Flash `d0` pin
    pub d0: Pa8<Input<Floating>>,

    /// QSPI Flash `d1` pin
    pub d1: Pa9<Input<Floating>>,

    /// QSPI Flash `d2` pin
    pub d2: Pa10<Input<Floating>>,

    /// QSPI Flash `d3` pin
    pub d3: Pa11<Input<Floating>>,
}

impl QSPIFlash {
    pub fn init(self, mclk: &mut MCLK, port: &mut Port, qspi: QSPI) -> qspi::Qspi<qspi::OneShot> {
        qspi::Qspi::new(
            mclk, port, qspi, self.sck, self.cs, self.d0, self.d1, self.d2, self.d3,
        )
    }
}

/// SD Card pins (uses `SERCOM6`)
pub struct SDCard {
    /// SD Card chip select pin
    pub cs: Pc19<Input<Floating>>,

    /// SD Card `mosi` pin
    pub mosi: Pc16<Input<Floating>>,

    /// SD Card `sck` pin
    pub sck: Pc17<Input<Floating>>,

    /// SD Card `miso` pin
    pub miso: Pc18<Input<Floating>>,

    /// SD Card detect pin
    pub det: Pd21<Input<Floating>>,
}

type Controller<TS> = embedded_sdmmc::Controller<
    SdMmcSpi<
        SPIMaster6<Sercom6Pad2<Pc18<PfC>>, Sercom6Pad0<Pc16<PfC>>, Sercom6Pad1<Pc17<PfC>>>,
        Pc19<Output<PushPull>>,
    >,
    TS,
>;

/// An initialized SPI SDMMC controller.
pub struct SDCardController<TS: TimeSource> {
    pub cont: Controller<TS>,
    sercom6_clk: Sercom6CoreClock,
}

impl<TS: TimeSource> SDCardController<TS> {
    /// Initializes the MMC card. An error is returned if there is no card
    /// or a communications error occurs.
    pub fn set_baud<B: Into<Hertz>>(&mut self, baud: B) {
        self.cont.device().spi().set_baud(baud, &self.sercom6_clk)
    }
}

impl<TS: TimeSource> core::ops::Deref for SDCardController<TS> {
    type Target = Controller<TS>;

    fn deref(&self) -> &Self::Target {
        &self.cont
    }
}

impl<TS: TimeSource> core::ops::DerefMut for SDCardController<TS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cont
    }
}

impl SDCard {
    /// Initialize the controller and its corresponding SPI bus peripheral.
    pub fn init<TS: TimeSource>(
        self,
        clocks: &mut GenericClockController,
        sercom6: SERCOM6,
        mclk: &mut MCLK,
        port: &mut Port,
        ts: TS,
    ) -> Result<(SDCardController<TS>, Pd21<Input<Floating>>), ()> {
        let gclk0 = clocks.gclk0();
        let sercom6_clk = clocks.sercom6_core(&gclk0).ok_or(())?;
        let spi = SPIMaster6::new(
            &sercom6_clk,
            Hertz::try_from(Hertz::from(400.kHz())).unwrap(),
            spi::Mode {
                phase: spi::Phase::CaptureOnFirstTransition,
                polarity: spi::Polarity::IdleLow,
            },
            sercom6,
            mclk,
            (
                self.miso.into_pad(port),
                self.mosi.into_pad(port),
                self.sck.into_pad(port),
            ),
        );

        let cs = self.cs.into_push_pull_output(port);

        Ok((
            SDCardController {
                cont: embedded_sdmmc::Controller::new(SdMmcSpi::new(spi, cs), ts),
                sercom6_clk,
            },
            self.det,
        ))
    }
}
