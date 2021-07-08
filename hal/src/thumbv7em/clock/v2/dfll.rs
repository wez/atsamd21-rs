use typenum::{U0, U1};

use crate::clock::types::{Counter, Enabled, PrivateIncrement};
use crate::clock::v2::{Source, SourceMarker};
use crate::time::{Hertz, U32Ext};
use crate::typelevel::Sealed;

use super::gclk::{Gclk0, GclkSource, GclkSourceEnum, GclkSourceMarker, GenNum};
use super::gclkio::NotGclkInput;
use super::pclk::{Dfll48, Pclk, PclkSourceMarker};

/// TODO
pub struct DfllToken {
    __: (),
}

/// TODO
impl DfllToken {
    /// TODO
    #[inline]
    pub(crate) unsafe fn new() -> Self {
        Self { __: () }
    }

    #[inline]
    fn oscctrl(&self) -> &crate::pac::oscctrl::RegisterBlock {
        unsafe { &*crate::pac::OSCCTRL::ptr() }
    }

    #[inline]
    fn dfllctrla(&self) -> &crate::pac::oscctrl::DFLLCTRLA {
        &self.oscctrl().dfllctrla
    }

    #[inline]
    fn dfllctrlb(&self) -> &crate::pac::oscctrl::DFLLCTRLB {
        &self.oscctrl().dfllctrlb
    }

    #[allow(dead_code)]
    #[inline]
    fn dfllval(&self) -> &crate::pac::oscctrl::DFLLVAL {
        &self.oscctrl().dfllval
    }

    #[inline]
    fn dfllmul(&self) -> &crate::pac::oscctrl::DFLLMUL {
        &self.oscctrl().dfllmul
    }

    #[inline]
    fn dfllsync(&self) -> &crate::pac::oscctrl::DFLLSYNC {
        &self.oscctrl().dfllsync
    }

    #[inline]
    fn wait_sync_enable(&self) {
        while self.dfllsync().read().enable().bit() == true {}
    }

    #[inline]
    fn wait_sync_dfllmul(&self) {
        while self.dfllsync().read().dfllmul().bit() == true {}
    }

    #[allow(dead_code)]
    #[inline]
    fn wait_sync_dfllval(&self) {
        while self.dfllsync().read().dfllval().bit() == true {}
    }

    #[inline]
    fn wait_sync_dfllctrlb(&self) {
        while self.dfllsync().read().dfllctrlb().bit() == true {}
    }

    #[inline]
    fn set_on_demand_mode(&mut self, value: bool) {
        self.dfllctrla().modify(|_, w| w.ondemand().bit(value));
    }

    #[inline]
    fn set_run_standby(&mut self, value: bool) {
        self.dfllctrla().modify(|_, w| w.runstdby().bit(value));
    }

    #[inline]
    fn enable(&mut self) {
        self.dfllctrla().modify(|_, w| w.enable().set_bit());
        self.wait_sync_enable();
    }

    #[inline]
    fn disable(&mut self) {
        self.dfllctrla().modify(|_, w| w.enable().clear_bit());
        self.wait_sync_enable();
    }

    #[inline]
    fn set_open_mode(&mut self) {
        self.dfllctrlb().modify(|_, w| w.mode().clear_bit());
        self.wait_sync_dfllctrlb();
    }

    #[inline]
    fn set_closed_mode(&mut self) {
        self.dfllctrlb().modify(|_, w| w.mode().set_bit());
        self.wait_sync_dfllctrlb();
    }

    #[inline]
    fn set_fine_maximum_step(&mut self, value: u8) {
        self.dfllmul()
            .modify(|_, w| unsafe { w.fstep().bits(value) });
        self.wait_sync_dfllmul();
    }

    #[inline]
    fn set_coarse_maximum_step(&mut self, value: u8) {
        self.dfllmul()
            .modify(|_, w| unsafe { w.cstep().bits(value) });
        self.wait_sync_dfllmul();
    }

    #[inline]
    fn set_multiplication_factor(&mut self, value: u16) {
        self.dfllmul().modify(|_, w| unsafe { w.mul().bits(value) });
        self.wait_sync_dfllmul();
    }
}

type MultiplicationFactor = u16;
type CoarseMaximumStep = u8;
type FineMaximumStep = u8;
type Fine = u8;
type Coarse = u8;

pub trait LoopMode: Sealed {
    fn enable(&self, token: &mut DfllToken);
}

pub struct OpenLoop {
    // TODO: Add support for custom fine and coarse? Otherwise remove it.
    #[allow(dead_code)]
    fine: Option<Fine>,
    #[allow(dead_code)]
    coarse: Option<Coarse>,
}
impl LoopMode for OpenLoop {
    fn enable(&self, token: &mut DfllToken) {
        token.set_open_mode();
    }
}
impl Sealed for OpenLoop {}
pub struct ClosedLoop<T: PclkSourceMarker> {
    reference_clk: Pclk<Dfll48, T>,
    coarse_maximum_step: CoarseMaximumStep,
    fine_maximum_step: FineMaximumStep,
}
impl<T: PclkSourceMarker> Sealed for ClosedLoop<T> {}
impl<T: PclkSourceMarker> LoopMode for ClosedLoop<T> {
    fn enable(&self, token: &mut DfllToken) {
        token.set_fine_maximum_step(self.fine_maximum_step);
        token.set_coarse_maximum_step(self.coarse_maximum_step);
        token.set_closed_mode();
    }
}

pub struct Dfll<TMode: LoopMode> {
    token: DfllToken,
    freq: Hertz,
    mode: TMode,
    multiplication_factor: MultiplicationFactor,
    run_standby: bool,
    on_demand_mode: bool,
}

impl<TMode: LoopMode> Dfll<TMode> {
    pub fn freq(&self) -> Hertz {
        Hertz(self.freq.0 * self.multiplication_factor as u32)
    }
    pub fn set_run_standby(&mut self, value: bool) {
        self.run_standby = value;
    }
    pub fn set_on_demand_mode(&mut self, value: bool) {
        self.on_demand_mode = value;
    }
}

impl Dfll<OpenLoop> {
    pub fn in_open_mode(token: DfllToken) -> Dfll<OpenLoop> {
        Self {
            token,
            freq: 48.mhz().into(),
            mode: OpenLoop {
                fine: None,
                coarse: None,
            },
            multiplication_factor: 1_u16,
            run_standby: false,
            on_demand_mode: true,
        }
    }
    pub fn free(self) -> DfllToken {
        self.token
    }
}

impl<T: PclkSourceMarker> Dfll<ClosedLoop<T>> {
    pub fn in_closed_mode(
        token: DfllToken,
        reference_clk: Pclk<Dfll48, T>,
        multiplication_factor: MultiplicationFactor,
        coarse_maximum_step: CoarseMaximumStep,
        fine_maximum_step: FineMaximumStep,
    ) -> Dfll<ClosedLoop<T>> {
        Self {
            token,
            freq: reference_clk.freq(),
            mode: ClosedLoop {
                reference_clk,
                coarse_maximum_step,
                fine_maximum_step,
            },
            multiplication_factor,
            run_standby: false,
            on_demand_mode: true,
        }
    }
    pub fn set_multiplication_factor(&mut self, multiplication_factor: MultiplicationFactor) {
        self.multiplication_factor = multiplication_factor;
    }
    pub fn set_coarse_maximum_step(&mut self, coarse_maximum_step: CoarseMaximumStep) {
        self.mode.coarse_maximum_step = coarse_maximum_step;
    }
    pub fn set_fine_maximum_step(&mut self, fine_maximum_step: FineMaximumStep) {
        self.mode.fine_maximum_step = fine_maximum_step;
    }
    pub fn free(self) -> (DfllToken, Pclk<Dfll48, T>) {
        (self.token, self.mode.reference_clk)
    }
}

impl<TMode: LoopMode> Enabled<Dfll<TMode>, U0> {
    /// TODO
    #[inline]
    pub fn disable(mut self) -> Dfll<TMode> {
        // TODO: Make sure Dfll is disabled correctly
        self.0.token.disable();
        self.0
    }
}

impl Enabled<Dfll<OpenLoop>, U1> {
    /// TODO
    pub fn to_closed_mode<T: PclkSourceMarker>(
        self,
        gclk0: Enabled<Gclk0<marker::Dfll>, U1>,
        reference_clk: Pclk<Dfll48, T>,
        multiplication_factor: MultiplicationFactor,
        coarse_maximum_step: CoarseMaximumStep,
        fine_maximum_step: FineMaximumStep,
    ) -> (
        Enabled<Dfll<ClosedLoop<T>>, U1>,
        Enabled<Gclk0<marker::Dfll>, U1>,
    ) {
        let token = self.0.free();
        let dfll = Dfll::in_closed_mode(
            token,
            reference_clk,
            multiplication_factor,
            coarse_maximum_step,
            fine_maximum_step,
        );
        (dfll.enable().inc(), gclk0)
    }
}

impl<T: PclkSourceMarker> Enabled<Dfll<ClosedLoop<T>>, U1> {
    /// TODO
    pub fn to_open_mode(
        self,
        gclk0: Enabled<Gclk0<marker::Dfll>, U1>,
    ) -> (
        Enabled<Dfll<OpenLoop>, U1>,
        Enabled<Gclk0<marker::Dfll>, U1>,
        Pclk<Dfll48, T>,
    ) {
        let (token, pclk) = self.0.free();
        let dfll = Dfll::in_open_mode(token);
        (dfll.enable().inc(), gclk0, pclk)
    }
}

//==============================================================================
// GclkSource
//==============================================================================

impl<G: GenNum, T: LoopMode, N: Counter> GclkSource<G> for Enabled<Dfll<T>, N> {
    type Type = marker::Dfll;
}

impl<T: LoopMode, N: Counter> Source for Enabled<Dfll<T>, N> {
    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}

pub mod marker {
    use super::*;

    pub enum Dfll {}

    impl Sealed for Dfll {}

    impl GclkSourceMarker for Dfll {
        const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::DFLL;
    }

    impl NotGclkInput for Dfll {}

    impl SourceMarker for Dfll {}
}
