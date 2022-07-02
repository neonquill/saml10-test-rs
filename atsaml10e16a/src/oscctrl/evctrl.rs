#[doc = "Register `EVCTRL` reader"]
pub struct R(crate::R<EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL` writer"]
pub struct W(crate::W<EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRL_SPEC>;
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
impl From<crate::W<EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFDEO` reader - Clock Failure Detector Event Output Enable"]
pub type CFDEO_R = crate::BitReader<bool>;
#[doc = "Field `CFDEO` writer - Clock Failure Detector Event Output Enable"]
pub type CFDEO_W<'a> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, 0>;
#[doc = "Field `TUNEEI` reader - Tune Event Input Enable"]
pub type TUNEEI_R = crate::BitReader<bool>;
#[doc = "Field `TUNEEI` writer - Tune Event Input Enable"]
pub type TUNEEI_W<'a> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, 1>;
#[doc = "Field `TUNEINV` reader - Tune Event Input Invert"]
pub type TUNEINV_R = crate::BitReader<bool>;
#[doc = "Field `TUNEINV` writer - Tune Event Input Invert"]
pub type TUNEINV_W<'a> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - Clock Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo(&self) -> CFDEO_R {
        CFDEO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tune Event Input Enable"]
    #[inline(always)]
    pub fn tuneei(&self) -> TUNEEI_R {
        TUNEEI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tune Event Input Invert"]
    #[inline(always)]
    pub fn tuneinv(&self) -> TUNEINV_R {
        TUNEINV_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo(&mut self) -> CFDEO_W {
        CFDEO_W::new(self)
    }
    #[doc = "Bit 1 - Tune Event Input Enable"]
    #[inline(always)]
    pub fn tuneei(&mut self) -> TUNEEI_W {
        TUNEEI_W::new(self)
    }
    #[doc = "Bit 2 - Tune Event Input Invert"]
    #[inline(always)]
    pub fn tuneinv(&mut self) -> TUNEINV_W {
        TUNEINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [evctrl::R](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrl::W](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
