#[doc = "Reader of register YSELECT"]
pub type R = crate::R<u16, super::YSELECT>;
#[doc = "Writer for register YSELECT"]
pub type W = crate::W<u16, super::YSELECT>;
#[doc = "Register YSELECT `reset()`'s with value 0"]
impl crate::ResetValue for super::YSELECT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Y line selection MUX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum YMUX_A {
    #[doc = "1: PTC Y0 Pin"]
    Y0 = 1,
    #[doc = "2: PTC Y1 Pin"]
    Y1 = 2,
    #[doc = "4: PTC Y2 Pin"]
    Y2 = 4,
    #[doc = "8: PTC Y3 Pin"]
    Y3 = 8,
    #[doc = "16: PTC Y4 Pin"]
    Y4 = 16,
    #[doc = "32: PTC Y5 Pin"]
    Y5 = 32,
    #[doc = "64: PTC Y6 Pin"]
    Y6 = 64,
    #[doc = "128: PTC Y7 Pin"]
    Y7 = 128,
    #[doc = "256: PTC Y8 Pin"]
    Y8 = 256,
    #[doc = "512: PTC Y9 Pin"]
    Y9 = 512,
    #[doc = "1024: PTC Y10 Pin"]
    Y10 = 1024,
    #[doc = "2048: PTC Y11 Pin"]
    Y11 = 2048,
    #[doc = "4096: PTC Y12 Pin"]
    Y12 = 4096,
    #[doc = "8192: PTC Y13 Pin"]
    Y13 = 8192,
    #[doc = "16384: PTC Y14 Pin"]
    Y14 = 16384,
    #[doc = "32768: PTC Y15 Pin"]
    Y15 = 32768,
}
impl From<YMUX_A> for u16 {
    #[inline(always)]
    fn from(variant: YMUX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `YMUX`"]
pub type YMUX_R = crate::R<u16, YMUX_A>;
impl YMUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, YMUX_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(YMUX_A::Y0),
            2 => Val(YMUX_A::Y1),
            4 => Val(YMUX_A::Y2),
            8 => Val(YMUX_A::Y3),
            16 => Val(YMUX_A::Y4),
            32 => Val(YMUX_A::Y5),
            64 => Val(YMUX_A::Y6),
            128 => Val(YMUX_A::Y7),
            256 => Val(YMUX_A::Y8),
            512 => Val(YMUX_A::Y9),
            1024 => Val(YMUX_A::Y10),
            2048 => Val(YMUX_A::Y11),
            4096 => Val(YMUX_A::Y12),
            8192 => Val(YMUX_A::Y13),
            16384 => Val(YMUX_A::Y14),
            32768 => Val(YMUX_A::Y15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `Y0`"]
    #[inline(always)]
    pub fn is_y0(&self) -> bool {
        *self == YMUX_A::Y0
    }
    #[doc = "Checks if the value of the field is `Y1`"]
    #[inline(always)]
    pub fn is_y1(&self) -> bool {
        *self == YMUX_A::Y1
    }
    #[doc = "Checks if the value of the field is `Y2`"]
    #[inline(always)]
    pub fn is_y2(&self) -> bool {
        *self == YMUX_A::Y2
    }
    #[doc = "Checks if the value of the field is `Y3`"]
    #[inline(always)]
    pub fn is_y3(&self) -> bool {
        *self == YMUX_A::Y3
    }
    #[doc = "Checks if the value of the field is `Y4`"]
    #[inline(always)]
    pub fn is_y4(&self) -> bool {
        *self == YMUX_A::Y4
    }
    #[doc = "Checks if the value of the field is `Y5`"]
    #[inline(always)]
    pub fn is_y5(&self) -> bool {
        *self == YMUX_A::Y5
    }
    #[doc = "Checks if the value of the field is `Y6`"]
    #[inline(always)]
    pub fn is_y6(&self) -> bool {
        *self == YMUX_A::Y6
    }
    #[doc = "Checks if the value of the field is `Y7`"]
    #[inline(always)]
    pub fn is_y7(&self) -> bool {
        *self == YMUX_A::Y7
    }
    #[doc = "Checks if the value of the field is `Y8`"]
    #[inline(always)]
    pub fn is_y8(&self) -> bool {
        *self == YMUX_A::Y8
    }
    #[doc = "Checks if the value of the field is `Y9`"]
    #[inline(always)]
    pub fn is_y9(&self) -> bool {
        *self == YMUX_A::Y9
    }
    #[doc = "Checks if the value of the field is `Y10`"]
    #[inline(always)]
    pub fn is_y10(&self) -> bool {
        *self == YMUX_A::Y10
    }
    #[doc = "Checks if the value of the field is `Y11`"]
    #[inline(always)]
    pub fn is_y11(&self) -> bool {
        *self == YMUX_A::Y11
    }
    #[doc = "Checks if the value of the field is `Y12`"]
    #[inline(always)]
    pub fn is_y12(&self) -> bool {
        *self == YMUX_A::Y12
    }
    #[doc = "Checks if the value of the field is `Y13`"]
    #[inline(always)]
    pub fn is_y13(&self) -> bool {
        *self == YMUX_A::Y13
    }
    #[doc = "Checks if the value of the field is `Y14`"]
    #[inline(always)]
    pub fn is_y14(&self) -> bool {
        *self == YMUX_A::Y14
    }
    #[doc = "Checks if the value of the field is `Y15`"]
    #[inline(always)]
    pub fn is_y15(&self) -> bool {
        *self == YMUX_A::Y15
    }
}
#[doc = "Write proxy for field `YMUX`"]
pub struct YMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> YMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: YMUX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PTC Y0 Pin"]
    #[inline(always)]
    pub fn y0(self) -> &'a mut W {
        self.variant(YMUX_A::Y0)
    }
    #[doc = "PTC Y1 Pin"]
    #[inline(always)]
    pub fn y1(self) -> &'a mut W {
        self.variant(YMUX_A::Y1)
    }
    #[doc = "PTC Y2 Pin"]
    #[inline(always)]
    pub fn y2(self) -> &'a mut W {
        self.variant(YMUX_A::Y2)
    }
    #[doc = "PTC Y3 Pin"]
    #[inline(always)]
    pub fn y3(self) -> &'a mut W {
        self.variant(YMUX_A::Y3)
    }
    #[doc = "PTC Y4 Pin"]
    #[inline(always)]
    pub fn y4(self) -> &'a mut W {
        self.variant(YMUX_A::Y4)
    }
    #[doc = "PTC Y5 Pin"]
    #[inline(always)]
    pub fn y5(self) -> &'a mut W {
        self.variant(YMUX_A::Y5)
    }
    #[doc = "PTC Y6 Pin"]
    #[inline(always)]
    pub fn y6(self) -> &'a mut W {
        self.variant(YMUX_A::Y6)
    }
    #[doc = "PTC Y7 Pin"]
    #[inline(always)]
    pub fn y7(self) -> &'a mut W {
        self.variant(YMUX_A::Y7)
    }
    #[doc = "PTC Y8 Pin"]
    #[inline(always)]
    pub fn y8(self) -> &'a mut W {
        self.variant(YMUX_A::Y8)
    }
    #[doc = "PTC Y9 Pin"]
    #[inline(always)]
    pub fn y9(self) -> &'a mut W {
        self.variant(YMUX_A::Y9)
    }
    #[doc = "PTC Y10 Pin"]
    #[inline(always)]
    pub fn y10(self) -> &'a mut W {
        self.variant(YMUX_A::Y10)
    }
    #[doc = "PTC Y11 Pin"]
    #[inline(always)]
    pub fn y11(self) -> &'a mut W {
        self.variant(YMUX_A::Y11)
    }
    #[doc = "PTC Y12 Pin"]
    #[inline(always)]
    pub fn y12(self) -> &'a mut W {
        self.variant(YMUX_A::Y12)
    }
    #[doc = "PTC Y13 Pin"]
    #[inline(always)]
    pub fn y13(self) -> &'a mut W {
        self.variant(YMUX_A::Y13)
    }
    #[doc = "PTC Y14 Pin"]
    #[inline(always)]
    pub fn y14(self) -> &'a mut W {
        self.variant(YMUX_A::Y14)
    }
    #[doc = "PTC Y15 Pin"]
    #[inline(always)]
    pub fn y15(self) -> &'a mut W {
        self.variant(YMUX_A::Y15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Y line selection MUX"]
    #[inline(always)]
    pub fn ymux(&self) -> YMUX_R {
        YMUX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Y line selection MUX"]
    #[inline(always)]
    pub fn ymux(&mut self) -> YMUX_W {
        YMUX_W { w: self }
    }
}
