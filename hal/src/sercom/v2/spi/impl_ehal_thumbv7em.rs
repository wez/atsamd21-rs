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
//! Remember that SAMx5x chips operate in 32-bit extension mode and use the
//! hardware `LENGTH` counter to set the number of bytes in each transaction.
//! The transaction [`Length`] is usually tracked at compile-time using
//! type-level integers from the [`typenum`] crate, but it can also be tracked
//! at run-time when using a [`DynLength`].
//!
//! The transaction `Length`s can be sub-divided into three groups:
//! - `Length`s of 1-4 bytes can be completed in a single read/write of the
//!   `DATA` register. These `Length`s are marked as [`AtomicSize`]s.
//! - `Length`s [`GreaterThan4`] are known at compile-time but cannot be
//!   completed atomically.
//! - A `DynLength` can be any length, but the value is only known at run-time.
//!
//! In general, transaction lengths with an `AtomicSize` implement embedded HAL
//! traits with the corresponding [`Word`] type. For example, [`Spi`] structs
//! using a transaction `Length` of 2 bytes implement `FullDuplex<u16>`. These
//! lengths implement both the blocking and non-blocking traits from embedded
//! HAL. The non-blocking traits are found in the [`spi`] and [`serial`]
//! modules, while the blocking traits are found in the [`blocking`] module.
//!
//! Transaction lengths `GreaterThan4` cannot be completed in a single read or
//! write of the `DATA` register, so these lengths do **NOT** implement the
//! non-blocking traits from the embedded HAL `spi` and `serial` modules.
//! Instead, they only implement traits from the `blocking` module. These traits
//! are implemented for `u8` types, e.g. `blocking::spi::Transfer<u8>`, and
//! operate on `[u8]` slices. The length of the slice is checked to ensure it
//! matches the transaction `Length`.
//!
//! Because a `DynLength` is not guaranteed to be an `AtomicSize`, the
//! corresponding `Spi` structs only implement the `blocking` traits as well.
//!
//! For a non-blocking alternative that can be used to transfer arbitrary-length
//! slices, see the [`spi_future`](super::super::spi_future) module.
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
use typenum::{U1, U2, U3, U4};

use crate::pac::sercom0::RegisterBlock;

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
impl<P, M, L> serial::Read<L::Word> for Spi<Config<P, M, L>, Rx>
where
    Config<P, M, L>: ValidConfig,
    P: ValidPads,
    M: MasterMode,
    L: Length,
    L::Word: PrimInt,
    u32: AsPrimitive<L::Word>,
{
    type Error = Error;

    #[inline]
    fn read(&mut self) -> nb::Result<L::Word, Error> {
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
impl<P, L> serial::Read<L::Word> for Spi<Config<P, Slave, L>, Rx>
where
    Config<P, Slave, L>: ValidConfig,
    P: ValidPads,
    L: Length,
    L::Word: PrimInt,
    u32: AsPrimitive<L::Word>,
{
    type Error = Error;

    #[inline]
    fn read(&mut self) -> nb::Result<L::Word, Error> {
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
    C::Word: PrimInt + AsPrimitive<u32>,
{
    type Error = Error;

    #[inline]
    fn write(&mut self, word: C::Word) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_status().contains(Status::LENERR) {
            Err(Error::LengthError.into())
        } else if self.read_flags().contains(Flags::DRE) {
            self.config.as_mut().regs.write_data(word.as_());
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), Error> {
        // Ignore buffer overflow errors
        if self.read_status().contains(Status::LENERR) {
            Err(Error::LengthError.into())
        } else if self.read_flags().contains(Flags::TXC) {
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
/// [`Capability`] and the transaction [`Length`] is `<= 4` bytes. When the
/// [`Length`] is `<= 4`, the [`Word`] is a primitive integer, with a size that
/// depends on the [`Length`] (`u8`, `u16` or `u32`).
impl<C> spi::FullDuplex<C::Word> for Spi<C, Duplex>
where
    C: ValidConfig,
    C::Word: PrimInt + AsPrimitive<u32>,
    u32: AsPrimitive<C::Word>,
{
    type Error = Error;

    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::RXC) {
            Ok(self.config.as_mut().regs.read_data().as_())
        } else {
            Err(WouldBlock)
        }
    }

    #[inline]
    fn send(&mut self, word: C::Word) -> nb::Result<(), Error> {
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::DRE) {
            self.config.as_mut().regs.write_data(word.as_());
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

//=============================================================================
// blocking::spi::Transfer
//=============================================================================

macro_rules! impl_blocking_spi_transfer {
    ( $($Length:ident),+ ) => {
        $(

            /// Implement [`Transfer`] for [`Spi`] structs that can [`Receive`]
            /// and have an [`AtomicSize`]
            ///
            /// The transaction [`Length`] must be `<= 4`. The transfer accepts
            /// a slice of primitive integers, depending on the `Length`
            /// (`u8`, `u16` or `u32`).
            ///
            /// [`Transfer`]: blocking::spi::Transfer
            impl<P, M, A> blocking::spi::Transfer<Word<$Length>> for Spi<Config<P, M, $Length>, A>
            where
                Config<P, M, $Length>: ValidConfig,
                P: ValidPads,
                M: OpMode,
                A: Receive,
            {
                type Error = Error;

                #[inline]
                fn transfer<'w>(&mut self, words: &'w mut [Word<$Length>]) -> Result<&'w [Word<$Length>], Error> {
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
                            self.config.as_mut().regs.write_data(word as u32);
                        }
                        if to_recv.len() > to_send.len() && flags.contains(Flags::RXC) {
                            let word = self.config.as_mut().regs.read_data() as Word<$Length>;
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

impl_blocking_spi_transfer!(U1, U2, U3, U4);

/// Implement [`Transfer`] for [`Spi`] structs that can [`Receive`] and have
/// long transaction [`Length`]s
///
/// The transaction [`Length`] must be `> 4`. The transfer accepts a slice of
/// `u8` with a length equal to the transaction [`Length`]. If the slice length
/// is incorrect, it will panic.
///
/// [`Transfer`]: blocking::spi::Transfer
impl<P, M, L, A> blocking::spi::Transfer<u8> for Spi<Config<P, M, L>, A>
where
    Config<P, M, L>: ValidConfig,
    P: ValidPads,
    M: OpMode,
    L: GreaterThan4,
    A: Receive,
{
    type Error = Error;

    #[inline]
    fn transfer<'w>(&mut self, buf: &'w mut [u8]) -> Result<&'w [u8], Error> {
        assert_eq!(buf.len(), L::USIZE);
        let sercom = unsafe { self.config.as_ref().sercom() };
        transfer_slice(sercom, buf)
    }
}

/// Implement [`Transfer`] for [`Spi`] structs that can [`Receive`] and have
/// [`DynLength`]
///
/// The transfer accepts a slice of `u8` with a length equal to the run-time
/// dynamic transaction length. If the slice length does not match the result
/// of [`Spi::get_dyn_length`], it will panic.
///
/// [`Transfer`]: blocking::spi::Transfer
impl<P, M, A> blocking::spi::Transfer<u8> for Spi<Config<P, M, DynLength>, A>
where
    Config<P, M, DynLength>: ValidConfig,
    P: ValidPads,
    M: OpMode,
    A: Receive,
{
    type Error = Error;

    #[inline]
    fn transfer<'w>(&mut self, buf: &'w mut [u8]) -> Result<&'w [u8], Error> {
        assert_eq!(buf.len(), self.get_dyn_length() as usize);
        let sercom = unsafe { self.config.as_ref().sercom() };
        transfer_slice(sercom, buf)
    }
}

//=============================================================================
// blocking::spi::Write
//=============================================================================

macro_rules! impl_blocking_spi_write {
    ( $($Length:ident),+ ) => {
        $(

            /// Implement [`Write`] for [`Spi`] structs with [`Duplex`]
            /// [`Capability`] and an [`AtomicSize`]
            ///
            /// The transaction `Length` must be `<= 4`. The transfer accepts
            /// a slice of primitive integers, depending on the `Length`
            /// (`u8`, `u16` or `u32`).
            ///
            /// [`Write`]: blocking::spi::Write
            impl<P, M> blocking::spi::Write<Word<$Length>> for Spi<Config<P, M, $Length>, Duplex>
            where
                Config<P, M, $Length>: ValidConfig,
                P: ValidPads,
                M: OpMode,
            {
                type Error = Error;

                #[inline]
                fn write(&mut self, words: &[Word<$Length>]) -> Result<(), Error> {
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
                            self.config.as_mut().regs.write_data(word as u32);
                        }
                        if to_recv > to_send.len() && flags.contains(Flags::RXC) {
                            self.config.as_mut().regs.read_data() as Word<$Length>;
                            to_recv -= 1;
                        }
                    }
                    Ok(())
                }
            }

            /// Implement [`Write`] for [`Spi`] structs with [`Tx`]
            /// [`Capability`] and an [`AtomicSize`]
            ///
            /// The transaction `Length` must be `<= 4`. The transfer accepts
            /// a slice of primitive integers, depending on the `Length`
            /// (`u8`, `u16` or `u32`).
            ///
            /// Because the `Capability` is `Tx`, this implementation never
            /// reads the DATA register and ignores all buffer overflow errors.
            ///
            /// [`Write`]: blocking::spi::Write
            impl<P, M> blocking::spi::Write<Word<$Length>> for Spi<Config<P, M, $Length>, Tx>
            where
                Config<P, M, $Length>: ValidConfig,
                P: ValidPads,
                M: OpMode,
            {
                type Error = Error;

                #[inline]
                fn write(&mut self, words: &[Word<$Length>]) -> Result<(), Error> {
                    // We are `Tx`, so we don't have to consider reading at all, ever.
                    for word in words {
                        loop {
                            // Ignore buffer overflow errors
                            if self.read_status().contains(Status::LENERR) {
                                return Err(Error::LengthError)
                            } else if self.read_flags().contains(Flags::DRE) {
                                self.config.as_mut().regs.write_data(*word as u32);
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

impl_blocking_spi_write!(U1, U2, U3, U4);

/// Implement [`Write`] for [`Spi`] structs with [`Duplex`] [`Capability`] and
/// long transaction [`Length`]s
///
/// The transaction [`Length`] must be `> 4`. The transfer accepts a `[u8]` with
/// a length equal to the transfer [`Length`]. If the slice length is incorrect,
/// it will panic.
///
/// [`Write`]: blocking::spi::Write
impl<P, M, L> blocking::spi::Write<u8> for Spi<Config<P, M, L>, Duplex>
where
    Config<P, M, L>: ValidConfig,
    P: ValidPads,
    M: OpMode,
    L: GreaterThan4,
{
    type Error = Error;

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        if buf.len() != L::USIZE {
            panic!("Slice length does not equal SPI transfer length");
        }
        let sercom = unsafe { self.config.as_ref().sercom() };
        write_slice(sercom, buf, true)
    }
}

/// Implement [`Write`] for [`Spi`] structs with [`Tx`] [`Capability`] and long
/// transaction [`Length`]s
///
/// The transaction [`Length`] must be `> 4`. The transfer accepts a `[u8]` with
/// a length equal to the transfer [`Length`]. If the slice length is incorrect,
/// it will panic.
///
/// Because the `Capability` is `Tx`, this implementation never reads the DATA
/// register and ignores all buffer overflow errors.
///
/// [`Write`]: blocking::spi::Write
impl<P, M, L> blocking::spi::Write<u8> for Spi<Config<P, M, L>, Tx>
where
    Config<P, M, L>: ValidConfig,
    P: ValidPads,
    M: OpMode,
    L: GreaterThan4,
{
    type Error = Error;

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        if buf.len() != L::USIZE {
            panic!("Slice length does not equal SPI transfer length");
        }
        let sercom = unsafe { self.config.as_ref().sercom() };
        write_slice(sercom, buf, false)
    }
}

/// Implement [`Write`] for [`Spi`] structs with [`Duplex`] [`Capability`] and
/// [`DynLength`]
///
/// The transfer accepts a `[u8]` with a length equal to the run-time dynamic
/// transaction length. If the slice length does not match the result of
/// [`Spi::get_dyn_length`], it will panic.
///
/// [`Write`]: blocking::spi::Write
impl<P, M> blocking::spi::Write<u8> for Spi<Config<P, M, DynLength>, Duplex>
where
    Config<P, M, DynLength>: ValidConfig,
    P: ValidPads,
    M: OpMode,
{
    type Error = Error;

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        if buf.len() != self.get_dyn_length() as usize {
            panic!("Slice length does not equal SPI transfer length");
        }
        let sercom = unsafe { self.config.as_ref().sercom() };
        write_slice(sercom, buf, true)
    }
}

/// Implement [`Write`] for [`Spi`] structs with [`Tx`] [`Capability`] and
/// [`DynLength`]
///
/// The transfer accepts a `[u8]` with a length equal to the run-time dynamic
/// transaction length. If the slice length does not match the result of
/// `Spi::get_dyn_length`], it will panic.
///
/// Because the `Capability` is `Tx`, this implementation never reads the DATA
/// register and ignores all buffer overflow errors.
///
/// [`Write`]: blocking::spi::Write
impl<P, M> blocking::spi::Write<u8> for Spi<Config<P, M, DynLength>, Tx>
where
    Config<P, M, DynLength>: ValidConfig,
    P: ValidPads,
    M: OpMode,
{
    type Error = Error;

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        if buf.len() != self.get_dyn_length() as usize {
            panic!("Slice length does not equal SPI transfer length");
        }
        let sercom = unsafe { self.config.as_ref().sercom() };
        write_slice(sercom, buf, false)
    }
}

//=============================================================================
// blocking::spi::WriteIter
//=============================================================================

macro_rules! impl_blocking_spi_write_iter {
    ( $($Length:ident),+ ) => {
        $(

            /// Implement [`WriteIter`] for [`Spi`] structs with [`Duplex`]
            /// [`Capability`] and an [`AtomicSize`]
            ///
            /// The transaction `Length` must be `<= 4`. The transfer accepts
            /// a slice of primitive integers, depending on the `Length`
            /// (`u8`, `u16` or `u32`).
            ///
            /// [`WriteIter`]: blocking::spi::WriteIter
            #[cfg(feature = "unproven")]
            impl<P, M> blocking::spi::WriteIter<Word<$Length>> for Spi<Config<P, M, $Length>, Duplex>
            where
                Config<P, M, $Length>: ValidConfig,
                P: ValidPads,
                M: OpMode,
            {
                type Error = Error;

                #[inline]
                fn write_iter<WI>(&mut self, words: WI) -> Result<(), Error>
                where
                    WI: IntoIterator<Item = Word<$Length>>,
                {
                    // We are `Duplex`, so we must receive as many words as we send,
                    // otherwise we could trigger an overflow. However, we don't know
                    // how many words there are to start with, so we have to send and
                    // receive them one at a time.
                    for word in words.into_iter() {
                        loop {
                            let flags = self.read_flags_errors()?;
                            if flags.contains(Flags::DRE) {
                                unsafe { self.write_data(word as u32) };
                                break
                            }
                        }
                        loop {
                            let flags = self.read_flags_errors()?;
                            if flags.contains(Flags::RXC) {
                                self.config.as_mut().regs.read_data() as Word<$Length>;
                                break
                            }
                        }
                    }
                    Ok(())
                }
            }
            /// Implement [`WriteIter`] for [`Spi`] structs with [`Tx`]
            /// [`Capability`] and an [`AtomicSize`]
            ///
            /// The transaction `Length` must be `<= 4`. The transfer accepts
            /// a slice of primitive integers, depending on the `Length`
            /// (`u8`, `u16` or `u32`).
            ///
            /// Because the `Capability` is `Tx`, this implementation never
            /// reads the DATA register and ignores all buffer overflow errors.
            ///
            /// [`WriteIter`]: blocking::spi::WriteIter
            #[cfg(feature = "unproven")]
            impl<P, M> blocking::spi::WriteIter<Word<$Length>> for Spi<Config<P, M, $Length>, Tx>
            where
                Config<P, M, $Length>: ValidConfig,
                P: ValidPads,
                M: OpMode,
            {
                type Error = Error;

                #[inline]
                fn write_iter<WI>(&mut self, words: WI) -> Result<(), Error>
                where
                    WI: IntoIterator<Item = Word<$Length>>,
                {
                    // We are `Tx`, so we don't have to consider reading at all, ever.
                    for word in words.into_iter() {
                        loop {
                            // Ignore buffer overflow errors
                            if self.read_status().contains(Status::LENERR) {
                                return Err(Error::LengthError)
                            } else if self.read_flags().contains(Flags::DRE) {
                                unsafe { self.write_data(word as u32) };
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

impl_blocking_spi_write_iter!(U1, U2, U3, U4);

//=============================================================================
// Helper functions
//=============================================================================

/// Transfer a `[u8]` slice four bytes at a time
///
/// This function exists to avoid both code duplication and monomorphization
/// bloat. It will take a `[u8]` and transfer it four bytes at a time.
fn transfer_slice<'w>(sercom: &RegisterBlock, buf: &'w mut [u8]) -> Result<&'w [u8], Error> {
    let cells = core::cell::Cell::from_mut(buf).as_slice_of_cells();
    let mut to_send = cells.iter();
    let mut to_recv = cells.iter();
    while to_recv.len() > 0 {
        let errors = sercom.spim().status.read();
        if errors.bufovf().bit_is_set() {
            return Err(Error::Overflow);
        }
        if errors.lenerr().bit_is_set() {
            return Err(Error::LengthError);
        }
        let flags = sercom.spim().intflag.read();
        if to_send.len() > 0 && flags.dre().bit_is_set() {
            let mut bytes = [0; 4];
            for byte in &mut bytes {
                match to_send.next() {
                    Some(cell) => *byte = cell.get(),
                    None => break,
                }
            }
            let word = u32::from_le_bytes(bytes);
            sercom.spim().data.write(|w| unsafe { w.data().bits(word) });
        }
        if to_recv.len() > to_send.len() && flags.rxc().bit_is_set() {
            let word = sercom.spim().data.read().data().bits();
            let bytes = word.to_le_bytes();
            for byte in bytes.iter() {
                match to_recv.next() {
                    Some(cell) => cell.set(*byte),
                    None => break,
                }
            }
        }
    }
    Ok(buf)
}

/// Write a `[u8]` four bytes at a time
///
/// This function exists to avoid both code duplication and monomorphization
/// bloat. It will take a `[u8]` and write four bytes at a time to the SPI on
/// every DRE flag. If the `duplex` argument is true, it will read as many times
/// as it writes. Otherwise, it will skip reading the `DATA` register entirely.
/// If `duplex` is false, buffer overflow errors are ignored
fn write_slice(sercom: &RegisterBlock, buf: &[u8], duplex: bool) -> Result<(), Error> {
    let mut to_send = buf.iter();
    let mut to_recv: usize = to_send.len();
    while to_recv > 0 {
        let errors = sercom.spim().status.read();
        if duplex && errors.bufovf().bit_is_set() {
            return Err(Error::Overflow);
        }
        if errors.lenerr().bit_is_set() {
            return Err(Error::LengthError);
        }
        let flags = sercom.spim().intflag.read();
        // Send the word
        if to_send.len() > 0 && flags.dre().bit_is_set() {
            let mut bytes = [0; 4];
            for byte in &mut bytes {
                match to_send.next() {
                    Some(d) => *byte = *d,
                    None => break,
                }
            }
            let word = u32::from_le_bytes(bytes);
            sercom.spim().data.write(|w| unsafe { w.data().bits(word) });
        }
        if duplex && to_recv > to_send.len() && flags.rxc().bit_is_set() {
            sercom.spim().data.read().data().bits();
            let diff = to_recv - to_send.len();
            to_recv -= if diff < 4 { diff } else { 4 };
        }
    }
    Ok(())
}
