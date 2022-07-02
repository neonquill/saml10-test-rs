#[doc = "Register `ENTRY1` reader"]
pub struct R(crate::R<ENTRY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENTRY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENTRY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENTRY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPRES` reader - Entry Present"]
pub type EPRES_R = crate::BitReader<bool>;
#[doc = "Field `FMT` reader - Format"]
pub type FMT_R = crate::BitReader<bool>;
#[doc = "Field `ADDOFF` reader - Address Offset"]
pub type ADDOFF_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0 - Entry Present"]
    #[inline(always)]
    pub fn epres(&self) -> EPRES_R {
        EPRES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Format"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 12:31 - Address Offset"]
    #[inline(always)]
    pub fn addoff(&self) -> ADDOFF_R {
        ADDOFF_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
#[doc = "CoreSight ROM Table Entry 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [entry1](index.html) module"]
pub struct ENTRY1_SPEC;
impl crate::RegisterSpec for ENTRY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [entry1::R](R) reader structure"]
impl crate::Readable for ENTRY1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ENTRY1 to value 0"]
impl crate::Resettable for ENTRY1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
