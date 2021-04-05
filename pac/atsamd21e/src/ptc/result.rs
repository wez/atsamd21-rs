#[doc = "Reader of register RESULT"]
pub type R = crate::R<u16, super::RESULT>;
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Result of conversion"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
}
