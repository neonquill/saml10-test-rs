#[doc = "Register `AHBMASK` reader"]
pub struct R(crate::R<AHBMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBMASK` writer"]
pub struct W(crate::W<AHBMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AHBMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APBA_` reader - AHB-APB Bridge A AHB Clock Mask"]
pub type APBA__R = crate::BitReader<bool>;
#[doc = "Field `APBA_` writer - AHB-APB Bridge A AHB Clock Mask"]
pub type APBA__W<'a> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, 0>;
#[doc = "Field `APBB_` reader - AHB-APB Bridge B AHB Clock Mask"]
pub type APBB__R = crate::BitReader<bool>;
#[doc = "Field `APBB_` writer - AHB-APB Bridge B AHB Clock Mask"]
pub type APBB__W<'a> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, 1>;
#[doc = "Field `APBC_` reader - AHB-APB Bridge C AHB Clock Mask"]
pub type APBC__R = crate::BitReader<bool>;
#[doc = "Field `APBC_` writer - AHB-APB Bridge C AHB Clock Mask"]
pub type APBC__W<'a> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, 2>;
#[doc = "Field `DMAC_` reader - DMAC AHB Clock Mask"]
pub type DMAC__R = crate::BitReader<bool>;
#[doc = "Field `DMAC_` writer - DMAC AHB Clock Mask"]
pub type DMAC__W<'a> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, 3>;
#[doc = "Field `DSU_` reader - DSU AHB Clock Mask"]
pub type DSU__R = crate::BitReader<bool>;
#[doc = "Field `DSU_` writer - DSU AHB Clock Mask"]
pub type DSU__W<'a> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, 4>;
#[doc = "Field `PAC_` reader - PAC AHB Clock Mask"]
pub type PAC__R = crate::BitReader<bool>;
#[doc = "Field `PAC_` writer - PAC AHB Clock Mask"]
pub type PAC__W<'a> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, 6>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL AHB Clock Mask"]
pub type NVMCTRL__R = crate::BitReader<bool>;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL AHB Clock Mask"]
pub type NVMCTRL__W<'a> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, 7>;
#[doc = "Field `TRAM_` reader - TRAM AHB Clock Mask"]
pub type TRAM__R = crate::BitReader<bool>;
#[doc = "Field `TRAM_` writer - TRAM AHB Clock Mask"]
pub type TRAM__W<'a> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, 12>;
impl R {
    #[doc = "Bit 0 - AHB-APB Bridge A AHB Clock Mask"]
    #[inline(always)]
    pub fn apba_(&self) -> APBA__R {
        APBA__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB-APB Bridge B AHB Clock Mask"]
    #[inline(always)]
    pub fn apbb_(&self) -> APBB__R {
        APBB__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB-APB Bridge C AHB Clock Mask"]
    #[inline(always)]
    pub fn apbc_(&self) -> APBC__R {
        APBC__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - PAC AHB Clock Mask"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - TRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn tram_(&self) -> TRAM__R {
        TRAM__R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB-APB Bridge A AHB Clock Mask"]
    #[inline(always)]
    pub fn apba_(&mut self) -> APBA__W {
        APBA__W::new(self)
    }
    #[doc = "Bit 1 - AHB-APB Bridge B AHB Clock Mask"]
    #[inline(always)]
    pub fn apbb_(&mut self) -> APBB__W {
        APBB__W::new(self)
    }
    #[doc = "Bit 2 - AHB-APB Bridge C AHB Clock Mask"]
    #[inline(always)]
    pub fn apbc_(&mut self) -> APBC__W {
        APBC__W::new(self)
    }
    #[doc = "Bit 3 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&mut self) -> DMAC__W {
        DMAC__W::new(self)
    }
    #[doc = "Bit 4 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&mut self) -> DSU__W {
        DSU__W::new(self)
    }
    #[doc = "Bit 6 - PAC AHB Clock Mask"]
    #[inline(always)]
    pub fn pac_(&mut self) -> PAC__W {
        PAC__W::new(self)
    }
    #[doc = "Bit 7 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W {
        NVMCTRL__W::new(self)
    }
    #[doc = "Bit 12 - TRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn tram_(&mut self) -> TRAM__W {
        TRAM__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbmask](index.html) module"]
pub struct AHBMASK_SPEC;
impl crate::RegisterSpec for AHBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbmask::R](R) reader structure"]
impl crate::Readable for AHBMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbmask::W](W) writer structure"]
impl crate::Writable for AHBMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBMASK to value 0x1fff"]
impl crate::Resettable for AHBMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1fff
    }
}
