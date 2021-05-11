//! TODO

use core::marker::PhantomData;

use seq_macro::seq;
use typenum::U0;

use crate::clock::v2::{Source, SourceMarker};
use crate::gpio::v2::{self as gpio, AlternateM, AnyPin, Pin, PinId};
use crate::time::Hertz;
use crate::typelevel::counted::Counted;
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

pub struct GclkIn<G, I>
where
    G: GenNum,
    I: GclkIo<G>,
{
    token: GclkInToken<G>,
    pin: Pin<I, AlternateM>,
    freq: Hertz,
}

impl<G, I> GclkIn<G, I>
where
    G: GenNum,
    I: GclkIo<G>,
{
    /// TODO
    pub fn enable<F>(token: GclkInToken<G>, pin: impl AnyPin<Id = I>, freq: F) -> Counted<Self, U0>
    where
        F: Into<Hertz>,
    {
        let pin = pin.into().into_alternate();
        let freq = freq.into();
        Counted::new(GclkIn { token, pin, freq })
    }

    /// TODO
    fn disable(self) -> (GclkInToken<G>, Pin<I, AlternateM>) {
        (self.token, self.pin)
    }
}

impl<G, I> Sealed for GclkIn<G, I>
where
    G: GenNum,
    I: GclkIo<G>,
{
}

impl<G, I> Counted<GclkIn<G, I>, U0>
where
    G: GenNum,
    I: GclkIo<G>,
{
    /// TODO
    pub fn disable(self) -> (GclkInToken<G>, Pin<I, AlternateM>) {
        self.0.disable()
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

impl SourceMarker for GclkInput {}

impl<G, I, N> GclkSource<G> for Counted<GclkIn<G, I>, N>
where
    G: GenNum,
    I: GclkIo<G>,
    N: Counter,
{
    type Type = GclkInput;
}

impl<G, I, N> Source for Counted<GclkIn<G, I>, N>
where
    G: GenNum,
    I: GclkIo<G>,
    N: Counter,
{
    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq
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
    pub fn new<H, N>(
        token: GclkOutToken<G>,
        pin: impl AnyPin<Id = I>,
        mut gclk: Counted<Gclk<G, H>, N>,
        pol: bool,
    ) -> (GclkOut<G, I>, Counted<Gclk<G, H>, N::Inc>)
    where
        H: GclkSourceType,
        N: Increment,
    {
        let freq = gclk.freq();
        let pin = pin.into().into_alternate();
        gclk.enable_gclk_out(pol);
        let gclk_out = GclkOut { token, freq, pin };
        (gclk_out, gclk.inc())
    }

    /// TODO
    pub fn freq(&self) -> Hertz {
        self.freq
    }

    /// TODO
    pub fn disable<H, N>(
        self,
        mut gclk: Counted<Gclk<G, H>, N>,
    ) -> (
        GclkOutToken<G>,
        Pin<I, AlternateM>,
        Counted<Gclk<G, H>, N::Dec>,
    )
    where
        H: GclkSourceType,
        N: Decrement,
    {
        gclk.disable_gclk_out();
        (self.token, self.pin, gclk.dec())
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
