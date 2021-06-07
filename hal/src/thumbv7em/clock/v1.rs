//! Configuring the system clock sources.
//! You will typically need to create an instance of `GenericClockController`
//! before you can set up most of the peripherals on the atsamd51 device.
//! The other types in this module are used to enforce at compile time
//! that the peripherals have been correctly configured.
use crate::target_device::gclk::genctrl::SRC_A::*;
use crate::target_device::gclk::pchctrl::GEN_A::*;
use crate::target_device::{self, GCLK, MCLK, NVMCTRL, OSC32KCTRL, OSCCTRL};
use crate::time::{Hertz, MegaHertz};

pub type ClockGenId = target_device::gclk::pchctrl::GEN_A;
pub type ClockSource = target_device::gclk::genctrl::SRC_A;

#[allow(non_camel_case_types)]
pub enum ClockId {
    DFLL48 = 0,
    FDPLL0,
    FDPLL1,
    SLOW_32K,
    EIC,
    FREQM_MSR,
    FREQM_REF,
    SERCOM0_CORE,
    SERCOM1_CORE,
    TC0_TC1,
    USB,
    EVSYS0,
    EVSYS1,
    EVSYS2,
    EVSYS3,
    EVSYS4,
    EVSYS5,
    EVSYS6,
    EVSYS7,
    EVSYS8,
    EVSYS9,
    EVSYS10,
    EVSYS11,
    SERCOM2_CORE,
    SERCOM3_CORE,
    TCC0_TCC1,
    TC2_TC3,
    CAN0,
    CAN1,
    TCC2_TCC3,
    TC4_TC5,
    PDEC,
    AC,
    CCL,
    SERCOM4_CORE,
    SERCOM5_CORE,
    SERCOM6_CORE,
    SERCOM7_CORE,
    TCC4,
    TC6_TC7,
    ADC0,
    ADC1,
    DAC,
    I2S0,
    I2S1,
    SDHC0,
    SDHC1,
    CM4_TRACE,
}

impl From<ClockId> for u8 {
    fn from(clock: ClockId) -> u8 {
        clock as u8
    }
}

/// Represents a configured clock generator.
/// Can be converted into the effective clock frequency.
/// Its primary purpose is to be passed in to methods
/// such as `GenericClockController::tcc2_tc3` to configure
/// the clock for a peripheral.
//#[derive(Clone, Copy)]
pub struct GClock {
    gclk: ClockGenId,
    freq: Hertz,
}

impl Into<Hertz> for GClock {
    fn into(self) -> Hertz {
        self.freq
    }
}

struct State {
    gclk: GCLK,
}

impl State {
    fn reset_gclk(&mut self) {
        self.gclk.ctrla.write(|w| w.swrst().set_bit());
        while self.gclk.ctrla.read().swrst().bit_is_set() || self.gclk.syncbusy.read().bits() != 0 {
        }
    }

    fn wait_for_sync(&mut self) {
        while self.gclk.syncbusy.read().bits() != 0 {}
    }

    fn set_gclk_divider_and_source(
        &mut self,
        gclk: ClockGenId,
        divider: u16,
        src: ClockSource,
        improve_duty_cycle: bool,
    ) {
        // validate the divisor factor based on gclk ID (see 14.8.3)
        let mut divisor_invalid = false;
        if gclk == GCLK1 {
            if divider as u32 >= 2_u32.pow(16) {
                divisor_invalid = true;
            }
        } else {
            if divider >= 2_u16.pow(8) {
                divisor_invalid = true;
            }
        }
        if divisor_invalid {
            panic!("invalid divisor {} for GCLK {}", divider, gclk as u8);
        }

        self.gclk.genctrl[u8::from(gclk) as usize].write(|w| unsafe {
            w.src().variant(src);
            w.div().bits(divider);
            // divide directly by divider, rather than 2^(n+1)
            w.divsel().clear_bit();
            w.idc().bit(improve_duty_cycle);
            w.genen().set_bit();
            w.oe().set_bit()
        });

        self.wait_for_sync();
    }

    fn enable_clock_generator(&mut self, clock: ClockId, generator: ClockGenId) {
        self.gclk.pchctrl[u8::from(clock) as usize].write(|w| unsafe {
            w.gen().bits(generator.into());
            w.chen().set_bit()
        });
        self.wait_for_sync();
    }

    fn configure_standby(&mut self, gclk: ClockGenId, enable: bool) {
        self.gclk.genctrl[u8::from(gclk) as usize].modify(|_, w| w.runstdby().bit(enable));
        self.wait_for_sync();
    }
}

/// `GenericClockController` encapsulates the GCLK hardware.
/// It provides a type safe way to configure the system clocks.
/// Initializing the `GenericClockController` instance configures
/// the system to run at 120MHz by taking the DFLL48
/// and feeding it into the DPLL0 hardware which multiplies the
/// signal by 2.5x.
pub struct GenericClockController {
    state: State,
    gclks: [Hertz; 12],
    used_clocks: u64,
}

impl GenericClockController {
    /// Reset the clock controller, configure the system to run
    /// at 120Mhz and reset various clock dividers.
    pub fn with_internal_32kosc(
        gclk: GCLK,
        mclk: &mut MCLK,
        osc32kctrl: &mut OSC32KCTRL,
        oscctrl: &mut OSCCTRL,
        nvmctrl: &mut NVMCTRL,
    ) -> Self {
        Self::new(gclk, mclk, osc32kctrl, oscctrl, nvmctrl, false)
    }

    /// Reset the clock controller, configure the system to run
    /// at 120Mhz and reset various clock dividers.
    pub fn with_external_32kosc(
        gclk: GCLK,
        mclk: &mut MCLK,
        osc32kctrl: &mut OSC32KCTRL,
        oscctrl: &mut OSCCTRL,
        nvmctrl: &mut NVMCTRL,
    ) -> Self {
        Self::new(gclk, mclk, osc32kctrl, oscctrl, nvmctrl, true)
    }

    fn new(
        gclk: GCLK,
        mclk: &mut MCLK,
        osc32kctrl: &mut OSC32KCTRL,
        oscctrl: &mut OSCCTRL,
        nvmctrl: &mut NVMCTRL,
        use_external_crystal: bool,
    ) -> Self {
        let mut state = State { gclk };

        set_flash_to_half_auto_wait_state(nvmctrl);
        enable_gclk_apb(mclk);

        if use_external_crystal {
            enable_external_32kosc(osc32kctrl);
            state.reset_gclk();
            state.set_gclk_divider_and_source(GCLK1, 1, XOSC32K, false);
        } else {
            enable_internal_32kosc(osc32kctrl);
            state.reset_gclk();
            state.set_gclk_divider_and_source(GCLK1, 1, OSCULP32K, false);
        }

        while state.gclk.syncbusy.read().genctrl().is_gclk0() {}

        #[cfg(feature = "usb")]
        configure_usb_correction(oscctrl);

        // GCLK5 set to 2MHz
        unsafe {
            state.gclk.genctrl[5].write(|w| {
                w.src().dfll();
                w.genen().set_bit();
                w.div().bits(24)
            });
        }

        while state.gclk.syncbusy.read().genctrl().is_gclk5() {}

        configure_and_enable_dpll0(oscctrl, &mut state.gclk);
        wait_for_dpllrdy(oscctrl);

        unsafe {
            // GCLK0 set to DPLL0 (120MHz)
            state.gclk.genctrl[0].write(|w| {
                w.src().dpll0();
                w.div().bits(1);
                w.oe().set_bit();
                w.genen().set_bit()
            });
        }

        while state.gclk.syncbusy.read().genctrl().is_gclk0() {}

        mclk.cpudiv.write(|w| w.div().div1());

        Self {
            state,
            gclks: [
                OSC120M_FREQ,
                OSC32K_FREQ,
                Hertz(0),
                Hertz(0),
                Hertz(0),
                MegaHertz(2).into(),
                Hertz(0),
                Hertz(0),
                Hertz(0),
                Hertz(0),
                Hertz(0),
                Hertz(0),
            ],
            used_clocks: 1u64 << u8::from(ClockId::FDPLL0),
        }
    }

    /// Returns a `GClock` for gclk0, the 120MHz oscillator.
    pub fn gclk0(&mut self) -> GClock {
        GClock {
            gclk: GCLK0,
            freq: self.gclks[0],
        }
    }

    /// Returns a `GClock` for gclk1, the 32KHz oscillator.
    pub fn gclk1(&mut self) -> GClock {
        GClock {
            gclk: GCLK1,
            freq: self.gclks[1],
        }
    }

    /// Returns the `GClock` for the specified clock generator.
    /// If that clock generator has not yet been configured,
    /// returns None.
    pub fn get_gclk(&mut self, gclk: ClockGenId) -> Option<GClock> {
        let idx = u8::from(gclk) as usize;
        if self.gclks[idx].0 == 0 {
            None
        } else {
            Some(GClock {
                gclk,
                freq: self.gclks[idx],
            })
        }
    }

    /// Configures a clock generator with the specified divider and
    /// source.
    /// `divider` is a linear divider to be applied to the clock
    /// source.  While the hardware also supports an exponential divider,
    /// this function doesn't expose that functionality at this time.
    /// `improve_duty_cycle` is a boolean that, when set to true, enables
    /// a 50/50 duty cycle for odd divider values.
    /// Returns a `GClock` for the configured clock generator.
    /// Returns `None` if the clock generator has already been configured.
    pub fn configure_gclk_divider_and_source(
        &mut self,
        gclk: ClockGenId,
        divider: u16,
        src: ClockSource,
        improve_duty_cycle: bool,
    ) -> Option<GClock> {
        let idx = u8::from(gclk) as usize;
        if self.gclks[idx].0 != 0 {
            return None;
        }
        self.state
            .set_gclk_divider_and_source(gclk, divider, src, improve_duty_cycle);
        let freq: Hertz = match src {
            XOSC32K | OSCULP32K => OSC32K_FREQ,
            GCLKGEN1 => self.gclks[1],
            DFLL => OSC48M_FREQ,
            DPLL0 => OSC120M_FREQ,
            XOSC0 | XOSC1 | GCLKIN | DPLL1 => unimplemented!(),
        };
        self.gclks[idx] = Hertz(freq.0 / divider as u32);
        Some(GClock { gclk, freq })
    }

    /// Enables or disables the given GClk from operation in standby.
    pub fn configure_standby(&mut self, gclk: ClockGenId, enable: bool) {
        self.state.configure_standby(gclk, enable)
    }
}

macro_rules! clock_generator {
    (
        $(
            $(#[$attr:meta])*
            ($id:ident, $Type:ident, $clock:ident, $PclkType:ident),
        )+
    ) => {

use crate::clock::v2::pclk::*;
use crate::sercom::*;

$(

/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
$(#[$attr])*
#[derive(Debug)]
pub struct $Type {
    freq: Hertz,
}

$(#[$attr])*
impl $Type {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz {
        self.freq
    }
}
$(#[$attr])*
impl Into<Hertz> for $Type {
    fn into(self) -> Hertz {
        self.freq
    }
}

/// V2 to V1 compatibility layer that allows to convert V2 [`Pclk`] constructs
/// into corresponding V1 `*Clock` types. Thus, user can manage V1 clocking
/// compatible peripherals while using V2 clocking API
$(#[$attr])*
impl<T: PclkSourceMarker> core::convert::From<Pclk<$PclkType, T>> for $Type {
    fn from(pclk: Pclk<$PclkType, T>) -> Self {
        $Type {
            freq: pclk.freq()
        }
    }
}


)+

impl GenericClockController {
    $(
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    $(#[$attr])*
    pub fn $id(&mut self, generator: &GClock) -> Option<$Type> {
        let bits: u64 = 1 << u8::from(ClockId::$clock) as u64;
        if (self.used_clocks & bits) != 0 {
            return None;
        }
        self.used_clocks |= bits;

        self.state.enable_clock_generator(ClockId::$clock, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some($Type{freq})
    }
    )+
}
    }
}

clock_generator!(
    (tc0_tc1, Tc0Tc1Clock, TC0_TC1, Tc0Tc1),
    (tcc0_tcc1, Tcc0Tcc1Clock, TCC0_TCC1, Tcc0Tcc1),
    (tc2_tc3, Tc2Tc3Clock, TC2_TC3, Tc2Tc3),
    (tcc2_tcc3, Tcc2Tcc3Clock, TCC2_TCC3, Tcc2Tcc3),
    (tc4_tc5, Tc4Tc5Clock, TC4_TC5, Tc4Tc5),
    (tcc4, Tcc4Clock, TCC4, Tcc4),
    (tc6_tc7, Tc6Tc7Clock, TC6_TC7, Tc6Tc7),
    (sercom0_core, Sercom0CoreClock, SERCOM0_CORE, Sercom0),
    (sercom1_core, Sercom1CoreClock, SERCOM1_CORE, Sercom1),
    (sercom2_core, Sercom2CoreClock, SERCOM2_CORE, Sercom2),
    (sercom3_core, Sercom3CoreClock, SERCOM3_CORE, Sercom3),
    (sercom4_core, Sercom4CoreClock, SERCOM4_CORE, Sercom4),
    (sercom5_core, Sercom5CoreClock, SERCOM5_CORE, Sercom5),
    #[cfg(feature = "min-samd51n")]
    (sercom6_core, Sercom6CoreClock, SERCOM6_CORE, Sercom6),
    #[cfg(feature = "min-samd51n")]
    (sercom7_core, Sercom7CoreClock, SERCOM7_CORE, Sercom7),
    (usb, UsbClock, USB, Usb),
    (adc0, Adc0Clock, ADC0, Adc0),
    (adc1, Adc1Clock, ADC1, Adc1),
    (eic, EicClock, EIC, Eic),
    (freq_m_msr, FreqmMsrClock, FREQM_MSR, FreqmMsr),
    (freq_m_ref, FreqmRefClock, FREQM_REF, FreqmRef),
    (evsys0, Evsys0Clock, EVSYS0, Evsys0),
    (evsys1, Evsys1Clock, EVSYS1, Evsys1),
    (evsys2, Evsys2Clock, EVSYS2, Evsys2),
    (evsys3, Evsys3Clock, EVSYS3, Evsys3),
    (evsys4, Evsys4Clock, EVSYS4, Evsys4),
    (evsys5, Evsys5Clock, EVSYS5, Evsys5),
    (evsys6, Evsys6Clock, EVSYS6, Evsys6),
    (evsys7, Evsys7Clock, EVSYS7, Evsys7),
    (evsys8, Evsys8Clock, EVSYS8, Evsys8),
    (evsys9, Evsys9Clock, EVSYS9, Evsys9),
    (evsys10, Evsys10Clock, EVSYS10, Evsys10),
    (evsys11, Evsys11Clock, EVSYS11, Evsys11),
    (can0, Can0Clock, CAN0, Can0),
    (can1, Can1Clock, CAN1, Can1),
    (pdec, PdecClock, PDEC, Pdec),
    (ac, AcClock, AC, Ac),
    (ccl, CclClock, CCL, Ccl),
    (dac, DacClock, DAC, Dac),
    (i2s0, I2S0Clock, I2S0, I2s0),
    (i2s1, I2S1Clock, I2S1, I2s1),
    (sdhc0, Sdhc0Clock, SDHC0, Sdhc0),
    (sdhc1, Sdhc1Clock, SDHC1, Sdhc1),
    (cm4_trace, Cm4TraceClock, CM4_TRACE, Cm4Trace),
);

/// The frequency of the 48Mhz source.
pub const OSC48M_FREQ: Hertz = Hertz(48_000_000);
/// The frequency of the 32Khz source.
pub const OSC32K_FREQ: Hertz = Hertz(32_768);
/// The frequency of the 120Mhz source.
pub const OSC120M_FREQ: Hertz = Hertz(120_000_000);

fn set_flash_to_half_auto_wait_state(nvmctrl: &mut NVMCTRL) {
    // Zero indicates zero wait states, one indicates one wait state, etc.,
    // up to 15 wait states.
    nvmctrl.ctrla.modify(|_, w| unsafe { w.rws().bits(0b0111) });
}

fn enable_gclk_apb(mclk: &mut MCLK) {
    mclk.apbamask.modify(|_, w| w.gclk_().set_bit());
}

/// Turn on the internal 32hkz oscillator
fn enable_internal_32kosc(osc32kctrl: &mut OSC32KCTRL) {
    osc32kctrl.osculp32k.modify(|_, w| {
        w.en32k().set_bit();
        w.en1k().set_bit()
    });
    osc32kctrl.rtcctrl.write(|w| w.rtcsel().ulp1k());
}

/// Turn on the external 32hkz oscillator
fn enable_external_32kosc(osc32kctrl: &mut OSC32KCTRL) {
    osc32kctrl.xosc32k.modify(|_, w| {
        w.ondemand().clear_bit();
        // Enable 32khz output
        w.en32k().set_bit();
        w.en1k().set_bit();
        // Crystal connected to xin32/xout32
        w.xtalen().set_bit();
        w.enable().set_bit();
        w.cgm().xt();
        w.runstdby().set_bit()
    });

    osc32kctrl.rtcctrl.write(|w| w.rtcsel().xosc1k());

    // Wait for the oscillator to stabilize
    while osc32kctrl.status.read().xosc32krdy().bit_is_clear() {}
}

fn wait_for_dpllrdy(oscctrl: &mut OSCCTRL) {
    while oscctrl.dpll[0].dpllstatus.read().lock().bit_is_clear()
        || oscctrl.dpll[0].dpllstatus.read().clkrdy().bit_is_clear()
    {}
}

/// Configure the dpll0 to run at 120MHz
fn configure_and_enable_dpll0(oscctrl: &mut OSCCTRL, gclk: &mut GCLK) {
    gclk.pchctrl[ClockId::FDPLL0 as usize].write(|w| {
        w.chen().set_bit();
        w.gen().gclk5()
    });
    unsafe {
        oscctrl.dpll[0].dpllratio.write(|w| {
            w.ldr().bits(59);
            w.ldrfrac().bits(0)
        });
    }
    oscctrl.dpll[0].dpllctrlb.write(|w| w.refclk().gclk());
    oscctrl.dpll[0].dpllctrla.write(|w| {
        w.enable().set_bit();
        w.ondemand().clear_bit()
    });
}

#[cfg(feature = "usb")]
/// Configure the dfll48m to calibrate against the 1Khz USB SOF reference.
fn configure_usb_correction(oscctrl: &mut OSCCTRL) {
    oscctrl.dfllmul.write(|w| unsafe {
        w.cstep().bits(0x1)
        .fstep().bits(0x1)
        // scaling factor for 1Khz SOF signal.
        .mul().bits((48_000_000u32 / 1000) as u16)
    });
    while oscctrl.dfllsync.read().dfllmul().bit_is_set() {}

    oscctrl.dfllctrlb.write(|w| {
        // closed loop mode
        w.mode().set_bit()
        // chill cycle disable
        .ccdis().set_bit()
        // usb correction
        .usbcrm().set_bit()
    });
    while oscctrl.dfllsync.read().dfllctrlb().bit_is_set() {}
}
