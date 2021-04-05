#[doc = "Reader of register FREQCTRL"]
pub type R = crate::R<u8, super::FREQCTRL>;
#[doc = "Writer for register FREQCTRL"]
pub type W = crate::W<u8, super::FREQCTRL>;
#[doc = "Register FREQCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FREQCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sample delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAMPLEDELAY_A {
    #[doc = "0: `0`"]
    FREQHOP1 = 0,
    #[doc = "1: `1`"]
    FREQHOP2 = 1,
    #[doc = "2: `10`"]
    FREQHOP3 = 2,
    #[doc = "3: `11`"]
    FREQHOP4 = 3,
    #[doc = "4: `100`"]
    FREQHOP5 = 4,
    #[doc = "5: `101`"]
    FREQHOP6 = 5,
    #[doc = "6: `110`"]
    FREQHOP7 = 6,
    #[doc = "7: `111`"]
    FREQHOP8 = 7,
    #[doc = "8: `1000`"]
    FREQHOP9 = 8,
    #[doc = "9: `1001`"]
    FREQHOP10 = 9,
    #[doc = "10: `1010`"]
    FREQHOP11 = 10,
    #[doc = "11: `1011`"]
    FREQHOP12 = 11,
    #[doc = "12: `1100`"]
    FREQHOP13 = 12,
    #[doc = "13: `1101`"]
    FREQHOP14 = 13,
    #[doc = "14: `1110`"]
    FREQHOP15 = 14,
    #[doc = "15: `1111`"]
    FREQHOP16 = 15,
}
impl From<SAMPLEDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPLEDELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAMPLEDELAY`"]
pub type SAMPLEDELAY_R = crate::R<u8, SAMPLEDELAY_A>;
impl SAMPLEDELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLEDELAY_A {
        match self.bits {
            0 => SAMPLEDELAY_A::FREQHOP1,
            1 => SAMPLEDELAY_A::FREQHOP2,
            2 => SAMPLEDELAY_A::FREQHOP3,
            3 => SAMPLEDELAY_A::FREQHOP4,
            4 => SAMPLEDELAY_A::FREQHOP5,
            5 => SAMPLEDELAY_A::FREQHOP6,
            6 => SAMPLEDELAY_A::FREQHOP7,
            7 => SAMPLEDELAY_A::FREQHOP8,
            8 => SAMPLEDELAY_A::FREQHOP9,
            9 => SAMPLEDELAY_A::FREQHOP10,
            10 => SAMPLEDELAY_A::FREQHOP11,
            11 => SAMPLEDELAY_A::FREQHOP12,
            12 => SAMPLEDELAY_A::FREQHOP13,
            13 => SAMPLEDELAY_A::FREQHOP14,
            14 => SAMPLEDELAY_A::FREQHOP15,
            15 => SAMPLEDELAY_A::FREQHOP16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREQHOP1`"]
    #[inline(always)]
    pub fn is_freqhop1(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP1
    }
    #[doc = "Checks if the value of the field is `FREQHOP2`"]
    #[inline(always)]
    pub fn is_freqhop2(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP2
    }
    #[doc = "Checks if the value of the field is `FREQHOP3`"]
    #[inline(always)]
    pub fn is_freqhop3(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP3
    }
    #[doc = "Checks if the value of the field is `FREQHOP4`"]
    #[inline(always)]
    pub fn is_freqhop4(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP4
    }
    #[doc = "Checks if the value of the field is `FREQHOP5`"]
    #[inline(always)]
    pub fn is_freqhop5(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP5
    }
    #[doc = "Checks if the value of the field is `FREQHOP6`"]
    #[inline(always)]
    pub fn is_freqhop6(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP6
    }
    #[doc = "Checks if the value of the field is `FREQHOP7`"]
    #[inline(always)]
    pub fn is_freqhop7(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP7
    }
    #[doc = "Checks if the value of the field is `FREQHOP8`"]
    #[inline(always)]
    pub fn is_freqhop8(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP8
    }
    #[doc = "Checks if the value of the field is `FREQHOP9`"]
    #[inline(always)]
    pub fn is_freqhop9(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP9
    }
    #[doc = "Checks if the value of the field is `FREQHOP10`"]
    #[inline(always)]
    pub fn is_freqhop10(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP10
    }
    #[doc = "Checks if the value of the field is `FREQHOP11`"]
    #[inline(always)]
    pub fn is_freqhop11(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP11
    }
    #[doc = "Checks if the value of the field is `FREQHOP12`"]
    #[inline(always)]
    pub fn is_freqhop12(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP12
    }
    #[doc = "Checks if the value of the field is `FREQHOP13`"]
    #[inline(always)]
    pub fn is_freqhop13(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP13
    }
    #[doc = "Checks if the value of the field is `FREQHOP14`"]
    #[inline(always)]
    pub fn is_freqhop14(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP14
    }
    #[doc = "Checks if the value of the field is `FREQHOP15`"]
    #[inline(always)]
    pub fn is_freqhop15(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP15
    }
    #[doc = "Checks if the value of the field is `FREQHOP16`"]
    #[inline(always)]
    pub fn is_freqhop16(&self) -> bool {
        *self == SAMPLEDELAY_A::FREQHOP16
    }
}
#[doc = "Write proxy for field `SAMPLEDELAY`"]
pub struct SAMPLEDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLEDELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLEDELAY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn freqhop1(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn freqhop2(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn freqhop3(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn freqhop4(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn freqhop5(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP5)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn freqhop6(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP6)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn freqhop7(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP7)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn freqhop8(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP8)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn freqhop9(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP9)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn freqhop10(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP10)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn freqhop11(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP11)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn freqhop12(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP12)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn freqhop13(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP13)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn freqhop14(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP14)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn freqhop15(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP15)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn freqhop16(self) -> &'a mut W {
        self.variant(SAMPLEDELAY_A::FREQHOP16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `FREQSPREADEN`"]
pub type FREQSPREADEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FREQSPREADEN`"]
pub struct FREQSPREADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQSPREADEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Sample delay"]
    #[inline(always)]
    pub fn sampledelay(&self) -> SAMPLEDELAY_R {
        SAMPLEDELAY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable frequency spread"]
    #[inline(always)]
    pub fn freqspreaden(&self) -> FREQSPREADEN_R {
        FREQSPREADEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sample delay"]
    #[inline(always)]
    pub fn sampledelay(&mut self) -> SAMPLEDELAY_W {
        SAMPLEDELAY_W { w: self }
    }
    #[doc = "Bit 4 - Enable frequency spread"]
    #[inline(always)]
    pub fn freqspreaden(&mut self) -> FREQSPREADEN_W {
        FREQSPREADEN_W { w: self }
    }
}
