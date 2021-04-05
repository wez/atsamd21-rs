#[doc = "Reader of register CTRLB"]
pub type R = crate::R<u8, super::CTRLB>;
#[doc = "Writer for register CTRLB"]
pub type W = crate::W<u8, super::CTRLB>;
#[doc = "Register CTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLB {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYNCFLAG`"]
pub type SYNCFLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCFLAG`"]
pub struct SYNCFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCFLAG_W<'a> {
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
    #[doc = "Bit 7 - Synchronisation flag"]
    #[inline(always)]
    pub fn syncflag(&self) -> SYNCFLAG_R {
        SYNCFLAG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Synchronisation flag"]
    #[inline(always)]
    pub fn syncflag(&mut self) -> SYNCFLAG_W {
        SYNCFLAG_W { w: self }
    }
}
