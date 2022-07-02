#[doc = "Register `DCIDR0` reader"]
pub struct R(crate::R<DCIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRMBL_0` reader - CoreSight component identification preamble"]
pub type PRMBL_0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CoreSight component identification preamble"]
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SCS Component Identification Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcidr0](index.html) module"]
pub struct DCIDR0_SPEC;
impl crate::RegisterSpec for DCIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcidr0::R](R) reader structure"]
impl crate::Readable for DCIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCIDR0 to value 0x0d"]
impl crate::Resettable for DCIDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d
    }
}
