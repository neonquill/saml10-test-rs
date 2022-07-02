#[doc = "Register `NONSECB` reader"]
pub struct R(crate::R<NONSECB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NONSECB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NONSECB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NONSECB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDAU_` reader - IDAU Non-Secure"]
pub type IDAU__R = crate::BitReader<bool>;
#[doc = "Field `DSU_` reader - DSU Non-Secure"]
pub type DSU__R = crate::BitReader<bool>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL Non-Secure"]
pub type NVMCTRL__R = crate::BitReader<bool>;
#[doc = "Field `DMAC_` reader - DMAC Non-Secure"]
pub type DMAC__R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - IDAU Non-Secure"]
    #[inline(always)]
    pub fn idau_(&self) -> IDAU__R {
        IDAU__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU Non-Secure"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL Non-Secure"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC Non-Secure"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Peripheral non-secure status - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nonsecb](index.html) module"]
pub struct NONSECB_SPEC;
impl crate::RegisterSpec for NONSECB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nonsecb::R](R) reader structure"]
impl crate::Readable for NONSECB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NONSECB to value 0x02"]
impl crate::Resettable for NONSECB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
