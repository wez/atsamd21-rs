#![deny(missing_docs)]
#![deny(warnings)]
//! # AHB bus clocks
//!
//! This module provides abstractions allowing to deal with a synchronous
//! clocking domain, specifically modules clocked via AHB bus. It provides type
//! representation for disabled and enabled synchronous clocks available through
//! AHB bus and means of switching.
//!
//! - [`AhbToken<T>`] type represents a disabled clock for a peripheral of type
//!   `T`: [`AhbType`]
//! - [`AhbClk<T>`] type represents an enabled clock for a peripheral of type
//!   `T:` [`AhbType`]
//!
//! One can enable a peripheral `T` synchronous clock via
//! [`AhbToken<T>::enable`] `->` [`AhbClk<T>`] method.
//!
//! One can disable a peripheral `T` synchronous clock via
//! [`AhbClk<T>::disable`] `->` [`AhbToken<T>`] method.
//!
//! Clocks in a default state are provided
//! - in an instance of a struct [`AhbClks`]
//! - in a field [`crate::clock::v2::Tokens::ahbs`]
//! - in a return value of [`crate::clock::v2::retrieve_clocks`]
use core::marker::PhantomData;

use paste::paste;

use crate::pac::{mclk, MCLK};

//==============================================================================
// Registers
//==============================================================================

/// [`Registers`] struct is a low-level access abstraction for HW register
/// calls. It is generic over [`AhbType`] as it needs appropriate mask values
/// depending on a peripheral.
struct Registers<A: AhbType> {
    ahb: PhantomData<A>,
}

impl<A: AhbType> Registers<A> {
    #[inline(always)]
    unsafe fn new() -> Self {
        Registers { ahb: PhantomData }
    }

    #[inline(always)]
    fn mclk(&mut self) -> &mclk::RegisterBlock {
        unsafe { &*MCLK::ptr() }
    }

    #[inline(always)]
    fn ahbmask(&mut self) -> &mclk::AHBMASK {
        &self.mclk().ahbmask
    }

    #[inline(always)]
    fn enable(&mut self) {
        self.ahbmask()
            .modify(|r, w| unsafe { w.bits(r.bits() | A::MASK) });
    }

    #[inline(always)]
    fn disable(&mut self) {
        self.ahbmask()
            .modify(|r, w| unsafe { w.bits(r.bits() & !A::MASK) });
    }
}

//==============================================================================
// AhbType
//==============================================================================

/// Trait implemented by a specific synchronous peripheral clock. Provides
/// essential compile-time informations needed during HW register writes
pub trait AhbType: crate::typelevel::Sealed {
    /// Associated constant providing information regarding which mask offset
    /// is applicable for this specific peripheral clock during HW register
    /// writes
    const ID: AhbId;
    /// Helper associated constant that expands to the specific mask from
    /// a provided offset
    const MASK: u32 = 1 << (Self::ID as u8);
}

/// Enum representing mask offsets for different peripheral clocks
#[allow(missing_docs)]
pub enum AhbId {
    Hpb0 = 0,
    Hpb1 = 1,
    Hpb2 = 2,
    Hpb3 = 3,
    Dsu = 4,
    Nvmctrl = 6,
    Cmcc = 8,
    Dmac = 9,
    Usb = 10,
    Pac = 12,
    Qspi = 13,
    #[cfg(any(feature = "same53", feature = "same54"))]
    Gmac = 14,
    Sdhc0 = 15,
    #[cfg(feature = "min-samd51n")]
    Sdhc1 = 16,
    #[cfg(any(feature = "same51", feature = "same53", feature = "same54"))]
    Can0 = 17,
    #[cfg(any(feature = "same51", feature = "same53", feature = "same54"))]
    Can1 = 18,
    Icm = 19,
    Pukcc = 20,
    Qspi2x = 21,
    NvmCtrlSmeeProm = 22,
    NvmCtrlCache = 23,
}

/// Macro implementing an [`AhbType`] for a marker type and defining one if
/// necessary
macro_rules! ahb_type {
    (false, $Type:ident) => {
        /// Marker type implementing [`AhbType`] for a specific synchronous peripheral
        /// clock
        pub enum $Type {}
        impl crate::typelevel::Sealed for $Type {}
        ahb_type!(true, $Type);
    };
    (true, $Type:ident) => {
        impl AhbType for $Type {
            const ID: AhbId = AhbId::$Type;
        }
    };
}

//==============================================================================
// AhbToken
//==============================================================================

/// A type representing a synchronous peripheral clock in a disabled state
pub struct AhbToken<A: AhbType> {
    regs: Registers<A>,
}

impl<A: AhbType> AhbToken<A> {
    /// Constructor
    ///
    /// Unsafe: There should always be only a single instance thereof. It is
    /// being provided by a framework in a [`AhbClks`] struct instance
    #[inline]
    unsafe fn new() -> Self {
        AhbToken {
            regs: Registers::new(),
        }
    }

    /// Enable a synchronous peripheral clock
    #[inline]
    pub fn enable(mut self) -> AhbClk<A> {
        self.regs.enable();
        AhbClk::new(self)
    }
}

//==============================================================================
// AhbClk
//==============================================================================

/// A type representing a synchronous peripheral clock in an enabled state
pub struct AhbClk<A: AhbType> {
    token: AhbToken<A>,
}

impl<A: AhbType> AhbClk<A> {
    /// Constructor
    #[inline]
    fn new(token: AhbToken<A>) -> Self {
        AhbClk { token }
    }

    /// Disable a synchronous peripheral clock
    #[inline]
    pub fn disable(mut self) -> AhbToken<A> {
        self.token.regs.disable();
        self.token
    }
}

//==============================================================================
// AhbClks
//==============================================================================

/// Root level macro generating all type definitions and trait implementations.
/// Among other things, it defines and populates an [`AhbClks`] struct with
/// appropriate fields.
macro_rules! ahb_clks {
    (
        $(
            $( #[$cfg:meta] )?
            ($exists:literal, $Type:ident),
        )+
    ) => {
        paste! {
            $(
                $( #[$cfg] )?
                ahb_type!($exists, $Type);
            )+
            /// Struct aggregating [`AhbClk`] type instances representing
            /// default states of synchronous peripheral clocks
            #[allow(missing_docs)]
            pub struct AhbClks {
                $(
                    $( #[$cfg] )?
                    pub [<$Type:lower>]: AhbClk<$Type>,
                )+
            }
            impl AhbClks {
                pub(super) unsafe fn new() -> Self {
                    AhbClks {
                        $(
                            $( #[$cfg] )?
                            [<$Type:lower>]: AhbClk::new(AhbToken::new()),
                        )+
                    }
                }
            }
        }
    };
}

ahb_clks!(
    (false, Hpb0),
    (false, Hpb1),
    (false, Hpb2),
    (false, Hpb3),
    (false, Dsu),
    (false, Nvmctrl),
    (false, Cmcc),
    (false, Dmac),
    (false, Usb),
    (false, Pac),
    (false, Qspi),
    #[cfg(any(feature = "same53", feature = "same54"))]
    (false, Gmac),
    (false, Sdhc0),
    #[cfg(feature = "min-samd51n")]
    (false, Sdhc1),
    #[cfg(any(feature = "same51", feature = "same53", feature = "same54"))]
    (false, Can0),
    #[cfg(any(feature = "same51", feature = "same53", feature = "same54"))]
    (false, Can1),
    (false, Icm),
    (false, Pukcc),
    (false, Qspi2x),
    (false, NvmCtrlSmeeProm),
    (false, NvmCtrlCache),
);
