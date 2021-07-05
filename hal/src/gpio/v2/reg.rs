#[cfg(any(feature = "samd11", feature = "samd21"))]
use crate::target_device::port::{
    CTRL, DIR, DIRCLR, DIRSET, DIRTGL, IN, OUT, OUTCLR, OUTSET, OUTTGL, PINCFG0_ as PINCFG,
    PMUX0_ as PMUX, WRCONFIG,
};

#[cfg(feature = "min-samd51g")]
use crate::target_device::port::group::{
    CTRL, DIR, DIRCLR, DIRSET, DIRTGL, IN, OUT, OUTCLR, OUTSET, OUTTGL, PINCFG, PMUX, WRCONFIG,
};

use crate::target_device::PORT;

use super::dynpin::*;

//==============================================================================
//  ModeFields
//==============================================================================

/// Collect all fields needed to set the [`PinMode`](super::PinMode)
#[derive(Default)]
struct ModeFields {
    dir: bool,
    inen: bool,
    pullen: bool,
    out: bool,
    pmuxen: bool,
    pmux: u8,
}

impl From<DynPinMode> for ModeFields {
    #[inline]
    fn from(mode: DynPinMode) -> Self {
        let mut fields = Self::default();
        use DynPinMode::*;
        match mode {
            Disabled(config) => {
                use DynDisabled::*;
                match config {
                    Floating => {
                        fields.pullen = false;
                        fields.out = false;
                    }
                    PullDown => {
                        fields.pullen = true;
                        fields.out = false;
                    }
                    PullUp => {
                        fields.pullen = true;
                        fields.out = true;
                    }
                }
            }
            Input(config) => {
                fields.inen = true;
                use DynInput::*;
                match config {
                    Floating => {
                        fields.pullen = false;
                        fields.out = false;
                    }
                    PullDown => {
                        fields.pullen = true;
                        fields.out = false;
                    }
                    PullUp => {
                        fields.pullen = true;
                        fields.out = true;
                    }
                }
            }
            Interrupt(config) => {
                fields.pmuxen = true;
                fields.pmux = 0;
                use DynInterrupt::*;
                match config {
                    Floating => {
                        fields.pullen = false;
                        fields.out = false;
                    }
                    PullDown => {
                        fields.pullen = true;
                        fields.out = false;
                    }
                    PullUp => {
                        fields.pullen = true;
                        fields.out = true;
                    }
                }
            }
            Output(config) => {
                fields.dir = true;
                use DynOutput::*;
                match config {
                    PushPull => {
                        fields.inen = false;
                    }
                    Readable => {
                        fields.inen = true;
                    }
                }
            }
            Alternate(config) => {
                fields.pmuxen = true;
                use DynAlternate::*;
                match config {
                    B => {
                        fields.pmux = 1;
                    }
                    C => {
                        fields.pmux = 2;
                    }
                    D => {
                        fields.pmux = 3;
                    }
                    E => {
                        fields.pmux = 4;
                    }
                    F => {
                        fields.pmux = 5;
                    }
                    G => {
                        fields.pmux = 6;
                    }
                    #[cfg(any(feature = "samd21", feature = "min-samd51g"))]
                    H => {
                        fields.pmux = 7;
                    }
                    #[cfg(feature = "min-samd51g")]
                    I => {
                        fields.pmux = 8;
                    }
                    #[cfg(feature = "min-samd51g")]
                    J => {
                        fields.pmux = 9;
                    }
                    #[cfg(feature = "min-samd51g")]
                    K => {
                        fields.pmux = 10;
                    }
                    #[cfg(feature = "min-samd51g")]
                    L => {
                        fields.pmux = 11;
                    }
                    #[cfg(feature = "min-samd51g")]
                    M => {
                        fields.pmux = 12;
                    }
                    #[cfg(feature = "min-samd51g")]
                    N => {
                        fields.pmux = 13;
                    }
                }
            }
        };
        fields
    }
}

//==============================================================================
//  GROUP
//==============================================================================

/// Represent the [`PORT`] register block
///
/// The SAMx5x PACs have a GROUP type to represent each [`PORT`] group, but the
/// SAMD11 and SAMD21 PACs do not. Manually re-implement it here.
#[repr(C)]
pub(super) struct GROUP {
    dir: DIR,
    dirclr: DIRCLR,
    dirset: DIRSET,
    dirtgl: DIRTGL,
    out: OUT,
    outclr: OUTCLR,
    outset: OUTSET,
    outtgl: OUTTGL,
    in_: IN,
    ctrl: CTRL,
    wrconfig: WRCONFIG,
    _padding1: [u8; 4],
    pmux: [PMUX; 16],
    pincfg: [PINCFG; 32],
    _padding2: [u8; 32],
}

//==============================================================================
//  RegisterInterface
//==============================================================================

/// Provide a safe register interface for pin objects
///
/// [`PORT`], like every PAC `struct`, is [`Send`] but not [`Sync`], because it
/// points to a `RegisterBlock` of `VolatileCell`s. Unfortunately, such an
/// interface is quite restrictive. Instead, it would be ideal if we could split
/// the [`PORT`] into independent pins that are both [`Send`] and [`Sync`].
///
/// [`PORT`] is a single, zero-sized marker `struct` that provides access to
/// every [`PORT`] register. Instead, we would like to create zero-sized marker
/// `struct`s for every pin, where each pin is only allowed to control its own
/// registers. Furthermore, each pin `struct` should be a singleton, so that
/// exclusive access to the `struct` also guarantees exclusive access to the
/// corresponding registers. Finally, the pin `struct`s should not have any
/// interior mutability. Together, these requirements would allow the pin
/// `struct`s to be both [`Send`] and [`Sync`].
///
/// This trait creates a safe API for accomplishing these goals. Implementers
/// supply a pin ID through the [`id`] function. The remaining functions provide
/// a safe API for accessing the registers associated with that pin ID. Any
/// modification of the registers requires `&mut self`, which destroys interior
/// mutability.
///
/// # Safety
///
/// Users should only implement the [`id`] function. No default function
/// implementations should be overridden. The implementing type must also have
/// "control" over the corresponding pin ID, i.e. it must guarantee that a each
/// pin ID is a singleton.
///
/// [`id`]: Self::id
pub(super) unsafe trait RegisterInterface {
    /// Provide a [`DynPinId`] identifying the set of registers controlled by
    /// this type.
    fn id(&self) -> DynPinId;

    /// Pointer to the array of [`GROUP`] register blocks
    const GROUPS: *const GROUP = PORT::ptr() as *const _;

    #[inline]
    fn group(&self) -> &GROUP {
        let offset = match self.id().group {
            DynGroup::A => 0,
            #[cfg(any(feature = "samd21", feature = "min-samd51g"))]
            DynGroup::B => 1,
            #[cfg(feature = "min-samd51n")]
            DynGroup::C => 2,
            #[cfg(feature = "min-samd51p")]
            DynGroup::D => 3,
        };
        // Safety: It is safe to create shared references to each PAC register
        // or register block, because all registers are wrapped in
        // `UnsafeCell`s. We should never create unique references to the
        // registers, to prevent any risk of UB.
        unsafe { &*Self::GROUPS.add(offset) }
    }

    #[inline]
    fn mask_32(&self) -> u32 {
        1 << self.id().num
    }

    #[inline]
    fn mask_16(&self) -> u16 {
        1 << (self.id().num & 0xF)
    }

    #[inline]
    fn hwsel(&self) -> bool {
        self.id().num & 0x10 != 0
    }

    #[inline]
    fn pincfg(&self) -> &PINCFG {
        &self.group().pincfg[self.id().num as usize]
    }

    /// Change the pin mode
    ///
    /// We use the WRCONFIG register to avoid using the PMUX register. Each PMUX
    /// register stores the PMUX values for two different pins.  Changing the
    /// PMUX value for one pin would require a read/modify/write operation that
    /// could be preempted by the other pin. This is fundamentally unsound. The
    /// WRCONFIG register lets us modify *only* the fields corresponding to this
    /// particular PinId/DynPinId.
    #[inline]
    fn change_mode(&mut self, mode: DynPinMode) {
        let ModeFields {
            dir,
            inen,
            pullen,
            out,
            pmuxen,
            pmux,
        } = mode.into();
        // The bit patterns here are guaranteed to be safe, because they can
        // ultimately be traced back to associated constants defined on the
        // `PinId` and `PinMode` traits, which are guaranteed to be correct.
        self.group().wrconfig.write(|w| unsafe {
            w.hwsel().bit(self.hwsel());
            w.wrpincfg().set_bit();
            w.wrpmux().set_bit();
            w.pmux().bits(pmux);
            w.pullen().bit(pullen);
            w.inen().bit(inen);
            w.pmuxen().bit(pmuxen);
            w.pinmask().bits(self.mask_16())
        });
        self.set_dir(dir);
        if pullen {
            self.write_pin(out)
        };
    }

    /// Set the direction of a pin
    #[inline]
    fn set_dir(&mut self, bit: bool) {
        let mask = self.mask_32();
        // Safety: DIRSET & DIRCLR are "mask" registers, and we only write the
        // bit for this pin ID
        unsafe {
            if bit {
                self.group().dirset.write(|w| w.bits(mask));
            } else {
                self.group().dirclr.write(|w| w.bits(mask));
            }
        }
    }

    /// Read the logic level of an input put
    #[inline]
    #[allow(dead_code)]
    fn read_pin(&self) -> bool {
        let mask = self.mask_32();
        self.group().in_.read().bits() & mask != 0
    }

    /// Write the logic level of an output pin
    #[inline]
    fn write_pin(&mut self, bit: bool) {
        let mask = self.mask_32();
        // Safety: OUTSET & OUTCLR are "mask" registers, and we only write the
        // bit for this pin ID
        unsafe {
            if bit {
                self.group().outset.write(|w| w.bits(mask));
            } else {
                self.group().outclr.write(|w| w.bits(mask));
            }
        }
    }

    /// Toggle the logic level of an output pin
    #[inline]
    fn toggle_pin(&mut self) {
        let mask = self.mask_32();
        // Safety: OUTTGL is a "mask" register, and we only write the bit for
        // this pin ID
        unsafe { self.group().outtgl.write(|w| w.bits(mask)) };
    }

    /// Read back the logic level of an output pin
    #[inline]
    #[allow(dead_code)]
    fn read_out_pin(&self) -> bool {
        let mask = self.mask_32();
        self.group().out.read().bits() & mask != 0
    }

    /// Read the drive strength of a pin
    #[inline]
    fn read_drive_strength(&self) -> bool {
        self.pincfg().read().drvstr().bit()
    }

    /// Write the drive strength of a pin
    #[inline]
    fn write_drive_strength(&mut self, bit: bool) {
        self.pincfg().modify(|_, w| w.drvstr().bit(bit));
    }
}
