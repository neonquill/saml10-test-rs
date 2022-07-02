#[doc = "Register `PINCFG[%s]` reader"]
pub struct R(crate::R<PINCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINCFG[%s]` writer"]
pub struct W(crate::W<PINCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINCFG_SPEC>;
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
impl From<crate::W<PINCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMUXEN` reader - Peripheral Multiplexer Enable"]
pub type PMUXEN_R = crate::BitReader<bool>;
#[doc = "Field `PMUXEN` writer - Peripheral Multiplexer Enable"]
pub type PMUXEN_W<'a> = crate::BitWriter<'a, u8, PINCFG_SPEC, bool, 0>;
#[doc = "Field `INEN` reader - Input Enable"]
pub type INEN_R = crate::BitReader<bool>;
#[doc = "Field `INEN` writer - Input Enable"]
pub type INEN_W<'a> = crate::BitWriter<'a, u8, PINCFG_SPEC, bool, 1>;
#[doc = "Field `PULLEN` reader - Pull Enable"]
pub type PULLEN_R = crate::BitReader<bool>;
#[doc = "Field `PULLEN` writer - Pull Enable"]
pub type PULLEN_W<'a> = crate::BitWriter<'a, u8, PINCFG_SPEC, bool, 2>;
#[doc = "Field `DRVSTR` reader - Output Driver Strength Selection"]
pub type DRVSTR_R = crate::BitReader<bool>;
#[doc = "Field `DRVSTR` writer - Output Driver Strength Selection"]
pub type DRVSTR_W<'a> = crate::BitWriter<'a, u8, PINCFG_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&self) -> PMUXEN_R {
        PMUXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    pub fn inen(&self) -> INEN_R {
        INEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&self) -> PULLEN_R {
        PULLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Output Driver Strength Selection"]
    #[inline(always)]
    pub fn drvstr(&self) -> DRVSTR_R {
        DRVSTR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PMUXEN_W {
        PMUXEN_W::new(self)
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    pub fn inen(&mut self) -> INEN_W {
        INEN_W::new(self)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&mut self) -> PULLEN_W {
        PULLEN_W::new(self)
    }
    #[doc = "Bit 6 - Output Driver Strength Selection"]
    #[inline(always)]
    pub fn drvstr(&mut self) -> DRVSTR_W {
        DRVSTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pincfg](index.html) module"]
pub struct PINCFG_SPEC;
impl crate::RegisterSpec for PINCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pincfg::R](R) reader structure"]
impl crate::Readable for PINCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pincfg::W](W) writer structure"]
impl crate::Writable for PINCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINCFG[%s]
to value 0"]
impl crate::Resettable for PINCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
