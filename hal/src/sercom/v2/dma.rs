//! Use the DMA Controller to perform transfers using the SERCOM peripheral
//!
//! See the [`mod@uart`] and [`mod@spi`] modules for the corresponding DMA
//! transfer implementations.

use crate::{
    dmac::{
        self,
        channel::{AnyChannel, Busy, CallbackStatus, Channel, InterruptFlags, Ready},
        transfer::BufferPair,
        Beat, Buffer, Transfer, TriggerAction,
    },
    sercom::v2::{
        spi::{self, AnySpi, Spi},
        uart::{self, Capability, Receive, Transmit, Uart},
        Sercom,
    },
};

//=============================================================================
// UART DMA transfers
//=============================================================================
unsafe impl<C, D> Buffer for Uart<C, D>
where
    C: uart::ValidConfig,
    C::Word: Beat,
    D: Capability,
{
    type Beat = C::Word;

    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        self.data_ptr()
    }

    #[inline]
    fn incrementing(&self) -> bool {
        false
    }

    #[inline]
    fn buffer_len(&self) -> usize {
        1
    }
}

impl<C, D> Uart<C, D>
where
    Self: Buffer<Beat = C::Word>,
    C: uart::ValidConfig,
    D: Receive,
{
    /// Transform an [`Uart`] into a DMA [`Transfer`]) and
    /// start receiving into the provided buffer.
    #[inline]
    pub fn receive_with_dma<Ch, B, W>(
        self,
        buf: B,
        mut channel: Ch,
        waker: W,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<Self, B>, W>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = C::Word> + 'static,
        W: FnOnce(CallbackStatus) + 'static,
    {
        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[cfg(feature = "min-samd51g")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(any(feature = "samd11", feature = "samd21"))]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: We use new_unchecked to avoid having to pass a 'static self as the
        // destination buffer. This is safe as long as we guarantee the destination
        // buffer is static.
        unsafe { dmac::Transfer::new_unchecked(channel, self, buf, false) }
            .with_waker(waker)
            .begin(C::Sercom::DMA_RX_TRIGGER, trigger_action)
    }
}

impl<C, D> Uart<C, D>
where
    Self: Buffer<Beat = C::Word>,
    C: uart::ValidConfig,
    D: Transmit,
{
    /// Transform an [`Uart`] into a DMA [`Transfer`]) and
    /// start sending the provided buffer.
    #[inline]
    pub fn send_with_dma<Ch, B, W>(
        self,
        buf: B,
        mut channel: Ch,
        waker: W,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<B, Self>, W>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = C::Word> + 'static,
        W: FnOnce(CallbackStatus) + 'static,
    {
        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[cfg(feature = "min-samd51g")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(any(feature = "samd11", feature = "samd21"))]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: We use new_unchecked to avoid having to pass a 'static self as the
        // destination buffer. This is safe as long as we guarantee the source buffer is
        // static.
        unsafe { dmac::Transfer::new_unchecked(channel, buf, self, false) }
            .with_waker(waker)
            .begin(C::Sercom::DMA_TX_TRIGGER, trigger_action)
    }
}

//=============================================================================
// SPI DMA transfers
//=============================================================================

#[cfg(feature = "min-samd51g")]
#[doc(hidden)]
pub trait CharSize: spi::AtomicLength {}

#[cfg(feature = "min-samd51g")]
impl<T: spi::AtomicLength> CharSize for T {}

#[cfg(any(feature = "samd11", feature = "samd21"))]
#[doc(hidden)]
pub trait CharSize: spi::CharSize {}

#[cfg(any(feature = "samd11", feature = "samd21"))]
impl<T: spi::CharSize> CharSize for T {}

unsafe impl<P, M, L> Buffer for Spi<spi::Config<P, M, L>>
where
    spi::Config<P, M, L>: spi::ValidConfig,
    P: spi::ValidPads,
    M: spi::MasterMode,
    L: CharSize,
    L::Word: Beat,
{
    type Beat = L::Word;

    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        unsafe {
            #[cfg(feature = "min-samd51g")]
            {
                self.sercom().spim().data.as_ptr() as *mut _
            }

            #[cfg(any(feature = "samd11", feature = "samd21"))]
            {
                self.sercom().spi().data.as_ptr() as *mut _
            }
        }
    }

    #[inline]
    fn incrementing(&self) -> bool {
        false
    }

    #[inline]
    fn buffer_len(&self) -> usize {
        1
    }
}

impl<P, M, L> Spi<spi::Config<P, M, L>>
where
    Self: Buffer<Beat = L::Word>,
    spi::Config<P, M, L>: spi::ValidConfig,
    P: spi::Tx,
    M: spi::MasterMode,
    L: CharSize,
    L::Word: Beat,
{
    /// Transform an [`Spi`] into a DMA [`Transfer`]) and
    /// start a send transaction.
    #[inline]
    pub fn send_with_dma<Ch, B, W>(
        self,
        buf: B,
        mut channel: Ch,
        waker: W,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<B, Self>, W>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = L::Word> + 'static,
        W: FnOnce(CallbackStatus) + 'static,
    {
        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[cfg(feature = "min-samd51g")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(any(feature = "samd11", feature = "samd21"))]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: We use new_unchecked to avoid having to pass a 'static self as the
        // destination buffer. This is safe as long as we guarantee the source buffer is
        // static.
        unsafe { Transfer::new_unchecked(channel, buf, self, false) }
            .with_waker(waker)
            .begin(<Self as AnySpi>::Sercom::DMA_TX_TRIGGER, trigger_action)
    }
}

impl<P, M, L> Spi<spi::Config<P, M, L>>
where
    Self: Buffer<Beat = L::Word>,
    spi::Config<P, M, L>: spi::ValidConfig,
    P: spi::Rx,
    M: spi::MasterMode,
    L: CharSize,
    L::Word: Beat,
{
    /// Transform an [`Spi`] into a DMA [`Transfer`]) and
    /// start a receive transaction.
    #[inline]
    pub fn receive_with_dma<Ch, B, W>(
        self,
        buf: B,
        mut channel: Ch,
        waker: W,
    ) -> Transfer<Channel<Ch::Id, Busy>, BufferPair<Self, B>, W>
    where
        Ch: AnyChannel<Status = Ready>,
        B: Buffer<Beat = L::Word> + 'static,
        W: FnOnce(CallbackStatus) + 'static,
    {
        channel
            .as_mut()
            .enable_interrupts(InterruptFlags::new().with_tcmpl(true));

        #[cfg(feature = "min-samd51g")]
        let trigger_action = TriggerAction::BURST;

        #[cfg(any(feature = "samd11", feature = "samd21"))]
        let trigger_action = TriggerAction::BEAT;

        // SAFETY: We use new_unchecked to avoid having to pass a 'static self as the
        // destination buffer. This is safe as long as we guarantee the destination
        // buffer is static.
        unsafe { Transfer::new_unchecked(channel, self, buf, false) }
            .with_waker(waker)
            .begin(<Self as AnySpi>::Sercom::DMA_RX_TRIGGER, trigger_action)
    }
}
