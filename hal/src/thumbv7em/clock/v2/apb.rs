use core::marker::PhantomData;

use paste::paste;

use crate::pac::{mclk, MCLK};

use crate::sercom::v2::*;

//==============================================================================
// Registers
//==============================================================================

/// TODO
struct Registers<A: ApbType> {
    apb: PhantomData<A>,
    auto: PhantomData<*const ()>,
}

unsafe impl<A: ApbType> Sync for Registers<A> {}

impl<A: ApbType> Registers<A> {
    /// TODO
    #[inline(always)]
    unsafe fn new() -> Self {
        Registers {
            apb: PhantomData,
            auto: PhantomData,
        }
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

    /// TODO
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

    /// TODO
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

/// TODO
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

/// TODO
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

/// TODO
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

/// TODO
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

/// TODO
pub enum ApbId {
    A(ApbAId),
    B(ApbBId),
    C(ApbCId),
    D(ApbDId),
}

//==============================================================================
// ApbType
//==============================================================================

/// TODO
pub trait ApbType {
    const ID: ApbId;
    const MASK: u32 = 1
        << match Self::ID {
            ApbId::A(id) => id as u8,
            ApbId::B(id) => id as u8,
            ApbId::C(id) => id as u8,
            ApbId::D(id) => id as u8,
        };
}

/// TODO
macro_rules! apb_type {
    // TODO
    (false, $Reg:ident, $Type:ident) => {
        pub enum $Type {}
        apb_type!(true, $Reg, $Type);
    };
    // TODO
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

/// TODO
pub struct ApbToken<A: ApbType> {
    regs: Registers<A>,
}

impl<A: ApbType> ApbToken<A> {
    /// TODO
    #[inline]
    unsafe fn new() -> Self {
        ApbToken {
            regs: Registers::new(),
        }
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> ApbClk<A> {
        self.regs.enable();
        ApbClk::new(self)
    }
}

//==============================================================================
// ApbClk
//==============================================================================

/// TODO
pub struct ApbClk<A: ApbType> {
    token: ApbToken<A>,
}

impl<A: ApbType> ApbClk<A> {
    /// TODO
    #[inline]
    fn new(token: ApbToken<A>) -> Self {
        ApbClk { token }
    }

    /// TODO
    #[inline]
    pub fn disable(mut self) -> ApbToken<A> {
        self.token.regs.disable();
        self.token
    }
}

//==============================================================================
// ApbClks
//==============================================================================

/// TODO
macro_rules! init_type {
    (enabled, $Type:ident) => { ApbClk<$Type> };
    (disabled, $Type:ident) => { ApbToken<$Type> };
}

/// TODO
macro_rules! init_expr {
    (enabled) => {
        ApbClk::new(ApbToken::new())
    };
    (disabled) => {
        ApbToken::new()
    };
}

/// TODO
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
            /// TODO
            pub struct ApbClks {
                $(
                    $(
                        $( #[$cfg] )?
                        pub [<$Type:lower>]: init_type!($init, $Type),
                    )+
                )+
            }
            impl ApbClks {
                /// TODO
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
