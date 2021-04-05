#[doc = "Reader of register XSELECTEN"]
pub type R = crate::R<u16, super::XSELECTEN>;
#[doc = "Writer for register XSELECTEN"]
pub type W = crate::W<u16, super::XSELECTEN>;
#[doc = "Register XSELECTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::XSELECTEN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `X0EN`"]
pub type X0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X0EN`"]
pub struct X0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X0EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `X1EN`"]
pub type X1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X1EN`"]
pub struct X1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `X2EN`"]
pub type X2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X2EN`"]
pub struct X2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `X3EN`"]
pub type X3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X3EN`"]
pub struct X3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X3EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `X4EN`"]
pub type X4EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X4EN`"]
pub struct X4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X4EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `X5EN`"]
pub type X5EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X5EN`"]
pub struct X5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X5EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `X6EN`"]
pub type X6EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X6EN`"]
pub struct X6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X6EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `X7EN`"]
pub type X7EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X7EN`"]
pub struct X7EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X7EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `X8EN`"]
pub type X8EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X8EN`"]
pub struct X8EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X8EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `X9EN`"]
pub type X9EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X9EN`"]
pub struct X9EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X9EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `X10EN`"]
pub type X10EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X10EN`"]
pub struct X10EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X10EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `X11EN`"]
pub type X11EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X11EN`"]
pub struct X11EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X11EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `X12EN`"]
pub type X12EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X12EN`"]
pub struct X12EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X12EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `X13EN`"]
pub type X13EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X13EN`"]
pub struct X13EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X13EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `X14EN`"]
pub type X14EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X14EN`"]
pub struct X14EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X14EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `X15EN`"]
pub type X15EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X15EN`"]
pub struct X15EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X15EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PTC X0 pin enable"]
    #[inline(always)]
    pub fn x0en(&self) -> X0EN_R {
        X0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PTC X1 pin enable"]
    #[inline(always)]
    pub fn x1en(&self) -> X1EN_R {
        X1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PTC X2 pin enable"]
    #[inline(always)]
    pub fn x2en(&self) -> X2EN_R {
        X2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PTC X3 pin enable"]
    #[inline(always)]
    pub fn x3en(&self) -> X3EN_R {
        X3EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PTC X4 pin enable"]
    #[inline(always)]
    pub fn x4en(&self) -> X4EN_R {
        X4EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PTC X5 pin enable"]
    #[inline(always)]
    pub fn x5en(&self) -> X5EN_R {
        X5EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PTC X6 pin enable"]
    #[inline(always)]
    pub fn x6en(&self) -> X6EN_R {
        X6EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PTC X7 pin enable"]
    #[inline(always)]
    pub fn x7en(&self) -> X7EN_R {
        X7EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PTC X8 pin enable"]
    #[inline(always)]
    pub fn x8en(&self) -> X8EN_R {
        X8EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PTC X9 pin enable"]
    #[inline(always)]
    pub fn x9en(&self) -> X9EN_R {
        X9EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PTC X10 pin enable"]
    #[inline(always)]
    pub fn x10en(&self) -> X10EN_R {
        X10EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PTC X11 pin enable"]
    #[inline(always)]
    pub fn x11en(&self) -> X11EN_R {
        X11EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PTC X12 pin enable"]
    #[inline(always)]
    pub fn x12en(&self) -> X12EN_R {
        X12EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PTC X13 pin enable"]
    #[inline(always)]
    pub fn x13en(&self) -> X13EN_R {
        X13EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PTC X14 pin enable"]
    #[inline(always)]
    pub fn x14en(&self) -> X14EN_R {
        X14EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PTC X15 pin enable"]
    #[inline(always)]
    pub fn x15en(&self) -> X15EN_R {
        X15EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PTC X0 pin enable"]
    #[inline(always)]
    pub fn x0en(&mut self) -> X0EN_W {
        X0EN_W { w: self }
    }
    #[doc = "Bit 1 - PTC X1 pin enable"]
    #[inline(always)]
    pub fn x1en(&mut self) -> X1EN_W {
        X1EN_W { w: self }
    }
    #[doc = "Bit 2 - PTC X2 pin enable"]
    #[inline(always)]
    pub fn x2en(&mut self) -> X2EN_W {
        X2EN_W { w: self }
    }
    #[doc = "Bit 3 - PTC X3 pin enable"]
    #[inline(always)]
    pub fn x3en(&mut self) -> X3EN_W {
        X3EN_W { w: self }
    }
    #[doc = "Bit 4 - PTC X4 pin enable"]
    #[inline(always)]
    pub fn x4en(&mut self) -> X4EN_W {
        X4EN_W { w: self }
    }
    #[doc = "Bit 5 - PTC X5 pin enable"]
    #[inline(always)]
    pub fn x5en(&mut self) -> X5EN_W {
        X5EN_W { w: self }
    }
    #[doc = "Bit 6 - PTC X6 pin enable"]
    #[inline(always)]
    pub fn x6en(&mut self) -> X6EN_W {
        X6EN_W { w: self }
    }
    #[doc = "Bit 7 - PTC X7 pin enable"]
    #[inline(always)]
    pub fn x7en(&mut self) -> X7EN_W {
        X7EN_W { w: self }
    }
    #[doc = "Bit 8 - PTC X8 pin enable"]
    #[inline(always)]
    pub fn x8en(&mut self) -> X8EN_W {
        X8EN_W { w: self }
    }
    #[doc = "Bit 9 - PTC X9 pin enable"]
    #[inline(always)]
    pub fn x9en(&mut self) -> X9EN_W {
        X9EN_W { w: self }
    }
    #[doc = "Bit 10 - PTC X10 pin enable"]
    #[inline(always)]
    pub fn x10en(&mut self) -> X10EN_W {
        X10EN_W { w: self }
    }
    #[doc = "Bit 11 - PTC X11 pin enable"]
    #[inline(always)]
    pub fn x11en(&mut self) -> X11EN_W {
        X11EN_W { w: self }
    }
    #[doc = "Bit 12 - PTC X12 pin enable"]
    #[inline(always)]
    pub fn x12en(&mut self) -> X12EN_W {
        X12EN_W { w: self }
    }
    #[doc = "Bit 13 - PTC X13 pin enable"]
    #[inline(always)]
    pub fn x13en(&mut self) -> X13EN_W {
        X13EN_W { w: self }
    }
    #[doc = "Bit 14 - PTC X14 pin enable"]
    #[inline(always)]
    pub fn x14en(&mut self) -> X14EN_W {
        X14EN_W { w: self }
    }
    #[doc = "Bit 15 - PTC X15 pin enable"]
    #[inline(always)]
    pub fn x15en(&mut self) -> X15EN_W {
        X15EN_W { w: self }
    }
}
