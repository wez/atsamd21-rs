//! Implement [`embedded_hal`] traits for [`Spi`] structs
//!
//! As noted in the [spi module](super) documentation, the embedded-hal trait
//! implementations vary by both [`Size`] and [`Capability`]. Each
//! implementation is optimized to take advantage of all information known at
//! compile-time, so it is importatnt to carefully read the documentation in
//! this module.
//!
//! # Variations by [`Size`]
//!
//! SAMD11 and SAMD21 chips do not have 32-bit extension mode, so their
//! transaction `Size` can only vary by the [`CharSize`]. Both options,
//! [`EightBit`] and [`NineBit`], are [`AtomicSize`]s, because each can be
//! completed with a single read/write of the `DATA` register. Consequently,
//! each can implement both the blocking and non-blocking embedded HAL traits.
//! These traits are implemented for the [`Word`] type of the corresponding
//! `CharSize`. For example, an [`Spi`] struct with a `NineBit` `CharSize` would
//! implement `spi::FullDuplex<u16>`.
//!
//! Note that embedded HAL does not offer a way to transmit slices in a
//! non-blocking fashion, but this can be done using interrupts and the
//! [`spi_future`](super::super::spi_future) module.
//!
//! # Variations by [`Capability`]
//!
//! The implementations in this module also seek to optimize as much as possible
//! based on the `Capability` of the `Spi` struct. They follow a few general
//! rules:
//! - [`Tx`] structs can never receive data, so their corresponding trait
//!   implementations never read the `DATA` register and can never return an
//!   [`Error::Overflow`].
//! - [`Rx`] structs in a [`MasterMode`](super::MasterMode) must initiate all
//!   transactions, so their implementations of non-blocking traits must track
//!   the state of on-going transactions.
//! - [`Duplex`] structs must always read as many bytes as they send, even when
//!   implementing `Write`-only traits, to ensure they never introduce an
//!   [`Error::Overflow`].
//!
//! # Notes on individual embedded HAL traits
//!
//! ## `spi::FullDuplex`
//!
//! `spi::FullDuplex` is only implemented for structs with `Duplex`
//! `Capability`. Although the embedded HAL documentation assumes a
//! `MasterMode`, this module also implements it for the [`Slave`] [`OpMode`].
//!
//! ## `serial::Read`
//!
//! `serial::Read` is only implemented for structs with `Rx` `Capability`. When
//! in a `MasterMode`, it initiates and tracks the state of the on-going
//! transactions. But this is not required when acting as a `Slave`.
//!
//! ## `serial::Write`
//!
//! `serial::Write` is only implemented for structs with `Tx` `Capability`.
//! These implementations never read the `DATA` register and ignore all
//! instances of [`Error::Overflow`].
//!
//! ## `blocking::serial::Write`
//!
//! This trait uses the `blocking::serial::write::Default` implementation for
//! implementers of `serial::Write`.
//!
//! ## `blocking::spi` traits
//!
//! These traits are implemented following all of the rules outlined above for
//! the different [`Size`] and [`Capability`] options.

use embedded_hal::{blocking, serial, spi};
use nb::Error::WouldBlock;
use num_traits::{AsPrimitive, PrimInt};

use super::*;

//=============================================================================
// serial::Read
//=============================================================================

/// Implement [`serial::Read`] for [`Rx`] [`Spi`] structs in a [`MasterMode`]
///
/// `serial::Read` is only implemented for `Spi` structs with `Rx`
/// [`Capability`]. In a `MasterMode`, `Read` has to initiate transactions, so
/// it keeps track of the transaction state. If a transaction is in progress,
/// it will wait on `RXC`. If not, it will wait on `DRE`, and then send `0`.
impl<P, M, C> serial::Read<C::Word> for Spi<Config<P, M, C>, Rx>
where
    Config<P, M, C>: ValidConfig,
    P: ValidPads,
    M: MasterMode,
    C: CharSize,
    C::Word: PrimInt,
    u16: AsPrimitive<C::Word>,
{
    type Error = Error;

    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Error> {
        let in_progress = self.capability.in_progress;
        let flags = self.read_flags_errors()?;
        if !in_progress && flags.contains(Flags::DRE) {
            unsafe { self.write_data(0) };
            self.capability.in_progress = true;
            Err(WouldBlock)
        } else if in_progress && flags.contains(Flags::RXC) {
            self.capability.in_progress = false;
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }
}

/// Implement [`serial::Read`] for [`Rx`] [`Spi`] structs in [`Slave`]
/// [`OpMode`]
///
/// `serial::Read` is only implemented for `Spi` structs with `Rx`
/// [`Capability`]. In `Slave` `OpMode`, `Read` does not have to initiate
/// transactions, so it does not have to store any internal state. It only has
/// to wait on `RXC`.
impl<P, C> serial::Read<C::Word> for Spi<Config<P, Slave, C>, Rx>
where
    Config<P, Slave, C>: ValidConfig,
    P: ValidPads,
    C: CharSize,
    C::Word: PrimInt,
    u16: AsPrimitive<C::Word>,
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

//=============================================================================
// serial::Write
//=============================================================================

/// Implement [`serial::Write`] for [`Tx`] [`Spi`] structs
///
/// `serial::Write` is only implemented for `Spi` structs with `Tx`
/// [`Capability`]. Because the `Capability` is `Tx`, this implementation never
/// reads the DATA register and ignores all buffer overflow errors.
impl<C> serial::Write<C::Word> for Spi<C, Tx>
where
    C: ValidConfig,
    C::Word: PrimInt + AsPrimitive<u16>,
{
    type Error = Error;

    #[inline]
    fn write(&mut self, word: C::Word) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_flags().contains(Flags::DRE) {
            unsafe { self.write_data(word.as_()) };
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_flags().contains(Flags::TXC) {
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

//=============================================================================
// blocking::serial::Write
//=============================================================================

impl<C> blocking::serial::write::Default<C::Word> for Spi<C, Tx>
where
    C: ValidConfig,
    Spi<C, Tx>: serial::Write<C::Word>,
{
}

//=============================================================================
// spi::FullDuplex
//=============================================================================

/// Implement [`spi::FullDuplex`] for [`Spi`] structs with [`AtomicSize`]
///
/// `spi::FullDuplex` is only implemented when the `Spi` struct has [`Duplex`]
/// [`Capability`]. The [`Word`] size used in the implementation depends on the
/// corresponding [`CharSize`].
impl<C> spi::FullDuplex<C::Word> for Spi<C, Duplex>
where
    C: ValidConfig,
    C::Word: PrimInt + AsPrimitive<u16>,
    u16: AsPrimitive<C::Word>,
{
    type Error = Error;

    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::RXC) {
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }

    #[inline]
    fn send(&mut self, word: C::Word) -> nb::Result<(), Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::DRE) {
            unsafe { self.write_data(word.as_()) };
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

//=============================================================================
// Note on macros
//=============================================================================

// Macros are necessary for the following implementations of the embedded HAL
// `blocking` traits because of a limitation in the Rust trait system. The
// compiler can't seem to recongnize that the `blocking::spi::*::Default` traits
// can never be implemented for [`Spi`] in downstream crates, because that would
// violate the orphan rules.

//=============================================================================
// blocking::spi::Transfer
//=============================================================================

macro_rules! impl_blocking_spi_transfer {
    ( $($CharSize:ident),+ ) => {
        $(
            /// Implement [`Transfer`] for [`Spi`] structs that can [`Receive`]
            ///
            /// The transfer accepts a slice of primitive integers, depending on
            /// the [`CharSize`] (`u8` or `u16`).
            ///
            /// [`Transfer`]: blocking::spi::Transfer
            impl<P, M, A> blocking::spi::Transfer<Word<$CharSize>> for Spi<Config<P, M, $CharSize>, A>
            where
                Config<P, M, $CharSize>: ValidConfig,
                P: ValidPads,
                M: OpMode,
                A: Receive,
            {
                type Error = Error;

                #[inline]
                fn transfer<'w>(&mut self, words: &'w mut [Word<$CharSize>]) -> Result<&'w [Word<$CharSize>], Error> {
                    let cells = core::cell::Cell::from_mut(words).as_slice_of_cells();
                    let mut to_send = cells.iter();
                    let mut to_recv = cells.iter();
                    while to_recv.len() > 0 {
                        let flags = self.read_flags_errors()?;
                        if to_send.len() > 0 && flags.contains(Flags::DRE) {
                            let word = match to_send.next() {
                                Some(cell) => cell.get(),
                                None => unreachable!(),
                            };
                            self.config.as_mut().regs.write_data(word as u16);
                        }
                        if to_recv.len() > to_send.len() && flags.contains(Flags::RXC) {
                            let word = self.config.as_mut().regs.read_data() as Word<$CharSize>;
                            match to_recv.next() {
                                Some(cell) => cell.set(word),
                                None => unreachable!(),
                            }
                        }
                    }
                    Ok(words)
                }
            }
        )+
    }
}

impl_blocking_spi_transfer!(EightBit, NineBit);

//=============================================================================
// blocking::spi::Write
//=============================================================================

macro_rules! impl_blocking_spi_write {
    ( $($CharSize:ident),+ ) => {
        $(
            /// Implement [`Write`] for [`Spi`] structs with [`Duplex`]
            /// [`Capability`]
            ///
            /// The transfer accepts a slice of primitive integers, depending on
            /// the [`CharSize`] (`u8` or `u16`).
            ///
            /// [`Write`]: blocking::spi::Write
            impl<P, M> blocking::spi::Write<Word<$CharSize>> for Spi<Config<P, M, $CharSize>, Duplex>
            where
                Config<P, M, $CharSize>: ValidConfig,
                P: ValidPads,
                M: OpMode,
            {
                type Error = Error;

                #[inline]
                fn write(&mut self, words: &[Word<$CharSize>]) -> Result<(), Error> {
                    // We are `Duplex`, so we must receive as many words as we send,
                    // otherwise we could trigger an overflow
                    let mut to_send = words.iter();
                    let mut to_recv = to_send.len();
                    while to_recv > 0 {
                        let flags = self.read_flags_errors()?;
                        if to_send.len() > 0 && flags.contains(Flags::DRE) {
                            let word = match to_send.next() {
                                Some(word) => *word,
                                None => unreachable!(),
                            };
                            self.config.as_mut().regs.write_data(word as u16);
                        }
                        if to_recv > to_send.len() && flags.contains(Flags::RXC) {
                            self.config.as_mut().regs.read_data();
                            to_recv -= 1;
                        }
                    }
                    Ok(())
                }
            }

            /// Implement [`Write`] for [`Spi`] structs with [`Tx`]
            /// [`Capability`]
            ///
            /// The transfer accepts a slice of primitive integers, depending on
            /// the [`CharSize`] (`u8` or `u16`).
            ///
            /// Because the `Capability` is `Tx`, this implementation never
            /// reads the DATA register and ignores all buffer overflow errors.
            ///
            /// [`Write`]: blocking::spi::Write
            impl<P, M> blocking::spi::Write<Word<$CharSize>> for Spi<Config<P, M, $CharSize>, Tx>
            where
                Config<P, M, $CharSize>: ValidConfig,
                P: ValidPads,
                M: OpMode,
            {
                type Error = Error;

                #[inline]
                fn write(&mut self, words: &[Word<$CharSize>]) -> Result<(), Error> {
                    // We are `Tx`, so we don't have to consider reading at all, ever.
                    for word in words {
                        loop {
                            // Ignore buffer overflow errors
                            if self.read_status().contains(Status::LENERR) {
                                return Err(Error::LengthError)
                            } else if self.read_flags().contains(Flags::DRE) {
                                self.config.as_mut().regs.write_data(*word as u16);
                                break
                            }
                        }
                    }
                    Ok(())
                }
            }
        )+
    }
}

impl_blocking_spi_write!(EightBit, NineBit);

//=============================================================================
// blocking::spi::WriteIter
//=============================================================================

#[cfg(feature = "unproven")]
macro_rules! impl_blocking_spi_write_iter {
    ( $($CharSize:ident),+ ) => {
        $(
            /// Implement [`WriteIter`] for [`Spi`] structs with [`Duplex`]
            /// [`Capability`]
            ///
            /// The transfer accepts a slice of primitive integers, depending on
            /// the [`CharSize`] (`u8` or `u16`).
            ///
            /// [`WriteIter`]: blocking::spi::WriteIter
            impl<P, M> blocking::spi::WriteIter<Word<$CharSize>> for Spi<Config<P, M, $CharSize>, Duplex>
            where
                Config<P, M, $CharSize>: ValidConfig,
                P: ValidPads,
                M: OpMode,
            {
                type Error = Error;

                #[inline]
                fn write_iter<WI>(&mut self, words: WI) -> Result<(), Error>
                where
                    WI: IntoIterator<Item = Word<$CharSize>>,
                {
                    // We are `Duplex`, so we must receive as many words as we send,
                    // otherwise we could trigger an overflow. However, we don't know
                    // how many words there are to start with, so we have to send and
                    // receive them one at a time.
                    for word in words.into_iter() {
                        loop {
                            let flags = self.read_flags_errors()?;
                            if flags.contains(Flags::DRE) {
                                unsafe { self.write_data(word as u16) };
                                break
                            }
                        }
                        loop {
                            let flags = self.read_flags_errors()?;
                            if flags.contains(Flags::RXC) {
                                self.config.as_mut().regs.read_data() as Word<$CharSize>;
                                break
                            }
                        }
                    }
                    Ok(())
                }
            }

            /// Implement [`WriteIter`] for [`Spi`] structs with [`Tx`]
            /// [`Capability`]
            ///
            /// The transfer accepts a slice of primitive integers, depending on
            /// the [`CharSize`] (`u8` or `u16`).
            ///
            /// Because the `Capability` is `Tx`, this implementation never
            /// reads the DATA register and ignores all buffer overflow errors.
            ///
            /// [`WriteIter`]: blocking::spi::WriteIter
            impl<P, M> blocking::spi::WriteIter<Word<$CharSize>> for Spi<Config<P, M, $CharSize>, Tx>
            where
                Config<P, M, $CharSize>: ValidConfig,
                P: ValidPads,
                M: OpMode,
            {
                type Error = Error;

                #[inline]
                fn write_iter<WI>(&mut self, words: WI) -> Result<(), Error>
                where
                    WI: IntoIterator<Item = Word<$CharSize>>,
                {
                    // We are `Tx`, so we don't have to consider reading at all, ever.
                    for word in words.into_iter() {
                        loop {
                            // Ignore buffer overflow errors
                            if self.read_status().contains(Status::LENERR) {
                                return Err(Error::LengthError)
                            } else if self.read_flags().contains(Flags::DRE) {
                                unsafe { self.write_data(word as u16) };
                                break
                            }
                        }
                    }
                    Ok(())
                }
            }
        )+
    };
}

#[cfg(feature = "unproven")]
impl_blocking_spi_write_iter!(EightBit, NineBit);
