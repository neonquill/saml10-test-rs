#[doc = "Register `DPIDR2` reader"]
pub struct R(crate::R<DPIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DES_1` reader - JEP106 identification code bits\\[6:4\\]"]
pub type DES_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JEDEC` reader - JEDEC assignee value is used"]
pub type JEDEC_R = crate::BitReader<bool>;
#[doc = "Field `REVISION` reader - Component revision"]
pub type REVISION_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - JEP106 identification code bits\\[6:4\\]"]
    #[inline(always)]
    pub fn des_1(&self) -> DES_1_R {
        DES_1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - JEDEC assignee value is used"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Component revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SCS Peripheral Identification Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpidr2](index.html) module"]
pub struct DPIDR2_SPEC;
impl crate::RegisterSpec for DPIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpidr2::R](R) reader structure"]
impl crate::Readable for DPIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DPIDR2 to value 0"]
impl crate::Resettable for DPIDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
