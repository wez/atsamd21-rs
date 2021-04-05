#[doc = "Reader of register XSELECT"]
pub type R = crate::R<u16, super::XSELECT>;
#[doc = "Writer for register XSELECT"]
pub type W = crate::W<u16, super::XSELECT>;
#[doc = "Register XSELECT `reset()`'s with value 0"]
impl crate::ResetValue for super::XSELECT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "X line selection MUX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum XMUX_A {
    #[doc = "1: PTC X0 Pin"]
    X0 = 1,
    #[doc = "2: PTC X1 Pin"]
    X1 = 2,
    #[doc = "4: PTC X2 Pin"]
    X2 = 4,
    #[doc = "8: PTC X3 Pin"]
    X3 = 8,
    #[doc = "16: PTC X4 Pin"]
    X4 = 16,
    #[doc = "32: PTC X5 Pin"]
    X5 = 32,
    #[doc = "64: PTC X6 Pin"]
    X6 = 64,
    #[doc = "128: PTC X7 Pin"]
    X7 = 128,
    #[doc = "256: PTC X8 Pin"]
    X8 = 256,
    #[doc = "512: PTC X9 Pin"]
    X9 = 512,
    #[doc = "1024: PTC X10 Pin"]
    X10 = 1024,
    #[doc = "2048: PTC X11 Pin"]
    X11 = 2048,
    #[doc = "4096: PTC X12 Pin"]
    X12 = 4096,
    #[doc = "8192: PTC X13 Pin"]
    X13 = 8192,
    #[doc = "16384: PTC X14 Pin"]
    X14 = 16384,
    #[doc = "32768: PTC X15 Pin"]
    X15 = 32768,
}
impl From<XMUX_A> for u16 {
    #[inline(always)]
    fn from(variant: XMUX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XMUX`"]
pub type XMUX_R = crate::R<u16, XMUX_A>;
impl XMUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, XMUX_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(XMUX_A::X0),
            2 => Val(XMUX_A::X1),
            4 => Val(XMUX_A::X2),
            8 => Val(XMUX_A::X3),
            16 => Val(XMUX_A::X4),
            32 => Val(XMUX_A::X5),
            64 => Val(XMUX_A::X6),
            128 => Val(XMUX_A::X7),
            256 => Val(XMUX_A::X8),
            512 => Val(XMUX_A::X9),
            1024 => Val(XMUX_A::X10),
            2048 => Val(XMUX_A::X11),
            4096 => Val(XMUX_A::X12),
            8192 => Val(XMUX_A::X13),
            16384 => Val(XMUX_A::X14),
            32768 => Val(XMUX_A::X15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `X0`"]
    #[inline(always)]
    pub fn is_x0(&self) -> bool {
        *self == XMUX_A::X0
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == XMUX_A::X1
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == XMUX_A::X2
    }
    #[doc = "Checks if the value of the field is `X3`"]
    #[inline(always)]
    pub fn is_x3(&self) -> bool {
        *self == XMUX_A::X3
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == XMUX_A::X4
    }
    #[doc = "Checks if the value of the field is `X5`"]
    #[inline(always)]
    pub fn is_x5(&self) -> bool {
        *self == XMUX_A::X5
    }
    #[doc = "Checks if the value of the field is `X6`"]
    #[inline(always)]
    pub fn is_x6(&self) -> bool {
        *self == XMUX_A::X6
    }
    #[doc = "Checks if the value of the field is `X7`"]
    #[inline(always)]
    pub fn is_x7(&self) -> bool {
        *self == XMUX_A::X7
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == XMUX_A::X8
    }
    #[doc = "Checks if the value of the field is `X9`"]
    #[inline(always)]
    pub fn is_x9(&self) -> bool {
        *self == XMUX_A::X9
    }
    #[doc = "Checks if the value of the field is `X10`"]
    #[inline(always)]
    pub fn is_x10(&self) -> bool {
        *self == XMUX_A::X10
    }
    #[doc = "Checks if the value of the field is `X11`"]
    #[inline(always)]
    pub fn is_x11(&self) -> bool {
        *self == XMUX_A::X11
    }
    #[doc = "Checks if the value of the field is `X12`"]
    #[inline(always)]
    pub fn is_x12(&self) -> bool {
        *self == XMUX_A::X12
    }
    #[doc = "Checks if the value of the field is `X13`"]
    #[inline(always)]
    pub fn is_x13(&self) -> bool {
        *self == XMUX_A::X13
    }
    #[doc = "Checks if the value of the field is `X14`"]
    #[inline(always)]
    pub fn is_x14(&self) -> bool {
        *self == XMUX_A::X14
    }
    #[doc = "Checks if the value of the field is `X15`"]
    #[inline(always)]
    pub fn is_x15(&self) -> bool {
        *self == XMUX_A::X15
    }
}
#[doc = "Write proxy for field `XMUX`"]
pub struct XMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> XMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XMUX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PTC X0 Pin"]
    #[inline(always)]
    pub fn x0(self) -> &'a mut W {
        self.variant(XMUX_A::X0)
    }
    #[doc = "PTC X1 Pin"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(XMUX_A::X1)
    }
    #[doc = "PTC X2 Pin"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(XMUX_A::X2)
    }
    #[doc = "PTC X3 Pin"]
    #[inline(always)]
    pub fn x3(self) -> &'a mut W {
        self.variant(XMUX_A::X3)
    }
    #[doc = "PTC X4 Pin"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(XMUX_A::X4)
    }
    #[doc = "PTC X5 Pin"]
    #[inline(always)]
    pub fn x5(self) -> &'a mut W {
        self.variant(XMUX_A::X5)
    }
    #[doc = "PTC X6 Pin"]
    #[inline(always)]
    pub fn x6(self) -> &'a mut W {
        self.variant(XMUX_A::X6)
    }
    #[doc = "PTC X7 Pin"]
    #[inline(always)]
    pub fn x7(self) -> &'a mut W {
        self.variant(XMUX_A::X7)
    }
    #[doc = "PTC X8 Pin"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut W {
        self.variant(XMUX_A::X8)
    }
    #[doc = "PTC X9 Pin"]
    #[inline(always)]
    pub fn x9(self) -> &'a mut W {
        self.variant(XMUX_A::X9)
    }
    #[doc = "PTC X10 Pin"]
    #[inline(always)]
    pub fn x10(self) -> &'a mut W {
        self.variant(XMUX_A::X10)
    }
    #[doc = "PTC X11 Pin"]
    #[inline(always)]
    pub fn x11(self) -> &'a mut W {
        self.variant(XMUX_A::X11)
    }
    #[doc = "PTC X12 Pin"]
    #[inline(always)]
    pub fn x12(self) -> &'a mut W {
        self.variant(XMUX_A::X12)
    }
    #[doc = "PTC X13 Pin"]
    #[inline(always)]
    pub fn x13(self) -> &'a mut W {
        self.variant(XMUX_A::X13)
    }
    #[doc = "PTC X14 Pin"]
    #[inline(always)]
    pub fn x14(self) -> &'a mut W {
        self.variant(XMUX_A::X14)
    }
    #[doc = "PTC X15 Pin"]
    #[inline(always)]
    pub fn x15(self) -> &'a mut W {
        self.variant(XMUX_A::X15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - X line selection MUX"]
    #[inline(always)]
    pub fn xmux(&self) -> XMUX_R {
        XMUX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - X line selection MUX"]
    #[inline(always)]
    pub fn xmux(&mut self) -> XMUX_W {
        XMUX_W { w: self }
    }
}
