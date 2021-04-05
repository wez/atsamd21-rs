#[doc = "Reader of register SERRES"]
pub type R = crate::R<u8, super::SERRES>;
#[doc = "Writer for register SERRES"]
pub type W = crate::W<u8, super::SERRES>;
#[doc = "Register SERRES `reset()`'s with value 0"]
impl crate::ResetValue for super::SERRES {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Resistor value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESISTOR_A {
    #[doc = "0: No series resistor"]
    RES0 = 0,
    #[doc = "1: 20 kiloohm series resistor"]
    RES20K = 1,
    #[doc = "2: 50 kiloohm series resistor"]
    RES50K = 2,
    #[doc = "3: 100 kiloohm series resistor"]
    RES100K = 3,
}
impl From<RESISTOR_A> for u8 {
    #[inline(always)]
    fn from(variant: RESISTOR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RESISTOR`"]
pub type RESISTOR_R = crate::R<u8, RESISTOR_A>;
impl RESISTOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESISTOR_A {
        match self.bits {
            0 => RESISTOR_A::RES0,
            1 => RESISTOR_A::RES20K,
            2 => RESISTOR_A::RES50K,
            3 => RESISTOR_A::RES100K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == RESISTOR_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES20K`"]
    #[inline(always)]
    pub fn is_res20k(&self) -> bool {
        *self == RESISTOR_A::RES20K
    }
    #[doc = "Checks if the value of the field is `RES50K`"]
    #[inline(always)]
    pub fn is_res50k(&self) -> bool {
        *self == RESISTOR_A::RES50K
    }
    #[doc = "Checks if the value of the field is `RES100K`"]
    #[inline(always)]
    pub fn is_res100k(&self) -> bool {
        *self == RESISTOR_A::RES100K
    }
}
#[doc = "Write proxy for field `RESISTOR`"]
pub struct RESISTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESISTOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESISTOR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No series resistor"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(RESISTOR_A::RES0)
    }
    #[doc = "20 kiloohm series resistor"]
    #[inline(always)]
    pub fn res20k(self) -> &'a mut W {
        self.variant(RESISTOR_A::RES20K)
    }
    #[doc = "50 kiloohm series resistor"]
    #[inline(always)]
    pub fn res50k(self) -> &'a mut W {
        self.variant(RESISTOR_A::RES50K)
    }
    #[doc = "100 kiloohm series resistor"]
    #[inline(always)]
    pub fn res100k(self) -> &'a mut W {
        self.variant(RESISTOR_A::RES100K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Resistor value"]
    #[inline(always)]
    pub fn resistor(&self) -> RESISTOR_R {
        RESISTOR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Resistor value"]
    #[inline(always)]
    pub fn resistor(&mut self) -> RESISTOR_W {
        RESISTOR_W { w: self }
    }
}
