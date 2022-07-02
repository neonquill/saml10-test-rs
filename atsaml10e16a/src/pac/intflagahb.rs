#[doc = "Register `INTFLAGAHB` reader"]
pub struct R(crate::R<INTFLAGAHB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGAHB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGAHB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGAHB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGAHB` writer"]
pub struct W(crate::W<INTFLAGAHB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGAHB_SPEC>;
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
impl From<crate::W<INTFLAGAHB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGAHB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_` reader - FLASH"]
pub type FLASH__R = crate::BitReader<bool>;
#[doc = "Field `FLASH_` writer - FLASH"]
pub type FLASH__W<'a> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, 0>;
#[doc = "Field `APBA_` reader - AHB-APB Bridge A"]
pub type APBA__R = crate::BitReader<bool>;
#[doc = "Field `APBA_` writer - AHB-APB Bridge A"]
pub type APBA__W<'a> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, 1>;
#[doc = "Field `APBB_` reader - AHB-APB Bridge B"]
pub type APBB__R = crate::BitReader<bool>;
#[doc = "Field `APBB_` writer - AHB-APB Bridge B"]
pub type APBB__W<'a> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, 2>;
#[doc = "Field `APBC_` reader - AHB-APB Bridge C"]
pub type APBC__R = crate::BitReader<bool>;
#[doc = "Field `APBC_` writer - AHB-APB Bridge C"]
pub type APBC__W<'a> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, 3>;
#[doc = "Field `HSRAMCPU_` reader - HSRAMCPU"]
pub type HSRAMCPU__R = crate::BitReader<bool>;
#[doc = "Field `HSRAMCPU_` writer - HSRAMCPU"]
pub type HSRAMCPU__W<'a> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, 4>;
#[doc = "Field `HSRAMDMAC_` reader - HSRAMDMAC"]
pub type HSRAMDMAC__R = crate::BitReader<bool>;
#[doc = "Field `HSRAMDMAC_` writer - HSRAMDMAC"]
pub type HSRAMDMAC__W<'a> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, 5>;
#[doc = "Field `HSRAMDSU_` reader - HSRAMDSU"]
pub type HSRAMDSU__R = crate::BitReader<bool>;
#[doc = "Field `HSRAMDSU_` writer - HSRAMDSU"]
pub type HSRAMDSU__W<'a> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, 6>;
#[doc = "Field `BROM_` reader - BROM"]
pub type BROM__R = crate::BitReader<bool>;
#[doc = "Field `BROM_` writer - BROM"]
pub type BROM__W<'a> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&self) -> FLASH__R {
        FLASH__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB-APB Bridge A"]
    #[inline(always)]
    pub fn apba_(&self) -> APBA__R {
        APBA__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB-APB Bridge B"]
    #[inline(always)]
    pub fn apbb_(&self) -> APBB__R {
        APBB__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AHB-APB Bridge C"]
    #[inline(always)]
    pub fn apbc_(&self) -> APBC__R {
        APBC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSRAMCPU"]
    #[inline(always)]
    pub fn hsramcpu_(&self) -> HSRAMCPU__R {
        HSRAMCPU__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSRAMDMAC"]
    #[inline(always)]
    pub fn hsramdmac_(&self) -> HSRAMDMAC__R {
        HSRAMDMAC__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSRAMDSU"]
    #[inline(always)]
    pub fn hsramdsu_(&self) -> HSRAMDSU__R {
        HSRAMDSU__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BROM"]
    #[inline(always)]
    pub fn brom_(&self) -> BROM__R {
        BROM__R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&mut self) -> FLASH__W {
        FLASH__W::new(self)
    }
    #[doc = "Bit 1 - AHB-APB Bridge A"]
    #[inline(always)]
    pub fn apba_(&mut self) -> APBA__W {
        APBA__W::new(self)
    }
    #[doc = "Bit 2 - AHB-APB Bridge B"]
    #[inline(always)]
    pub fn apbb_(&mut self) -> APBB__W {
        APBB__W::new(self)
    }
    #[doc = "Bit 3 - AHB-APB Bridge C"]
    #[inline(always)]
    pub fn apbc_(&mut self) -> APBC__W {
        APBC__W::new(self)
    }
    #[doc = "Bit 4 - HSRAMCPU"]
    #[inline(always)]
    pub fn hsramcpu_(&mut self) -> HSRAMCPU__W {
        HSRAMCPU__W::new(self)
    }
    #[doc = "Bit 5 - HSRAMDMAC"]
    #[inline(always)]
    pub fn hsramdmac_(&mut self) -> HSRAMDMAC__W {
        HSRAMDMAC__W::new(self)
    }
    #[doc = "Bit 6 - HSRAMDSU"]
    #[inline(always)]
    pub fn hsramdsu_(&mut self) -> HSRAMDSU__W {
        HSRAMDSU__W::new(self)
    }
    #[doc = "Bit 7 - BROM"]
    #[inline(always)]
    pub fn brom_(&mut self) -> BROM__W {
        BROM__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bridge interrupt flag status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagahb](index.html) module"]
pub struct INTFLAGAHB_SPEC;
impl crate::RegisterSpec for INTFLAGAHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagahb::R](R) reader structure"]
impl crate::Readable for INTFLAGAHB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagahb::W](W) writer structure"]
impl crate::Writable for INTFLAGAHB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAGAHB to value 0"]
impl crate::Resettable for INTFLAGAHB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
