#[doc = "Reader of register PERBUF_DITH6_MODE"]
pub type R = crate::R<u32, super::PERBUF_DITH6_MODE>;
#[doc = "Writer for register PERBUF_DITH6_MODE"]
pub type W = crate::W<u32, super::PERBUF_DITH6_MODE>;
#[doc = "Register PERBUF_DITH6_MODE `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PERBUF_DITH6_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `DITHERBUF`"]
pub type DITHERBUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DITHERBUF`"]
pub struct DITHERBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHERBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `PERBUF`"]
pub type PERBUF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERBUF`"]
pub struct PERBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PERBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 6)) | (((value as u32) & 0x0003_ffff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&self) -> DITHERBUF_R {
        DITHERBUF_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&self) -> PERBUF_R {
        PERBUF_R::new(((self.bits >> 6) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&mut self) -> DITHERBUF_W {
        DITHERBUF_W { w: self }
    }
    #[doc = "Bits 6:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&mut self) -> PERBUF_W {
        PERBUF_W { w: self }
    }
}
