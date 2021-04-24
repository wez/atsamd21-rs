use crate::pac::oscctrl::RegisterBlock;

use crate::time::{Hertz, U32Ext};
use crate::typelevel::{Lockable, Unlockable, Increment, Decrement, Count, Zero, One, Sealed};

use super::super::gclk::{GenNum, GclkSourceEnum, GclkSource, GclkSourceType};

struct Registers;

impl Registers {
    /// TODO
    #[inline]
    unsafe fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    #[inline]
    fn oscctrl(&self) -> &RegisterBlock {
        unsafe { &*crate::pac::OSCCTRL::ptr() }
    }
}

/// TODO
pub struct Dfll<N = Zero>
where
    N: Count,
{
    #[allow(dead_code)]
    regs: Registers,
    count: N,
}

impl<N: Count> Dfll<N> {
    pub fn freq(&self) -> Hertz {
        48.mhz().into()
    }
}

impl Dfll<One> {
    /// TODO
    #[inline]
    pub(crate) unsafe fn init() -> Self {
        let regs = Registers::new();
        let count = One::new();
        Self { regs, count }
    }
}

impl<N: Count> Sealed for Dfll<N> {}


//==============================================================================
// Lockable
//==============================================================================

impl<N> Lockable for Dfll<N>
where
    N: Increment,
{
    type Locked = Dfll<N::Inc>;
    fn lock(self) -> Self::Locked {
        let Dfll { regs, count } = self;
        let count = count.inc();
        Dfll { regs, count }
    }
}

//==============================================================================
// Unlockable
//==============================================================================

impl<N> Unlockable for Dfll<N>
where
    N: Decrement,
{
    type Unlocked = Dfll<N::Dec>;
    fn unlock(self) -> Self::Unlocked {
        let Dfll { regs, count } = self;
        let count = count.dec();
        Dfll { regs, count }
    }
}

//==============================================================================
// GclkSource
//==============================================================================

pub enum Fll {}

impl Sealed for Fll {}

impl GclkSourceType for Fll {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::DFLL;
}

impl<G: GenNum, N: Count> GclkSource<G> for Dfll<N> {
    type Type = Fll;
    #[inline]
    fn freq(&self) -> Hertz {
        self.freq()
    }
}
