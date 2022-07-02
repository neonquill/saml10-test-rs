#[doc = "Register `DDEVTYPE` reader"]
pub struct R(crate::R<DDEVTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDEVTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDEVTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDEVTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAJOR` reader - Major type"]
pub type MAJOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUB` reader - Sub-type"]
pub type SUB_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Major type"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sub-type"]
    #[inline(always)]
    pub fn sub(&self) -> SUB_R {
        SUB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SCS Device Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddevtype](index.html) module"]
pub struct DDEVTYPE_SPEC;
impl crate::RegisterSpec for DDEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddevtype::R](R) reader structure"]
impl crate::Readable for DDEVTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDEVTYPE to value 0"]
impl crate::Resettable for DDEVTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
