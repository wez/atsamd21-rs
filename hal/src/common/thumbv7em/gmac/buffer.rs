#![allow(dead_code)]

use super::private;
use generic_array::{ArrayLength, GenericArray};
use vcell::VolatileCell;

#[repr(C, align(8))]
pub struct TxBufferDescriptor {
    address: VolatileCell<u32>,
    status: VolatileCell<u32>,
}
#[allow(clippy::len_without_is_empty)]
impl TxBufferDescriptor {
    pub fn init(&mut self, buf: &[u8], is_last: bool) {
        self.set_address(buf.as_ptr() as u32);
        self.reset_status();
        self.set_wrap(is_last);
    }
    /// Byte address of buffer
    pub fn address(&self) -> u32 {
        self.address.get()
    }
    fn set_address(&mut self, address: u32) {
        self.address.set(address);
    }
    fn set_wrap(&mut self, wrap: bool) {
        if wrap {
            self.address.set(self.address.get() | (1 << 30));
        } else {
            self.address.set(self.address.get() & !(1 << 30));
        }
    }
    pub(crate) fn reset_status(&mut self) {
        self.status.set(1 << 31); // only `used` bit
    }
    pub(crate) fn reset_with_len(&mut self, len: usize, last_buf: bool) {
        self.status.set((len as u32) & ((1 << 14) - 1));
        if last_buf {
            self.address.set(self.address.get() | (1 << 16));
        }
    }
    /// Length of buffer
    pub fn len(&self) -> u16 {
        (self.status.get() & ((1 << 14) - 1)) as u16
    }
    /// Last buffer, when set this bit will indicate the last buffer in the
    /// current frame has been reached.
    pub fn last_buf(&self) -> bool {
        (self.status.get() >> 16) & 1 == 1
    }
    /// No CRC to be appended by MAC. When set, this implies that the data in
    /// the buffers already contains a valid CRC, hence no CRC or padding is to
    /// be appended to the current frame by the MAC. This control bit must be
    /// set for the first buffer in a frame and will be ignored for the
    /// subsequent buffers of aframe.
    ///
    /// Note that this bit must be clear when using the transmit IP/TCP/UDP
    /// checksum generation offload, otherwise checksum generation and
    /// substitution will not occur.
    pub fn no_crc(&self) -> bool {
        (self.status.get() >> 17) & 1 == 1
    }
    /// Transmit IP/TCP/UDP checksum generation offload errors:
    ///
    /// 000: No Error.
    ///
    /// 001: The Packet was identified as a VLAN type, but the header was not
    /// fully complete, or had an error in it.
    ///
    /// 010: The Packet was identified as a SNAP type, but the header was not
    /// fully complete, or had an error in it.
    ///
    /// 011: The Packet was not of an IP type, or the IP packet was invalidly
    /// short, or the IP was not of type IPv4/IPv6.
    ///
    /// 100: The Packet was not identified as VLAN, SNAP or IP.
    ///
    /// 101: Non supported packet fragmentation occurred. For IPv4 packets, the
    /// IP checksum was generated andinserted.
    ///
    /// 110: Packet type detected was not TCP or UDP. TCP/UDP checksum was
    /// therefore not generated. For IPv4packets, the IP checksum was generated
    /// and inserted.
    ///
    /// 111: A premature end of packet was detected and the TCP/UDP checksum
    /// could not be generated
    pub fn checksum_err(&self) -> u8 {
        (self.status.get() >> 20 & 0x7) as u8
    }
    // Late collision, transmit error detected.
    pub fn lco(&self) -> bool {
        (self.status.get() >> 26) & 1 == 1
    }
    /// Transmit frame corruption due to AHB error—set if an error occurs while
    /// midway through reading transmit frame from the AHB, including HRESP
    /// errors and buffers exhausted mid frame (if the buffers run out during
    /// transmission of a frame then transmission stops, FCS shall be bad and
    /// GTXER asserted). Also set if single frame is too large for configured
    /// packet buffer memory size.
    pub fn exhausted(&self) -> bool {
        (self.status.get() >> 27) & 1 == 1
    }
    /// Retry limit exceeded, transmit error detected
    pub fn error(&self) -> bool {
        (self.status.get() >> 29) & 1 == 1
    }
    /// Wrap — marks last descriptor in transmit buffer descriptor list. This
    /// can be set for any buffer within the frame.
    pub fn wrap(&self) -> bool {
        (self.status.get() >> 30) & 1 == 1
    }
    /// Used — must be zero for the GMAC to read data to the transmit buffer.
    /// The GMAC sets this to one for the first buffer of a frame once it
    /// has been successfully transmitted. Software must clear this bit
    /// before the buffer can be used again.
    pub fn used(&self) -> bool {
        (self.status.get() >> 31) & 1 == 1
    }
    /// Mark this buffer as used so the GMAC does not read it
    pub fn set_used(&mut self) {
        self.status.set(self.status.get() | (1 << 31));
    }
}

#[repr(C, align(8))]
pub struct RxBufferDescriptor {
    address: VolatileCell<u32>,
    status: VolatileCell<u32>,
}
#[allow(clippy::len_without_is_empty)]
impl RxBufferDescriptor {
    pub fn init(&mut self, buf: &[u8], is_last: bool) {
        self.set_address(buf.as_ptr() as u32);
        self.reset_status();
        self.set_wrap(is_last);
    }
    /// Ownership — needs to be zero for the GMAC to write data to the receive
    /// buffer. The GMAC sets this to one once it has successfully written a
    /// frame to memory. Software has to clear this bit before the buffer can be
    /// used again.
    pub fn ownership(&self) -> bool {
        self.address.get() & 1 == 1
    }
    pub(crate) fn set_ownership(&self, value: bool) {
        if value {
            self.address.set(self.address.get() | 1);
        } else {
            self.address.set(self.address.get() & !1);
        }
    }
    /// Wrap — marks last descriptor in receive buffer descriptor list.
    pub fn wrap(&self) -> bool {
        (self.address.get() >> 1) & 1 == 1
    }
    /// Address of beginning of buffer
    pub fn address(&self) -> u32 {
        self.address.get() >> 2
    }
    fn set_address(&mut self, address: u32) {
        assert!(address < (1 << 30));
        self.address.set(address & ((1 << 30) - 1));
    }
    /// Returns this receive buffer to the GMAC
    pub(crate) fn reset_status(&mut self) {
        self.status.set(0);
    }
    fn set_wrap(&mut self, wrap: bool) {
        if wrap {
            self.address.set(self.address.get() | (1 << 1));
        } else {
            self.address.set(self.address.get() & !(1 << 1));
        }
    }

    /// These bits represent the length of the received frame which may or may
    /// not include FCS depending onw hether FCS discard mode is enabled. With
    /// FCS discard mode disabled: (bit 17 clear in Network Configuration
    /// Register)
    ///
    /// Least significant 12 bits for length of frame including FCS. If jumbo
    /// frames are enabled, these 12 bits areconcatenated with bit[13] of
    /// the descriptor above.
    ///
    /// With FCS discard mode enabled: (bit 17 set in Network Configuration
    /// Register)
    ///
    /// Least significant 12 bits for length of frame excluding FCS. If jumbo
    /// frames are enabled, these 12 bits are concatenated with bit[13] of
    /// the descriptor above.
    pub fn len(&self) -> u16 {
        (self.status.get() & ((1 << 13) - 1)) as u16
    }
    /// This bit has a different meaning depending on whether jumbo frames and
    /// ignore FCS modes are enabled. If neither mode is enabled this bit will
    /// be zero. With jumbo frame mode enabled: (bit 3 set in Network
    /// Configuration Register) Additional bit for length of frame
    /// (bit[13]), that is concatenated with bits[12:0]
    ///
    /// With ignore FCS mode enabled and jumbo frames disabled: (bit 26 set in
    /// Network Configuration Registerand bit 3 clear in Network
    /// Configuration Register) This indicates per frame FCS
    /// status as follows:
    ///
    /// 0: Frame had good FCS
    ///
    /// 1: Frame had bad FCS, but was copied to memory as ignore FCS enabled.
    pub fn fcs(&self) -> bool {
        (self.status.get() >> 13) & 1 == 1
    }
    /// Start of frame—when set the buffer contains the start of a frame. If
    /// both bits 15 and 14 are set, the buffer contains a whole frame.
    pub fn sof(&self) -> bool {
        (self.status.get() >> 14) & 1 == 1
    }
    /// End of frame—when set the buffer contains the end of a frame. If end of
    /// frame is not set, then the only valid status bit is start of frame (bit
    /// 14).
    pub fn eof(&self) -> bool {
        (self.status.get() >> 15) & 1 == 1
    }
    /// Canonical format indicator (CFI) bit (only valid if bit 21
    /// (`vlan_detected()`) is set).
    pub fn cfi(&self) -> bool {
        (self.status.get() >> 16) & 1 == 1
    }
    /// VLAN priority — only valid if bit 21 (`vlan_detected()`) is set.
    pub fn vlan_priority(&self) -> u8 {
        (self.status.get() >> 17 & 0x7) as u8
    }
    /// Priority tag detected—type ID of 0x8100 and null VLAN identifier. For
    /// packets incorporating the stackedVLAN processing feature, this bit will
    /// be set if the second VLAN tag has a type ID of 0x8100 and a null VLAN
    /// identifier.
    pub fn priority_detected(&self) -> bool {
        (self.status.get() >> 20) & 1 == 1
    }
    /// VLAN tag detected—type ID of 0x8100. For packets incorporating the
    /// stacked VLAN processing feature, this bit will be set if the second VLAN
    /// tag has a type ID of 0x8100
    pub fn vlan_detected(&self) -> bool {
        (self.status.get() >> 21) & 1 == 1
    }
    /// This bit has a different meaning depending on whether RX checksum
    /// offloading is enabled. With RX checksum offloading disabled: (bit 24
    /// clear in Network Configuration) Type ID register match. Encoded as
    /// follows:
    ///
    /// 00: Type ID register 1 match
    ///
    /// 01: Type ID register 2 match
    ///
    /// 10: Type ID register 3 match
    ///
    /// 11: Type ID register 4 match
    ///
    /// If more than one Type ID is matched only one is indicated with priority
    /// 4 down to 1. With RX checksum offloading enabled: (bit 24 set in Network
    /// Configuration Register)
    ///
    /// 00: Neither the IP header checksum nor the TCP/UDP checksum was checked.
    ///
    /// 01: The IP header checksum was checked and was correct. Neither the TCP
    /// nor UDP checksum waschecked.
    ///
    /// 10: Both the IP header and TCP checksum were checked and were correct.
    ///
    /// 11: Both the IP header and UDP checksum were checked and were correct.
    pub fn type_id_match(&self) -> u8 {
        (self.status.get() >> 22 & 0x3) as u8
    }
    /// This bit has a different meaning depending on whether RX checksum
    /// offloading is enabled. With RX checksum offloading disabled: (bit 24
    /// clear in Network Configuration Register)
    /// Type ID register match found, bit 22 and bit 23 indicate which type ID
    /// register causes the match. With RX checksum offloading enabled: (bit
    /// 24 set in Network Configuration Register)
    ///
    /// 0: The frame was not SNAP encoded and/or had a VLAN tag with the
    /// Canonical Format Indicator (CFI) bitset.
    ///
    /// 1: The frame was SNAP encoded and had either no VLAN tag or a VLAN tag
    /// with the CFI bit not set.
    pub fn checksum_offload(&self) -> bool {
        (self.status.get() >> 24) & 1 == 1
    }
    /// Specific Address Register match. Encoded as follows:
    ///
    /// 00: Specific Address Register 1 match
    ///
    /// 01: Specific Address Register 2 match
    ///
    /// 10: Specific Address Register 3 match
    ///
    /// 11: Specific Address Register 4 match
    ///
    /// If more than one specific address is matched only one is indicated with
    /// priority 4 down to 1.
    pub fn addr_match(&self) -> u8 {
        ((self.status.get() >> 25) & 0x3) as u8
    }
    /// Specific Address Register match found, bit 25 and bit 26 indicate which
    /// Specific Address Register causes the match.
    pub fn ext_addr_match(&self) -> bool {
        (self.status.get() >> 27) & 1 == 1
    }
    // Unicast hash match
    pub fn uni_hash_match(&self) -> bool {
        (self.status.get() >> 29) & 1 == 1
    }
    /// Multicast hash match
    pub fn multi_hash_match(&self) -> bool {
        (self.status.get() >> 30) & 1 == 1
    }
    /// Global all ones broadcast address detected
    pub fn boardcast_detect(&self) -> bool {
        (self.status.get() >> 31) & 1 == 1
    }
}

impl Default for RxBufferDescriptor {
    fn default() -> Self {
        RxBufferDescriptor {
            address: VolatileCell::new(0),
            status: VolatileCell::new(0),
        }
    }
}
impl Default for TxBufferDescriptor {
    fn default() -> Self {
        TxBufferDescriptor {
            address: VolatileCell::new(0),
            status: VolatileCell::new(0),
        }
    }
}

pub trait BufferDescriptor: private::Sealed {
    fn init(&mut self, buf: &[u8], is_last: bool);
}
impl BufferDescriptor for RxBufferDescriptor {
    fn init(&mut self, buf: &[u8], is_last: bool) {
        self.init(buf, is_last);
    }
}
impl BufferDescriptor for TxBufferDescriptor {
    fn init(&mut self, buf: &[u8], is_last: bool) {
        self.init(buf, is_last);
    }
}

/// Wrapper type to ensure proper alignment of buffers
#[repr(align(32))]
pub struct GmacBuffer<Size: ArrayLength<u8>> {
    buffer: GenericArray<u8, Size>,
}
impl<Size: ArrayLength<u8>> core::ops::Deref for GmacBuffer<Size> {
    type Target = GenericArray<u8, Size>;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
impl<Size: ArrayLength<u8>> core::ops::DerefMut for GmacBuffer<Size> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}
impl<Size: ArrayLength<u8>> core::convert::From<GenericArray<u8, Size>> for GmacBuffer<Size> {
    fn from(buf: GenericArray<u8, Size>) -> Self {
        GmacBuffer { buffer: buf }
    }
}
impl<Size: ArrayLength<u8>> Default for GmacBuffer<Size> {
    fn default() -> Self {
        GmacBuffer {
            buffer: GenericArray::default(),
        }
    }
}

pub struct GmacBufferSet<T, Count, Size>
where
    T: 'static + BufferDescriptor,
    Count: 'static + ArrayLength<T> + ArrayLength<GmacBuffer<Size>>,
    Size: 'static + ArrayLength<u8>,
{
    descriptors: &'static mut GenericArray<T, Count>,
    buffers: &'static mut GenericArray<GmacBuffer<Size>, Count>,
}

impl<T, Count, Size> GmacBufferSet<T, Count, Size>
where
    T: BufferDescriptor,
    Count: ArrayLength<T> + ArrayLength<GmacBuffer<Size>>,
    Size: ArrayLength<u8>,
{
    pub fn new(
        descriptors: &'static mut GenericArray<T, Count>,
        buffers: &'static mut GenericArray<GmacBuffer<Size>, Count>,
    ) -> Self {
        let count = Count::to_usize();
        for (idx, (descriptor, buffer)) in descriptors.iter_mut().zip(buffers.iter()).enumerate() {
            descriptor.init(&buffer[..], idx >= count - 1);
        }
        GmacBufferSet {
            descriptors,
            buffers,
        }
    }

    pub(crate) fn get_buffer(&self, index: usize) -> &[u8] {
        &self.buffers[index][..]
    }
    pub(crate) fn get_buffer_mut(&mut self, index: usize) -> &mut [u8] {
        &mut self.buffers[index][..]
    }
    pub(crate) fn get_descriptor(&self, index: usize) -> &T {
        &self.descriptors[index]
    }
    pub(crate) fn get_descriptor_mut(&mut self, index: usize) -> &mut T {
        &mut self.descriptors[index]
    }
}

#[cfg(feature = "nightly")]
impl<T, Count, Size> GmacBufferSet<T, Count, Size>
where
    T: BufferDescriptor + Default,
    Count: ArrayLength<T> + ArrayLength<GmacBuffer<Size>>,
    Size: ArrayLength<u8>,
{
    pub const fn create() -> Self {
        let mut descriptors: GenericArray<T, Count> = GenericArray::default();
        let buffers: GenericArray<GmacBuffer<Size>, Count> = GenericArray::default();
        let count = Count::to_usize();
        let mut idx = 0;
        while count > 0 {
            let descriptor = &mut descriptors[idx];
            let buffer = &buffers[idx];
            descriptor.init(&buffer[..], idx >= count - 1);
            idx -= 1;
        }
        GmacBufferSet {
            descriptors,
            buffers,
        }
    }
}

pub trait BufferSet<T>: private::Sealed {
    const COUNT: usize;
    const SIZE: usize;
    fn address(&self) -> u32;
    fn get_buffer(&self, index: usize) -> &[u8];
    fn get_buffer_mut(&mut self, index: usize) -> &mut [u8];
    fn get_descriptor(&self, index: usize) -> &T;
    fn get_descriptor_mut(&mut self, index: usize) -> &mut T;
}

impl<T, Count, Size> BufferSet<T> for GmacBufferSet<T, Count, Size>
where
    T: BufferDescriptor,
    Count: ArrayLength<T> + ArrayLength<GmacBuffer<Size>>,
    Size: ArrayLength<u8>,
{
    const COUNT: usize = Count::USIZE;
    const SIZE: usize = Size::USIZE;

    fn address(&self) -> u32 {
        self.descriptors.as_ptr() as u32
    }
    fn get_buffer(&self, index: usize) -> &[u8] {
        &self.get_buffer(index)[..]
    }
    fn get_buffer_mut(&mut self, index: usize) -> &mut [u8] {
        &mut self.get_buffer_mut(index)[..]
    }
    fn get_descriptor(&self, index: usize) -> &T {
        self.get_descriptor(index)
    }
    fn get_descriptor_mut(&mut self, index: usize) -> &mut T {
        self.get_descriptor_mut(index)
    }
}
