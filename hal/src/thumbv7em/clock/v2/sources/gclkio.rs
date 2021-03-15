//! TODO

use core::marker::PhantomData;

use seq_macro::seq;

use crate::pac::gclk::genctrl::SRC_A;

use crate::gpio::v2::{self as gpio, AlternateM, AnyPin, Pin, PinId};
use crate::time::Hertz;
use crate::typelevel::*;

use super::super::gclk::*;
use super::{SourceForGclk, SourceType};

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

    /// TODO
    pub fn enable<P, F>(self, pin: P, freq: F) -> GclkIn<G, P::Id>
    where
        P: AnyPin,
        P::Id: GclkIo<G>,
        F: Into<Hertz>,
    {
        GclkIn::new(self, pin.into().into_alternate(), freq.into())
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
    fn new(token: GclkInToken<G>, pin: Pin<I, AlternateM>, freq: Hertz) -> Self {
        GclkIn { token, pin, freq }
    }

    /// TODO
    pub fn disable(self) -> (GclkInToken<G>, Pin<I, AlternateM>) {
        (self.token, self.pin)
    }
}

//==============================================================================
// GclkIn SourceType
//==============================================================================

impl<G, I> Sealed for GclkIn<G, I>
where
    G: GenNum,
    I: GclkIo<G>,
{
}

impl<G, I> SourceType for GclkIn<G, I>
where
    G: GenNum,
    I: GclkIo<G>,
{
    const GCLK_SRC: SRC_A = SRC_A::GCLKIN;

    #[inline]
    fn freq(&self) -> Hertz {
        self.freq
    }
}

impl<G, I> SourceForGclk<G> for GclkIn<G, I>
where
    G: GenNum,
    I: GclkIo<G>,
{
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

    /// TODO
    pub fn enable<H, P>(self, mut gclk: H, pin: P, pol: bool) -> (GclkOut<G, P::Id>, H::Lock)
    where
        H: AnyGclk<GenNum = G>,
        P: AnyPin,
        P::Id: GclkIo<G>,
    {
        gclk.as_mut().enable_gclk_out(pol);
        (
            GclkOut::new(self, gclk.as_ref().freq(), pin.into().into_alternate()),
            // TODO
            unsafe { gclk.lock() },
        )
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
    fn new(token: GclkOutToken<G>, freq: Hertz, pin: Pin<I, AlternateM>) -> GclkOut<G, I> {
        GclkOut { token, freq, pin }
    }

    /// TODO
    pub fn freq(&self) -> Hertz {
        self.freq
    }

    /// TODO
    pub fn disable<H>(self, mut gclk: H) -> (GclkOutToken<G>, H::Unlock, Pin<I, AlternateM>)
    where
        H: AnyGclk<GenNum = G>,
    {
        gclk.as_mut().disable_gclk_out();
        // TODO
        (self.token, unsafe { gclk.unlock() }, self.pin)
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
