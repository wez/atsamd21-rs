//! # GCLK - Generic Clock Controller
//! TODO
//!
//! Functionality:
//!
//! * Provides 12 Generic Clock Generators fed by clock sources implementing the
//!   [super::Source]
//! trait
//! * Each Generic Clock Generator provides clock division
//! * Generic Clock Generator output may be consumed by one or many Peripheral
//!   Channels [super::pclk]
//! * The Peripheral Channels outputs the clock to the peripheral modules

use core::marker::PhantomData;

use seq_macro::seq;
use typenum::{U0, U1};

use crate::pac;
use crate::pac::NVMCTRL;

pub use crate::pac::gclk::genctrl::{DIVSEL_A, SRC_A as GclkSourceEnum};
pub use crate::pac::gclk::{RegisterBlock, GENCTRL};

use super::gclkio::NotGclkInput;
use crate::clock::types::{Counter, Decrement, Enabled, Increment, PrivateIncrement};
use crate::clock::v2::{Source, SourceMarker};
use crate::time::Hertz;
use crate::typelevel::Sealed;

//==============================================================================
// GclkToken
//==============================================================================

/// A [`GclkToken`] equals a hardware register
/// Provide a safe register interface for [`Gclk`]s
///
/// This `struct` takes ownership of a [`GenNum`] and provides an API to
/// access the corresponding registers
pub struct GclkToken<G: GenNum> {
    gen: PhantomData<G>,
}

impl<G: GenNum> GclkToken<G> {
    /// Create a new instance of [`GclkToken`]
    ///
    /// # Safety
    ///
    /// Users must never create two simulatenous instances of this `struct` with
    /// the same [`GenNum`]
    #[inline]
    pub(super) unsafe fn new() -> Self {
        GclkToken { gen: PhantomData }
    }

    /// Used to mask out the correct bit based on [`GenNum`]
    #[inline]
    fn mask(&self) -> u16 {
        1 << G::NUM
    }

    /// Provides the base pointer to the [`Gclk`] registers
    ///
    /// # Safety
    ///
    /// Only one [GclkToken] accessible at any given time
    #[inline]
    fn gclk(&self) -> &RegisterBlock {
        unsafe { &*pac::GCLK::ptr() }
    }

    /// Provides a pointer to the individual Generator Control [`GENCTRL`]
    /// registers
    ///
    /// Each GCLK 0 to 11 has its own Generator Control [`GENCTRL`] register
    /// controlling the settings of that specific generator
    #[inline]
    fn genctrl(&self) -> &GENCTRL {
        &self.gclk().genctrl[G::NUM]
    }

    /// Block until synchronization has completed
    ///
    /// Used for any registers annotated with
    ///
    /// * "Write-Synchronized"
    /// * "Read-Synchronized"
    ///
    /// in the Property field
    #[inline]
    fn wait_syncbusy(&self) {
        while self.gclk().syncbusy.read().genctrl().bits() & self.mask() != 0 {}
    }

    /// Set the clock source for the [`Gclk`] generator
    #[inline]
    fn set_source(&mut self, variant: GclkSourceEnum) {
        self.genctrl().modify(|_, w| w.src().variant(variant));
        self.wait_syncbusy();
    }

    /// When dividing an input clock with a odd division factor the duty-cycle
    /// is not 50-50, enabling this ensures 50-50 duty-cycle on the
    /// resulting generator clock
    #[inline]
    fn improve_duty_cycle(&mut self, flag: bool) {
        self.genctrl().modify(|_, w| w.idc().bit(flag));
    }

    /// Enable output of the generator clock over [`GCLK_IO`][GclkIo] pins
    ///
    /// `polarity` sets the "Output Off Value" (OOV) which sets the state
    /// of the pin when the output is disabled.
    ///
    /// Example: `polarity` = true sets the pin high when the output is
    /// disabled with [`disable_gclk_out`]
    #[inline]
    fn enable_gclk_out(&mut self, polarity: bool) {
        self.genctrl().modify(|_, w| {
            w.oe().set_bit();
            w.oov().bit(polarity)
        });
        self.wait_syncbusy();
    }

    /// Set the divider for the [`Gclk`]
    ///
    /// [`Gclk0`] and [`Gclk2`] to [`Gclk11`] has 8 division factor bits
    /// [`Gclk1`] has 16 division factor bits
    #[inline]
    fn set_div(&mut self, div: G::DividerType) {
        // Get the divsel variant and the divider value
        let (variant, div) = div.get_inner();
        self.genctrl().modify(|_, w| unsafe {
            w.divsel().variant(variant);
            w.div().bits(div)
        });
        self.wait_syncbusy();
    }

    /// Deactivate outputting generator clock over `GCLK_IO` pins
    ///
    /// Pin state depends on the `polarity` value set when the output was
    /// enabled with ['enable_gclk_out`]
    #[inline]
    fn disable_gclk_out(&mut self) {
        self.genctrl().modify(|_, w| w.oe().clear_bit());
        self.wait_syncbusy();
    }

    /// Enable the clock generator
    #[inline]
    fn enable(&mut self) {
        self.genctrl().modify(|_, w| w.genen().set_bit());
        self.wait_syncbusy();
    }

    /// Disable the clock generator
    #[inline]
    fn disable(&mut self) {
        self.genctrl().modify(|_, w| w.genen().clear_bit());
        self.wait_syncbusy();
    }
}

//==============================================================================
// GenNum
//==============================================================================

/// Trait ensuring all `GenNum` has a numeric identifier
pub trait GenNum: Sealed {
    const NUM: usize;
    type DividerType: GclkDivider;
}

/// Trait allowing to pick all `GenX` except [`Gen0`]
pub trait NotGen0: GenNum {}
/// Trait allowing to pick all `GenX` except [`Gen1`]
pub trait NotGen1: GenNum {}

/// [`Gclk0`] is directly coupled to `MCLK` which provides the synchronous
/// clocking and the main clock
///
/// [`NotGen0`] can be used to exclude this [`Gen0`]
pub enum Gen0 {}
impl Sealed for Gen0 {}
impl NotGen1 for Gen0 {}
impl GenNum for Gen0 {
    const NUM: usize = 0;
    type DividerType = GclkDiv;
}

/// [`Gclk1`] has the ability to be fed into other [`Gclk`]s as a source
///
/// [`NotGen1`] can be used to exclude this [`Gen1`]
///
/// Increased division factor, see [`Gclk1Div`]
pub enum Gen1 {}
impl Sealed for Gen1 {}
impl NotGen0 for Gen1 {}
impl GenNum for Gen1 {
    const NUM: usize = 1;
    type DividerType = Gclk1Div;
}

seq!(N in 2..=11 {
    /// Generic Clock Generator
    ///
    /// [`Gclk2`] to [`Gclk11`]
    ///
    /// Standard division factor, see [`GclkDiv`]
    pub enum Gen#N {}
    impl Sealed for Gen#N {}
    impl NotGen0 for Gen#N {}
    impl NotGen1 for Gen#N {}
    impl GenNum for Gen#N {
        const NUM: usize = N;
        type DividerType = GclkDiv;
    }
});

//==============================================================================
// Div
//==============================================================================

/// Trait for [`GclkDiv`] providing the inner components
pub trait GclkDivider: Sealed + Default + Copy {
    fn get_inner(&self) -> (DIVSEL_A, u16);
    fn get_div(&self) -> u32;
}

/// Enum expressing all possible division factors for all [`Gclk`]s except
/// [`Gclk1`]
///
/// Represents a generic clock generator divider
///
/// * `Div(u8)` expresses the divider directly
/// * `Div2Pow8` equals a division factor of `2^8 = 256`
/// * `Div2Pow9` equals a division factor of `2^9 = 512`
///
/// ## Background
///
/// Division is interpreted differently depending on state of `DIVSEL` flag
///
/// In `DIVSEL` mode `DIV1` (register value 0) the division factor is directly
/// interpreted from the `DIV` register.  The division factor is a u8 value
///
/// In `DIVSEL` mode `DIV2` (register value 1) the division factor is calculated
/// as
///
/// ```
/// division_factor = 2.pow(1 + DIV_register)
/// ```
///
/// The maximum division factor is 512 even though the register could be able to
/// express much larger dividers. Hardware ignores any larger value, effectively
/// constrained at max division factor.
///
/// See the datasheet for more details
#[derive(Clone, Copy)]
pub enum GclkDiv {
    /// Express the division factor directly, both 0 and 1 perform no division
    /// of the clock
    Div(u8),
    /// Provides a division factor of `2^8 = 256`
    Div2Pow8,
    /// Provides a division factor of `2^9 = 512`
    Div2Pow9,
}

/// Enum expressing all possible division factors for [`Gclk1`]
///
/// Represents [`Gclk1`] clock generator divider
///
/// Division is interpreted differently depending on state of `DIVSEL` flag
///
/// In `DIVSEL` mode `DIV1` (register value 0) the division factor is directly
/// interpreted from the `DIV` register.  The division factor is a u16 value
///
/// In `DIVSEL` mode `DIV2` (register value 1) the division factor is calculated
/// as
///
/// ```
/// division_factor = 2.pow(1 + DIV_register)
/// ```
///
/// The maximum division factor is 131072 even though the register could be able
/// to express much larger dividers. Hardware ignores any larger value,
/// effectively constrained at max division factor.
///
/// See the datasheet for more details
#[derive(Clone, Copy)]
pub enum Gclk1Div {
    /// Express the division factor directly, both 0 and 1 perform no division
    /// of the clock
    Div(u16),
    /// Provides a division factor of `2^16 = 65536`
    Div2Pow16,
    /// Provides a division factor of `2^17 = 131072`
    Div2Pow17,
}

impl Sealed for GclkDiv {}
impl Sealed for Gclk1Div {}

impl Default for GclkDiv {
    fn default() -> Self {
        Self::Div(0)
    }
}

impl Default for Gclk1Div {
    fn default() -> Self {
        Self::Div(0)
    }
}

impl GclkDivider for Gclk1Div {
    fn get_div(&self) -> u32 {
        match self {
            // Maximum reach of DIV1 mode is 65535
            Gclk1Div::Div(div) => (*div).into(),
            // Set the divider to be 65536
            // 2^(1 + 15) = 65536
            Gclk1Div::Div2Pow16 => 65536,
            // Set the divider to be 131072
            // 2^(1 + 16) = 131072
            Gclk1Div::Div2Pow17 => 131072,
        }
    }

    fn get_inner(&self) -> (DIVSEL_A, u16) {
        match self {
            // Maximum reach of DIV1 mode is 65535
            Gclk1Div::Div(div) => (DIVSEL_A::DIV1, *div),
            // Set the divider to be 65536
            // 2^(1 + 15) = 65536
            Gclk1Div::Div2Pow16 => (DIVSEL_A::DIV2, 15),
            // Set the divider to be 131072
            // 2^(1 + 16) = 131072
            Gclk1Div::Div2Pow17 => (DIVSEL_A::DIV2, 16),
        }
    }
}

impl GclkDivider for GclkDiv {
    fn get_div(&self) -> u32 {
        match self {
            // Maximum reach of DIV1 mode is 255
            GclkDiv::Div(div) => (*div).into(),
            // Set the divider to be 256
            // 2^(1 + 7) = 256
            GclkDiv::Div2Pow8 => 256,
            // Set the divider to be 512
            // 2^(1 + 8) = 512
            GclkDiv::Div2Pow9 => 512,
        }
    }

    fn get_inner(&self) -> (DIVSEL_A, u16) {
        match self {
            // Maximum reach of DIV1 mode is 255
            GclkDiv::Div(div) => (DIVSEL_A::DIV1, (*div).into()),
            // Set the divider to be 256
            // 2^(1 + 7) = 256
            GclkDiv::Div2Pow8 => (DIVSEL_A::DIV2, 7),
            // Set the divider to be 512
            // 2^(1 + 8) = 512
            GclkDiv::Div2Pow9 => (DIVSEL_A::DIV2, 8),
        }
    }
}

//==============================================================================
// GclkSource
//==============================================================================

/// All possible input sources to [`Gclk`]s enumerated
///
/// Provided by PAC in
/// ```
/// genctrl::SRC_A
/// ```
pub trait GclkSourceMarker: SourceMarker {
    const GCLK_SRC: GclkSourceEnum;
}

/// [`GclkSource`] must implement `freq()` since it is a [`Source`]
pub trait GclkSource<G: GenNum>: Source {
    type Type: GclkSourceMarker;
}

impl<G: GenNum> SourceMarker for G {}

impl<G, T, N> Source for Enabled<Gclk<G, T>, N>
where
    G: GenNum,
    T: GclkSourceMarker,
    N: Counter,
{
    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}

//==============================================================================
// Gclk
//==============================================================================

/// [`Gclk`] is a generic clock generator
pub struct Gclk<G, T>
where
    G: GenNum,
    T: GclkSourceMarker,
{
    /// Unique [`GclkToken`]
    token: GclkToken<G>,
    /// Clock source feeding the [`Gclk`]
    src: PhantomData<T>,
    /// Frequency of the clock source [`Gclk.src`] feeding the [`Gclk`]
    src_freq: Hertz,
    /// [`Gclk`] divider, modifying the [`Gclk.src_freq`]uency; affecting the
    /// output frequency
    div: G::DividerType,
    /// Improve duty cycle, used to ensure 50-50 duty cycle with odd dividers
    improve_duty_cycle: bool,
}

impl<G, T> Gclk<G, T>
where
    G: GenNum,
    T: GclkSourceMarker,
{
    /// Create a new [`Gclk`]
    ///
    /// Hardware calls are deferred until `.enable()` is called
    /// except for the divider
    #[inline]
    pub fn new<S>(token: GclkToken<G>, source: S) -> (Gclk<G, T>, S::Inc)
    where
        S: GclkSource<G, Type = T> + Increment,
    {
        let src_freq = source.freq();
        let improve_duty_cycle = false;
        let config = Gclk {
            token,
            src: PhantomData,
            src_freq,
            div: G::DividerType::default(),
            improve_duty_cycle,
        };
        (config, source.inc())
    }

    /// Deconstruct the [`Gclk`] and return the inner [`GclkToken`]
    #[inline]
    pub fn free<S>(self, source: S) -> (GclkToken<G>, S::Dec)
    where
        S: GclkSource<G, Type = T> + Decrement,
    {
        (self.token, source.dec())
    }

    /// Swap [`Gclk`] source
    ///
    /// Provided a [`GclkSource`] the [`Gclk`] is updated,
    /// the old clock source token released and returned
    #[inline]
    pub fn swap<Old, New>(self, old: Old, new: New) -> (Gclk<G, New::Type>, Old::Dec, New::Inc)
    where
        Old: GclkSource<G, Type = T> + Decrement,
        New: GclkSource<G> + Increment,
    {
        let (token, old) = self.free(old);
        let (config, new) = Gclk::new(token, new);
        (config, old, new)
    }

    /// When dividing an input clock with a odd division factor the duty-cycle
    /// is not 50-50, enabling this ensures 50-50 duty-cycle on the
    /// resulting generator clock
    #[inline]
    pub fn improve_duty_cycle(mut self, flag: bool) -> Self {
        self.improve_duty_cycle = flag;
        self
    }

    /// Returns the actual frequency of the [`Gclk`]
    ///
    /// The division factor set via `.div(GclkDiv)` stores the *actual* desired
    /// division factor, while hardware expresses this a bit differently
    /// under the hood
    ///
    /// Hardware has two modes, see [`GclkDiv`] and [`Gclk1Div`], where the
    /// `DIVSEL` register sets if the division factor is directly stored in
    /// register or if calculated as done in [`Gclk1Div`]
    ///
    /// A division factor of 0 is valid from the hardware point of view,
    /// equal to a division factor of 1, meaning "no division/passthrough"
    #[inline]
    pub fn freq(&self) -> Hertz {
        // Handle the allowed case with DIV-field set to zero
        let div = self.div.get_div();
        match div {
            0 => Hertz(self.src_freq.0),
            _ => Hertz(self.src_freq.0 / div),
        }
    }

    /// Set the desired [`Gclk`] clock divider
    ///
    /// See [`GclkDiv`] for possible divider factors
    #[inline]
    pub fn div(mut self, div: G::DividerType) -> Self {
        self.div = div;
        self
    }

    /// Enabling a [`Gclk`] modifies hardware to match the configuration
    /// stored within.
    #[inline]
    pub fn enable(mut self) -> Enabled<Gclk<G, T>, U0> {
        self.token.set_source(T::GCLK_SRC);
        self.token.improve_duty_cycle(self.improve_duty_cycle);
        self.token.set_div(self.div);
        self.token.enable();
        Enabled::new(self)
    }
}

/// The [`Gclk`] generic clock generator
///
/// Provide methods for enable and disable gclk_out to
/// [`GclkOutSource`][super::sources::GclkOutSource]
impl<G, T, N> Enabled<Gclk<G, T>, N>
where
    G: GenNum,
    T: GclkSourceMarker,
    N: Counter,
{
    /// Enable the [`Gclk`] clock output
    ///
    /// `polarity` sets the "Output Off Value" which is
    /// the pin state when disabled
    #[inline]
    pub(super) fn enable_gclk_out(&mut self, polarity: bool) {
        self.0.token.enable_gclk_out(polarity);
    }

    /// Disable the [`Gclk`] clock output
    ///
    /// Pin state assumes the value as specified in
    /// `enable_gclk_out(polarity)`
    #[inline]
    pub(super) fn disable_gclk_out(&mut self) {
        self.0.token.disable_gclk_out();
    }
}

impl<G, T> Enabled<Gclk<G, T>, U0>
where
    G: GenNum,
    T: GclkSourceMarker,
{
    /// Disable the [`Gclk`], possible
    /// when nothing depends on the `Gclk`
    #[inline]
    pub fn disable(mut self) -> Gclk<G, T> {
        self.0.token.disable();
        self.0
    }
}

impl<T: GclkSourceMarker> Enabled<Gclk0<T>, U1> {
    /// Swap source for [`Gclk0`]
    ///
    /// [`Gclk0`] is special since it always has `Mclk`
    /// as a dependency, with only one dependency (`Mclk`)
    /// allow to swap the clock source.
    #[inline]
    pub fn swap<Old, New>(
        self,
        old: Old,
        new: New,
    ) -> (Enabled<Gclk0<New::Type>, U1>, Old::Dec, New::Inc)
    where
        Old: GclkSource<Gen0, Type = T> + Decrement,
        New: GclkSource<Gen0> + Increment,
    {
        let (config, old, new) = self.0.swap(old, new);
        (config.enable().inc(), old, new)
    }

    /// Set the desired [`Gclk`] clock divider
    ///
    /// See [`GclkDiv`] for possible divider factors
    #[inline]
    pub fn div(self, div: GclkDiv) -> Self {
        Enabled::new(self.0.div(div))
    }

    /// When dividing an input clock with a odd division factor the duty-cycle
    /// is not 50-50, enabling this ensures 50-50 duty-cycle on the
    /// resulting generator clock
    #[inline]
    pub fn improve_duty_cycle(self, flag: bool) -> Self {
        Enabled::new(self.0.improve_duty_cycle(flag))
    }
}

//==============================================================================
// Gclk aliases
//==============================================================================

seq!(G in 0..=11 {
    /// `GclkX` aliased to `Gclk<GenX>`
    pub type Gclk#G<S> = Gclk<Gen#G, S>;
});

//==============================================================================
// Gclk1 SourceMarker
//==============================================================================

impl GclkSourceMarker for Gen1 {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::GCLKGEN1;
}

impl NotGclkInput for Gen1 {}

macro_rules! impl_gclk1_source {
    ($GenNum:ident) => {
        impl<T, N> GclkSource<$GenNum> for Enabled<Gclk1<T>, N>
        where
            T: GclkSourceMarker,
            N: Counter,
        {
            type Type = Gen1;
        }
    };
}

impl_gclk1_source!(Gen0);

seq!(N in 2..=11 {
    impl_gclk1_source!(Gen#N);
});

//==============================================================================
// Gclks
//==============================================================================

seq!(N in 2..=11 {
    /// [`Gclk`] tokens ensuring there only exists one instance of each [`Gclk`]
    pub struct Tokens {
        pub gclk1: GclkToken<Gen1>,
        #( pub gclk#N: GclkToken<Gen#N>, )*
    }

    impl Tokens {
        pub(super) fn new(nvmctrl: &mut NVMCTRL) -> Self {
            // Use auto wait states
            nvmctrl.ctrla.modify(|_, w| w.autows().set_bit());
            // Return all tokens when initially created
            unsafe {
                Tokens {
                    gclk1: GclkToken::new(),
                    #( gclk#N: GclkToken::new(), )*
                }
            }
        }
    }
});
