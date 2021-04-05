#[doc = "Reader of register COMPCAP"]
pub type R = crate::R<u16, super::COMPCAP>;
#[doc = "Writer for register COMPCAP"]
pub type W = crate::W<u16, super::COMPCAP>;
#[doc = "Register COMPCAP `reset()`'s with value 0"]
impl crate::ResetValue for super::COMPCAP {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u16) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Value"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
