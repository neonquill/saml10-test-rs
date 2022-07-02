#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAMINV` reader - RAM Inversion Bit"]
pub type RAMINV_R = crate::BitReader<bool>;
#[doc = "Field `DRP` reader - Data Remanence Prevention Ongoing"]
pub type DRP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RAM Inversion Bit"]
    #[inline(always)]
    pub fn raminv(&self) -> RAMINV_R {
        RAMINV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Remanence Prevention Ongoing"]
    #[inline(always)]
    pub fn drp(&self) -> DRP_R {
        DRP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
