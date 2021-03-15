use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::oscctrl::RegisterBlock;

use crate::time::{Hertz, U32Ext};
use crate::typelevel::*;

use super::super::gclk::GenNum;
use super::{SourceForGclk, SourceType};

struct Registers;

impl Registers {
    /// TODO
    #[inline]
    unsafe fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    #[inline]
    fn oscctrl(&self) -> *const RegisterBlock {
        crate::pac::OSCCTRL::ptr()
    }
}

/// TODO
pub struct Dfll {
    #[allow(dead_code)]
    regs: Registers,
}

impl Dfll {
    /// TODO
    #[inline]
    pub(super) unsafe fn new() -> Self {
        let regs = Registers::new();
        Self { regs }
    }
}

impl Sealed for Dfll {}

impl SourceType for Dfll {
    const GCLK_SRC: SRC_A = SRC_A::DFLL;
    #[inline]
    fn freq(&self) -> Hertz {
        48.mhz().into()
    }
}

impl<G: GenNum> SourceForGclk<G> for Dfll {}
