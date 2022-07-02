#[doc = "Register `DDEVARCH` reader"]
pub struct R(crate::R<DDEVARCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDEVARCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDEVARCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDEVARCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ARCHPART` reader - Architecture Part"]
pub type ARCHPART_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ARCHVER` reader - Architecture Version"]
pub type ARCHVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REVISION` reader - Revision"]
pub type REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESENT` reader - DEVARCH Present"]
pub type PRESENT_R = crate::BitReader<bool>;
#[doc = "Field `ARCHITECT` reader - Architect"]
pub type ARCHITECT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Architecture Part"]
    #[inline(always)]
    pub fn archpart(&self) -> ARCHPART_R {
        ARCHPART_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Architecture Version"]
    #[inline(always)]
    pub fn archver(&self) -> ARCHVER_R {
        ARCHVER_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - DEVARCH Present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - Architect"]
    #[inline(always)]
    pub fn architect(&self) -> ARCHITECT_R {
        ARCHITECT_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "SCS Device Architecture Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddevarch](index.html) module"]
pub struct DDEVARCH_SPEC;
impl crate::RegisterSpec for DDEVARCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddevarch::R](R) reader structure"]
impl crate::Readable for DDEVARCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDEVARCH to value 0x4770_2a04"]
impl crate::Resettable for DDEVARCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4770_2a04
    }
}
