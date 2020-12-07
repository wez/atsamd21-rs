use bitfield::bitfield;
use core::fmt::Debug;

/// IEEE 802.3 Clause 22.2.4 defined Standard Registers.
/// The MII basic register set consists of two registers referred to as the
/// Control register (Register 0) and the Status register (Register 1). All
/// PHYs that provide an MII shall incorporate the basic register set. All PHYs
/// that provide a GMII shall incorporate an extended basic register set
/// consisting of the Control register (Register 0), Status register
/// (Register 1), and Extended Status register (Register 15). The status and
/// control functions defined here are considered basic and fundamental to
/// 100 Mb/s and 1000 Mb/s PHYs. Registers 2 through 14 are part of the
/// extended register set. The format of Registers 4 through 10 are defined for
/// the specific Auto-Negotiation protocol used (Clause 28 or Clause 37). The
/// format of these registers is selected by the bit settings of Registers 1
/// and 15.
pub enum MdioRegister {
    /// Basic Control
    BMCR = 0x0,
    /// Basic Status
    BMSR = 0x1,
    /// PHY Idendifier 1
    PHYID1 = 0x2,
    /// PHY Idendifier 2
    PHYID2 = 0x3,
    /// Auto_Negotiation Advertisement
    ANA = 0x4,
    /// Auto_negotiation Link Partner Base Page Ability
    ANLPA = 0x5,
    /// Auto-negotiation Expansion
    ANE = 0x6,
    /// Auto-negotiation Next Page Transmit
    ANNPT = 0x7,
    /// Auto-Negotiation Link Partner Received Next Page
    ANLPRNP = 0x8,
    /// MASTER-SLAVE Control Register
    MSC = 0x9,
    /// MASTER-SLAVE Status Register
    MSS = 0xa,
    /// PSE Control Register
    PSEC = 0xb,
    /// PSE Status Register
    PSES = 0xc,
    /// MMD Access Control Register
    MMDAC = 0xd,
    /// MMD Access Address Data Register
    MMDAAD = 0xe,
    /// Extended Status
    EXTS = 0xf,
    /* Register 16 to 31 are Reserved for Vendor Specific */
    Vendor0 = 0x10,
    Vendor1 = 0x11,
    Vendor2 = 0x12,
    Vendor3 = 0x13,
    Vendor4 = 0x14,
    Vendor5 = 0x15,
    Vendor6 = 0x16,
    Vendor7 = 0x17,
    Vendor8 = 0x18,
    Vendor9 = 0x19,
    Vendor10 = 0x1a,
    Vendor11 = 0x1b,
    Vendor12 = 0x1c,
    Vendor13 = 0x1d,
    Vendor14 = 0x1e,
    Vendor15 = 0x1f,
}

pub enum BmcrRegisterMask {
    Reset = 1 << 15,
    Loopback = 1 << 14,
    SpeedSelectLsb = 1 << 13,
    AutoNeg = 1 << 12,
    PowerDown = 1 << 11,
    Isolate = 1 << 10,
    RestartAutoNeg = 1 << 9,
    DuplexMode = 1 << 8,
    CollisionTest = 1 << 7,
    SpeedSelectMsb = 1 << 6,
    Unidir = 1 << 5,
}
pub enum BmsrRegisterMask {
    /// 100BASE-T4 Capable
    _100BaseT4 = 1 << 15,
    /// 100BASE-TX Full Duplex Capable
    _100BaseTxFd = 1 << 14,
    /// 100BASE-TX Half Duplex Capable
    _100BaseTxHd = 1 << 13,
    /// 10BASE-T Full Duplex Capable
    _10BaseTFd = 1 << 12,
    /// 10BASE-T Half Duplex Capable
    _10BaseTHd = 1 << 11,
    /// 1000BASE-T2 Full Duplex Capable
    _1000BaseT2Fd = 1 << 10,
    /// 1000BASE-T2 Half Duplex Capable
    _1000BaseT2Hd = 1 << 9,
    /// 1 = Extend Status Information In Reg 15
    ExtendStatus = 1 << 8,
    /// Unidirectional ability
    UnidirAbility = 1 << 7,
    /// MII Frame Preamble Suppression
    MfPreambSuppr = 1 << 6,
    /// Auto-negotiation Complete
    AutoNegComp = 1 << 5,
    /// Remote Fault
    RemoteFault = 1 << 4,
    /// Auto Configuration Ability
    AutoNegAbility = 1 << 3,
    /// Link Status
    LinkStatus = 1 << 2,
    /// Jabber Detect
    JabberDetect = 1 << 1,
    /// Extended Capability
    ExtendedCapability = 1 << 0,
}
