#[doc = "Register `DPIDR4` reader"]
pub struct R(crate::R<DPIDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPIDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPIDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPIDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DES_2` reader - JEP106 continuation code"]
pub type DES_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` reader - 4KB count"]
pub type SIZE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code"]
    #[inline(always)]
    pub fn des_2(&self) -> DES_2_R {
        DES_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB count"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SCS Peripheral Identification Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpidr4](index.html) module"]
pub struct DPIDR4_SPEC;
impl crate::RegisterSpec for DPIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpidr4::R](R) reader structure"]
impl crate::Readable for DPIDR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DPIDR4 to value 0"]
impl crate::Resettable for DPIDR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
