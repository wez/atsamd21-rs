//! TODO

use crate::pac::gclk::genctrl::SRC_A;

use crate::time::Hertz;
use crate::typelevel::{Count, CountOps, Is, NoneT, One, LockCount, Sealed, Zero};

use super::gclk::GenNum;

pub mod gclkio;
pub use gclkio::*;

pub mod dpll;
pub use dpll::*;

pub mod xosc;
pub use xosc::*;

pub mod dfll;
pub use dfll::*;

pub mod xosc32k;
pub use xosc32k::*;

pub mod osculp32k;
pub use osculp32k::*;

/// TODO
pub struct Sources {
    pub gclk_io: gclkio::Tokens,
    pub dfll: Source<Dfll, One>,
    pub dpll0: DpllConfig<Pll0>,
    pub dpll1: DpllConfig<Pll1>,
    pub osc_ulp_32k: OscUlp32k,
}

impl Sources {
    /// TODO
    pub(super) unsafe fn new() -> Sources {
        Sources {
            gclk_io: gclkio::Tokens::new(),
            dfll: Source::new(Dfll::new()).lock(),
            dpll0: DpllConfig::default(),
            dpll1: DpllConfig::default(),
            osc_ulp_32k: OscUlp32k::new(),
        }
    }
}

//==============================================================================
// SourceType
//==============================================================================

/// TODO
/// Marker trait for mapping a type to its corresponding SRC_A constant
pub trait SourceType: Sealed {
    const GCLK_SRC: SRC_A;
    fn freq(&self) -> Hertz;
}

/// TODO
pub trait OptionalSourceType: Sealed {}

impl<S: SourceType> OptionalSourceType for S {}

impl OptionalSourceType for NoneT {}

/// TODO
pub trait SomeSourceType: Sealed + SourceType {}

impl<S: SourceType> SomeSourceType for S {}

//==============================================================================
// SourceForGclk
//==============================================================================

/// TODO
/// Implemented for sources that can be used by Gclks
pub trait SourceForGclk<G: GenNum>: SourceType {}

//==============================================================================
// Source
//==============================================================================

/// TODO
/// Wrapper struct for different clock sources
/// Adds lock counting
pub struct Source<S, N = Zero>
where
    S: SourceType,
    N: Count,
{
    source: S,
    count: N,
}

impl<S, N> Source<S, N>
where
    S: SourceType,
    N: Count,
{
    /// TODO
    #[inline]
    fn create(source: S, count: N) -> Self {
        Source { source, count }
    }
}

impl<S> Source<S>
where
    S: SourceType,
{
    /// TODO
    #[inline]
    pub fn new(source: S) -> Self {
        Self::create(source, Zero::new())
    }

    /// TODO
    #[inline]
    pub fn free(self) -> S {
        self.source
    }
}

//==============================================================================
// AnySource
//==============================================================================

/// TODO
/// Type family for clock sources
pub trait AnySource
where
    Self: Sealed,
    Self: Is<Type = SpecificSource<Self>>,
    Self: LockCount,
{
    /// TODO
    type Source: SourceType;

    /// TODO
    type Count: Count;

    /// TODO
    fn freq(&self) -> Hertz;
}

/// TODO
pub type SpecificSource<S> = Source<<S as AnySource>::Source, <S as AnySource>::Count>;

impl<S, N> Sealed for Source<S, N>
where
    S: SourceType,
    N: Count,
{
}

impl<S: AnySource> AsRef<S> for SpecificSource<S> {
    fn as_ref(&self) -> &S {
        // Always safe because S == SpecificSource<S>
        unsafe { core::mem::transmute(self) }
    }
}

impl<S: AnySource> AsMut<S> for SpecificSource<S> {
    fn as_mut(&mut self) -> &mut S {
        // Always safe because S == SpecificSource<S>
        unsafe { core::mem::transmute(self) }
    }
}

impl<S, N> LockCount for Source<S, N>
where
    S: SourceType,
    N: Count + CountOps,
{
    type Lock = Source<S, N::Add>;
    type Unlock = Source<S, N::Sub>;

    #[inline]
    unsafe fn lock(self) -> Self::Lock {
        Source::create(self.source, self.count.add())
    }

    #[inline]
    unsafe fn unlock(self) -> Self::Unlock {
        Source::create(self.source, self.count.sub())
    }
}

impl<S, N> AnySource for Source<S, N>
where
    S: SourceType,
    N: Count + CountOps,
{
    type Source = S;
    type Count = N;

    #[inline]
    fn freq(&self) -> Hertz {
        self.source.freq()
    }
}
