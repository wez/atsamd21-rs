//! Character size definitions

use crate::typelevel::Sealed;
use num_traits::{AsPrimitive, PrimInt};

/// Size of the SERCOM's `DATA` register
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub type DataSize = u16;

/// Size of the SERCOM's `DATA` register
#[cfg(any(feature = "min-samd51g"))]
pub type DataSize = u32;

/// Type-level `enum` representing the UART character size
///
/// The UART character size affects the word size for the embedded HAL traits.
/// Eight or less bit transactions use a `u8` word, while nine-bit transactions
/// use a `u16` word.
pub trait CharSize: Sealed {
    /// Word size for the character size
    type Word: 'static + PrimInt + AsPrimitive<DataSize>;

    /// Bits to write into the `LENGTH` register
    const BITS: u8;
}

/// Type-level `enum` indicating a [`CharSize`] that is not dynamic
pub trait FixedCharSize: CharSize {}

/// Type alias to recover the `Word` type from an implementation of [`CharSize`]
pub type Word<C> = <C as CharSize>::Word;

/// [`CharSize`] variant for 5-bit transactions
pub enum FiveBit {}

/// [`CharSize`] variant for 6-bit transactions
pub enum SixBit {}

/// [`CharSize`] variant for 7-bit transactions
pub enum SevenBit {}

/// [`CharSize`] variant for 8-bit transactions
pub enum EightBit {}

/// [`CharSize`] variant for 9-bit transactions
pub enum NineBit {}

/// Dynamic [`CharSize`] that can be changed on the fly
pub enum DynCharSize {}

/// `enum` version of [`CharSize`]
#[repr(u8)]
pub enum CharSizeEnum {
    FiveBit = FiveBit::BITS,
    SixBit = SixBit::BITS,
    SevenBit = SevenBit::BITS,
    EightBit = EightBit::BITS,
    NineBit = NineBit::BITS,
}

impl From<u8> for CharSizeEnum {
    fn from(item: u8) -> CharSizeEnum {
        match item {
            FiveBit::BITS => CharSizeEnum::FiveBit,
            SixBit::BITS => CharSizeEnum::SixBit,
            SevenBit::BITS => CharSizeEnum::SevenBit,
            EightBit::BITS => CharSizeEnum::EightBit,
            NineBit::BITS => CharSizeEnum::NineBit,
            _ => unreachable!(),
        }
    }
}

impl Sealed for FiveBit {}
impl CharSize for FiveBit {
    type Word = u8;
    const BITS: u8 = 0x5;
}
impl FixedCharSize for FiveBit {}

impl Sealed for SixBit {}
impl CharSize for SixBit {
    type Word = u8;
    const BITS: u8 = 0x6;
}
impl FixedCharSize for SixBit {}

impl Sealed for SevenBit {}
impl CharSize for SevenBit {
    type Word = u8;
    const BITS: u8 = 0x7;
}
impl FixedCharSize for SevenBit {}

impl Sealed for EightBit {}
impl CharSize for EightBit {
    type Word = u8;
    const BITS: u8 = 0x0;
}
impl FixedCharSize for EightBit {}

impl Sealed for NineBit {}
impl CharSize for NineBit {
    type Word = u16;
    const BITS: u8 = 0x1;
}
impl FixedCharSize for NineBit {}

impl Sealed for DynCharSize {}
impl CharSize for DynCharSize {
    type Word = u16;
    // Irrelevant for DynCharSize
    const BITS: u8 = 0x0;
}
