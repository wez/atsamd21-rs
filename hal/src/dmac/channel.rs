//! # Abstractions over individual DMA channels
//!
//! # Initializing
//!
//! Individual channels should be initialized through the
//! [`Channel::init`] method. This will return a `Channel<Id, Ready>` ready for
//! use by a [`Transfer`](super::transfer::Transfer). Initializing a channel
//! requires setting a priority level, as well as enabling or disabling
//! interrupt requests (only for the specific channel being initialized).
//!
//! # Burst Length and FIFO Threshold (SAMD51/SAME5x only)
//!
//! The transfer burst length can be configured through the
//! [`Channel::burst_length`] method. A burst is an atomic,
//! uninterruptible transfer which length corresponds to a number of beats. See
//! SAMD5x/E5x datasheet section 22.6.1.1 for more information. The FIFO
//! threshold can be configured through the
//! [`Channel::fifo_threshold`] method. This enables the channel
//! to wait for multiple Beats before sending a Burst. See SAMD5x/E5x datasheet
//! section 22.6.2.8 for more information.
//!
//! # Channel status
//!
//! Channels can be in any of three statuses: [`Uninitialized`], [`Ready`], and
//! [`Busy`]. These statuses are checked at compile time to ensure they are
//! properly initialized before launching DMA transfers.
//!
//! # Resetting
//!
//! Calling the [`Channel::reset`] method will reset the channel to its
//! `Uninitialized` state. You will be required to call [`Channel::init`]
//! again before being able to use it with a `Transfer`.

use super::dma_controller::{ChId, PriorityLevel, TriggerAction, TriggerSource};
use crate::{
    target_device::{self, DMAC},
    typelevel::{Is, Sealed},
};

use core::{marker::PhantomData, mem};
use modular_bitfield::prelude::*;
use target_device::Peripherals;

#[cfg(feature = "min-samd51g")]
use super::dma_controller::{BurstLength, FifoThreshold};

#[cfg(feature = "min-samd51g")]
use crate::target_device::dmac::CHANNEL;

//==============================================================================
// Channel Status
//==============================================================================
pub trait Status: Sealed {}

/// Uninitialized channel
pub enum Uninitialized {}
impl Sealed for Uninitialized {}
impl Status for Uninitialized {}
/// Initialized and ready to transfer channel
pub enum Ready {}
impl Sealed for Ready {}
impl Status for Ready {}
/// Busy channel
pub enum Busy {}
impl Sealed for Busy {}
impl Status for Busy {}

//==============================================================================
// AnyChannel
//==============================================================================
pub trait AnyChannel: Sealed + Is<Type = SpecificChannel<Self>> {
    type Status: Status;
    type Id: ChId;
}

pub type SpecificChannel<C> = Channel<<C as AnyChannel>::Id, <C as AnyChannel>::Status>;

pub type ChannelStatus<C> = <C as AnyChannel>::Status;
pub type ChannelId<C> = <C as AnyChannel>::Id;

impl<Id, S> Sealed for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
}

impl<Id, S> AnyChannel for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
    type Id = Id;
    type Status = S;
}

impl<Id, S> AsRef<Self> for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<Id, S> AsMut<Self> for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

//==============================================================================
// Channel
//==============================================================================
/// DMA channel, capable of executing
/// [`Transfer`](super::transfer::Transfer)s.
pub struct Channel<Id: ChId, S: Status> {
    _id: PhantomData<Id>,
    _status: PhantomData<S>,
}

#[inline]
pub(crate) fn new_chan<Id: ChId>(_id: PhantomData<Id>) -> Channel<Id, Uninitialized> {
    Channel {
        _id,
        _status: PhantomData,
    }
}

/// These methods may be used on any DMA channel in any configuration
impl<Id: ChId, S: Status> Channel<Id, S> {
    /// Set channel ID and run the closure. A closure is needed to ensure
    /// the registers are accessed in an interrupt-safe way, as the SAMD21
    /// DMAC is a little funky - It requires setting the channel number in
    /// the CHID register, then access the channel control registers.
    /// If an interrupt were to change the CHID register, we would be faced
    /// with undefined behaviour.
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    fn with_chid<F: FnMut(&DMAC)>(&mut self, mut fun: F) {
        cortex_m::interrupt::free(|_| {
            // SAFETY: This is ONLY safe if the individual channels are GUARANTEED not to
            // mess with either:
            // - The global DMAC configuration
            // - The configuration of other channels.
            //
            // In practice, this means that the DMAC registers should only be accessed
            // through the `with_chid` method.
            let dmac = unsafe {
                let dmac = Peripherals::steal().DMAC;
                dmac.chid.modify(|_, w| w.id().bits(Id::U8));
                dmac
            };

            fun(&dmac);
        });
    }

    /// Set channel ID and run the closure. A closure is needed to ensure
    /// the registers are accessed in an interrupt-safe way, as the SAMD21
    /// DMAC is a little funky. For the SAMD51/SAMEx, we simply take a reference
    /// to the correct channel number and run the closure on that.
    #[cfg(feature = "min-samd51g")]
    #[inline]
    fn with_chid<F: FnMut(&CHANNEL)>(&mut self, fun: F) {
        // SAFETY: This is ONLY safe if the individual channels are GUARANTEED not to
        // mess with either:
        // - The global DMAC configuration
        // - The configuration of other channels.
        //
        // In practice, this means that the DMAC registers should only be accessed
        // through the `with_chid` method.
        let dmac = unsafe { Peripherals::steal().DMAC };
        let mut ch = dmac.channel[Id::USIZE];
        fun(&mut ch);
    }

    /// Configure the DMA channel so that it is ready to be used by a
    /// [`Transfer`](super::transfer::Transfer).
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status
    #[inline]
    pub fn init(mut self, lvl: PriorityLevel) -> Channel<Id, Ready> {
        // Software reset the channel for good measure
        self._reset_private();

        self.with_chid(|d| {
            #[cfg(any(feature = "samd11", feature = "samd21"))]
            // Setup priority level
            d.chctrlb.modify(|_, w| w.lvl().bits(lvl as u8));

            #[cfg(feature = "min-samd51g")]
            d.chprilvl.modify(|_, w| w.prilvl().bits(lvl as u8));
        });

        Channel {
            _id: self._id,
            _status: PhantomData,
        }
    }

    #[inline]
    pub fn enable_interrupts(&mut self, flags: InterruptFlags) {
        // SAFETY: This is safe as InterruptFlags is only capable of writing in
        // non-reserved bits
        self.with_chid(|d| d.chintenset.write(|w| unsafe { w.bits(flags.into()) }))
    }

    #[inline]
    pub fn disable_interrupts(&mut self, flags: InterruptFlags) {
        // SAFETY: This is safe as InterruptFlags is only capable of writing in
        // non-reserved bits
        self.with_chid(|d| d.chintenclr.write(|w| unsafe { w.bits(flags.into()) }))
    }

    #[inline]
    fn _reset_private(&mut self) {
        self.with_chid(|d| {
            // Reset the channel to its startup state and wait for reset to complete
            d.chctrla.modify(|_, w| w.swrst().set_bit());
            while d.chctrla.read().swrst().bit_is_set() {}
        })
    }

    #[inline]
    fn _trigger_private(&mut self) {
        // SAFETY: This is safe because we are only writing to a bit that belongs to
        // this channel.
        unsafe {
            Peripherals::steal()
                .DMAC
                .swtrigctrl
                .modify(|_, w| w.bits(1 << Id::U8));
        }
    }
}

/// These methods may only be used on a `Ready` DMA channel
impl<Id: ChId> Channel<Id, Ready> {
    /// Issue a software reset to the channel. This will return the channel to
    /// its startup state
    #[inline]
    pub fn reset(mut self) -> Channel<Id, Uninitialized> {
        self._reset_private();

        Channel {
            _id: self._id,
            _status: PhantomData,
        }
    }

    /// Set the FIFO threshold length. The channel will wait until it has
    /// received the selected number of Beats before triggering the Burst
    /// transfer, reducing the DMA transfer latency.
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn fifo_threshold(&mut self, threshold: FifoThreshold) {
        self.with_chid(|d| {
            d.chctrla.modify(|_, w| w.threshold().bits(threshold as u8));
        })
    }

    /// Set burst length for the channel, in number of beats. A burst transfer
    /// is an atomic, uninterruptible operation.
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn burst_length(&mut self, burst_length: BurstLength) {
        self.with_chid(|d| {
            d.chctrla
                .modify(|_, w| w.burstlen().bits(burst_length as u8));
        })
    }

    /// Start transfer on channel using the specified trigger source.
    ///
    /// # Return
    ///
    /// A `Channel` with a `Busy` status.
    #[inline]
    pub(crate) fn start(
        mut self,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) -> Channel<Id, Busy> {
        // Set the channel ID. We assume the CHID register doesn't change
        // for the duration of this function.
        self.with_chid(|d| {
            // Configure the trigger source and trigger action
            // SAFETY: This is actually safe because we are writing the correct enum value
            // (imported from the PAC) into the register

            #[cfg(any(feature = "samd11", feature = "samd21"))]
            let trigger_channel = &d.chctrlb;

            #[cfg(feature = "min-samd51g")]
            let trigger_channel = &d.chctrla;

            // SAFETY: This is safe as we only write valid bits into the registers because
            // of TriggerSource and TriggerAction.
            unsafe {
                trigger_channel.modify(|_, w| {
                    w.trigsrc().bits(trig_src as u8);
                    w.trigact().bits(trig_act as u8)
                });
            }

            // Start channel
            d.chctrla.modify(|_, w| w.enable().set_bit());
        });

        // If trigger source is DISABLE, manually trigger transfer
        if trig_src == TriggerSource::DISABLE {
            self._trigger_private();
        }

        Channel {
            _id: self._id,
            _status: PhantomData,
        }
    }
}

/// These methods may only be used on a `Busy` DMA channel
impl<Id: ChId> Channel<Id, Busy> {
    /// Issue a software trigger to the channel
    #[inline]
    pub(crate) fn software_trigger(&mut self) {
        self._trigger_private();
    }

    /// Stop transfer on channel whether or not the transfer has completed
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status, ready to be reused by a new
    /// [`Transfer`](super::transfer::Transfer)
    #[inline]
    pub(crate) fn stop(mut self) -> Channel<Id, Ready> {
        self.with_chid(|d| d.chctrla.modify(|_, w| w.enable().clear_bit()));
        self.free()
    }

    /// Returns whether or not the transfer is complete.
    ///
    /// BUSYCH is set when the channel is ACTIVELY transferring;
    /// PENDCH is set when a trigger request has been received
    /// but the transfer hasn't been started yet.
    /// Therefore, when a trigger request is issued, PENDCH will be set first,
    /// then when the arbiter begins to service the channel, PENDCH is cleared
    /// and BUSYCH is set. To make sure the transfer is actually complete, the
    /// channel needs to be both NOT PENDING and NOT BUSY.
    #[inline]
    pub(crate) fn xfer_complete(&self) -> bool {
        let id = Id::U8;
        // SAFETY: This is safe as we only read bits that belong to this channel.
        let dmac = unsafe { Peripherals::steal().DMAC };
        dmac.busych.read().bits() & (1 << id) == 0 && dmac.pendch.read().bits() & (1 << id) == 0
    }

    /// Wait for the channel to clear its busy status, then release the channel.
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status, ready to be reused by a new
    /// [`Transfer`](super::transfer::Transfer)
    #[inline]
    pub(crate) fn free(self) -> Channel<Id, Ready> {
        while !self.xfer_complete() {}
        Channel {
            _id: self._id,
            _status: PhantomData,
        }
    }

    #[inline]
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    pub(super) fn callback(&mut self) {
        let mut xfer_complete = false;
        self.with_chid(|d| {
            // Transfer complete
            if d.chintflag.read().tcmpl().bit_is_set() {
                // TODO Do something here
                xfer_complete = true;
                d.chintflag.modify(|_, w| w.tcmpl().set_bit());
            }

            // Transfer error
            if d.chintflag.read().terr().bit_is_set() {
                // TODO Do something here
                d.chintflag.modify(|_, w| w.terr().set_bit());
            }

            // Channel suspended
            if d.chintflag.read().susp().bit_is_set() {
                // TODO Do something here
                d.chintflag.modify(|_, w| w.susp().set_bit());
            }
        });
    }
}

/// Interrupt sources available to a DMA channel
#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
pub struct InterruptFlags {
    /// Transfer error
    pub terr: bool,
    /// Transfer complete
    pub tcmpl: bool,
    /// Transfer suspended
    pub susp: bool,
    #[skip]
    _reserved: B5,
}
