//! TODO

use core::marker::PhantomData;

use num_traits::AsPrimitive;
use seq_macro::seq;

use crate::pac;
use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::gclk::pchctrl::GEN_A;
use crate::pac::NVMCTRL;

use crate::time::{Hertz, U32Ext};
use crate::typelevel::{Count, CountOps, Is, NoneT, One, LockCount, Sealed, Zero};

use super::sources::{AnySource, Dfll, OptionalSourceType, SourceForGclk, SourceType};

//==============================================================================
// Registers
//==============================================================================

/// TODO
struct Registers<G: GenNum> {
    gen: PhantomData<G>,
}

impl<G: GenNum> Registers<G> {
    /// TODO
    #[inline]
    unsafe fn new() -> Self {
        Registers { gen: PhantomData }
    }

    #[inline]
    fn mask(&self) -> u16 {
        1 << G::NUM
    }

    #[inline]
    fn gclk(&self) -> *const pac::gclk::RegisterBlock {
        pac::GCLK::ptr()
    }

    #[inline]
    fn gclk_mut(&mut self) -> *mut pac::gclk::RegisterBlock {
        self.gclk() as *mut _
    }

    /// TODO
    #[inline]
    fn genctrl(&mut self) -> &mut pac::gclk::GENCTRL {
        let gclk = self.gclk_mut();
        // TODO
        unsafe { &mut (*gclk).genctrl[G::NUM] }
    }

    /// TODO
    #[inline]
    fn wait_syncbusy(&self) {
        let gclk = self.gclk();
        // TODO
        unsafe { while (*gclk).syncbusy.read().genctrl().bits() & self.mask() != 0 {} }
    }

    /// TODO
    #[inline]
    fn set_source(&mut self, variant: SRC_A) {
        self.genctrl().modify(|_, w| w.src().variant(variant));
    }

    /// TODO
    #[inline]
    fn set_div(&mut self, div: Div<G>) {
        match div {
            Div::Div(div) => {
                self.genctrl().modify(|_, w| unsafe {
                    w.divsel().div1();
                    w.div().bits(div.as_())
                });
            }
            Div::Max => {
                self.genctrl().modify(|_, w| unsafe {
                    w.divsel().div2();
                    w.div().bits(0)
                });
            }
        }
    }

    /// TODO
    #[inline]
    fn improve_duty_cycle(&mut self, flag: bool) {
        self.genctrl().modify(|_, w| w.idc().bit(flag));
    }

    /// TODO
    #[inline]
    fn enable_gclk_out(&mut self, pol: bool) {
        self.genctrl().modify(|_, w| {
            w.oe().set_bit();
            w.oov().bit(pol)
        });
    }

    /// TODO
    #[inline]
    fn disable_gclk_out(&mut self) {
        self.genctrl().modify(|_, w| w.oe().clear_bit());
    }

    /// TODO
    #[inline]
    fn enable(&mut self) {
        self.genctrl().modify(|_, w| w.genen().set_bit());
        self.wait_syncbusy();
    }

    /// TODO
    #[inline]
    fn disable(&mut self) {
        self.genctrl().modify(|_, w| w.genen().clear_bit());
    }
}

//==============================================================================
// GenNum
//==============================================================================

/// TODO
pub trait GenNum: Sealed {
    const NUM: usize;
    const GCLK: GEN_A;
    type Div: Copy + AsPrimitive<u16> + AsPrimitive<u32>;
    const DIV_MAX: u32;
}

/// TODO
pub trait NotGen0: GenNum {}

/// TODO
pub enum Gen0 {}
impl Sealed for Gen0 {}
impl GenNum for Gen0 {
    const NUM: usize = 0;
    const GCLK: GEN_A = GEN_A::GCLK0;
    type Div = u8;
    const DIV_MAX: u32 = 512;
}

/// TODO
pub enum Gen1 {}
impl Sealed for Gen1 {}
impl NotGen0 for Gen1 {}
impl GenNum for Gen1 {
    const NUM: usize = 1;
    const GCLK: GEN_A = GEN_A::GCLK1;
    type Div = u16;
    const DIV_MAX: u32 = 131072;
}

macro_rules! gen_num {
    ( $GenNum:ident, $NUM:literal, $VARIANT:ident ) => {
        /// TODO
        pub enum $GenNum {}
        impl Sealed for $GenNum {}
        impl NotGen0 for $GenNum {}
        impl GenNum for $GenNum {
            const NUM: usize = $NUM;
            const GCLK: GEN_A = GEN_A::$VARIANT;
            type Div = u8;
            const DIV_MAX: u32 = 512;
        }
    };
}

//==============================================================================
// Div
//==============================================================================

/// TODO
/// Represents a generator divider. The division factor is a u8 or u16 value,
/// depending on the generator. Generator 1 accepts a u16, while all others
/// accept a u8. The upper bits of the `Div` variant are ignored for generators
/// other than Generator 1. The `DIVSEL` field can be used to boost the division
/// factor to a single value above the normal range. Use the `Max` variant to
/// set the `DIVSEL` field appropriately. See the datasheet for more details.
pub enum Div<G: GenNum> {
    Div(G::Div),
    Max,
}

impl<G: GenNum> Clone for Div<G> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<G: GenNum> Copy for Div<G> {}

impl<G: GenNum> Div<G> {
    pub fn as_u32(&self) -> u32 {
        match self {
            Div::Div(div) => div.as_(),
            Div::Max => G::DIV_MAX,
        }
    }
}

//==============================================================================
// GclkConfig
//==============================================================================

/// TODO
pub struct GclkConfig<G, S = NoneT>
where
    G: GenNum,
    S: OptionalSourceType,
{
    regs: Registers<G>,
    src: PhantomData<S>,
    freq: Hertz,
    div: u32,
}

impl<G, S> Sealed for GclkConfig<G, S>
where
    G: GenNum,
    S: OptionalSourceType,
{
}

impl<G, S> GclkConfig<G, S>
where
    G: GenNum,
    S: OptionalSourceType,
{
    /// TODO
    #[inline]
    unsafe fn new<F: Into<Hertz>>(freq: F, div: u32) -> Self {
        GclkConfig {
            regs: Registers::new(),
            src: PhantomData,
            freq: freq.into(),
            div,
        }
    }

    /// TODO
    #[inline]
    fn change_source<T>(self) -> GclkConfig<G, T>
    where
        T: OptionalSourceType,
    {
        GclkConfig {
            regs: self.regs,
            src: PhantomData,
            freq: self.freq,
            div: self.div,
        }
    }
}

impl<G: GenNum> GclkConfig<G> {
    /// TODO
    #[inline]
    unsafe fn default() -> Self {
        GclkConfig::new(0.hz(), 1)
    }

    /// TODO
    #[inline]
    pub fn set_source<New>(mut self, source: New) -> (GclkConfig<G, New::Source>, New::Lock)
    where
        New: AnySource,
        New::Source: SourceForGclk<G>,
    {
        self.regs.set_source(New::Source::GCLK_SRC);
        let mut config = self.change_source();
        config.freq = source.freq();
        // TODO
        (config, unsafe { source.lock() })
    }
}

impl<G, S> GclkConfig<G, S>
where
    G: GenNum,
    S: SourceForGclk<G>,
{
    /// TODO
    #[inline]
    pub fn unset_source<Old>(self, source: Old) -> (GclkConfig<G>, Old::Unlock)
    where
        Old: AnySource<Source = S>,
    {
        // Leave the register unchanged
        let config = self.change_source();
        // TODO
        (config, unsafe { source.unlock() })
    }

    /// TODO
    #[inline]
    pub fn swap_source<Old, New>(
        self,
        old: Old,
        new: New,
    ) -> (GclkConfig<G, New::Source>, Old::Unlock, New::Lock)
    where
        Old: AnySource<Source = S>,
        New: AnySource,
        New::Source: SourceForGclk<G>,
    {
        let (config, old) = self.unset_source(old);
        let (config, new) = config.set_source(new);
        (config, old, new)
    }

    /// TODO
    #[inline]
    pub fn div(mut self, div: Div<G>) -> Self {
        self.regs.set_div(div);
        self.div = div.as_u32();
        self
    }

    /// TODO
    #[inline]
    pub fn improve_duty_cycle(mut self, flag: bool) -> Self {
        self.regs.improve_duty_cycle(flag);
        self
    }

    /// TODO
    #[inline]
    pub fn freq(&self) -> Hertz {
        Hertz(self.freq.0 / self.div)
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> Gclk<G, S> {
        self.regs.enable();
        Gclk::create(self, Zero::new())
    }
}

//==============================================================================
// Gclk
//==============================================================================

/// TODO
pub struct Gclk<G, S = NoneT, N = Zero>
where
    G: GenNum,
    S: OptionalSourceType,
    N: Count,
{
    config: GclkConfig<G, S>,
    count: N,
}

impl<G, S, N> Gclk<G, S, N>
where
    G: GenNum,
    S: OptionalSourceType,
    N: Count,
{
    fn create(config: GclkConfig<G, S>, count: N) -> Self {
        Gclk { config, count }
    }

    /// TODO
    #[inline]
    pub unsafe fn disable_unchecked(mut self) -> GclkConfig<G, S> {
        self.config.regs.disable();
        self.config
    }
}

impl<G, S> Gclk<G, S>
where
    G: NotGen0,
    S: SourceForGclk<G>,
{
    /// TODO
    #[inline]
    pub fn disable(mut self) -> GclkConfig<G, S> {
        self.config.regs.disable();
        self.config
    }
}

impl<G, N> Gclk<G, NoneT, N>
where
    G: GenNum,
    N: Count,
{
    /// TODO
    #[inline]
    pub unsafe fn set_source<New>(self, source: New) -> (Gclk<G, New::Source, N>, New::Lock)
    where
        New: AnySource,
        New::Source: SourceForGclk<G>,
    {
        let (config, source) = self.config.set_source(source);
        (Gclk::create(config, self.count), source)
    }
}

impl<G, S, N> Gclk<G, S, N>
where
    G: GenNum,
    S: SourceForGclk<G>,
    N: Count,
{
    /// TODO
    #[inline]
    pub unsafe fn unset_source<Old>(self, source: Old) -> (Gclk<G, NoneT, N>, Old::Unlock)
    where
        Old: AnySource<Source = S>,
    {
        let (config, source) = self.config.unset_source(source);
        (Gclk::create(config, self.count), source)
    }

    /// TODO
    #[inline]
    pub unsafe fn swap_source<Old, New>(
        self,
        old: Old,
        new: New,
    ) -> (Gclk<G, New::Source, N>, Old::Unlock, New::Lock)
    where
        Old: AnySource<Source = S>,
        New: AnySource,
        New::Source: SourceForGclk<G>,
    {
        let (config, old, new) = self.config.swap_source(old, new);
        (Gclk::create(config, self.count), old, new)
    }

    /// TODO
    #[inline]
    pub unsafe fn div(&mut self, div: Div<G>) {
        self.config.regs.set_div(div);
        self.config.div = div.as_u32();
    }

    /// TODO
    #[inline]
    pub unsafe fn improve_duty_cycle(&mut self, flag: bool) {
        self.config.regs.improve_duty_cycle(flag);
    }

    /// TODO
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.config.freq()
    }

    /// TODO
    #[inline]
    pub(super) fn enable_gclk_out(&mut self, pol: bool) {
        self.config.regs.enable_gclk_out(pol);
    }

    /// TODO
    #[inline]
    pub(super) fn disable_gclk_out(&mut self) {
        self.config.regs.disable_gclk_out();
    }
}

//==============================================================================
// Gclk aliases
//==============================================================================

seq!(G in 0..=11 {
    /// TODO
    pub type Gclk#G<S, N> = Gclk<Gen#G, S, N>;
});

//==============================================================================
// Gclk1 SourceType
//==============================================================================

impl<S, N> SourceType for Gclk<Gen1, S, N>
where
    S: SourceForGclk<Gen1>,
    N: Count,
{
    const GCLK_SRC: SRC_A = SRC_A::GCLKGEN1;

    #[inline]
    fn freq(&self) -> Hertz {
        self.freq()
    }
}

macro_rules! impl_gclk1_source_for {
    ($GenNum:ident) => {
        impl<S, N> SourceForGclk<$GenNum> for Gclk1<S, N>
        where
            S: SourceForGclk<Gen1>,
            N: Count,
        {
        }
    };
}

impl_gclk1_source_for!(Gen0);

seq!(N in 2..=11 {
    impl_gclk1_source_for!(Gen#N);
});

//==============================================================================
// AnyGclk
//==============================================================================

pub trait AnyGclk
where
    Self: Sealed,
    Self: Is<Type = SpecificGclk<Self>>,
    Self: LockCount,
{
    /// TODO
    type GenNum: GenNum;

    /// TODO
    type Source: SourceForGclk<Self::GenNum>;

    /// TODO
    type Count: Count;

    /// TODO
    fn freq(&self) -> Hertz;
}

/// TODO
pub type SpecificGclk<G> =
    Gclk<<G as AnyGclk>::GenNum, <G as AnyGclk>::Source, <G as AnyGclk>::Count>;

impl<G, S, N> Sealed for Gclk<G, S, N>
where
    G: GenNum,
    S: OptionalSourceType,
    N: Count,
{
}

impl<G: AnyGclk> AsRef<G> for SpecificGclk<G> {
    fn as_ref(&self) -> &G {
        // Always a NOP, since G == SpecificGclk<G>
        unsafe { core::mem::transmute(self) }
    }
}

impl<G: AnyGclk> AsMut<G> for SpecificGclk<G> {
    fn as_mut(&mut self) -> &mut G {
        // Always a NOP, since G == SpecificGclk<G>
        unsafe { core::mem::transmute(self) }
    }
}

impl<G, S, N> LockCount for Gclk<G, S, N>
where
    G: GenNum,
    S: SourceForGclk<G>,
    N: Count + CountOps,
{
    type Lock = Gclk<G, S, N::Add>;
    type Unlock = Gclk<G, S, N::Sub>;

    #[inline]
    unsafe fn lock(self) -> Self::Lock {
        Gclk::create(self.config, self.count.add())
    }

    #[inline]
    unsafe fn unlock(self) -> Self::Unlock {
        Gclk::create(self.config, self.count.sub())
    }
}

impl<G, S, N> AnyGclk for Gclk<G, S, N>
where
    G: GenNum,
    S: SourceForGclk<G>,
    N: Count + CountOps,
{
    type GenNum = G;
    type Source = S;
    type Count = N;

    #[inline]
    fn freq(&self) -> Hertz {
        self.freq()
    }
}

//==============================================================================
// Gclks
//==============================================================================

seq!(N in 2..=11 {

    #( gen_num!( Gen#N, N, GCLK#N ); )*

    /// TODO
    pub struct Gclks {
        pub gclk0: Gclk<Gen0, Dfll, One>,
        pub gclk1: GclkConfig<Gen1>,
        #( pub gclk#N: GclkConfig<Gen#N>, )*
    }

    impl Gclks {
        pub(super) fn new(nvmctrl: &mut NVMCTRL) -> Self {
            // Use auto wait states
            nvmctrl.ctrla.modify(|_, w| w.autows().set_bit());
            // TODO
            unsafe {
                Gclks {
                    gclk0: Gclk::create(
                        GclkConfig::new(48.mhz(), 1),
                        One::new()
                    ),
                    gclk1: GclkConfig::default(),
                    #( gclk#N: GclkConfig::default(), )*
                }
            }
        }
    }
});
