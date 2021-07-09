//! Flag definitions

use bitflags::bitflags;
use core::convert::TryFrom;

//=============================================================================
// Interrupt flags
//=============================================================================
const DRE: u8 = 0x01;
const TXC: u8 = 0x02;
const RXC: u8 = 0x04;
const RXS: u8 = 0x08;
pub(super) const CTSIC: u8 = 0x10;
const RXBRK: u8 = 0x20;
const ERROR: u8 = 0x80;

/// Interrupt flags available for RX transactions
pub const RX_FLAG_MASK: u8 = RXC | RXS | RXBRK | ERROR;
/// Interrupt flags available for TX transactions
pub const TX_FLAG_MASK: u8 = DRE | TXC;
/// Interrupt flags available for Duplex transactions
pub const DUPLEX_FLAG_MASK: u8 = RX_FLAG_MASK | TX_FLAG_MASK;

bitflags! {
    /// Interrupt bit flags for UART Rx transactions
    ///
    /// The available interrupt flags are `DRE`, `TXC`, `RXC`, `RXS`, `CTSIC`, `RXBRK` and
    /// `ERROR`. The binary format of the underlying bits exactly matches the
    /// INTFLAG bits.
    pub struct Flags: u8 {
        const DRE = DRE;
        const TXC = TXC;
        const RXC = RXC;
        const RXS = RXS ;
        const CTSIC = CTSIC;
        const RXBRK = RXBRK;
        const ERROR = ERROR;
    }
}

//=============================================================================
// Status flags
//=============================================================================

const PERR: u16 = 0x01;
const FERR: u16 = 0x02;
const BUFOVF: u16 = 0x04;
const CTS: u16 = 0x08;
const ISF: u16 = 0x10;
const COLL: u16 = 0x20;

/// Status flags available for RX transactions
pub const RX_STATUS_MASK: u16 = PERR | FERR | BUFOVF | ISF | COLL;
/// Status flags available for Duplex transactions
pub const DUPLEX_STATUS_MASK: u16 = RX_STATUS_MASK;

bitflags! {
    /// Status flags for UART Rx transactions
    ///
    /// The available status flags are `PERR`, `FERR`, `BUFOVF`,
    /// `CTS`, `ISF` and `COLL`.
    /// The binary format of the underlying bits exactly matches
    /// the STATUS bits.
    pub struct Status: u16 {
        const PERR = PERR;
        const FERR = FERR;
        const BUFOVF = BUFOVF;
        const CTS = CTS;
        const ISF = ISF;
        const COLL = COLL;
    }
}

//=============================================================================
// Error
//=============================================================================

/// Errors available for UART transactions
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// Detected a parity error
    ParityError,
    /// Detected a frame error
    FrameError,
    /// Detected a buffer overflow
    Overflow,
    /// Detected an inconsistent sync field
    InconsistentSyncField,
    /// Detected a collision
    CollisionDetected,
}

impl TryFrom<Status> for () {
    type Error = Error;

    #[inline]
    fn try_from(errors: Status) -> Result<(), Error> {
        use Error::*;
        if errors.contains(Status::PERR) {
            Err(ParityError)
        } else if errors.contains(Status::FERR) {
            Err(FrameError)
        } else if errors.contains(Status::BUFOVF) {
            Err(Overflow)
        } else if errors.contains(Status::ISF) {
            Err(InconsistentSyncField)
        } else if errors.contains(Status::COLL) {
            Err(CollisionDetected)
        } else {
            Ok(())
        }
    }
}

impl From<Error> for Status {
    #[inline]
    fn from(err: Error) -> Self {
        use Error::*;
        match err {
            ParityError => Status::PERR,
            FrameError => Status::FERR,
            Overflow => Status::BUFOVF,
            InconsistentSyncField => Status::ISF,
            CollisionDetected => Status::COLL,
        }
    }
}
