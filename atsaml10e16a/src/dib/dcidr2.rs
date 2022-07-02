#[doc = "Register `DCIDR2` reader"]
pub struct R(crate::R<DCIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRMBL_2` reader - CoreSight component identification preamble"]
pub type PRMBL_2_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CoreSight component identification preamble"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SCS Component Identification Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcidr2](index.html) module"]
pub struct DCIDR2_SPEC;
impl crate::RegisterSpec for DCIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcidr2::R](R) reader structure"]
impl crate::Readable for DCIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCIDR2 to value 0x05"]
impl crate::Resettable for DCIDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
