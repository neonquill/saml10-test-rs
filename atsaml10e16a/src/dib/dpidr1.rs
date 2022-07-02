#[doc = "Register `DPIDR1` reader"]
pub struct R(crate::R<DPIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PART_1` reader - Part number bits\\[11:8\\]"]
pub type PART_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DES_0` reader - JEP106 identification code bits \\[3:0\\]"]
pub type DES_0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Part number bits\\[11:8\\]"]
    #[inline(always)]
    pub fn part_1(&self) -> PART_1_R {
        PART_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 identification code bits \\[3:0\\]"]
    #[inline(always)]
    pub fn des_0(&self) -> DES_0_R {
        DES_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SCS Peripheral Identification Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpidr1](index.html) module"]
pub struct DPIDR1_SPEC;
impl crate::RegisterSpec for DPIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpidr1::R](R) reader structure"]
impl crate::Readable for DPIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DPIDR1 to value 0"]
impl crate::Resettable for DPIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
