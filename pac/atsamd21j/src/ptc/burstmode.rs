#[doc = "Reader of register BURSTMODE"]
pub type R = crate::R<u8, super::BURSTMODE>;
#[doc = "Writer for register BURSTMODE"]
pub type W = crate::W<u8, super::BURSTMODE>;
#[doc = "Register BURSTMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::BURSTMODE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTSLOWPOWEREN`"]
pub type CTSLOWPOWEREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSLOWPOWEREN`"]
pub struct CTSLOWPOWEREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSLOWPOWEREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `BURSTMODE`"]
pub type BURSTMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BURSTMODE`"]
pub struct BURSTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Enable clear to send low power mode"]
    #[inline(always)]
    pub fn ctslowpoweren(&self) -> CTSLOWPOWEREN_R {
        CTSLOWPOWEREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Burst mode setting"]
    #[inline(always)]
    pub fn burstmode(&self) -> BURSTMODE_R {
        BURSTMODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Enable clear to send low power mode"]
    #[inline(always)]
    pub fn ctslowpoweren(&mut self) -> CTSLOWPOWEREN_W {
        CTSLOWPOWEREN_W { w: self }
    }
    #[doc = "Bits 4:7 - Burst mode setting"]
    #[inline(always)]
    pub fn burstmode(&mut self) -> BURSTMODE_W {
        BURSTMODE_W { w: self }
    }
}
