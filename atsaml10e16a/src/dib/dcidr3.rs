#[doc = "Register `DCIDR3` reader"]
pub struct R(crate::R<DCIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRMBL_3` reader - CoreSight component identification preamble"]
pub type PRMBL_3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CoreSight component identification preamble"]
    #[inline(always)]
    pub fn prmbl_3(&self) -> PRMBL_3_R {
        PRMBL_3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SCS Component Identification Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcidr3](index.html) module"]
pub struct DCIDR3_SPEC;
impl crate::RegisterSpec for DCIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcidr3::R](R) reader structure"]
impl crate::Readable for DCIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCIDR3 to value 0xb1"]
impl crate::Resettable for DCIDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb1
    }
}
