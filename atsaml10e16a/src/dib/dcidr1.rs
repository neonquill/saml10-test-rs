#[doc = "Register `DCIDR1` reader"]
pub struct R(crate::R<DCIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRMBL_1` reader - CoreSight component identification preamble"]
pub type PRMBL_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLASS` reader - CoreSight component class"]
pub type CLASS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CoreSight component identification preamble"]
    #[inline(always)]
    pub fn prmbl_1(&self) -> PRMBL_1_R {
        PRMBL_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CoreSight component class"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SCS Component Identification Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcidr1](index.html) module"]
pub struct DCIDR1_SPEC;
impl crate::RegisterSpec for DCIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcidr1::R](R) reader structure"]
impl crate::Readable for DCIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCIDR1 to value 0x90"]
impl crate::Resettable for DCIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x90
    }
}
