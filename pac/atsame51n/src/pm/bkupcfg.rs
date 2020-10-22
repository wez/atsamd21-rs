#[doc = "Reader of register BKUPCFG"]
pub type R = crate::R<u8, super::BKUPCFG>;
#[doc = "Writer for register BKUPCFG"]
pub type W = crate::W<u8, super::BKUPCFG>;
#[doc = "Register BKUPCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::BKUPCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Ram Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRAMCFG_A {
    #[doc = "0: All the backup RAM is retained"]
    RET = 0,
    #[doc = "1: Only the first 4Kbytes of the backup RAM is retained"]
    PARTIAL = 1,
    #[doc = "2: All the backup RAM is turned OFF"]
    OFF = 2,
}
impl From<BRAMCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: BRAMCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BRAMCFG`"]
pub type BRAMCFG_R = crate::R<u8, BRAMCFG_A>;
impl BRAMCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BRAMCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BRAMCFG_A::RET),
            1 => Val(BRAMCFG_A::PARTIAL),
            2 => Val(BRAMCFG_A::OFF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RET`"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == BRAMCFG_A::RET
    }
    #[doc = "Checks if the value of the field is `PARTIAL`"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == BRAMCFG_A::PARTIAL
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BRAMCFG_A::OFF
    }
}
#[doc = "Write proxy for field `BRAMCFG`"]
pub struct BRAMCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> BRAMCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRAMCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All the backup RAM is retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut W {
        self.variant(BRAMCFG_A::RET)
    }
    #[doc = "Only the first 4Kbytes of the backup RAM is retained"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut W {
        self.variant(BRAMCFG_A::PARTIAL)
    }
    #[doc = "All the backup RAM is turned OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(BRAMCFG_A::OFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn bramcfg(&self) -> BRAMCFG_R {
        BRAMCFG_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn bramcfg(&mut self) -> BRAMCFG_W {
        BRAMCFG_W { w: self }
    }
}
