#[doc = "Reader of register YSELECTEN"]
pub type R = crate::R<u16, super::YSELECTEN>;
#[doc = "Writer for register YSELECTEN"]
pub type W = crate::W<u16, super::YSELECTEN>;
#[doc = "Register YSELECTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::YSELECTEN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Y0EN`"]
pub type Y0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y0EN`"]
pub struct Y0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y0EN_W<'a> {
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
#[doc = "Reader of field `Y1EN`"]
pub type Y1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y1EN`"]
pub struct Y1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y1EN_W<'a> {
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
#[doc = "Reader of field `Y2EN`"]
pub type Y2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y2EN`"]
pub struct Y2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y2EN_W<'a> {
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
#[doc = "Reader of field `Y3EN`"]
pub type Y3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y3EN`"]
pub struct Y3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y3EN_W<'a> {
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
#[doc = "Reader of field `Y4EN`"]
pub type Y4EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y4EN`"]
pub struct Y4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y4EN_W<'a> {
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
#[doc = "Reader of field `Y5EN`"]
pub type Y5EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y5EN`"]
pub struct Y5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y5EN_W<'a> {
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
#[doc = "Reader of field `Y6EN`"]
pub type Y6EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y6EN`"]
pub struct Y6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y6EN_W<'a> {
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
#[doc = "Reader of field `Y7EN`"]
pub type Y7EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y7EN`"]
pub struct Y7EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y7EN_W<'a> {
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
#[doc = "Reader of field `Y8EN`"]
pub type Y8EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y8EN`"]
pub struct Y8EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y8EN_W<'a> {
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
#[doc = "Reader of field `Y9EN`"]
pub type Y9EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y9EN`"]
pub struct Y9EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y9EN_W<'a> {
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
#[doc = "Reader of field `Y10EN`"]
pub type Y10EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y10EN`"]
pub struct Y10EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y10EN_W<'a> {
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
#[doc = "Reader of field `Y11EN`"]
pub type Y11EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y11EN`"]
pub struct Y11EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y11EN_W<'a> {
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
#[doc = "Reader of field `Y12EN`"]
pub type Y12EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y12EN`"]
pub struct Y12EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y12EN_W<'a> {
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
#[doc = "Reader of field `Y13EN`"]
pub type Y13EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y13EN`"]
pub struct Y13EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y13EN_W<'a> {
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
#[doc = "Reader of field `Y14EN`"]
pub type Y14EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y14EN`"]
pub struct Y14EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y14EN_W<'a> {
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
#[doc = "Reader of field `Y15EN`"]
pub type Y15EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Y15EN`"]
pub struct Y15EN_W<'a> {
    w: &'a mut W,
}
impl<'a> Y15EN_W<'a> {
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
    #[doc = "Bit 0 - PTC Y0 pin enable"]
    #[inline(always)]
    pub fn y0en(&self) -> Y0EN_R {
        Y0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PTC Y1 pin enable"]
    #[inline(always)]
    pub fn y1en(&self) -> Y1EN_R {
        Y1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PTC Y2 pin enable"]
    #[inline(always)]
    pub fn y2en(&self) -> Y2EN_R {
        Y2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PTC Y3 pin enable"]
    #[inline(always)]
    pub fn y3en(&self) -> Y3EN_R {
        Y3EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PTC Y4 pin enable"]
    #[inline(always)]
    pub fn y4en(&self) -> Y4EN_R {
        Y4EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PTC Y5 pin enable"]
    #[inline(always)]
    pub fn y5en(&self) -> Y5EN_R {
        Y5EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PTC Y6 pin enable"]
    #[inline(always)]
    pub fn y6en(&self) -> Y6EN_R {
        Y6EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PTC Y7 pin enable"]
    #[inline(always)]
    pub fn y7en(&self) -> Y7EN_R {
        Y7EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PTC Y8 pin enable"]
    #[inline(always)]
    pub fn y8en(&self) -> Y8EN_R {
        Y8EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PTC Y9 pin enable"]
    #[inline(always)]
    pub fn y9en(&self) -> Y9EN_R {
        Y9EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PTC Y10 pin enable"]
    #[inline(always)]
    pub fn y10en(&self) -> Y10EN_R {
        Y10EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PTC Y11 pin enable"]
    #[inline(always)]
    pub fn y11en(&self) -> Y11EN_R {
        Y11EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PTC Y12 pin enable"]
    #[inline(always)]
    pub fn y12en(&self) -> Y12EN_R {
        Y12EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PTC Y13 pin enable"]
    #[inline(always)]
    pub fn y13en(&self) -> Y13EN_R {
        Y13EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PTC Y14 pin enable"]
    #[inline(always)]
    pub fn y14en(&self) -> Y14EN_R {
        Y14EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PTC Y15 pin enable"]
    #[inline(always)]
    pub fn y15en(&self) -> Y15EN_R {
        Y15EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PTC Y0 pin enable"]
    #[inline(always)]
    pub fn y0en(&mut self) -> Y0EN_W {
        Y0EN_W { w: self }
    }
    #[doc = "Bit 1 - PTC Y1 pin enable"]
    #[inline(always)]
    pub fn y1en(&mut self) -> Y1EN_W {
        Y1EN_W { w: self }
    }
    #[doc = "Bit 2 - PTC Y2 pin enable"]
    #[inline(always)]
    pub fn y2en(&mut self) -> Y2EN_W {
        Y2EN_W { w: self }
    }
    #[doc = "Bit 3 - PTC Y3 pin enable"]
    #[inline(always)]
    pub fn y3en(&mut self) -> Y3EN_W {
        Y3EN_W { w: self }
    }
    #[doc = "Bit 4 - PTC Y4 pin enable"]
    #[inline(always)]
    pub fn y4en(&mut self) -> Y4EN_W {
        Y4EN_W { w: self }
    }
    #[doc = "Bit 5 - PTC Y5 pin enable"]
    #[inline(always)]
    pub fn y5en(&mut self) -> Y5EN_W {
        Y5EN_W { w: self }
    }
    #[doc = "Bit 6 - PTC Y6 pin enable"]
    #[inline(always)]
    pub fn y6en(&mut self) -> Y6EN_W {
        Y6EN_W { w: self }
    }
    #[doc = "Bit 7 - PTC Y7 pin enable"]
    #[inline(always)]
    pub fn y7en(&mut self) -> Y7EN_W {
        Y7EN_W { w: self }
    }
    #[doc = "Bit 8 - PTC Y8 pin enable"]
    #[inline(always)]
    pub fn y8en(&mut self) -> Y8EN_W {
        Y8EN_W { w: self }
    }
    #[doc = "Bit 9 - PTC Y9 pin enable"]
    #[inline(always)]
    pub fn y9en(&mut self) -> Y9EN_W {
        Y9EN_W { w: self }
    }
    #[doc = "Bit 10 - PTC Y10 pin enable"]
    #[inline(always)]
    pub fn y10en(&mut self) -> Y10EN_W {
        Y10EN_W { w: self }
    }
    #[doc = "Bit 11 - PTC Y11 pin enable"]
    #[inline(always)]
    pub fn y11en(&mut self) -> Y11EN_W {
        Y11EN_W { w: self }
    }
    #[doc = "Bit 12 - PTC Y12 pin enable"]
    #[inline(always)]
    pub fn y12en(&mut self) -> Y12EN_W {
        Y12EN_W { w: self }
    }
    #[doc = "Bit 13 - PTC Y13 pin enable"]
    #[inline(always)]
    pub fn y13en(&mut self) -> Y13EN_W {
        Y13EN_W { w: self }
    }
    #[doc = "Bit 14 - PTC Y14 pin enable"]
    #[inline(always)]
    pub fn y14en(&mut self) -> Y14EN_W {
        Y14EN_W { w: self }
    }
    #[doc = "Bit 15 - PTC Y15 pin enable"]
    #[inline(always)]
    pub fn y15en(&mut self) -> Y15EN_W {
        Y15EN_W { w: self }
    }
}
