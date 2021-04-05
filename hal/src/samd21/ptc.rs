use crate::clock::GenericClockController;
use crate::gpio::v1;
use crate::gpio::v2::*;
use crate::hal::adc::{Channel, OneShot};
use crate::target_device::{
    ptc::{
        convctrl::ADCACCUM_A, freqctrl::SAMPLEDELAY_A, serres::RESISTOR_A, xselect::XMUX_A,
        yselect::YMUX_A,
    },
    PM, PTC,
};

pub struct Ptc<PTC> {
    ptc: PTC,
}

impl Ptc<PTC> {
    pub fn ptc(ptc: PTC, pm: &mut PM, clocks: &mut GenericClockController) -> Self {
        // Enable PTC in the APBC mask
        pm.apbcmask.modify(|_, w| w.ptc_().set_bit());
        let gclk1 = clocks.gclk1();
        // Enable the PTC clock
        clocks.ptc(&gclk1).expect("ptc clock setup failed");
        while ptc.ctrlb.read().syncflag().bit_is_set() {}

        // Reset the PTC module
        ptc.ctrla.modify(|_, w| w.swrst().set_bit());
        while ptc.ctrlb.read().syncflag().bit_is_set() {}

        // Magic writes? Honestly dunno what these are for.
        // f7 => 11110111
        // fb => 11111011
        // fc => 11111100
        ptc.unk4c04.write(|w| unsafe { w.bits(0xf7) });
        ptc.unk4c04.write(|w| unsafe { w.bits(0xfb) });
        ptc.unk4c04.write(|w| unsafe { w.bits(0xfc) });
        while ptc.ctrlb.read().syncflag().bit_is_set() {}

        // Next in the init sequence in the FreeTouch repo, writes of the following two
        // values are made to FREQCTRL:
        // 9f => 10011111
        //       ---baaaa
        // ef => 11101111
        //       ---baaaa
        // The upper three bits are unused, so I'm unsure what the point of this is
        // beyond setting all of the SAMPLEDELAY field to 1 and toggling
        // FREQSPREADEN. Furthermore, the next thing done is setting SAMPLEDELAY
        // to 0, thus ending up with (in theory):   11101111
        // & 11110000
        // ----------
        //   11100000
        // So honestly, I'm just going to set them to 0 in one step.
        ptc.freqctrl.write(|w| {
            w.freqspreaden().clear_bit();
            w.sampledelay().variant(SAMPLEDELAY_A::FREQHOP1)
        });
        while ptc.ctrlb.read().syncflag().bit_is_set() {}

        // Software init
        ptc.ctrlc.write(|w| w.init().set_bit());
        // Set to run in standby
        ptc.ctrla.write(|w| w.runstdby().set_bit());
        while ptc.ctrlb.read().syncflag().bit_is_set() {}

        // Set interrupt enables
        ptc.intenclr.write(|w| {
            w.wco().set_bit();
            w.eoc().set_bit()
        });
        while ptc.ctrlb.read().syncflag().bit_is_set() {}

        Self { ptc }
    }

    pub fn compcap(&mut self, compcap: u16) {
        self.ptc
            .compcap
            .write(|w| unsafe { w.value().bits(compcap) });
        while self.ptc.ctrlb.read().syncflag().bit_is_set() {}
    }

    pub fn intcap(&mut self, intcap: u8) {
        self.ptc.intcap.write(|w| unsafe { w.value().bits(intcap) });
        while self.ptc.ctrlb.read().syncflag().bit_is_set() {}
    }

    pub fn oversample(&mut self, oversample: ADCACCUM_A) {
        self.ptc
            .convctrl
            .write(|w| w.adcaccum().variant(oversample));
        while self.ptc.ctrlb.read().syncflag().bit_is_set() {}
    }

    pub fn sample_delay(&mut self, sampledelay: SAMPLEDELAY_A) {
        match sampledelay {
            SAMPLEDELAY_A::FREQHOP1 => self.ptc.freqctrl.write(|w| w.freqspreaden().clear_bit()),
            _ => self.ptc.freqctrl.write(|w| w.freqspreaden().set_bit()),
        }
        self.ptc
            .freqctrl
            .write(|w| w.sampledelay().variant(sampledelay));
        while self.ptc.ctrlb.read().syncflag().bit_is_set() {}
    }

    pub fn series_resistance(&mut self, serres: RESISTOR_A) {
        self.ptc.serres.write(|w| w.resistor().variant(serres));
        while self.ptc.ctrlb.read().syncflag().bit_is_set() {}
    }

    pub fn xselect(&mut self, xselect: XMUX_A) {
        self.ptc.xselect.write(|w| w.xmux().variant(xselect));
    }

    pub fn yselect(&mut self, yselect: YMUX_A) {
        self.ptc.yselect.write(|w| w.ymux().variant(yselect));
    }

    fn power_up(&mut self) {
        while self.ptc.ctrlb.read().syncflag().bit_is_set() {}
        self.ptc.ctrla.modify(|_, w| w.enable().set_bit());
        while self.ptc.ctrlb.read().syncflag().bit_is_set() {}
    }

    fn power_down(&mut self) {
        while self.ptc.ctrlb.read().syncflag().bit_is_set() {}
        self.ptc.ctrla.modify(|_, w| w.enable().clear_bit());
        while self.ptc.ctrlb.read().syncflag().bit_is_set() {}
    }

    fn convert(&mut self) -> u16 {
        self.ptc
            .burstmode
            .write(|w| unsafe { w.burstmode().bits(0xa4) });
        while self.ptc.ctrlb.read().syncflag().bit_is_set() {}

        self.ptc.convctrl.write(|w| w.convert().set_bit());
        while self.ptc.ctrlb.read().syncflag().bit_is_set() {}

        self.ptc.result.read().result().bits()
    }
}

impl<WORD, PIN> OneShot<PTC, WORD, PIN> for Ptc<PTC>
where
    WORD: From<u16>,
    PIN: Channel<PTC, ID = u8>,
{
    type Error = ();

    fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
        let channel = PIN::channel();
        while self.ptc.ctrlb.read().syncflag().bit_is_set() {}

        // Select and enable specified channel
        if channel > 15 {
            let channel = channel - 16;
            self.ptc
                .xselect
                .write(|w| unsafe { w.xmux().bits(1 << channel) });
            self.ptc
                .xselecten
                .modify(|r, w| unsafe { w.bits(r.bits() | (1 << channel)) });
        } else {
            self.ptc
                .yselect
                .write(|w| unsafe { w.ymux().bits(1 << channel) });
            self.ptc
                .yselecten
                .modify(|r, w| unsafe { w.bits(r.bits() | (1 << channel)) });
        };
        self.power_up();
        let result = self.convert();
        self.power_down();

        Ok(result.into())
    }
}

macro_rules! ptc_pins {
    ($($PinId:ident: $Chan:literal),+) => {
        $(
            impl Channel<PTC> for Pin<$PinId, AlternateB> {
                type ID = u8;
                fn channel() -> u8 { $Chan }
            }
        )+
    }
}

/// Implement ['Channel`] for [`v1::Pin`]s based on the implementations for
/// `v2` [`Pin`]s
impl<I> Channel<PTC> for v1::Pin<I, v1::PfB>
where
    I: PinId,
    Pin<I, AlternateB>: Channel<PTC, ID = u8>,
{
    type ID = u8;
    fn channel() -> u8 {
        Pin::<I, AlternateB>::channel()
    }
}

ptc_pins! {
    PA02: 0,
    PA03: 1,
    PA04: 2,
    PA05: 3,
    PA06: 4,
    PA07: 5,
    PA08: 16,
    PA09: 17,
    PA10: 18,
    PA11: 19,
    PA16: 20,
    PA17: 21,
    PA18: 22,
    PA19: 23,
    PA22: 26,
    PA23: 27
}

#[cfg(any(feature = "samd21g", feature = "samd21j"))]
ptc_pins! {
    PB02: 8,
    PB03: 9,
    PB08: 14,
    PB09: 15,
    PA20: 24,
    PA21: 25
}

#[cfg(feature = "samd21j")]
ptc_pins! {
    PB00: 6,
    PB01: 7,
    PB04: 10,
    PB05: 11,
    PB06: 12,
    PB07: 13,
    PB12: 28,
    PB13: 29,
    PB14: 30,
    PB15: 31
}
