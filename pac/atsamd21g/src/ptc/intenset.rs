#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u8, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u8, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EOC`"]
pub type EOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOC`"]
pub struct EOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `WCO`"]
pub type WCO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WCO`"]
pub struct WCO_W<'a> {
    w: &'a mut W,
}
impl<'a> WCO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt on end of comparison"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watch crystal oscillator interrupt"]
    #[inline(always)]
    pub fn wco(&self) -> WCO_R {
        WCO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on end of comparison"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W {
        EOC_W { w: self }
    }
    #[doc = "Bit 1 - Watch crystal oscillator interrupt"]
    #[inline(always)]
    pub fn wco(&mut self) -> WCO_W {
        WCO_W { w: self }
    }
}
