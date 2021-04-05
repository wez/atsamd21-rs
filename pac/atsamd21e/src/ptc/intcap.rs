#[doc = "Reader of register INTCAP"]
pub type R = crate::R<u8, super::INTCAP>;
#[doc = "Writer for register INTCAP"]
pub type W = crate::W<u8, super::INTCAP>;
#[doc = "Register INTCAP `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCAP {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Value"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
