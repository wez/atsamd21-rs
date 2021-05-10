//! TODO

use core::marker::PhantomData;

use num_traits::AsPrimitive;
use seq_macro::seq;

use crate::pac;
use crate::pac::NVMCTRL;

pub use crate::pac::gclk::{GENCTRL, RegisterBlock};
pub use crate::pac::gclk::genctrl::SRC_A as GclkSourceEnum;

use crate::time::Hertz;
use crate::typelevel::{Count, Increment, Decrement, Lockable, Unlockable, Is, Sealed, Zero, One};

use super::sources::dfll::Fll;

//==============================================================================
// Registers
//==============================================================================

/// TODO
pub type GclkToken<G> = Registers<G>;

/// TODO
pub struct Registers<G: GenNum> {
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
    fn gclk(&self) -> &RegisterBlock {
        unsafe { &*pac::GCLK::ptr() }
    }

    /// TODO
    #[inline]
    fn genctrl(&self) -> &GENCTRL {
        &self.gclk().genctrl[G::NUM]
    }

    /// TODO
    #[inline]
    fn wait_syncbusy(&self) {
        while self.gclk().syncbusy.read().genctrl().bits() & self.mask() != 0 {}
    }

    /// TODO
    #[inline]
    fn set_source(&mut self, variant: GclkSourceEnum) {
        self.genctrl().modify(|_, w| w.src().variant(variant));
        self.wait_syncbusy();
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
        self.wait_syncbusy();
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
        self.wait_syncbusy();
    }

    /// TODO
    #[inline]
    fn disable_gclk_out(&mut self) {
        self.genctrl().modify(|_, w| w.oe().clear_bit());
        self.wait_syncbusy();
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
        self.wait_syncbusy();
    }
}

//==============================================================================
// GenNum
//==============================================================================

/// TODO
pub trait GenNum: Sealed {
    const NUM: usize;
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
    type Div = u8;
    const DIV_MAX: u32 = 512;
}

/// TODO
pub enum Gen1 {}
impl Sealed for Gen1 {}
impl NotGen0 for Gen1 {}
impl GenNum for Gen1 {
    const NUM: usize = 1;
    type Div = u16;
    const DIV_MAX: u32 = 131072;
}

seq!(N in 2..=11 {
    /// TODO
    pub enum Gen#N {}
    impl Sealed for Gen#N {}
    impl NotGen0 for Gen#N {}
    impl GenNum for Gen#N {
        const NUM: usize = N;
        type Div = u8;
        const DIV_MAX: u32 = 512;
    }
});

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
// GclkSource
//==============================================================================

/// TODO
pub trait GclkSourceType: Sealed {
    const GCLK_SRC: GclkSourceEnum;
}

/// TODO
pub trait GclkSource<G: GenNum>: Sealed {
    type Type: GclkSourceType;
    fn freq(&self) -> Hertz;
}

//==============================================================================
// GclkConfig
//==============================================================================

/// TODO
pub struct GclkConfig<G, T>
where
    G: GenNum,
    T: GclkSourceType,
{
    token: GclkToken<G>,
    src: PhantomData<T>,
    freq: Hertz,
    div: u32,
}

impl GclkConfig<Gen0, Fll> {
    unsafe fn init(freq: impl Into<Hertz>) -> Self {
        let token = GclkToken::new();
        GclkConfig {
            token,
            src: PhantomData,
            freq: freq.into(),
            div: 1,
        }
    }
}

impl<G, T> GclkConfig<G, T>
where
    G: GenNum,
    T: GclkSourceType,
{
    /// TODO
    #[inline]
    pub fn new<S>(mut token: GclkToken<G>, source: S) -> (GclkConfig<G, T>, S::Locked)
    where
        S: GclkSource<G, Type = T> + Lockable,
    {
        let freq = source.freq();
        let div = 1;
        token.set_source(T::GCLK_SRC);
        let config = GclkConfig {
            token,
            src: PhantomData,
            freq,
            div,
        };
        (config, source.lock())
    }
}

impl<G, T> GclkConfig<G, T>
where
    G: GenNum,
    T: GclkSourceType,
{
    /// TODO
    #[inline]
    pub fn free<S>(self, source: S) -> (GclkToken<G>, S::Unlocked)
    where
        S: GclkSource<G, Type = T> + Unlockable,
    {
        (self.token, source.unlock())
    }
}

impl<G, T> GclkConfig<G, T>
where
    G: GenNum,
    T: GclkSourceType,
{
    /// TODO
    #[inline]
    pub fn swap<Old, New>(self, old: Old, new: New) -> (GclkConfig<G, New::Type>, Old::Unlocked, New::Locked)
    where
        Old: GclkSource<G, Type = T> + Unlockable,
        New: GclkSource<G> + Lockable,
    {
        let (token, old) = self.free(old);
        let (config, new) = GclkConfig::new(token, new);
        (config, old, new)
    }

    /// TODO
    #[inline]
    pub fn div(mut self, div: Div<G>) -> Self {
        self.token.set_div(div);
        self.div = div.as_u32();
        self
    }

    /// TODO
    #[inline]
    pub fn improve_duty_cycle(mut self, flag: bool) -> Self {
        self.token.improve_duty_cycle(flag);
        self
    }

    /// TODO
    #[inline]
    pub fn freq(&self) -> Hertz {
        Hertz(self.freq.0 / self.div)
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> Gclk<G, T> {
        self.token.enable();
        Gclk::create(self, Zero::new())
    }
}

//==============================================================================
// Gclk
//==============================================================================

/// TODO
pub struct Gclk<G, T, N = Zero>
where
    G: GenNum,
    T: GclkSourceType,
    N: Count,
{
    config: GclkConfig<G, T>,
    count: N,
}

impl Gclk0<Fll, One> {
    pub(super) unsafe fn init(freq: impl Into<Hertz>) -> Self {
        let config = GclkConfig::init(freq);
        let count = One::new();
        Gclk::create(config, count)
    }
}

impl<G, T> Gclk<G, T>
where
    G: NotGen0,
    T: GclkSourceType,
{
    /// TODO
    #[inline]
    pub fn disable(mut self) -> GclkConfig<G, T> {
        self.config.token.disable();
        self.config
    }
}

impl<G, T, N> Gclk<G, T, N>
where
    G: GenNum,
    T: GclkSourceType,
    N: Count,
{
    #[inline]
    fn create(config: GclkConfig<G, T>, count: N) -> Self {
        Gclk { config, count }
    }

    /// TODO
    #[inline]
    pub unsafe fn disable_unchecked(mut self) -> GclkConfig<G, T> {
        self.config.token.disable();
        self.config
    }

    /// TODO
    #[inline]
    pub unsafe fn swap<Old, New>(self, old: Old, new: New) -> (Gclk<G, New::Type, N>, Old::Unlocked, New::Locked)
    where
        Old: GclkSource<G, Type = T> + Unlockable,
        New: GclkSource<G> + Lockable,
    {
        let (config, old, new) = self.config.swap(old, new);
        (Gclk::create(config, self.count), old, new)
    }

    /// TODO
    #[inline]
    pub unsafe fn div(&mut self, div: Div<G>) {
        self.config.token.set_div(div);
        self.config.div = div.as_u32();
    }

    /// TODO
    #[inline]
    pub unsafe fn improve_duty_cycle(&mut self, flag: bool) {
        self.config.token.improve_duty_cycle(flag);
    }

    /// TODO
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.config.freq()
    }

    /// TODO
    #[inline]
    pub(super) fn enable_gclk_out(&mut self, pol: bool) {
        self.config.token.enable_gclk_out(pol);
    }

    /// TODO
    #[inline]
    pub(super) fn disable_gclk_out(&mut self) {
        self.config.token.disable_gclk_out();
    }
}

//==============================================================================
// Lockable
//==============================================================================

impl<G, T, N> Lockable for Gclk<G, T, N>
where
    G: GenNum,
    T: GclkSourceType,
    N: Increment,
{
    type Locked = Gclk<G, T, N::Inc>;
    fn lock(self) -> Self::Locked {
        Gclk::create(self.config, self.count.inc())
    }
}

//==============================================================================
// Unlockable
//==============================================================================

impl<G, T, N> Unlockable for Gclk<G, T, N>
where
    G: GenNum,
    T: GclkSourceType,
    N: Decrement,
{
    type Unlocked = Gclk<G, T, N::Dec>;
    fn unlock(self) -> Self::Unlocked {
        Gclk::create(self.config, self.count.dec())
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

impl GclkSourceType for Gen1 {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::GCLKGEN1;
}


macro_rules! impl_gclk1_source {
    ($GenNum:ident) => {
        impl<T, N> GclkSource<$GenNum> for Gclk1<T, N>
        where
            T: GclkSourceType,
            N: Count,
        {
            type Type = Gen1;

            #[inline]
            fn freq(&self) -> Hertz {
                self.freq()
            }
        }
    };
}

impl_gclk1_source!(Gen0);

seq!(N in 2..=11 {
    impl_gclk1_source!(Gen#N);
});

//==============================================================================
// AnyGclk
//==============================================================================

pub trait AnyGclk
where
    Self: Sealed,
    Self: Is<Type = SpecificGclk<Self>>,
{
    /// TODO
    type GenNum: GenNum;

    /// TODO
    type Source: GclkSourceType;

    /// TODO
    type Count: Count;
}

/// TODO
pub type SpecificGclk<G> =
    Gclk<<G as AnyGclk>::GenNum, <G as AnyGclk>::Source, <G as AnyGclk>::Count>;

impl<G, T, N> Sealed for Gclk<G, T, N>
where
    G: GenNum,
    T: GclkSourceType,
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

impl<G, T, N> AnyGclk for Gclk<G, T, N>
where
    G: GenNum,
    T: GclkSourceType,
    N: Count,
{
    type GenNum = G;
    type Source = T;
    type Count = N;
}

//==============================================================================
// Gclks
//==============================================================================

seq!(N in 2..=11 {
    /// TODO
    pub struct Tokens {
        pub gclk1: GclkToken<Gen1>,
        #( pub gclk#N: GclkToken<Gen#N>, )*
    }

    impl Tokens {
        pub(super) fn new(nvmctrl: &mut NVMCTRL) -> Self {
            // Use auto wait states
            nvmctrl.ctrla.modify(|_, w| w.autows().set_bit());
            // TODO
            unsafe {
                Tokens {
                    gclk1: GclkToken::new(),
                    #( gclk#N: GclkToken::new(), )*
                }
            }
        }
    }
});
