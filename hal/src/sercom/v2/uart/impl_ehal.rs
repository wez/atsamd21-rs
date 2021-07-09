//! `embedded-hal` trait implementations for [`Uart`]s

use super::{DataSize, Error, Flags, Receive, Transmit, Uart, ValidConfig};
use embedded_hal::{
    blocking,
    serial::{Read, Write},
};
use nb::Error::WouldBlock;
use num_traits::AsPrimitive;

impl<C, D> Read<C::Word> for Uart<C, D>
where
    C: ValidConfig,
    D: Receive,
    DataSize: AsPrimitive<C::Word>,
{
    type Error = Error;

    /// Wait for an `RXC` flag, then read the word
    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::RXC) {
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }
}

impl<C, D> Write<C::Word> for Uart<C, D>
where
    C: ValidConfig,
    D: Transmit,
{
    type Error = core::convert::Infallible;

    /// Wait for a `DRE` flag, then write a word
    #[inline]
    fn write(&mut self, word: C::Word) -> nb::Result<(), Self::Error> {
        // Ignore buffer overflow errors
        if self.read_flags().contains(Flags::DRE) {
            unsafe { self.write_data(word.as_()) };
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }

    /// Wait for a `TXC` flag
    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // Ignore buffer overflow errors
        if self.read_flags().contains(Flags::TXC) {
            self.clear_flags(Flags::TXC);
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

impl<C, D> blocking::serial::write::Default<C::Word> for Uart<C, D>
where
    C: ValidConfig,
    D: Transmit,
    Uart<C, D>: Write<C::Word>,
{
}
