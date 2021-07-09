#[doc = "Reader of register CTRLA"]
pub type R = crate::R<u32, super::CTRLA>;
#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u32, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: USART with external clock"]
    USART_EXT_CLK = 0,
    #[doc = "1: USART with internal clock"]
    USART_INT_CLK = 1,
    #[doc = "2: SPI in slave operation"]
    SPI_SLAVE = 2,
    #[doc = "3: SPI in master operation"]
    SPI_MASTER = 3,
    #[doc = "4: I2C slave operation"]
    I2C_SLAVE = 4,
    #[doc = "5: I2C master operation"]
    I2C_MASTER = 5,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::USART_EXT_CLK),
            1 => Val(MODE_A::USART_INT_CLK),
            2 => Val(MODE_A::SPI_SLAVE),
            3 => Val(MODE_A::SPI_MASTER),
            4 => Val(MODE_A::I2C_SLAVE),
            5 => Val(MODE_A::I2C_MASTER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `USART_EXT_CLK`"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == MODE_A::USART_EXT_CLK
    }
    #[doc = "Checks if the value of the field is `USART_INT_CLK`"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == MODE_A::USART_INT_CLK
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODE_A::SPI_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == MODE_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE`"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == MODE_A::I2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == MODE_A::I2C_MASTER
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut W {
        self.variant(MODE_A::USART_EXT_CLK)
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut W {
        self.variant(MODE_A::USART_INT_CLK)
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODE_A::SPI_SLAVE)
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODE_A::SPI_MASTER)
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut W {
        self.variant(MODE_A::I2C_SLAVE)
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(MODE_A::I2C_MASTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNSTDBY`"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `IBON`"]
pub type IBON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBON`"]
pub struct IBON_W<'a> {
    w: &'a mut W,
}
impl<'a> IBON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TXINV`"]
pub type TXINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXINV`"]
pub struct TXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RXINV`"]
pub type RXINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXINV`"]
pub struct RXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAMPR_A {
    #[doc = "0: 16x over-sampling using arithmetic baudrate generation"]
    _16X_ARITHMETIC = 0,
    #[doc = "1: 16x over-sampling using fractional baudrate generation"]
    _16X_FRACTIONAL = 1,
    #[doc = "2: 8x over-sampling using arithmetic baudrate generation"]
    _8X_ARITHMETIC = 2,
    #[doc = "3: 8x over-sampling using fractional baudrate generation"]
    _8X_FRACTIONAL = 3,
    #[doc = "4: 3x over-sampling using arithmetic baudrate generation"]
    _3X_ARITHMETIC = 4,
}
impl From<SAMPR_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAMPR`"]
pub type SAMPR_R = crate::R<u8, SAMPR_A>;
impl SAMPR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAMPR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAMPR_A::_16X_ARITHMETIC),
            1 => Val(SAMPR_A::_16X_FRACTIONAL),
            2 => Val(SAMPR_A::_8X_ARITHMETIC),
            3 => Val(SAMPR_A::_8X_FRACTIONAL),
            4 => Val(SAMPR_A::_3X_ARITHMETIC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16X_ARITHMETIC`"]
    #[inline(always)]
    pub fn is_16x_arithmetic(&self) -> bool {
        *self == SAMPR_A::_16X_ARITHMETIC
    }
    #[doc = "Checks if the value of the field is `_16X_FRACTIONAL`"]
    #[inline(always)]
    pub fn is_16x_fractional(&self) -> bool {
        *self == SAMPR_A::_16X_FRACTIONAL
    }
    #[doc = "Checks if the value of the field is `_8X_ARITHMETIC`"]
    #[inline(always)]
    pub fn is_8x_arithmetic(&self) -> bool {
        *self == SAMPR_A::_8X_ARITHMETIC
    }
    #[doc = "Checks if the value of the field is `_8X_FRACTIONAL`"]
    #[inline(always)]
    pub fn is_8x_fractional(&self) -> bool {
        *self == SAMPR_A::_8X_FRACTIONAL
    }
    #[doc = "Checks if the value of the field is `_3X_ARITHMETIC`"]
    #[inline(always)]
    pub fn is_3x_arithmetic(&self) -> bool {
        *self == SAMPR_A::_3X_ARITHMETIC
    }
}
#[doc = "Write proxy for field `SAMPR`"]
pub struct SAMPR_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "16x over-sampling using arithmetic baudrate generation"]
    #[inline(always)]
    pub fn _16x_arithmetic(self) -> &'a mut W {
        self.variant(SAMPR_A::_16X_ARITHMETIC)
    }
    #[doc = "16x over-sampling using fractional baudrate generation"]
    #[inline(always)]
    pub fn _16x_fractional(self) -> &'a mut W {
        self.variant(SAMPR_A::_16X_FRACTIONAL)
    }
    #[doc = "8x over-sampling using arithmetic baudrate generation"]
    #[inline(always)]
    pub fn _8x_arithmetic(self) -> &'a mut W {
        self.variant(SAMPR_A::_8X_ARITHMETIC)
    }
    #[doc = "8x over-sampling using fractional baudrate generation"]
    #[inline(always)]
    pub fn _8x_fractional(self) -> &'a mut W {
        self.variant(SAMPR_A::_8X_FRACTIONAL)
    }
    #[doc = "3x over-sampling using arithmetic baudrate generation"]
    #[inline(always)]
    pub fn _3x_arithmetic(self) -> &'a mut W {
        self.variant(SAMPR_A::_3X_ARITHMETIC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Transmit Data Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXPO_A {
    #[doc = "0: TxD on PAD0, XCK on PAD1"]
    TXPO_0 = 0,
    #[doc = "2: TxD on PAD0, RTS/TE on PAD2, CTS on PAD3"]
    TXPO_2 = 2,
    #[doc = "3: TxD on PAD0, XCK on PAD1, RTS/TE on PAD2"]
    TXPO_3 = 3,
}
impl From<TXPO_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXPO`"]
pub type TXPO_R = crate::R<u8, TXPO_A>;
impl TXPO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXPO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXPO_A::TXPO_0),
            2 => Val(TXPO_A::TXPO_2),
            3 => Val(TXPO_A::TXPO_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXPO_0`"]
    #[inline(always)]
    pub fn is_txpo_0(&self) -> bool {
        *self == TXPO_A::TXPO_0
    }
    #[doc = "Checks if the value of the field is `TXPO_2`"]
    #[inline(always)]
    pub fn is_txpo_2(&self) -> bool {
        *self == TXPO_A::TXPO_2
    }
    #[doc = "Checks if the value of the field is `TXPO_3`"]
    #[inline(always)]
    pub fn is_txpo_3(&self) -> bool {
        *self == TXPO_A::TXPO_3
    }
}
#[doc = "Write proxy for field `TXPO`"]
pub struct TXPO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TxD on PAD0, XCK on PAD1"]
    #[inline(always)]
    pub fn txpo_0(self) -> &'a mut W {
        self.variant(TXPO_A::TXPO_0)
    }
    #[doc = "TxD on PAD0, RTS/TE on PAD2, CTS on PAD3"]
    #[inline(always)]
    pub fn txpo_2(self) -> &'a mut W {
        self.variant(TXPO_A::TXPO_2)
    }
    #[doc = "TxD on PAD0, XCK on PAD1, RTS/TE on PAD2"]
    #[inline(always)]
    pub fn txpo_3(self) -> &'a mut W {
        self.variant(TXPO_A::TXPO_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Receive Data Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXPO_A {
    #[doc = "0: SERCOM PAD\\[0\\]
is used for data reception"]
    PAD0 = 0,
    #[doc = "1: SERCOM PAD\\[1\\]
is used for data reception"]
    PAD1 = 1,
    #[doc = "2: SERCOM PAD\\[2\\]
is used for data reception"]
    PAD2 = 2,
    #[doc = "3: SERCOM PAD\\[3\\]
is used for data reception"]
    PAD3 = 3,
}
impl From<RXPO_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXPO`"]
pub type RXPO_R = crate::R<u8, RXPO_A>;
impl RXPO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPO_A {
        match self.bits {
            0 => RXPO_A::PAD0,
            1 => RXPO_A::PAD1,
            2 => RXPO_A::PAD2,
            3 => RXPO_A::PAD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == RXPO_A::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == RXPO_A::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == RXPO_A::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == RXPO_A::PAD3
    }
}
#[doc = "Write proxy for field `RXPO`"]
pub struct RXPO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPO_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SERCOM PAD\\[0\\]
is used for data reception"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(RXPO_A::PAD0)
    }
    #[doc = "SERCOM PAD\\[1\\]
is used for data reception"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut W {
        self.variant(RXPO_A::PAD1)
    }
    #[doc = "SERCOM PAD\\[2\\]
is used for data reception"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(RXPO_A::PAD2)
    }
    #[doc = "SERCOM PAD\\[3\\]
is used for data reception"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut W {
        self.variant(RXPO_A::PAD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `SAMPA`"]
pub type SAMPA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAMPA`"]
pub struct SAMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Frame Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORM_A {
    #[doc = "0: USART frame"]
    USART_FRAME_NO_PARITY = 0,
    #[doc = "1: USART frame with parity"]
    USART_FRAME_WITH_PARITY = 1,
    #[doc = "2: LIN Master - Break and sync generation"]
    USART_FRAME_LIN_MASTER_MODE = 2,
    #[doc = "4: Auto-baud - break detection and auto-baud"]
    USART_FRAME_AUTO_BAUD_NO_PARITY = 4,
    #[doc = "5: Auto-baud - break detection and auto-baud with parity"]
    USART_FRAME_AUTO_BAUD_WITH_PARITY = 5,
    #[doc = "7: ISO 7816"]
    USART_FRAME_ISO_7816 = 7,
}
impl From<FORM_A> for u8 {
    #[inline(always)]
    fn from(variant: FORM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FORM`"]
pub type FORM_R = crate::R<u8, FORM_A>;
impl FORM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FORM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FORM_A::USART_FRAME_NO_PARITY),
            1 => Val(FORM_A::USART_FRAME_WITH_PARITY),
            2 => Val(FORM_A::USART_FRAME_LIN_MASTER_MODE),
            4 => Val(FORM_A::USART_FRAME_AUTO_BAUD_NO_PARITY),
            5 => Val(FORM_A::USART_FRAME_AUTO_BAUD_WITH_PARITY),
            7 => Val(FORM_A::USART_FRAME_ISO_7816),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_NO_PARITY`"]
    #[inline(always)]
    pub fn is_usart_frame_no_parity(&self) -> bool {
        *self == FORM_A::USART_FRAME_NO_PARITY
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_WITH_PARITY`"]
    #[inline(always)]
    pub fn is_usart_frame_with_parity(&self) -> bool {
        *self == FORM_A::USART_FRAME_WITH_PARITY
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_LIN_MASTER_MODE`"]
    #[inline(always)]
    pub fn is_usart_frame_lin_master_mode(&self) -> bool {
        *self == FORM_A::USART_FRAME_LIN_MASTER_MODE
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_AUTO_BAUD_NO_PARITY`"]
    #[inline(always)]
    pub fn is_usart_frame_auto_baud_no_parity(&self) -> bool {
        *self == FORM_A::USART_FRAME_AUTO_BAUD_NO_PARITY
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_AUTO_BAUD_WITH_PARITY`"]
    #[inline(always)]
    pub fn is_usart_frame_auto_baud_with_parity(&self) -> bool {
        *self == FORM_A::USART_FRAME_AUTO_BAUD_WITH_PARITY
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_ISO_7816`"]
    #[inline(always)]
    pub fn is_usart_frame_iso_7816(&self) -> bool {
        *self == FORM_A::USART_FRAME_ISO_7816
    }
}
#[doc = "Write proxy for field `FORM`"]
pub struct FORM_W<'a> {
    w: &'a mut W,
}
impl<'a> FORM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "USART frame"]
    #[inline(always)]
    pub fn usart_frame_no_parity(self) -> &'a mut W {
        self.variant(FORM_A::USART_FRAME_NO_PARITY)
    }
    #[doc = "USART frame with parity"]
    #[inline(always)]
    pub fn usart_frame_with_parity(self) -> &'a mut W {
        self.variant(FORM_A::USART_FRAME_WITH_PARITY)
    }
    #[doc = "LIN Master - Break and sync generation"]
    #[inline(always)]
    pub fn usart_frame_lin_master_mode(self) -> &'a mut W {
        self.variant(FORM_A::USART_FRAME_LIN_MASTER_MODE)
    }
    #[doc = "Auto-baud - break detection and auto-baud"]
    #[inline(always)]
    pub fn usart_frame_auto_baud_no_parity(self) -> &'a mut W {
        self.variant(FORM_A::USART_FRAME_AUTO_BAUD_NO_PARITY)
    }
    #[doc = "Auto-baud - break detection and auto-baud with parity"]
    #[inline(always)]
    pub fn usart_frame_auto_baud_with_parity(self) -> &'a mut W {
        self.variant(FORM_A::USART_FRAME_AUTO_BAUD_WITH_PARITY)
    }
    #[doc = "ISO 7816"]
    #[inline(always)]
    pub fn usart_frame_iso_7816(self) -> &'a mut W {
        self.variant(FORM_A::USART_FRAME_ISO_7816)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Communication Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMODE_A {
    #[doc = "0: Asynchronous Communication"]
    ASYNC = 0,
    #[doc = "1: Synchronous Communication"]
    SYNC = 1,
}
impl From<CMODE_A> for bool {
    #[inline(always)]
    fn from(variant: CMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMODE`"]
pub type CMODE_R = crate::R<bool, CMODE_A>;
impl CMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMODE_A {
        match self.bits {
            false => CMODE_A::ASYNC,
            true => CMODE_A::SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async_(&self) -> bool {
        *self == CMODE_A::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == CMODE_A::SYNC
    }
}
#[doc = "Write proxy for field `CMODE`"]
pub struct CMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Asynchronous Communication"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(CMODE_A::ASYNC)
    }
    #[doc = "Synchronous Communication"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(CMODE_A::SYNC)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: TxD Change:- Rising XCK edge, RxD Sample:- Falling XCK edge"]
    IDLE_LOW = 0,
    #[doc = "1: TxD Change:- Falling XCK edge, RxD Sample:- Rising XCK edge"]
    IDLE_HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::IDLE_LOW,
            true => CPOL_A::IDLE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_LOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOL_A::IDLE_LOW
    }
    #[doc = "Checks if the value of the field is `IDLE_HIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOL_A::IDLE_HIGH
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TxD Change:- Rising XCK edge, RxD Sample:- Falling XCK edge"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOL_A::IDLE_LOW)
    }
    #[doc = "TxD Change:- Falling XCK edge, RxD Sample:- Rising XCK edge"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOL_A::IDLE_HIGH)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Data Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DORD_A {
    #[doc = "0: MSB is transmitted first"]
    MSB = 0,
    #[doc = "1: LSB is transmitted first"]
    LSB = 1,
}
impl From<DORD_A> for bool {
    #[inline(always)]
    fn from(variant: DORD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DORD`"]
pub type DORD_R = crate::R<bool, DORD_A>;
impl DORD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DORD_A {
        match self.bits {
            false => DORD_A::MSB,
            true => DORD_A::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == DORD_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == DORD_A::LSB
    }
}
#[doc = "Write proxy for field `DORD`"]
pub struct DORD_W<'a> {
    w: &'a mut W,
}
impl<'a> DORD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DORD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MSB is transmitted first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(DORD_A::MSB)
    }
    #[doc = "LSB is transmitted first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(DORD_A::LSB)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&self) -> IBON_R {
        IBON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit Data Invert"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive Data Invert"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - Sample"]
    #[inline(always)]
    pub fn sampr(&self) -> SAMPR_R {
        SAMPR_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline(always)]
    pub fn txpo(&self) -> TXPO_R {
        TXPO_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    pub fn rxpo(&self) -> RXPO_R {
        RXPO_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline(always)]
    pub fn sampa(&self) -> SAMPA_R {
        SAMPA_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FORM_R {
        FORM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    pub fn cmode(&self) -> CMODE_R {
        CMODE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DORD_R {
        DORD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&mut self) -> IBON_W {
        IBON_W { w: self }
    }
    #[doc = "Bit 9 - Transmit Data Invert"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W {
        TXINV_W { w: self }
    }
    #[doc = "Bit 10 - Receive Data Invert"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W {
        RXINV_W { w: self }
    }
    #[doc = "Bits 13:15 - Sample"]
    #[inline(always)]
    pub fn sampr(&mut self) -> SAMPR_W {
        SAMPR_W { w: self }
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline(always)]
    pub fn txpo(&mut self) -> TXPO_W {
        TXPO_W { w: self }
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    pub fn rxpo(&mut self) -> RXPO_W {
        RXPO_W { w: self }
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline(always)]
    pub fn sampa(&mut self) -> SAMPA_W {
        SAMPA_W { w: self }
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&mut self) -> FORM_W {
        FORM_W { w: self }
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    pub fn cmode(&mut self) -> CMODE_W {
        CMODE_W { w: self }
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&mut self) -> DORD_W {
        DORD_W { w: self }
    }
}
