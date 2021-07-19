#![deny(missing_docs)]
#![deny(warnings)]
//! # APBx bus clocks
//!
//! This module provides abstractions allowing to deal with a synchronous
//! clocking domain, specifically modules clocked via APB bus. It provides type
//! representation for disabled and enabled synchronous clocks available through
//! APB bus and means of switching.
//!
//! - [`ApbToken<T>`] type represents a disabled clock for a peripheral of type
//!   `T`: [`ApbType`]
//! - [`ApbClk<T>`] type represents an enabled clock for a peripheral of type
//!   `T:` [`ApbType`]
//!
//! One can enable a peripheral `T` synchronous clock via
//! [`ApbToken<T>::enable`] `->` [`ApbClk<T>`] method.
//!
//! One can disable a peripheral `T` synchronous clock via
//! [`ApbClk<T>::disable`] `->` [`ApbToken<T>`] method.
//!
//! Clocks in a default state are provided
//! - in an instance of a struct [`ApbClks`]
//! - in a field [`crate::clock::v2::Tokens::apbs`]
//! - in a return value of [`crate::clock::v2::retrieve_clocks`]
use core::marker::PhantomData;

use paste::paste;

use crate::pac::{mclk, MCLK};

use crate::sercom::v2::*;

//==============================================================================
// Registers
//==============================================================================

/// [`Registers`] struct is a low-level access abstraction for HW register
/// calls. It is generic over [`ApbType`] as it needs appropriate mask values
/// and register variants depending on a peripheral.
struct Registers<A: ApbType> {
    apb: PhantomData<A>,
}

impl<A: ApbType> Registers<A> {
    #[inline(always)]
    unsafe fn new() -> Self {
        Registers { apb: PhantomData }
    }

    #[inline(always)]
    fn mclk(&mut self) -> &mclk::RegisterBlock {
        unsafe { &*MCLK::ptr() }
    }

    #[inline(always)]
    fn apbamask(&mut self) -> &mclk::APBAMASK {
        &self.mclk().apbamask
    }

    #[inline(always)]
    fn apbbmask(&mut self) -> &mclk::APBBMASK {
        &self.mclk().apbbmask
    }

    #[inline(always)]
    fn apbcmask(&mut self) -> &mclk::APBCMASK {
        &self.mclk().apbcmask
    }

    #[inline(always)]
    fn apbdmask(&mut self) -> &mclk::APBDMASK {
        &self.mclk().apbdmask
    }

    #[inline(always)]
    fn enable(&mut self) {
        unsafe {
            match A::ID {
                ApbId::A(_) => {
                    self.apbamask().modify(|r, w| w.bits(r.bits() | A::MASK));
                }
                ApbId::B(_) => {
                    self.apbbmask().modify(|r, w| w.bits(r.bits() | A::MASK));
                }
                ApbId::C(_) => {
                    self.apbcmask().modify(|r, w| w.bits(r.bits() | A::MASK));
                }
                ApbId::D(_) => {
                    self.apbdmask().modify(|r, w| w.bits(r.bits() | A::MASK));
                }
            }
        }
    }

    #[inline(always)]
    fn disable(&mut self) {
        unsafe {
            match A::ID {
                ApbId::A(_) => {
                    self.apbamask().modify(|r, w| w.bits(r.bits() & !A::MASK));
                }
                ApbId::B(_) => {
                    self.apbbmask().modify(|r, w| w.bits(r.bits() & !A::MASK));
                }
                ApbId::C(_) => {
                    self.apbcmask().modify(|r, w| w.bits(r.bits() & !A::MASK));
                }
                ApbId::D(_) => {
                    self.apbdmask().modify(|r, w| w.bits(r.bits() & !A::MASK));
                }
            }
        }
    }
}

//==============================================================================
// ApbIds
//==============================================================================

/// Enum representing mask offsets for peripheral clocks contained within a
/// bridge `A` of APB bus
#[allow(missing_docs)]
pub enum ApbAId {
    Pac = 0,
    Pm = 1,
    Mclk = 2,
    RstC = 3,
    OscCtrl = 4,
    Osc32kCtrl = 5,
    SupC = 6,
    Gclk = 7,
    Wdt = 8,
    Rtc = 9,
    Eic = 10,
    FreqM = 11,
    Sercom0 = 12,
    Sercom1 = 13,
    Tc0 = 14,
    Tc1 = 15,
}

/// Enum representing mask offsets for peripheral clocks contained within a
/// bridge `B` of APB bus
#[allow(missing_docs)]
pub enum ApbBId {
    Usb = 0,
    Dsu = 1,
    NvmCtrl = 2,
    Port = 4,
    EvSys = 7,
    Sercom2 = 9,
    Sercom3 = 10,
    Tcc0 = 11,
    Tcc1 = 12,
    Tc2 = 13,
    Tc3 = 14,
    RamEcc = 16,
}

/// Enum representing mask offsets for peripheral clocks contained within a
/// bridge `C` of APB bus
#[allow(missing_docs)]
pub enum ApbCId {
    #[cfg(any(feature = "same53", feature = "same54"))]
    Gmac = 2,
    Tcc2 = 3,
    #[cfg(feature = "min-samd51j")]
    Tcc3 = 4,
    #[cfg(feature = "min-samd51j")]
    Tc4 = 5,
    #[cfg(feature = "min-samd51j")]
    Tc5 = 6,
    Pdec = 7,
    Ac = 8,
    Aes = 9,
    Trng = 10,
    Icm = 11,
    Qspi = 13,
    Ccl = 14,
}

/// Enum representing mask offsets for peripheral clocks contained within a
/// bridge `D` of APB bus
#[allow(missing_docs)]
pub enum ApbDId {
    Sercom4 = 0,
    Sercom5 = 1,
    #[cfg(feature = "min-samd51n")]
    Sercom6 = 2,
    #[cfg(feature = "min-samd51n")]
    Sercom7 = 3,
    #[cfg(feature = "min-samd51j")]
    Tcc4 = 4,
    #[cfg(feature = "min-samd51n")]
    Tc6 = 5,
    #[cfg(feature = "min-samd51n")]
    Tc7 = 6,
    Adc0 = 7,
    Adc1 = 8,
    Dac = 9,
    #[cfg(feature = "min-samd51j")]
    I2S = 10,
    Pcc = 11,
}

/// Enum representing mask offsets across all APB bus bridges
#[allow(missing_docs)]
pub enum ApbId {
    A(ApbAId),
    B(ApbBId),
    C(ApbCId),
    D(ApbDId),
}

//==============================================================================
// ApbType
//==============================================================================

/// Trait implemented by a specific synchronous peripheral clock. Provides
/// essential compile-time informations needed during HW register writes
pub trait ApbType: crate::typelevel::Sealed {
    /// Associated constant providing information regarding which mask offset
    /// and bridge variant of APB bus is applicable for this specific peripheral
    /// clock during HW register writes
    const ID: ApbId;
    /// Helper associated constant that expands to the specific mask from
    /// provided offset
    const MASK: u32 = 1
        << match Self::ID {
            ApbId::A(id) => id as u8,
            ApbId::B(id) => id as u8,
            ApbId::C(id) => id as u8,
            ApbId::D(id) => id as u8,
        };
}

/// Macro implementing an [`ApbType`] for a marker type and defining one if
/// necessary
macro_rules! apb_type {
    (false, $Reg:ident, $Type:ident) => {
        /// Marker type implementing [`ApbType`] for a specific synchronous peripheral
        /// clock
        pub enum $Type {}
        impl crate::typelevel::Sealed for $Type {}
        apb_type!(true, $Reg, $Type);
    };
    (true, $Reg:ident, $Type:ident) => {
        paste! {
            impl ApbType for $Type {
                const ID: ApbId = ApbId::$Reg([<Apb $Reg Id>]::$Type);
            }
        }
    };
}

//==============================================================================
// ApbToken
//==============================================================================

/// A type representing a synchronous peripheral clock in a disabled state
pub struct ApbToken<A: ApbType> {
    regs: Registers<A>,
}

impl<A: ApbType> ApbToken<A> {
    /// Constructor
    ///
    /// Unsafe: There should always be only a single instance thereof. It is
    /// being provided by a framework in a [`ApbClks`] struct instance
    #[inline]
    unsafe fn new() -> Self {
        ApbToken {
            regs: Registers::new(),
        }
    }

    /// Enable a synchronous peripheral clock
    #[inline]
    pub fn enable(mut self) -> ApbClk<A> {
        self.regs.enable();
        ApbClk::new(self)
    }
}

//==============================================================================
// ApbClk
//==============================================================================

/// A type representing a synchronous peripheral clock in an enabled state
pub struct ApbClk<A: ApbType> {
    token: ApbToken<A>,
}

impl<A: ApbType> ApbClk<A> {
    /// Constructor
    #[inline]
    fn new(token: ApbToken<A>) -> Self {
        ApbClk { token }
    }

    /// Disable a synchronous peripheral clock
    #[inline]
    pub fn disable(mut self) -> ApbToken<A> {
        self.token.regs.disable();
        self.token
    }
}

//==============================================================================
// ApbClks
//==============================================================================

/// Helper macro providing a full type definition depending on a default state
/// of a synchronous peripheral clock
macro_rules! init_type {
    (enabled, $Type:ident) => { ApbClk<$Type> };
    (disabled, $Type:ident) => { ApbToken<$Type> };
}

/// Helper macro generating an appropriate initialization code depending on a
/// default state of a synchronous peripheral clock
macro_rules! init_expr {
    (enabled) => {
        ApbClk::new(ApbToken::new())
    };
    (disabled) => {
        ApbToken::new()
    };
}

/// Root level macro generating all type definitions and trait implementations.
/// Among other things, it defines and populates an [`ApbClks`] struct with
/// appropriate fields.
macro_rules! apb_clks {
    (
        $(
            $Reg:ident:
            [
                $(
                    $( #[$cfg:meta] )?
                    ($exists:literal, $Type:ident, $init:ident),
                )+
            ],
        )+
    ) => {
        paste! {
            $(
                $(
                    $( #[$cfg] )?
                    apb_type!($exists, $Reg, $Type);
                )+
            )+
            /// Struct aggregating [`ApbClk`] and [`ApbToken`] type instances
            /// representing default states of synchronous peripheral clocks
            #[allow(missing_docs)]
            pub struct ApbClks {
                $(
                    $(
                        $( #[$cfg] )?
                        pub [<$Type:lower>]: init_type!($init, $Type),
                    )+
                )+
            }
            impl ApbClks {
                pub(super) unsafe fn new() -> Self {
                    ApbClks {
                        $(
                            $(
                                $( #[$cfg] )?
                                [<$Type:lower>]: init_expr!($init),
                            )+
                        )+
                    }
                }
            }
        }
    };
}

apb_clks!(
    A:
    [
        (false, Pac, enabled),
        (false, Pm, enabled),
        (false, Mclk, enabled),
        (false, RstC, enabled),
        (false, OscCtrl, enabled),
        (false, Osc32kCtrl, enabled),
        (false, SupC, enabled),
        (false, Gclk, enabled),
        (false, Wdt, enabled),
        (false, Rtc, enabled),
        (false, Eic, enabled),
        (false, FreqM, disabled),
        (true, Sercom0, disabled),
        (true, Sercom1, disabled),
        (false, Tc0, disabled),
        (false, Tc1, disabled),
    ],
    B:
    [
        (false, Usb, disabled),
        (false, Dsu, enabled),
        (false, NvmCtrl, enabled),
        (false, Port, enabled),
        (false, EvSys, disabled),
        (true, Sercom2, disabled),
        (true, Sercom3, disabled),
        (false, Tcc0, disabled),
        (false, Tcc1, disabled),
        (false, Tc2, disabled),
        (false, Tc3, disabled),
        (false, RamEcc, enabled),
    ],
    C:
    [
        #[cfg(any(feature = "same53", feature = "same54"))]
        (false, Gmac, enabled),
        (false, Tcc2, disabled),
        #[cfg(feature = "min-samd51j")]
        (false, Tcc3, disabled),
        #[cfg(feature = "min-samd51j")]
        (false, Tc4, enabled),
        #[cfg(feature = "min-samd51j")]
        (false, Tc5, disabled),
        (false, Pdec, disabled),
        (false, Ac, disabled),
        (false, Aes, disabled),
        (false, Trng, disabled),
        (false, Icm, disabled),
        (false, Qspi, enabled),
        (false, Ccl, disabled),
    ],
    D:
    [
        (true, Sercom4, disabled),
        (true, Sercom5, disabled),
        #[cfg(feature = "min-samd51n")]
        (true, Sercom6, disabled),
        #[cfg(feature = "min-samd51n")]
        (true, Sercom7, disabled),
        #[cfg(feature = "min-samd51j")]
        (false, Tcc4, disabled),
        #[cfg(feature = "min-samd51n")]
        (false, Tc6, disabled),
        #[cfg(feature = "min-samd51n")]
        (false, Tc7, disabled),
        (false, Adc0, disabled),
        (false, Adc1, disabled),
        (false, Dac, disabled),
        #[cfg(feature = "min-samd51j")]
        (false, I2S, disabled),
        (false, Pcc, disabled),
    ],
);
