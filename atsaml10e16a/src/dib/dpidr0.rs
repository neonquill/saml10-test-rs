#[doc = "Register `DPIDR0` reader"]
pub struct R(crate::R<DPIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PART_0` reader - Part number bits\\[7:0\\]"]
pub type PART_0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Part number bits\\[7:0\\]"]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SCS Peripheral Identification Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpidr0](index.html) module"]
pub struct DPIDR0_SPEC;
impl crate::RegisterSpec for DPIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpidr0::R](R) reader structure"]
impl crate::Readable for DPIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DPIDR0 to value 0"]
impl crate::Resettable for DPIDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
