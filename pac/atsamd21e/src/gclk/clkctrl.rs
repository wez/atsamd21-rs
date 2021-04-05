#[doc = "Reader of register CLKCTRL"]
pub type R = crate::R<u16, super::CLKCTRL>;
#[doc = "Writer for register CLKCTRL"]
pub type W = crate::W<u16, super::CLKCTRL>;
#[doc = "Register CLKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Generic Clock Selection ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ID_A {
    #[doc = "0: DFLL48"]
    DFLL48 = 0,
    #[doc = "1: FDPLL"]
    FDPLL = 1,
    #[doc = "2: FDPLL32K"]
    FDPLL32K = 2,
    #[doc = "3: WDT"]
    WDT = 3,
    #[doc = "4: RTC"]
    RTC = 4,
    #[doc = "5: EIC"]
    EIC = 5,
    #[doc = "6: USB"]
    USB = 6,
    #[doc = "7: EVSYS_0"]
    EVSYS_0 = 7,
    #[doc = "8: EVSYS_1"]
    EVSYS_1 = 8,
    #[doc = "9: EVSYS_2"]
    EVSYS_2 = 9,
    #[doc = "10: EVSYS_3"]
    EVSYS_3 = 10,
    #[doc = "11: EVSYS_4"]
    EVSYS_4 = 11,
    #[doc = "12: EVSYS_5"]
    EVSYS_5 = 12,
    #[doc = "13: EVSYS_6"]
    EVSYS_6 = 13,
    #[doc = "14: EVSYS_7"]
    EVSYS_7 = 14,
    #[doc = "15: EVSYS_8"]
    EVSYS_8 = 15,
    #[doc = "16: EVSYS_9"]
    EVSYS_9 = 16,
    #[doc = "17: EVSYS_10"]
    EVSYS_10 = 17,
    #[doc = "18: EVSYS_11"]
    EVSYS_11 = 18,
    #[doc = "19: SERCOMX_SLOW"]
    SERCOMX_SLOW = 19,
    #[doc = "20: SERCOM0_CORE"]
    SERCOM0_CORE = 20,
    #[doc = "21: SERCOM1_CORE"]
    SERCOM1_CORE = 21,
    #[doc = "22: SERCOM2_CORE"]
    SERCOM2_CORE = 22,
    #[doc = "23: SERCOM3_CORE"]
    SERCOM3_CORE = 23,
    #[doc = "24: SERCOM4_CORE"]
    SERCOM4_CORE = 24,
    #[doc = "25: SERCOM5_CORE"]
    SERCOM5_CORE = 25,
    #[doc = "26: TCC0_TCC1"]
    TCC0_TCC1 = 26,
    #[doc = "27: TCC2_TC3"]
    TCC2_TC3 = 27,
    #[doc = "28: TC4_TC5"]
    TC4_TC5 = 28,
    #[doc = "29: TC6_TC7"]
    TC6_TC7 = 29,
    #[doc = "30: ADC"]
    ADC = 30,
    #[doc = "31: AC_DIG"]
    AC_DIG = 31,
    #[doc = "32: AC_ANA"]
    AC_ANA = 32,
    #[doc = "33: DAC"]
    DAC = 33,
    #[doc = "35: I2S_0"]
    I2S_0 = 35,
    #[doc = "36: I2S_1"]
    I2S_1 = 36,
    #[doc = "34: PTCReserved"]
    PTC = 34,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u8, ID_A>;
impl ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ID_A::DFLL48),
            1 => Val(ID_A::FDPLL),
            2 => Val(ID_A::FDPLL32K),
            3 => Val(ID_A::WDT),
            4 => Val(ID_A::RTC),
            5 => Val(ID_A::EIC),
            6 => Val(ID_A::USB),
            7 => Val(ID_A::EVSYS_0),
            8 => Val(ID_A::EVSYS_1),
            9 => Val(ID_A::EVSYS_2),
            10 => Val(ID_A::EVSYS_3),
            11 => Val(ID_A::EVSYS_4),
            12 => Val(ID_A::EVSYS_5),
            13 => Val(ID_A::EVSYS_6),
            14 => Val(ID_A::EVSYS_7),
            15 => Val(ID_A::EVSYS_8),
            16 => Val(ID_A::EVSYS_9),
            17 => Val(ID_A::EVSYS_10),
            18 => Val(ID_A::EVSYS_11),
            19 => Val(ID_A::SERCOMX_SLOW),
            20 => Val(ID_A::SERCOM0_CORE),
            21 => Val(ID_A::SERCOM1_CORE),
            22 => Val(ID_A::SERCOM2_CORE),
            23 => Val(ID_A::SERCOM3_CORE),
            24 => Val(ID_A::SERCOM4_CORE),
            25 => Val(ID_A::SERCOM5_CORE),
            26 => Val(ID_A::TCC0_TCC1),
            27 => Val(ID_A::TCC2_TC3),
            28 => Val(ID_A::TC4_TC5),
            29 => Val(ID_A::TC6_TC7),
            30 => Val(ID_A::ADC),
            31 => Val(ID_A::AC_DIG),
            32 => Val(ID_A::AC_ANA),
            33 => Val(ID_A::DAC),
            35 => Val(ID_A::I2S_0),
            36 => Val(ID_A::I2S_1),
            34 => Val(ID_A::PTC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DFLL48`"]
    #[inline(always)]
    pub fn is_dfll48(&self) -> bool {
        *self == ID_A::DFLL48
    }
    #[doc = "Checks if the value of the field is `FDPLL`"]
    #[inline(always)]
    pub fn is_fdpll(&self) -> bool {
        *self == ID_A::FDPLL
    }
    #[doc = "Checks if the value of the field is `FDPLL32K`"]
    #[inline(always)]
    pub fn is_fdpll32k(&self) -> bool {
        *self == ID_A::FDPLL32K
    }
    #[doc = "Checks if the value of the field is `WDT`"]
    #[inline(always)]
    pub fn is_wdt(&self) -> bool {
        *self == ID_A::WDT
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == ID_A::RTC
    }
    #[doc = "Checks if the value of the field is `EIC`"]
    #[inline(always)]
    pub fn is_eic(&self) -> bool {
        *self == ID_A::EIC
    }
    #[doc = "Checks if the value of the field is `USB`"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == ID_A::USB
    }
    #[doc = "Checks if the value of the field is `EVSYS_0`"]
    #[inline(always)]
    pub fn is_evsys_0(&self) -> bool {
        *self == ID_A::EVSYS_0
    }
    #[doc = "Checks if the value of the field is `EVSYS_1`"]
    #[inline(always)]
    pub fn is_evsys_1(&self) -> bool {
        *self == ID_A::EVSYS_1
    }
    #[doc = "Checks if the value of the field is `EVSYS_2`"]
    #[inline(always)]
    pub fn is_evsys_2(&self) -> bool {
        *self == ID_A::EVSYS_2
    }
    #[doc = "Checks if the value of the field is `EVSYS_3`"]
    #[inline(always)]
    pub fn is_evsys_3(&self) -> bool {
        *self == ID_A::EVSYS_3
    }
    #[doc = "Checks if the value of the field is `EVSYS_4`"]
    #[inline(always)]
    pub fn is_evsys_4(&self) -> bool {
        *self == ID_A::EVSYS_4
    }
    #[doc = "Checks if the value of the field is `EVSYS_5`"]
    #[inline(always)]
    pub fn is_evsys_5(&self) -> bool {
        *self == ID_A::EVSYS_5
    }
    #[doc = "Checks if the value of the field is `EVSYS_6`"]
    #[inline(always)]
    pub fn is_evsys_6(&self) -> bool {
        *self == ID_A::EVSYS_6
    }
    #[doc = "Checks if the value of the field is `EVSYS_7`"]
    #[inline(always)]
    pub fn is_evsys_7(&self) -> bool {
        *self == ID_A::EVSYS_7
    }
    #[doc = "Checks if the value of the field is `EVSYS_8`"]
    #[inline(always)]
    pub fn is_evsys_8(&self) -> bool {
        *self == ID_A::EVSYS_8
    }
    #[doc = "Checks if the value of the field is `EVSYS_9`"]
    #[inline(always)]
    pub fn is_evsys_9(&self) -> bool {
        *self == ID_A::EVSYS_9
    }
    #[doc = "Checks if the value of the field is `EVSYS_10`"]
    #[inline(always)]
    pub fn is_evsys_10(&self) -> bool {
        *self == ID_A::EVSYS_10
    }
    #[doc = "Checks if the value of the field is `EVSYS_11`"]
    #[inline(always)]
    pub fn is_evsys_11(&self) -> bool {
        *self == ID_A::EVSYS_11
    }
    #[doc = "Checks if the value of the field is `SERCOMX_SLOW`"]
    #[inline(always)]
    pub fn is_sercomx_slow(&self) -> bool {
        *self == ID_A::SERCOMX_SLOW
    }
    #[doc = "Checks if the value of the field is `SERCOM0_CORE`"]
    #[inline(always)]
    pub fn is_sercom0_core(&self) -> bool {
        *self == ID_A::SERCOM0_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM1_CORE`"]
    #[inline(always)]
    pub fn is_sercom1_core(&self) -> bool {
        *self == ID_A::SERCOM1_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM2_CORE`"]
    #[inline(always)]
    pub fn is_sercom2_core(&self) -> bool {
        *self == ID_A::SERCOM2_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM3_CORE`"]
    #[inline(always)]
    pub fn is_sercom3_core(&self) -> bool {
        *self == ID_A::SERCOM3_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM4_CORE`"]
    #[inline(always)]
    pub fn is_sercom4_core(&self) -> bool {
        *self == ID_A::SERCOM4_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM5_CORE`"]
    #[inline(always)]
    pub fn is_sercom5_core(&self) -> bool {
        *self == ID_A::SERCOM5_CORE
    }
    #[doc = "Checks if the value of the field is `TCC0_TCC1`"]
    #[inline(always)]
    pub fn is_tcc0_tcc1(&self) -> bool {
        *self == ID_A::TCC0_TCC1
    }
    #[doc = "Checks if the value of the field is `TCC2_TC3`"]
    #[inline(always)]
    pub fn is_tcc2_tc3(&self) -> bool {
        *self == ID_A::TCC2_TC3
    }
    #[doc = "Checks if the value of the field is `TC4_TC5`"]
    #[inline(always)]
    pub fn is_tc4_tc5(&self) -> bool {
        *self == ID_A::TC4_TC5
    }
    #[doc = "Checks if the value of the field is `TC6_TC7`"]
    #[inline(always)]
    pub fn is_tc6_tc7(&self) -> bool {
        *self == ID_A::TC6_TC7
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == ID_A::ADC
    }
    #[doc = "Checks if the value of the field is `AC_DIG`"]
    #[inline(always)]
    pub fn is_ac_dig(&self) -> bool {
        *self == ID_A::AC_DIG
    }
    #[doc = "Checks if the value of the field is `AC_ANA`"]
    #[inline(always)]
    pub fn is_ac_ana(&self) -> bool {
        *self == ID_A::AC_ANA
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == ID_A::DAC
    }
    #[doc = "Checks if the value of the field is `I2S_0`"]
    #[inline(always)]
    pub fn is_i2s_0(&self) -> bool {
        *self == ID_A::I2S_0
    }
    #[doc = "Checks if the value of the field is `I2S_1`"]
    #[inline(always)]
    pub fn is_i2s_1(&self) -> bool {
        *self == ID_A::I2S_1
    }
    #[doc = "Checks if the value of the field is `PTC`"]
    #[inline(always)]
    pub fn is_ptc(&self) -> bool {
        *self == ID_A::PTC
    }
}
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DFLL48"]
    #[inline(always)]
    pub fn dfll48(self) -> &'a mut W {
        self.variant(ID_A::DFLL48)
    }
    #[doc = "FDPLL"]
    #[inline(always)]
    pub fn fdpll(self) -> &'a mut W {
        self.variant(ID_A::FDPLL)
    }
    #[doc = "FDPLL32K"]
    #[inline(always)]
    pub fn fdpll32k(self) -> &'a mut W {
        self.variant(ID_A::FDPLL32K)
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub fn wdt(self) -> &'a mut W {
        self.variant(ID_A::WDT)
    }
    #[doc = "RTC"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut W {
        self.variant(ID_A::RTC)
    }
    #[doc = "EIC"]
    #[inline(always)]
    pub fn eic(self) -> &'a mut W {
        self.variant(ID_A::EIC)
    }
    #[doc = "USB"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut W {
        self.variant(ID_A::USB)
    }
    #[doc = "EVSYS_0"]
    #[inline(always)]
    pub fn evsys_0(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_0)
    }
    #[doc = "EVSYS_1"]
    #[inline(always)]
    pub fn evsys_1(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_1)
    }
    #[doc = "EVSYS_2"]
    #[inline(always)]
    pub fn evsys_2(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_2)
    }
    #[doc = "EVSYS_3"]
    #[inline(always)]
    pub fn evsys_3(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_3)
    }
    #[doc = "EVSYS_4"]
    #[inline(always)]
    pub fn evsys_4(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_4)
    }
    #[doc = "EVSYS_5"]
    #[inline(always)]
    pub fn evsys_5(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_5)
    }
    #[doc = "EVSYS_6"]
    #[inline(always)]
    pub fn evsys_6(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_6)
    }
    #[doc = "EVSYS_7"]
    #[inline(always)]
    pub fn evsys_7(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_7)
    }
    #[doc = "EVSYS_8"]
    #[inline(always)]
    pub fn evsys_8(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_8)
    }
    #[doc = "EVSYS_9"]
    #[inline(always)]
    pub fn evsys_9(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_9)
    }
    #[doc = "EVSYS_10"]
    #[inline(always)]
    pub fn evsys_10(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_10)
    }
    #[doc = "EVSYS_11"]
    #[inline(always)]
    pub fn evsys_11(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_11)
    }
    #[doc = "SERCOMX_SLOW"]
    #[inline(always)]
    pub fn sercomx_slow(self) -> &'a mut W {
        self.variant(ID_A::SERCOMX_SLOW)
    }
    #[doc = "SERCOM0_CORE"]
    #[inline(always)]
    pub fn sercom0_core(self) -> &'a mut W {
        self.variant(ID_A::SERCOM0_CORE)
    }
    #[doc = "SERCOM1_CORE"]
    #[inline(always)]
    pub fn sercom1_core(self) -> &'a mut W {
        self.variant(ID_A::SERCOM1_CORE)
    }
    #[doc = "SERCOM2_CORE"]
    #[inline(always)]
    pub fn sercom2_core(self) -> &'a mut W {
        self.variant(ID_A::SERCOM2_CORE)
    }
    #[doc = "SERCOM3_CORE"]
    #[inline(always)]
    pub fn sercom3_core(self) -> &'a mut W {
        self.variant(ID_A::SERCOM3_CORE)
    }
    #[doc = "SERCOM4_CORE"]
    #[inline(always)]
    pub fn sercom4_core(self) -> &'a mut W {
        self.variant(ID_A::SERCOM4_CORE)
    }
    #[doc = "SERCOM5_CORE"]
    #[inline(always)]
    pub fn sercom5_core(self) -> &'a mut W {
        self.variant(ID_A::SERCOM5_CORE)
    }
    #[doc = "TCC0_TCC1"]
    #[inline(always)]
    pub fn tcc0_tcc1(self) -> &'a mut W {
        self.variant(ID_A::TCC0_TCC1)
    }
    #[doc = "TCC2_TC3"]
    #[inline(always)]
    pub fn tcc2_tc3(self) -> &'a mut W {
        self.variant(ID_A::TCC2_TC3)
    }
    #[doc = "TC4_TC5"]
    #[inline(always)]
    pub fn tc4_tc5(self) -> &'a mut W {
        self.variant(ID_A::TC4_TC5)
    }
    #[doc = "TC6_TC7"]
    #[inline(always)]
    pub fn tc6_tc7(self) -> &'a mut W {
        self.variant(ID_A::TC6_TC7)
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut W {
        self.variant(ID_A::ADC)
    }
    #[doc = "AC_DIG"]
    #[inline(always)]
    pub fn ac_dig(self) -> &'a mut W {
        self.variant(ID_A::AC_DIG)
    }
    #[doc = "AC_ANA"]
    #[inline(always)]
    pub fn ac_ana(self) -> &'a mut W {
        self.variant(ID_A::AC_ANA)
    }
    #[doc = "DAC"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(ID_A::DAC)
    }
    #[doc = "I2S_0"]
    #[inline(always)]
    pub fn i2s_0(self) -> &'a mut W {
        self.variant(ID_A::I2S_0)
    }
    #[doc = "I2S_1"]
    #[inline(always)]
    pub fn i2s_1(self) -> &'a mut W {
        self.variant(ID_A::I2S_1)
    }
    #[doc = "PTCReserved"]
    #[inline(always)]
    pub fn ptc(self) -> &'a mut W {
        self.variant(ID_A::PTC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Generic Clock Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GEN_A {
    #[doc = "0: Generic clock generator 0"]
    GCLK0 = 0,
    #[doc = "1: Generic clock generator 1"]
    GCLK1 = 1,
    #[doc = "2: Generic clock generator 2"]
    GCLK2 = 2,
    #[doc = "3: Generic clock generator 3"]
    GCLK3 = 3,
    #[doc = "4: Generic clock generator 4"]
    GCLK4 = 4,
    #[doc = "5: Generic clock generator 5"]
    GCLK5 = 5,
    #[doc = "6: Generic clock generator 6"]
    GCLK6 = 6,
    #[doc = "7: Generic clock generator 7"]
    GCLK7 = 7,
    #[doc = "8: Generic clock generator 8"]
    GCLK8 = 8,
}
impl From<GEN_A> for u8 {
    #[inline(always)]
    fn from(variant: GEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GEN`"]
pub type GEN_R = crate::R<u8, GEN_A>;
impl GEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GEN_A::GCLK0),
            1 => Val(GEN_A::GCLK1),
            2 => Val(GEN_A::GCLK2),
            3 => Val(GEN_A::GCLK3),
            4 => Val(GEN_A::GCLK4),
            5 => Val(GEN_A::GCLK5),
            6 => Val(GEN_A::GCLK6),
            7 => Val(GEN_A::GCLK7),
            8 => Val(GEN_A::GCLK8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GEN_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GEN_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GEN_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GEN_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GEN_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GEN_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GEN_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GEN_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GEN_A::GCLK8
    }
}
#[doc = "Write proxy for field `GEN`"]
pub struct GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut W {
        self.variant(GEN_A::GCLK0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut W {
        self.variant(GEN_A::GCLK1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut W {
        self.variant(GEN_A::GCLK2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut W {
        self.variant(GEN_A::GCLK3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut W {
        self.variant(GEN_A::GCLK4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut W {
        self.variant(GEN_A::GCLK5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn gclk6(self) -> &'a mut W {
        self.variant(GEN_A::GCLK6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn gclk7(self) -> &'a mut W {
        self.variant(GEN_A::GCLK7)
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn gclk8(self) -> &'a mut W {
        self.variant(GEN_A::GCLK8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLKEN`"]
pub type CLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN`"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
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
#[doc = "Reader of field `WRTLOCK`"]
pub type WRTLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRTLOCK`"]
pub struct WRTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRTLOCK_W<'a> {
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
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&mut self) -> GEN_W {
        GEN_W { w: self }
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WRTLOCK_W {
        WRTLOCK_W { w: self }
    }
}
