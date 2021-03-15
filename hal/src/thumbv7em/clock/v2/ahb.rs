use core::marker::PhantomData;

use paste::paste;

use crate::pac::{mclk, MCLK};

//==============================================================================
// Registers
//==============================================================================

/// TODO
struct Registers<A: AhbType> {
    ahb: PhantomData<A>,
    auto: PhantomData<*const ()>,
}

unsafe impl<A: AhbType> Sync for Registers<A> {}

impl<A: AhbType> Registers<A> {
    /// TODO
    #[inline(always)]
    unsafe fn new() -> Self {
        Registers {
            ahb: PhantomData,
            auto: PhantomData,
        }
    }

    #[inline(always)]
    fn mclk(&mut self) -> &mclk::RegisterBlock {
        unsafe { &*MCLK::ptr() }
    }

    #[inline(always)]
    fn ahbmask(&mut self) -> &mclk::AHBMASK {
        &self.mclk().ahbmask
    }
    /// TODO
    #[inline(always)]
    fn enable(&mut self) {
        self.ahbmask()
            .modify(|r, w| unsafe { w.bits(r.bits() | A::MASK) });
    }

    /// TODO
    #[inline(always)]
    fn disable(&mut self) {
        self.ahbmask()
            .modify(|r, w| unsafe { w.bits(r.bits() & !A::MASK) });
    }
}

//==============================================================================
// AhbType
//==============================================================================

/// TODO
pub trait AhbType {
    const ID: AhbId;
    const MASK: u32 = 1 << (Self::ID as u8);
}

/// TODO
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

/// TODO
macro_rules! ahb_type {
    // TODO
    (false, $Type:ident) => {
        pub enum $Type {}
        ahb_type!(true, $Type);
    };
    // TODO
    (true, $Type:ident) => {
        impl AhbType for $Type {
            const ID: AhbId = AhbId::$Type;
        }
    };
}

//==============================================================================
// AhbToken
//==============================================================================

/// TODO
pub struct AhbToken<A: AhbType> {
    regs: Registers<A>,
}

impl<A: AhbType> AhbToken<A> {
    /// TODO
    #[inline]
    unsafe fn new() -> Self {
        AhbToken {
            regs: Registers::new(),
        }
    }

    /// TODO
    #[inline]
    pub fn enable(mut self) -> AhbClk<A> {
        self.regs.enable();
        AhbClk::new(self)
    }
}

//==============================================================================
// AhbClk
//==============================================================================

/// TODO
pub struct AhbClk<A: AhbType> {
    token: AhbToken<A>,
}

impl<A: AhbType> AhbClk<A> {
    /// TODO
    #[inline]
    fn new(token: AhbToken<A>) -> Self {
        AhbClk { token }
    }

    /// TODO
    #[inline]
    pub fn disable(mut self) -> AhbToken<A> {
        self.token.regs.disable();
        self.token
    }
}

//==============================================================================
// AhbClks
//==============================================================================

/// TODO
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
            /// TODO
            pub struct AhbClks {
                $(
                    $( #[$cfg] )?
                    pub [<$Type:lower>]: AhbClk<$Type>,
                )+
            }
            impl AhbClks {
                /// TODO
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
