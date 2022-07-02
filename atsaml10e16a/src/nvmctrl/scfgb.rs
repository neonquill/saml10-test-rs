#[doc = "Register `SCFGB` reader"]
pub struct R(crate::R<SCFGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFGB` writer"]
pub struct W(crate::W<SCFGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFGB_SPEC>;
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
impl From<crate::W<SCFGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCREN` reader - Boot Configuration Row Read Enable"]
pub type BCREN_R = crate::BitReader<bool>;
#[doc = "Field `BCREN` writer - Boot Configuration Row Read Enable"]
pub type BCREN_W<'a> = crate::BitWriter<'a, u32, SCFGB_SPEC, bool, 0>;
#[doc = "Field `BCWEN` reader - Boot Configuration Row Write Enable"]
pub type BCWEN_R = crate::BitReader<bool>;
#[doc = "Field `BCWEN` writer - Boot Configuration Row Write Enable"]
pub type BCWEN_W<'a> = crate::BitWriter<'a, u32, SCFGB_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Boot Configuration Row Read Enable"]
    #[inline(always)]
    pub fn bcren(&self) -> BCREN_R {
        BCREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boot Configuration Row Write Enable"]
    #[inline(always)]
    pub fn bcwen(&self) -> BCWEN_R {
        BCWEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Configuration Row Read Enable"]
    #[inline(always)]
    pub fn bcren(&mut self) -> BCREN_W {
        BCREN_W::new(self)
    }
    #[doc = "Bit 1 - Boot Configuration Row Write Enable"]
    #[inline(always)]
    pub fn bcwen(&mut self) -> BCWEN_W {
        BCWEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure Boot Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfgb](index.html) module"]
pub struct SCFGB_SPEC;
impl crate::RegisterSpec for SCFGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scfgb::R](R) reader structure"]
impl crate::Readable for SCFGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfgb::W](W) writer structure"]
impl crate::Writable for SCFGB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCFGB to value 0x03"]
impl crate::Resettable for SCFGB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
