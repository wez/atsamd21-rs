#[doc = "Reader of register CONVCTRL"]
pub type R = crate::R<u8, super::CONVCTRL>;
#[doc = "Writer for register CONVCTRL"]
pub type W = crate::W<u8, super::CONVCTRL>;
#[doc = "Register CONVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CONVCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC Accumulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCACCUM_A {
    #[doc = "0: 1 sample per report"]
    OVERSAMPLE1 = 0,
    #[doc = "1: 2 samples per report"]
    OVERSAMPLE2 = 1,
    #[doc = "2: 4 samples per report"]
    OVERSAMPLE4 = 2,
    #[doc = "3: 8 samples per report"]
    OVERSAMPLE8 = 3,
    #[doc = "4: 16 samples per report"]
    OVERSAMPLE16 = 4,
    #[doc = "5: 32 samples per report"]
    OVERSAMPLE32 = 5,
    #[doc = "6: 64 samples per report"]
    OVERSAMPLE64 = 6,
}
impl From<ADCACCUM_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCACCUM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCACCUM`"]
pub type ADCACCUM_R = crate::R<u8, ADCACCUM_A>;
impl ADCACCUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADCACCUM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADCACCUM_A::OVERSAMPLE1),
            1 => Val(ADCACCUM_A::OVERSAMPLE2),
            2 => Val(ADCACCUM_A::OVERSAMPLE4),
            3 => Val(ADCACCUM_A::OVERSAMPLE8),
            4 => Val(ADCACCUM_A::OVERSAMPLE16),
            5 => Val(ADCACCUM_A::OVERSAMPLE32),
            6 => Val(ADCACCUM_A::OVERSAMPLE64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OVERSAMPLE1`"]
    #[inline(always)]
    pub fn is_oversample1(&self) -> bool {
        *self == ADCACCUM_A::OVERSAMPLE1
    }
    #[doc = "Checks if the value of the field is `OVERSAMPLE2`"]
    #[inline(always)]
    pub fn is_oversample2(&self) -> bool {
        *self == ADCACCUM_A::OVERSAMPLE2
    }
    #[doc = "Checks if the value of the field is `OVERSAMPLE4`"]
    #[inline(always)]
    pub fn is_oversample4(&self) -> bool {
        *self == ADCACCUM_A::OVERSAMPLE4
    }
    #[doc = "Checks if the value of the field is `OVERSAMPLE8`"]
    #[inline(always)]
    pub fn is_oversample8(&self) -> bool {
        *self == ADCACCUM_A::OVERSAMPLE8
    }
    #[doc = "Checks if the value of the field is `OVERSAMPLE16`"]
    #[inline(always)]
    pub fn is_oversample16(&self) -> bool {
        *self == ADCACCUM_A::OVERSAMPLE16
    }
    #[doc = "Checks if the value of the field is `OVERSAMPLE32`"]
    #[inline(always)]
    pub fn is_oversample32(&self) -> bool {
        *self == ADCACCUM_A::OVERSAMPLE32
    }
    #[doc = "Checks if the value of the field is `OVERSAMPLE64`"]
    #[inline(always)]
    pub fn is_oversample64(&self) -> bool {
        *self == ADCACCUM_A::OVERSAMPLE64
    }
}
#[doc = "Write proxy for field `ADCACCUM`"]
pub struct ADCACCUM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCACCUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCACCUM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 sample per report"]
    #[inline(always)]
    pub fn oversample1(self) -> &'a mut W {
        self.variant(ADCACCUM_A::OVERSAMPLE1)
    }
    #[doc = "2 samples per report"]
    #[inline(always)]
    pub fn oversample2(self) -> &'a mut W {
        self.variant(ADCACCUM_A::OVERSAMPLE2)
    }
    #[doc = "4 samples per report"]
    #[inline(always)]
    pub fn oversample4(self) -> &'a mut W {
        self.variant(ADCACCUM_A::OVERSAMPLE4)
    }
    #[doc = "8 samples per report"]
    #[inline(always)]
    pub fn oversample8(self) -> &'a mut W {
        self.variant(ADCACCUM_A::OVERSAMPLE8)
    }
    #[doc = "16 samples per report"]
    #[inline(always)]
    pub fn oversample16(self) -> &'a mut W {
        self.variant(ADCACCUM_A::OVERSAMPLE16)
    }
    #[doc = "32 samples per report"]
    #[inline(always)]
    pub fn oversample32(self) -> &'a mut W {
        self.variant(ADCACCUM_A::OVERSAMPLE32)
    }
    #[doc = "64 samples per report"]
    #[inline(always)]
    pub fn oversample64(self) -> &'a mut W {
        self.variant(ADCACCUM_A::OVERSAMPLE64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CONVERT`"]
pub type CONVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONVERT`"]
pub struct CONVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONVERT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - ADC Accumulator"]
    #[inline(always)]
    pub fn adcaccum(&self) -> ADCACCUM_R {
        ADCACCUM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - Start conversion"]
    #[inline(always)]
    pub fn convert(&self) -> CONVERT_R {
        CONVERT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC Accumulator"]
    #[inline(always)]
    pub fn adcaccum(&mut self) -> ADCACCUM_W {
        ADCACCUM_W { w: self }
    }
    #[doc = "Bit 7 - Start conversion"]
    #[inline(always)]
    pub fn convert(&mut self) -> CONVERT_W {
        CONVERT_W { w: self }
    }
}
