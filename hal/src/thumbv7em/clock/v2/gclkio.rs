//! TODO

use core::marker::PhantomData;

use seq_macro::seq;
use typenum::U0;

use crate::clock::types::{Counter, Decrement, Enabled, Increment};
use crate::clock::v2::{Source, SourceMarker};
use crate::gpio::v2::{self as gpio, AlternateM, AnyPin, Pin, PinId};
use crate::time::Hertz;
use crate::typelevel::*;

use super::gclk::*;

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
    pub fn enable<F>(token: GclkInToken<G>, pin: impl AnyPin<Id = I>, freq: F) -> Enabled<Self, U0>
    where
        F: Into<Hertz>,
    {
        let pin = pin.into().into_alternate();
        let freq = freq.into();
        Enabled::new(GclkIn { token, pin, freq })
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

impl<G, I> Enabled<GclkIn<G, I>, U0>
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

pub trait NotGclkInput: GclkSourceMarker {}

impl Sealed for GclkInput {}

impl GclkSourceMarker for GclkInput {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::GCLKIN;
}

impl SourceMarker for GclkInput {}

impl<G, I, N> GclkSource<G> for Enabled<GclkIn<G, I>, N>
where
    G: GenNum,
    I: GclkIo<G>,
    N: Counter,
{
    type Type = GclkInput;
}

impl<G, I, N> Source for Enabled<GclkIn<G, I>, N>
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
// GclkOutSource
//==============================================================================

pub trait GclkOutSourceMarker: GenNum + SourceMarker {}

impl<G: GenNum> GclkOutSourceMarker for G {}

mod private {
    use super::*;
    pub trait GclkOutSource: Source {
        fn enable_gclk_out(&mut self, polarity: bool);
        fn disable_gclk_out(&mut self);
    }
}

pub(crate) use private::GclkOutSource as PrivateGclkOutSource;

pub trait GclkOutSource: PrivateGclkOutSource {
    type Type: GclkOutSourceMarker;
}

// TODO: Look up source code if there are some inconsistencies
// Like here: G, H, N; instead of G, T, N
impl<G, H, N> GclkOutSource for Enabled<Gclk<G, H>, N>
where
    G: GclkOutSourceMarker,
    H: GclkSourceMarker + NotGclkInput,
    N: Counter,
{
    type Type = G;
}

impl<G, H, N> PrivateGclkOutSource for Enabled<Gclk<G, H>, N>
where
    G: GclkOutSourceMarker,
    H: GclkSourceMarker + NotGclkInput,
    N: Counter,
{
    fn enable_gclk_out(&mut self, polarity: bool) {
        // TODO: Are these for sure not recursive?
        self.enable_gclk_out(polarity);
    }
    fn disable_gclk_out(&mut self) {
        self.disable_gclk_out();
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
    pub fn enable<S>(
        token: GclkOutToken<G>,
        pin: impl AnyPin<Id = I>,
        mut gclk: S,
        polarity: bool,
    ) -> (GclkOut<G, I>, S::Inc)
    where
        S: GclkOutSource<Type = G> + Increment,
    {
        let freq = gclk.freq();
        let pin = pin.into().into_alternate();
        gclk.enable_gclk_out(polarity);
        let gclk_out = GclkOut { token, freq, pin };
        (gclk_out, gclk.inc())
    }

    /// TODO
    pub fn freq(&self) -> Hertz {
        self.freq
    }

    /// TODO
    pub fn disable<S>(self, mut gclk: S) -> (GclkOutToken<G>, Pin<I, AlternateM>, S::Dec)
    where
        S: GclkOutSource<Type = G> + Decrement,
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
