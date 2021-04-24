//! TODO

use core::marker::PhantomData;

use seq_macro::seq;

use crate::gpio::v2::{self as gpio, AlternateM, AnyPin, Pin, PinId};
use crate::time::Hertz;
use crate::typelevel::*;

use super::super::gclk::*;

//==============================================================================
// GclkIo
//==============================================================================

/// TODO
pub trait GclkIo<G: GenNum>: PinId {}

impl GclkIo<Gen4> for gpio::PA10 {}
impl GclkIo<Gen5> for gpio::PA11 {}

impl GclkIo<Gen0> for gpio::PA14 {}
impl GclkIo<Gen1> for gpio::PA15 {}
impl GclkIo<Gen2> for gpio::PA16 {}
impl GclkIo<Gen3> for gpio::PA17 {}

impl GclkIo<Gen1> for gpio::PA27 {}
impl GclkIo<Gen0> for gpio::PA30 {}

impl GclkIo<Gen4> for gpio::PB10 {}
impl GclkIo<Gen5> for gpio::PB11 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<Gen6> for gpio::PB12 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<Gen7> for gpio::PB13 {}

#[cfg(feature = "min-samd51j")]
impl GclkIo<Gen0> for gpio::PB14 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<Gen1> for gpio::PB15 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<Gen2> for gpio::PB16 {}
#[cfg(feature = "min-samd51j")]
impl GclkIo<Gen3> for gpio::PB17 {}
#[cfg(feature = "min-samd51n")]
impl GclkIo<Gen4> for gpio::PB18 {}
#[cfg(feature = "min-samd51n")]
impl GclkIo<Gen5> for gpio::PB19 {}
#[cfg(feature = "min-samd51n")]
impl GclkIo<Gen6> for gpio::PB20 {}
#[cfg(feature = "min-samd51n")]
impl GclkIo<Gen7> for gpio::PB21 {}

impl GclkIo<Gen0> for gpio::PB22 {}
impl GclkIo<Gen1> for gpio::PB23 {}

//==============================================================================
// GclkInToken
//==============================================================================

/// TODO
pub struct GclkInToken<G: GenNum> {
    gen: PhantomData<G>,
}

impl<G> GclkInToken<G>
where
    G: GenNum,
{
    /// TODO
    unsafe fn new() -> GclkInToken<G> {
        GclkInToken { gen: PhantomData }
    }
}

//==============================================================================
// GclkIn
//==============================================================================

pub struct GclkIn<G, I, N = Zero>
where
    G: GenNum,
    I: GclkIo<G>,
    N: Count,
{
    token: GclkInToken<G>,
    pin: Pin<I, AlternateM>,
    freq: Hertz,
    count: N,
}

impl<G, I> GclkIn<G, I>
where
    G: GenNum,
    I: GclkIo<G>,
{
    /// TODO
    pub fn new<F>(token: GclkInToken<G>, pin: impl AnyPin<Id = I>, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        let pin = pin.into().into_alternate();
        let freq = freq.into();
        let count = Zero::new();
        GclkIn { token, pin, freq, count }
    }

    /// TODO
    pub fn disable(self) -> (GclkInToken<G>, Pin<I, AlternateM>) {
        (self.token, self.pin)
    }
}

impl<G, I, N> Sealed for GclkIn<G, I, N>
where
    G: GenNum,
    I: GclkIo<G>,
    N: Count,
{
}

//==============================================================================
// Lockable
//==============================================================================

impl<G, I, N> Lockable for GclkIn<G, I, N>
where
    G: GenNum,
    I: GclkIo<G>,
    N: Increment,
{
    type Locked = GclkIn<G, I, N::Inc>;
    fn lock(self) -> Self::Locked {
        let GclkIn { token, pin, freq, count } = self;
        let count = count.inc();
        GclkIn { token, pin, freq, count }
    }
}

//==============================================================================
// Unlockable
//==============================================================================

impl<G, I, N> Unlockable for GclkIn<G, I, N>
where
    G: GenNum,
    I: GclkIo<G>,
    N: Decrement,
{
    type Unlocked = GclkIn<G, I, N::Dec>;
    fn unlock(self) -> Self::Unlocked {
        let GclkIn { token, pin, freq, count } = self;
        let count = count.dec();
        GclkIn { token, pin, freq, count }
    }
}

//==============================================================================
// GclkSource
//==============================================================================

pub enum GclkInput {}

impl Sealed for GclkInput {}

impl GclkSourceType for GclkInput {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::GCLKIN;
}

impl<G, I, N> GclkSource<G> for GclkIn<G, I, N>
where
    G: GenNum,
    I: GclkIo<G>,
    N: Count,
{
    type Type = GclkInput;

    #[inline]
    fn freq(&self) -> Hertz {
        self.freq
    }
}

//==============================================================================
// GclkOutToken
//==============================================================================

pub struct GclkOutToken<G: GenNum> {
    gen: PhantomData<G>,
}

impl<G: GenNum> GclkOutToken<G> {
    /// TODO
    unsafe fn new() -> GclkOutToken<G> {
        GclkOutToken { gen: PhantomData }
    }
}

//==============================================================================
// GclkOut
//==============================================================================

pub struct GclkOut<G, I>
where
    G: GenNum,
    I: GclkIo<G>,
{
    token: GclkOutToken<G>,
    freq: Hertz,
    pin: Pin<I, AlternateM>,
}

impl<G, I> GclkOut<G, I>
where
    G: GenNum,
    I: GclkIo<G>,
{
    /// TODO
    pub fn new<H>(
        token: GclkOutToken<G>,
        pin: impl AnyPin<Id = I>,
        mut gclk: H,
        pol: bool
    ) -> (GclkOut<G, I>, H::Locked)
    where
        H: AnyGclk<GenNum = G> + Lockable,
    {
        let freq = gclk.as_ref().freq();
        let pin = pin.into().into_alternate();
        gclk.as_mut().enable_gclk_out(pol);
        let gclk_out = GclkOut { token, freq, pin };
        (gclk_out, gclk.lock())
    }

    /// TODO
    pub fn freq(&self) -> Hertz {
        self.freq
    }

    /// TODO
    pub fn disable<H>(self, mut gclk: H) -> (GclkOutToken<G>, Pin<I, AlternateM>, H::Unlocked)
    where
        H: AnyGclk<GenNum = G> + Unlockable,
    {
        gclk.as_mut().disable_gclk_out();
        (self.token, self.pin, gclk.unlock())
    }
}

//==============================================================================
// GclkIns
//==============================================================================

seq!(N in 0..=11 {
    pub struct Tokens {
        #( pub gclk_in#N: GclkInToken<Gen#N>, )*
        #( pub gclk_out#N: GclkOutToken<Gen#N>, )*
    }

    impl Tokens {
        // TODO
        pub(super) unsafe fn new() -> Tokens {
            Tokens {
                #( gclk_in#N: GclkInToken::new(), )*
                #( gclk_out#N: GclkOutToken::new(), )*
            }
        }
    }
});
