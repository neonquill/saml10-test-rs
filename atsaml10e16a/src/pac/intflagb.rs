#[doc = "Register `INTFLAGB` reader"]
pub struct R(crate::R<INTFLAGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGB` writer"]
pub struct W(crate::W<INTFLAGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGB_SPEC>;
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
impl From<crate::W<INTFLAGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDAU_` reader - IDAU"]
pub type IDAU__R = crate::BitReader<bool>;
#[doc = "Field `IDAU_` writer - IDAU"]
pub type IDAU__W<'a> = crate::BitWriter<'a, u32, INTFLAGB_SPEC, bool, 0>;
#[doc = "Field `DSU_` reader - DSU"]
pub type DSU__R = crate::BitReader<bool>;
#[doc = "Field `DSU_` writer - DSU"]
pub type DSU__W<'a> = crate::BitWriter<'a, u32, INTFLAGB_SPEC, bool, 1>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL"]
pub type NVMCTRL__R = crate::BitReader<bool>;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL"]
pub type NVMCTRL__W<'a> = crate::BitWriter<'a, u32, INTFLAGB_SPEC, bool, 2>;
#[doc = "Field `DMAC_` reader - DMAC"]
pub type DMAC__R = crate::BitReader<bool>;
#[doc = "Field `DMAC_` writer - DMAC"]
pub type DMAC__W<'a> = crate::BitWriter<'a, u32, INTFLAGB_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - IDAU"]
    #[inline(always)]
    pub fn idau_(&self) -> IDAU__R {
        IDAU__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IDAU"]
    #[inline(always)]
    pub fn idau_(&mut self) -> IDAU__W {
        IDAU__W::new(self)
    }
    #[doc = "Bit 1 - DSU"]
    #[inline(always)]
    pub fn dsu_(&mut self) -> DSU__W {
        DSU__W::new(self)
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline(always)]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W {
        NVMCTRL__W::new(self)
    }
    #[doc = "Bit 3 - DMAC"]
    #[inline(always)]
    pub fn dmac_(&mut self) -> DMAC__W {
        DMAC__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral interrupt flag status - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagb](index.html) module"]
pub struct INTFLAGB_SPEC;
impl crate::RegisterSpec for INTFLAGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagb::R](R) reader structure"]
impl crate::Readable for INTFLAGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagb::W](W) writer structure"]
impl crate::Writable for INTFLAGB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAGB to value 0"]
impl crate::Resettable for INTFLAGB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
