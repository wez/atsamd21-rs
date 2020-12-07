#![allow(dead_code)]

use crate::target_device::{GMAC, MCLK};
mod buffer;
pub use buffer::*;
mod pad;
pub use pad::*;

pub struct Gmac<P: GmacPadout, RxBuf, TxBuf, RxCallback, TxCallback> {
    _padout: P,
    gmac: GMAC,
    rx_buffers: RxBuf,
    tx_buffers: TxBuf,
    rx_buf_index: usize,
    tx_buf_index: usize,
    last_tx_buf_index: usize,
    rx_callback: Option<RxCallback>,
    tx_callback: Option<TxCallback>,
}

impl<P, RxBuf, TxBuf, RxCallback, TxCallback> Gmac<P, RxBuf, TxBuf, RxCallback, TxCallback>
where
    P: GmacPadout,
    RxBuf: BufferSet<RxBufferDescriptor>,
    TxBuf: BufferSet<TxBufferDescriptor>,
    RxCallback: FnMut(&mut Gmac<P, RxBuf, TxBuf, RxCallback, TxCallback>),
    TxCallback: FnMut(&mut Gmac<P, RxBuf, TxBuf, RxCallback, TxCallback>),
{
    pub fn new(padout: P, mclk: &MCLK, gmac: GMAC, rx_buffers: RxBuf, tx_buffers: TxBuf) -> Self {
        mclk.ahbmask.modify(|_, w| w.gmac_().set_bit());
        mclk.apbcmask.modify(|_, w| w.gmac_().set_bit());

        gmac.ncr.write_with_zero(|w| {
            w.lbl().clear_bit(); // disable loopback local
            w.mpe().set_bit(); // management port enable
            w.westat().clear_bit(); // Disable write for Static Register
            w.bp().clear_bit(); // Disable Back pressure
            w.enpbpr().clear_bit(); // Disable PFC Priority-based Pause Reception
            w.txpbpf().clear_bit() // Disable PFC Priority-based Pause Frame
        });
        unsafe {
            gmac.ncfgr.write_with_zero(|w| {
                w.spd().set_bit(); // 100Mbps Speed
                w.fd().set_bit(); // Enable Full Duplex
                w.dnvlan().clear_bit(); // Do not discard Non-VLAN Frames
                w.jframe().clear_bit(); // Disable Jumbo Frames
                w.caf().clear_bit(); // Disable Copy All Frames
                w.nbc().clear_bit(); // Disable No broadcast
                w.mtihen().clear_bit(); // Disable Multicast Hash Enable
                w.unihen().clear_bit(); // Disable Unicast Hash Enable
                w.maxfs().set_bit(); // 1536 Maximum Frame Size
                w.rty().clear_bit(); // Disable Retry Test
                w.pen().clear_bit(); // Disable Pause Enable
                w.rxbufo().bits(0); // Receive Buffer Offset <0-3>
                w.lferd().clear_bit(); // Disable Length Field Error Frame Discard
                w.rfcs().clear_bit(); // Disable Remove FCS
                w.clk().bits(4) // MDC Clock Division 4 => 64
            });
        }

        match P::MODE {
            GmacMode::Rmii => gmac.ur.write_with_zero(|w| w.mii().clear_bit()),
            GmacMode::Mii => gmac.ur.write_with_zero(|w| w.mii().set_bit()),
        }

        unsafe {
            gmac.dcfgr.write_with_zero(|w| {
                w.fbldo().bits(4); // Fixed Burst Length for DMA Data Operations, 4 => Always use INCR4 AHB bursts
                w.esma().clear_bit(); // Disable Endian Swap Mode Enable for Management Descriptor Accesses
                w.espa().clear_bit(); // Disable Endian Swap Mode Enable for Packet Data Accesses
                w.rxbms().bits(3); // Receiver Packet Buffer Memory Size Select, 3 => 4KB
                w.txpbms().set_bit(); // Transmitter Packet Buffer Memory Size Select, 1 => 4KB
                w.txcoen().clear_bit(); // Disable Transmitter Checksum Generation Offload Enable
                w.drbs().bits(2); // DMA Receive Buffer Size <1-255>, multiples of 64, 2 => 128B
                w.ddrp().clear_bit() // Disable DMA Discard Received Packets
            });
        }

        // Disable Wake on LAN (for now...)
        gmac.wol.write_with_zero(|w| w);

        gmac.ipgs.write_with_zero(|w| {
            let mul = 1; // IPG Stretch Multiple <0-15>
            let div = 1; // IPG Stretch Divide <1-16>
            unsafe { w.fl().bits(mul << 8 | div) }
        });

        unsafe {
            gmac.tbqb
                .write_with_zero(|w| w.addr().bits(tx_buffers.address()));
            gmac.rbqb
                .write_with_zero(|w| w.addr().bits(rx_buffers.address()));
        }

        // NVIC::mask(Interrupt::GMAC);
        // NVIC::unpend(Interrupt::GMAC);
        // unsafe {
        //     //TODO Should this really be here?
        //     NVIC::unmask(Interrupt::GMAC);
        // }

        Gmac {
            _padout: padout,
            gmac,
            rx_buffers,
            tx_buffers,
            rx_buf_index: 0,
            tx_buf_index: 0,
            last_tx_buf_index: 0,
            rx_callback: None,
            tx_callback: None,
        }
    }

    pub fn deinit(self) -> (P, GMAC, RxBuf, TxBuf) {
        unsafe {
            /* Disable all GMAC Interrupt */
            self.gmac.idr.write_with_zero(|w| w.bits(0xFFFFFFFF));
            /* Disable transmit/receive */
            self.gmac.ncr.write_with_zero(|w| w.bits(0));
        }
        // Disable Interrupt
        //crate::target_device::NVIC::mask(crate::target_device::Interrupt::GMAC);

        (self._padout, self.gmac, self.rx_buffers, self.tx_buffers)
    }

    pub fn set_rx_callback(&mut self, callback: RxCallback) {
        self.rx_callback = Some(callback)
    }
    pub fn set_tx_callback(&mut self, callback: TxCallback) {
        self.tx_callback = Some(callback)
    }

    pub fn enable(&mut self) {
        self.gmac.ncr.modify(|_, w| {
            w.rxen().set_bit();
            w.txen().set_bit()
        });
    }
    pub fn disable(&mut self) {
        self.gmac.ncr.modify(|_, w| {
            w.rxen().clear_bit();
            w.txen().clear_bit()
        });
    }

    pub fn set_mac_filter(&mut self, index: MacFilterIndex, mac: &MacAddress) {
        unsafe {
            let first_part = mac.0.as_ptr() as *const u32;
            let last_part = (mac.0.as_ptr() as *const u16).offset(2);
            self.gmac.sa[index as usize]
                .sab
                .write_with_zero(|w| w.addr().bits(*first_part));
            self.gmac.sa[index as usize]
                .sat
                .write_with_zero(|w| w.addr().bits(*last_part));
            // No TID filter
            self.gmac.tidm[index as usize].write_with_zero(|w| w);
        }
    }
    pub fn set_mac_filter_with_type_id(
        &mut self,
        index: MacFilterIndex,
        mac: &MacAddress,
        type_id: TypeId,
    ) {
        self.set_mac_filter(index, mac);
        unsafe {
            self.gmac.tidm[index as usize].write_with_zero(|w| {
                w.tid().bits(type_id.0);
                w.enid().set_bit()
            });
        }
    }

    /// # Safety
    /// This function can write any data to any PHY register. Extreme caution
    /// must be taken to not write invalid data!
    pub unsafe fn write_phy_reg(&mut self, addr: u8, reg: u8, data: u16) {
        self.gmac.ncr.modify(|_, w| w.mpe().set_bit());
        self.gmac.man.write_with_zero(|w| {
            w.op().bits(1); /* 0x01 write operation */
            w.cltto().set_bit(); /* Clause 22/45 operation */
            w.wtn().bits(2); /* Must be written to 0x2 */
            w.phya().bits(addr);
            w.rega().bits(reg);
            w.data().bits(data)
        });
        /* Wait for the write operation complete */
        while self.gmac.nsr.read().idle().bit_is_clear() {}
        self.gmac.ncr.modify(|_, w| w.mpe().clear_bit());
    }
    pub fn read_phy_reg(&mut self, addr: u8, reg: u8) -> u16 {
        self.gmac.ncr.modify(|_, w| w.mpe().set_bit());
        unsafe {
            self.gmac.man.write_with_zero(|w| {
                w.op().bits(2); /* 0x02 read operation */
                w.cltto().set_bit(); /* Clause 22/45 operation */
                w.wtn().bits(2); /* Must be written to 0x2 */
                w.phya().bits(addr);
                w.rega().bits(reg)
            });
        }
        /* Wait for the read operation complete */
        while self.gmac.nsr.read().idle().bit_is_clear() {}

        let data = self.gmac.man.read().data().bits();

        self.gmac.ncr.modify(|_, w| w.mpe().clear_bit());

        data
    }

    pub fn write(&mut self, buf: &[u8]) -> nb::Result<(), ()> {
        let last_descriptor = self.tx_buffers.get_descriptor(self.last_tx_buf_index);
        if last_descriptor.used() && !last_descriptor.last_buf() {
            // Set used flag from first descriptor to last descriptor, as DMA
            // only sets the first used flag when it's done
            for idx in 0..TxBuf::COUNT {
                let mut pos = self.last_tx_buf_index + idx;
                if pos >= TxBuf::COUNT {
                    pos -= TxBuf::COUNT;
                }
                let descriptor = self.tx_buffers.get_descriptor_mut(pos);
                descriptor.set_used();
                if descriptor.last_buf() {
                    break;
                }
            }
        }

        if !self.tx_buffers.get_descriptor(self.tx_buf_index).used() {
            // All buffers are full
            return Err(nb::Error::WouldBlock);
        }

        /* Check if have enough buffers, the first buffer already checked */
        if buf.len() > TxBuf::SIZE {
            for idx in 0..TxBuf::COUNT {
                let mut pos = self.tx_buf_index + idx;
                if pos >= TxBuf::COUNT {
                    pos -= TxBuf::COUNT;
                }

                let descriptor = self.tx_buffers.get_descriptor(pos);
                if !descriptor.used() {
                    return Err(nb::Error::WouldBlock);
                }

                if buf.len() - (TxBuf::SIZE * idx) < TxBuf::SIZE {
                    break;
                }
            }
        }
        self.last_tx_buf_index = self.tx_buf_index;

        /* Write data to transmit buffer */
        let mut len = buf.len();
        let mut buf = buf;
        for _ in 0..TxBuf::COUNT {
            let blen = len.min(TxBuf::SIZE);
            self.tx_buffers
                .get_buffer_mut(self.tx_buf_index)
                .copy_from_slice(&buf[..blen]);
            len -= blen;
            buf = &buf[blen..];

            let descriptor = self.tx_buffers.get_descriptor_mut(self.tx_buf_index);
            // This is probably ok to do here, because the docs state
            // (24.6.3.4 Transmit AHB Buffers):
            //
            // If transmission stops due to a transmit error or a used bit being read,
            // transmission restarts from the first buffer descriptor of the
            // frame being transmitted when the transmit start bit is rewritten.
            //
            // Therefore it is not a problem if he GMAC reads this partial frame,
            // because it will retry if it failed when NCR.TSART is written
            if len > 0 {
                /* Here the Used flag is set to zero */
                descriptor.reset_with_len(blen, false);
            } else {
                /* Here the Used flag is set to zero */
                descriptor.reset_with_len(blen, true);
            }

            self.tx_buf_index += 1;
            if self.tx_buf_index == TxBuf::COUNT {
                self.tx_buf_index = 0;
                // Wrap is set on init and does not need to be set here
                // self.tx_buffers
                //     .get_descriptor_mut(TxBuf::COUNT - 1)
                //     .set_wrap(true);
            }
            if len == 0 {
                // Whole frame written
                break;
            }
        }

        /* Data synchronization barrier */
        cortex_m::asm::dsb();

        /* Active Transmit */
        self.gmac.ncr.modify(|_, w| w.tstart().set_bit());

        Ok(())
    }

    /// Read raw data from MAC
    pub fn read(&mut self, buf: &mut [u8]) -> nb::Result<usize, ()> {
        let mut sof = None; // Start of Frame index
        let mut eof = None; // End of Frame index
        let mut total_len = 0; // Total length of received package
        let mut len = 0;
        let mut last_idx = 0;
        for idx in 0..RxBuf::COUNT {
            last_idx = idx;
            let mut pos = self.rx_buf_index + idx;
            if pos >= RxBuf::COUNT {
                pos -= RxBuf::COUNT;
            }
            let descriptor = self.rx_buffers.get_descriptor(pos);
            if !descriptor.ownership() {
                /* No more data for Ethernet package */
                break;
            }
            if descriptor.sof() {
                sof = Some(idx);
            }
            if descriptor.eof() && sof.is_some() {
                /* eof now indicates the number of bufs the frame used */
                eof = Some(idx);
                /* Length of the whole frame is written to the last descriptor */
                len = (descriptor.len() as usize).min(buf.len());
                /* Break process since the last data has been found */
                break;
            }
        }

        let j = if let Some(eof) = eof {
            eof + 1
        } else if let Some(sof) = sof {
            sof
        } else {
            last_idx
        };

        let mut buf = &mut buf[..];
        for i in 0..j {
            if let Some(eof) = eof {
                if i >= sof.unwrap_or(usize::MAX) && i <= eof && len > 0 {
                    let n = len.min(RxBuf::SIZE);
                    let buffer = self.rx_buffers.get_buffer(self.rx_buf_index);
                    buf.copy_from_slice(&buffer[..n]);
                    buf = &mut buf[n..];
                    total_len += n;
                    len -= n;
                }
            }
            self.rx_buffers
                .get_descriptor_mut(self.rx_buf_index)
                .set_ownership(false);
            self.rx_buf_index += 1;
            if self.rx_buf_index == RxBuf::COUNT {
                self.rx_buf_index = 0;
            }
        }

        Ok(total_len)
    }
    /// Get next valid package length from the MAC
    ///
    /// The application can use this function to fetch the length of the next
    /// package, allocate a buffer with this length, and then invoke `read()`
    /// to read out the package data.
    pub fn get_read_len(&mut self) -> usize {
        let mut total_len = 0usize;
        let mut sof = false;
        for idx in 0..RxBuf::COUNT {
            let mut pos = self.rx_buf_index + idx;
            if pos >= RxBuf::COUNT {
                pos -= RxBuf::COUNT;
            }
            let descriptor = self.rx_buffers.get_descriptor(pos);
            if !descriptor.ownership() {
                /* No more data for Ethernet package */
                break;
            }
            if descriptor.sof() {
                sof = true;
            }
            if sof {
                total_len += descriptor.len() as usize;
            }
            if descriptor.eof() {
                /* Break process since the last data has been found */
                break;
            }
        }
        total_len
    }

    pub fn gmac_handler(&mut self) {
        let tsr = self.gmac.tsr.read().bits();
        let rsr = self.gmac.rsr.read().bits();
        // Must be Clear ISR (Clear on read)
        self.gmac.isr.read().bits();

        if (tsr & (1 << 5)) == 1 << 5 {
            // Frame transmited
            unsafe {
                self.gmac.tsr.write_with_zero(|w| w.bits(tsr));
            }
            if self.tx_buffers.get_descriptor(self.tx_buf_index).used() {
                if let Some(mut cb) = self.tx_callback.take() {
                    // Call transmitted callback
                    cb(self);
                    self.tx_callback = Some(cb);
                }
            }
        }

        if (rsr & (1 << 1)) == 1 << 1 {
            // Frame received
            if let Some(mut cb) = self.rx_callback.take() {
                // Call received callback
                cb(self);
                self.rx_callback = Some(cb);
            }
        }
        unsafe {
            self.gmac.rsr.write_with_zero(|w| w.bits(rsr));
        }
    }
}

#[derive(Copy, Clone)]
pub enum MacFilterIndex {
    Filter0 = 0,
    Filter1 = 1,
    Filter2 = 2,
    Filter3 = 3,
}
#[derive(Copy, Clone)]
pub struct MacAddress(pub [u8; 6]);
#[derive(Copy, Clone)]
pub struct TypeId(pub u16);

impl From<[u8; 6]> for MacAddress {
    fn from(addr: [u8; 6]) -> Self {
        MacAddress(addr)
    }
}
impl TypeId {
    pub fn ip() -> Self {
        TypeId(0x0600)
    }
    pub fn vlan() -> Self {
        TypeId(0x8100)
    }
}

mod private {
    use generic_array::ArrayLength;
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl Sealed for super::RxBufferDescriptor {}
    impl Sealed for super::TxBufferDescriptor {}
    impl<
            T: super::BufferDescriptor,
            Count: ArrayLength<T> + ArrayLength<super::GmacBuffer<Size>>,
            Size: ArrayLength<u8>,
        > Sealed for super::GmacBufferSet<T, Count, Size>
    {
    }
}
