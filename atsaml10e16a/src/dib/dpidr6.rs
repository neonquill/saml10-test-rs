#[doc = "Register `DPIDR6` reader"]
pub struct R(crate::R<DPIDR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPIDR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPIDR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPIDR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "SCS Peripheral Identification Register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpidr6](index.html) module"]
pub struct DPIDR6_SPEC;
impl crate::RegisterSpec for DPIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpidr6::R](R) reader structure"]
impl crate::Readable for DPIDR6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DPIDR6 to value 0"]
impl crate::Resettable for DPIDR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
