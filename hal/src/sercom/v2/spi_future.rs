#![allow(broken_intra_doc_links)]
//! A [`Future`]-like interface for SPI transactions
//!
//! An [`SpiFuture`] takes ownership of an [`Spi`] `struct` and a `[u8]`-like
//! buffer. It then executes a full-duplex SPI transaction using iterrupts. On
//! each `RXC` or `DRE` interrupt, the [`SpiFuture`] reads or sends `STEP` bytes
//! of the buffer, where `STEP` is a value that depends on [`CharSize`], for
//! SAMD11 & SAMD21 chips, or [`Length`], for SAMD51 & SAME5x chips.
//!
//! The provided buffer must implement [`AsRef`] and [`AsMut`] for `[u8]`, it
//! must have an appropriate length (see below), and it must have a `'static`
//! lifetime, i.e. it must be owned or a `&'static mut` reference.
//!
//! [`SpiFuture`] has extra, optional capabilities as well. It can accept a
//! function or closure that will be called on completion of the transaction,
//! acting like a [`Waker`]. And it can take a GPIO [`Pin`] to use as the SS
//! line. If provided, the [`Pin`] will be set low at the beginnging of the
//! transfer and brought high at completion.
//!
//! Calling [`start`] will enable the `DRE` and `RXC` interrupts and begin the
//! transaction.
//!
//! ```
//! use atsamd_hal::gpio::v2::{Pin, PA10, PushPullOutput};
//! use atsamd_hal::sercom::v2::spi::AnySpi;
//! use atsamd_hal::sercom::v2::spi_future::SpiFuture;
//!
//! fn wake_up() {
//!     //...
//! }
//!
//! fn use_future(spi: impl AnySpi, ss: Pin<PA10, PushPullOutput>) {
//!     let buf = [0_u8; 12];
//!     let future = SpiFuture::new(spi, buf)
//!         .with_waker(wake_up)
//!         .with_ss(ss);
//!     future.start();
//! }
//! ```
//!
//! When sending and receiving finish, the [`SpiFuture`] will automatically
//! disable the `DRE` and `RXC` interrupts. To test whether an [`SpiFuture`] is
//! complete, use the [`poll`] method. While the transaction is in progress, it
//! will return [`Poll::Pending`]. When the transaction is complete, it will
//! return [`Poll::Ready`]. Once complete, you can consume the [`SpiFuture`] and
//! [`free`] the constituent pieces. Doing so before the transfer has completed
//! is `unsafe`.
//!
//! The actual transfer is performed by the [`send`] and [`recv`] methods, which
//! should be called from the `DRE` and `RXC` interrupt handlers, respectively.
//!
//! ## `STEP` size and buffer length
//!
//! For SAMD11 & SAMD21 chips, `STEP` is equal to the number of bytes in the
//! corresponding the [`CharSize::Word`] type, i.e. 1 for [`EightBit`] and 2 for
//! [`NineBit`]. For SAMD51 & SAME5x chips, `STEP` is equal to the [`Length`] or
//! 4, whichever is less.
//!
//! The provided buffer must have an appropriate length. For SAMD11 & SAMD21
//! chips, as well as SAMDx5x chips with [`Length`]` <= 4`, a single write of
//! `STEP` bytes represents an entire SPI transaction. In these cases, the
//! provided buffer must represent an integer number of transactions. For
//! example, a SAMD51 [`Spi`] struct with a [`Length`] of 3 could use buffers of
//! length 3, 6, 9, etc. For longer [`Length`] values, the provided buffer must
//! represent exactly one SPI transaction, so the buffer length must be equal to
//! [`Length`]. For example, a SAMD51 [`Spi`] struct with a [`Length`] of 17
//! could only use a buffer with exactly 17 bytes.
//!
//! Keep in mind that [`SpiFuture`] works only with `[u8]`-like things, which
//! can introduce some limitations.
//!
//! Suppose you plan to execute [`NineBit`] transactions with a SAMD21 chip.
//! Your data may come in the form of a `[u16]` slice. However, to use it with
//! [`SpiFuture`], you will need reformulate it as a `[u8]` slice. The easiest
//! way to do so is probably to transmute the slice to `[u8]` or cast the
//! reference to `&'static mut [u8]`. Both of these operations are sound,
//! because [`u8`] has no alignment requirements.
//!
//! In another scenario, suppose you wanted to use a SAMx5x chip with a
//! transaction [`Length`] of 3 bytes. Your data might come in the form of a
//! `[u32]` slice. In this situation, it would **not** be appropriate to
//! transmute or cast the data to a `[u8]` slice. [`SpiFuture`] expects the data
//! to be a *byte-packed* `[u8]` slice, so the extra byte in each `u32` makes it
//! incompatible.
//!
//! ## [RTIC] example
//!
//! The [RTIC] framework provides a convenient way to store a `static`ally
//! allocated [`SpiFuture`], so that it can be accessed by both the interrupt
//! handlers and user code. The following example shows how [`SpiFuture`]s might
//! be used for a series of transactions. It was written for a SAMx5x chip, and
//! it uses features from the latest release of [RTIC], `v0.6-alpha.0`.
//!
//! ```
//! use core::task::Poll;
//! use atsamd_hal::gpio::v2::{PA08, PA09, PA10, PA11, Pin, PushPullOutput};
//! use atsamd_hal::sercom::v2::Sercom0;
//! use atsamd_hal::sercom::v2::pad::{IoSet1, Pad0, Pad1, Pad3};
//! use atsamd_hal::sercom::v2::spi::{self, Master, lengths::U12};
//! use atsamd_hal::sercom::v2::spi_future::SpiFuture;
//!
//! type Pads = spi::Pads<Sercom0, IoSet1, (Pad0, PA08), (Pad3, PA11), (Pad1, PA09)>;
//! type SS = Pin<PA10, PushPullOutput>;
//! type Spi = spi::Spi<spi::Config<Pads, Master, U12>>;
//! type Future = SpiFuture<Spi, [u8; 12], SS, fn()>;
//!
//! //...
//!
//! #[resources]
//! struct Resources {
//!     #[task_local]
//!     #[init(None)]
//!     opt_spi_ss: Option<(Spi, SS)>,
//!
//!     #[lock_free]
//!     #[init(None)]
//!     opt_future: Option<Future>,
//! }
//!
//! #[task(resources = [opt_spi_ss, opt_future])]
//! fn task(ctx: task::Context) {
//!     let task::Context { opt_spi_ss, opt_future } = ctx;
//!     match opt_future {
//!         Some(future) => {
//!             if let Poll::Ready(_) = future.poll() {
//!                 let (spi, buf, ss) = opt_future.take().unwrap().free();
//!                 *opt_spi_ss = Some((spi, ss));
//!                 consume_data(buf);
//!             }
//!         }
//!         None => {
//!             if let Some((spi, ss)) = opt_spi_ss.take() {
//!                 let buf: [u8; 12] = produce_data();
//!                 let future = opt_future.get_or_insert(
//!                     SpiFuture::new(spi, buf)
//!                         .with_waker(|| { task::spawn().ok(); })
//!                         .with_ss(ss)
//!                 );
//!                 future.start();
//!             }
//!         }
//!     }
//! }
//!
//! #[task(binds = SERCOM0_0, resources = [opt_future])]
//! fn dre(ctx: dre::Context) {
//!     ctx.resources.opt_future.as_mut().unwrap().send();
//! }
//!
//! #[task(binds = SERCOM0_2, resources = [opt_future])]
//! fn rxc(ctx: rxc::Context) {
//!     ctx.resources.opt_future.as_mut().unwrap().recv();
//! }
//!
//! //...
//! ```
//!
//! [`start`]: SpiFuture::start
//! [`poll`]: SpiFuture::poll
//! [`free`]: SpiFuture::free
//! [`send`]: SpiFuture::send
//! [`recv`]: SpiFuture::recv
//! [`Spi`]: super::spi::Spi
//! [`CharSize`]: super::spi::CharSize
//! [`CharSize::Word`]: super::spi::CharSize::Word
//! [`EightBit`]: super::spi::EightBit
//! [`NineBit`]: super::spi::NineBit
//! [`Length`]: super::spi::Length
//! [`Pin`]: crate::gpio::v2::pin::Pin
//! [`Future`]: core::future::Future
//! [`Waker`]: core::task::Waker
//! [`Poll`]: core::task::Poll
//! [RTIC]: https://rtic.rs/

use core::convert::Infallible;
use core::task::Poll;

use embedded_hal::digital::v2::OutputPin;

use crate::gpio::v2::pin::{OptionalPin, SomePin};
use crate::typelevel::NoneT;

use super::spi::{AnySpi, Error, Flags};

#[cfg(feature = "min-samd51g")]
use {
    super::spi::{
        Capability, Config, DynLength, OpMode, Spi, StaticLength, ValidConfig, ValidPads,
    },
    typenum::Unsigned,
};

#[cfg(any(feature = "samd11", feature = "samd21"))]
use core::mem::size_of;

#[cfg(any(feature = "samd11", feature = "samd21"))]
type Data = u16;

#[cfg(feature = "min-samd51g")]
type Data = u32;

//=============================================================================
// CheckBufLen
//=============================================================================

/// Trait used to verify the [`SpiFuture`] buffer length
pub trait CheckBufLen: AnySpi {
    #[cfg(feature = "min-samd51g")]
    /// [`Spi`] transaction length
    ///
    /// This value is zero for an [`Spi`] with [`DynLength`]
    const LEN: usize = <Self::Size as Unsigned>::USIZE;

    #[cfg(any(feature = "samd11", feature = "samd21"))]
    /// [`Spi`] transaction length
    ///
    /// [`Spi`]: super::spi::Spi
    const LEN: usize = size_of::<Self::Word>();

    /// Return the [`Spi`] transaction length
    ///
    /// In most cases, this returns the corresponding constant. For SAMx5x chips
    /// with [`DynLength`], this returns the result of [`Spi::get_dyn_length`].
    ///
    /// [`Spi`]: super::spi::Spi
    #[inline]
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Step size through the [`SpiFuture`] buffer
    ///
    /// This is equal to the number of bytes sent in each write to the SPI DATA
    /// register. It is zero for an [`Spi`] with [`DynLength`].
    ///
    /// [`Spi`]: super::spi::Spi
    const STEP: usize = if Self::LEN > 4 { 4 } else { Self::LEN };

    /// Return the step size through the [`SpiFuture`] buffer
    ///
    /// In most cases, this returns the corresponding constant. For SAMx5x chips
    /// with [`DynLength`], this returns a result calculated from [`Self::len`].
    #[inline]
    fn step(&self) -> usize {
        Self::STEP
    }

    /// Check that the buffer has a valid length
    ///
    /// If the transaction length is greater than four, then the size of the
    /// buffer must equal the transaction length. If the transaction length is
    /// less than or equal to four, then the size of the buffer must be an
    /// integer multiple of the transaction length.
    ///
    /// Both of these statements apply regardless of whether the [`Spi`] has a
    /// [`DynLength`].
    ///
    /// [`Spi`]: super::spi::Spi
    #[inline]
    fn check_buf_len(&self, buf: &impl AsRef<[u8]>) {
        let buf_len = buf.as_ref().len();
        let self_len = self.len();
        if (self_len > 4 && buf_len != self_len) || (self_len <= 4 && buf_len % self_len != 0) {
            panic!("Invalid SpiFuture buffer length");
        }
    }
}

#[cfg(any(feature = "samd11", feature = "samd21"))]
impl<S: AnySpi> CheckBufLen for S {}

#[cfg(feature = "min-samd51g")]
impl<P, M, L, A> CheckBufLen for Spi<Config<P, M, L>, A>
where
    Config<P, M, L>: ValidConfig,
    P: ValidPads,
    M: OpMode,
    L: StaticLength,
    A: Capability,
{
}

#[cfg(feature = "min-samd51g")]
impl<P, M, A> CheckBufLen for Spi<Config<P, M, DynLength>, A>
where
    Config<P, M, DynLength>: ValidConfig,
    P: ValidPads,
    M: OpMode,
    A: Capability,
{
    #[inline]
    fn len(&self) -> usize {
        self.get_dyn_length() as usize
    }

    #[inline]
    fn step(&self) -> usize {
        let len = self.len();
        if len > 4 {
            4
        } else {
            len
        }
    }
}

//=============================================================================
// ControlSS
//=============================================================================

/// Trait used to control the SS line during an [`SpiFuture`] transaction
pub trait ControlSS: OptionalPin {
    /// If an SS pin is present, assert it by bringing it low
    fn assert(&mut self);

    /// If an SS pin is present, deassert it by bringing it high
    fn deassert(&mut self);
}

impl<P> ControlSS for P
where
    P: SomePin + OutputPin<Error = Infallible>,
{
    #[inline]
    fn assert(&mut self) {
        self.set_low().unwrap();
    }

    #[inline]
    fn deassert(&mut self) {
        self.set_high().unwrap();
    }
}

impl ControlSS for NoneT {
    fn assert(&mut self) {}
    fn deassert(&mut self) {}
}

//=============================================================================
// SpiFuture
//=============================================================================

/// A [`Future`]-like interface for SPI transactions
///
/// See the [module-level](self) documentation for more details.
///
/// [`Future`]: core::future::Future
pub struct SpiFuture<S, B, SS = NoneT, W = fn()>
where
    S: CheckBufLen,
    B: AsRef<[u8]> + AsMut<[u8]> + 'static,
    SS: ControlSS,
    W: FnOnce() + 'static,
{
    spi: S,
    buf: B,
    sent: usize,
    rcvd: usize,
    ss: SS,
    waker: Option<W>,
}

impl<S, B> SpiFuture<S, B>
where
    S: CheckBufLen,
    B: AsRef<[u8]> + AsMut<[u8]> + 'static,
{
    /// Create a new [`SpiFuture`] with no SS pin or waker
    #[inline]
    pub fn new(spi: S, buf: B) -> Self {
        spi.check_buf_len(&buf);
        SpiFuture {
            spi,
            buf,
            sent: 0,
            rcvd: 0,
            ss: NoneT,
            waker: None,
        }
    }
}

impl<S, B, W> SpiFuture<S, B, NoneT, W>
where
    S: CheckBufLen,
    B: AsRef<[u8]> + AsMut<[u8]> + 'static,
    W: FnOnce() + 'static,
{
    /// Add an SS pin to the [`SpiFuture`]
    ///
    /// This function changes the `SS` type, so it must take an owned `self`.
    #[inline]
    pub fn with_ss<SS>(self, pin: SS) -> SpiFuture<S, B, SS, W>
    where
        SS: SomePin + OutputPin<Error = Infallible>,
    {
        SpiFuture {
            spi: self.spi,
            buf: self.buf,
            sent: self.sent,
            rcvd: self.rcvd,
            ss: pin,
            waker: self.waker,
        }
    }
}

impl<S, B, SS> SpiFuture<S, B, SS>
where
    S: CheckBufLen,
    B: AsRef<[u8]> + AsMut<[u8]> + 'static,
    SS: ControlSS,
{
    /// Add a waker to the [`SpiFuture`]
    ///
    /// This function changes the waker type, so it must take an owned `self`.
    #[inline]
    pub fn with_waker<W>(self, waker: W) -> SpiFuture<S, B, SS, W>
    where
        W: FnOnce() + 'static,
    {
        SpiFuture {
            spi: self.spi,
            buf: self.buf,
            sent: self.sent,
            rcvd: self.rcvd,
            ss: self.ss,
            waker: Some(waker),
        }
    }
}

impl<S, B, SS, W> SpiFuture<S, B, SS, W>
where
    S: CheckBufLen,
    B: AsRef<[u8]> + AsMut<[u8]> + 'static,
    SS: ControlSS,
    W: FnOnce() + 'static,
{
    /// Start the [`SpiFuture`] transaction
    ///
    /// This will assert the SS pin, if present, and enable the `DRE` and `RXC`
    /// interrupts.
    #[inline]
    pub fn start(&mut self) {
        self.ss.assert();
        self.spi.as_mut().enable_interrupts(Flags::DRE | Flags::RXC);
    }

    /// Send the next set of bytes from the buffer
    ///
    /// This method should be called from the `DRE` interrupt handler. Once all
    /// bytes of the transaction have been sent, this function will
    /// automatically disable the `DRE` interrupt.
    pub fn send(&mut self) -> Result<(), Error> {
        let _ = self.spi.as_ref().read_flags_errors()?;
        let buf = self.buf.as_ref();
        if let Some(buf) = buf.get(self.sent..) {
            let mut data = buf.into_iter();
            let mut bytes = [0; 4];
            let mut iter = bytes.iter_mut();
            for _ in 0..self.spi.step() {
                match (iter.next(), data.next()) {
                    (Some(b), Some(d)) => *b = *d,
                    _ => break,
                }
            }
            let word = u32::from_le_bytes(bytes);
            unsafe { self.spi.as_mut().write_data(word as Data) };
            self.sent += self.spi.step();
        }
        if self.sent >= buf.len() {
            self.spi.as_mut().disable_interrupts(Flags::DRE);
        }
        Ok(())
    }

    /// Received the next set of bytes and write them to the buffer
    ///
    /// This method should be called from the `RXC` interrupt handler. Once all
    /// bytes of the transaction have been received, this function will
    /// automatically disable the `RXC` interrupt, deassert the SS pin (if
    /// present), and call the waker (if present).
    pub fn recv(&mut self) -> Result<(), Error> {
        let _ = self.spi.as_ref().read_flags_errors()?;
        let buf = self.buf.as_mut();
        if self.rcvd < self.sent {
            let buf = unsafe { buf.get_unchecked_mut(self.rcvd..) };
            let mut data = buf.into_iter();
            let word = unsafe { self.spi.as_mut().read_data() as u32 };
            let bytes = word.to_le_bytes();
            let mut iter = bytes.iter();
            for _ in 0..self.spi.step() {
                match (data.next(), iter.next()) {
                    (Some(d), Some(b)) => *d = *b,
                    _ => break,
                }
            }
            self.rcvd += self.spi.step();
        }
        if self.rcvd >= buf.len() {
            self.spi.as_mut().disable_interrupts(Flags::RXC);
            self.ss.deassert();
            if let Some(waker) = self.waker.take() {
                waker();
            }
        }
        Ok(())
    }

    /// Poll the [`SpiFuture`]
    ///
    /// This function will return [`Poll::Pending`] until all bytes have been
    /// received. When the transaction is complete, it will return
    /// [`Poll::Ready`] with a reference to the completed buffer.
    #[inline]
    pub fn poll(&self) -> Poll<&[u8]> {
        let buf = self.buf.as_ref();
        if self.rcvd >= buf.len() {
            Poll::Ready(buf)
        } else {
            Poll::Pending
        }
    }

    /// Consume the [`SpiFuture`] and free its components
    ///
    /// If the transaction is complete, this function will consume the
    /// [`SpiFuture`] and return the [`Spi`](super::spi::Spi) object, the
    /// buffer, and the SS pin, if present.
    ///
    /// If the transaction is not complete, it will return `Err(self)`.
    #[inline]
    pub fn free(self) -> Result<(S, B, SS), Self> {
        if self.rcvd >= self.buf.as_ref().len() {
            Ok((self.spi, self.buf, self.ss))
        } else {
            Err(self)
        }
    }

    /// Consume the [`SpiFuture`] and free its components without checking for
    /// completion
    ///
    /// Ending the transaction prematurely could leave the [`Spi`] in an
    /// inconsistent state. It is not safe to call this function unless the
    /// transaction is complete.
    ///
    /// [`Spi`]: super::spi::Spi
    #[inline]
    pub unsafe fn free_unchecked(self) -> (S, B, SS) {
        (self.spi, self.buf, self.ss)
    }
}
