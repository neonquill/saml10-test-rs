#[doc = "Register `DEMCR` reader"]
pub struct R(crate::R<DEMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEMCR` writer"]
pub struct W(crate::W<DEMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEMCR_SPEC>;
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
impl From<crate::W<DEMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VC_CORERESET` reader - Core reset Halting debug vector catch enable"]
pub type VC_CORERESET_R = crate::BitReader<bool>;
#[doc = "Field `VC_CORERESET` writer - Core reset Halting debug vector catch enable"]
pub type VC_CORERESET_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 0>;
#[doc = "Field `VC_MMERR` reader - MemManage exception Halting debug vector catch enable"]
pub type VC_MMERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_MMERR` writer - MemManage exception Halting debug vector catch enable"]
pub type VC_MMERR_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 4>;
#[doc = "Field `VC_NOCPERR` reader - UsageFault exception coprocessor access Halting debug vector catch enable"]
pub type VC_NOCPERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_NOCPERR` writer - UsageFault exception coprocessor access Halting debug vector catch enable"]
pub type VC_NOCPERR_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 5>;
#[doc = "Field `VC_CHKERR` reader - UsageFault exception checking error Halting debug vector catch enable"]
pub type VC_CHKERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_CHKERR` writer - UsageFault exception checking error Halting debug vector catch enable"]
pub type VC_CHKERR_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 6>;
#[doc = "Field `VC_STATERR` reader - UsageFault exception state information error Halting debug vector catch enable"]
pub type VC_STATERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_STATERR` writer - UsageFault exception state information error Halting debug vector catch enable"]
pub type VC_STATERR_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 7>;
#[doc = "Field `VC_BUSERR` reader - BusFault exception Halting debug vector catch enable"]
pub type VC_BUSERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_BUSERR` writer - BusFault exception Halting debug vector catch enable"]
pub type VC_BUSERR_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 8>;
#[doc = "Field `VC_INTERR` reader - Excception entry and return faults Halting debug vector catch enable"]
pub type VC_INTERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_INTERR` writer - Excception entry and return faults Halting debug vector catch enable"]
pub type VC_INTERR_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 9>;
#[doc = "Field `VC_HARDERR` reader - HardFault exception Halting debug vector catch enable"]
pub type VC_HARDERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_HARDERR` writer - HardFault exception Halting debug vector catch enable"]
pub type VC_HARDERR_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 10>;
#[doc = "Field `VC_SFERR` reader - SecureFault exception Halting debug vector catch enable"]
pub type VC_SFERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_SFERR` writer - SecureFault exception Halting debug vector catch enable"]
pub type VC_SFERR_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 11>;
#[doc = "Field `MON_EN` reader - DebugMonitor enable"]
pub type MON_EN_R = crate::BitReader<bool>;
#[doc = "Field `MON_EN` writer - DebugMonitor enable"]
pub type MON_EN_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 16>;
#[doc = "Field `MON_PEND` reader - DebugMonitor pending state"]
pub type MON_PEND_R = crate::BitReader<bool>;
#[doc = "Field `MON_PEND` writer - DebugMonitor pending state"]
pub type MON_PEND_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 17>;
#[doc = "Field `MON_STEP` reader - Enable DebugMonitor stepping"]
pub type MON_STEP_R = crate::BitReader<bool>;
#[doc = "Field `MON_STEP` writer - Enable DebugMonitor stepping"]
pub type MON_STEP_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 18>;
#[doc = "Field `MON_REQ` reader - DebugMonitor semaphore bit"]
pub type MON_REQ_R = crate::BitReader<bool>;
#[doc = "Field `MON_REQ` writer - DebugMonitor semaphore bit"]
pub type MON_REQ_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 19>;
#[doc = "Field `SDME` reader - Secure DebugMonitor enable"]
pub type SDME_R = crate::BitReader<bool>;
#[doc = "Field `SDME` writer - Secure DebugMonitor enable"]
pub type SDME_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 20>;
#[doc = "Field `TRCENA` reader - Global DWT and ITM features enable"]
pub type TRCENA_R = crate::BitReader<bool>;
#[doc = "Field `TRCENA` writer - Global DWT and ITM features enable"]
pub type TRCENA_W<'a> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, 24>;
impl R {
    #[doc = "Bit 0 - Core reset Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_corereset(&self) -> VC_CORERESET_R {
        VC_CORERESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - MemManage exception Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_mmerr(&self) -> VC_MMERR_R {
        VC_MMERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UsageFault exception coprocessor access Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_nocperr(&self) -> VC_NOCPERR_R {
        VC_NOCPERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UsageFault exception checking error Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_chkerr(&self) -> VC_CHKERR_R {
        VC_CHKERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UsageFault exception state information error Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_staterr(&self) -> VC_STATERR_R {
        VC_STATERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BusFault exception Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_buserr(&self) -> VC_BUSERR_R {
        VC_BUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Excception entry and return faults Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_interr(&self) -> VC_INTERR_R {
        VC_INTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HardFault exception Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_harderr(&self) -> VC_HARDERR_R {
        VC_HARDERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SecureFault exception Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_sferr(&self) -> VC_SFERR_R {
        VC_SFERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - DebugMonitor enable"]
    #[inline(always)]
    pub fn mon_en(&self) -> MON_EN_R {
        MON_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DebugMonitor pending state"]
    #[inline(always)]
    pub fn mon_pend(&self) -> MON_PEND_R {
        MON_PEND_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable DebugMonitor stepping"]
    #[inline(always)]
    pub fn mon_step(&self) -> MON_STEP_R {
        MON_STEP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DebugMonitor semaphore bit"]
    #[inline(always)]
    pub fn mon_req(&self) -> MON_REQ_R {
        MON_REQ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Secure DebugMonitor enable"]
    #[inline(always)]
    pub fn sdme(&self) -> SDME_R {
        SDME_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Global DWT and ITM features enable"]
    #[inline(always)]
    pub fn trcena(&self) -> TRCENA_R {
        TRCENA_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core reset Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_corereset(&mut self) -> VC_CORERESET_W {
        VC_CORERESET_W::new(self)
    }
    #[doc = "Bit 4 - MemManage exception Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_mmerr(&mut self) -> VC_MMERR_W {
        VC_MMERR_W::new(self)
    }
    #[doc = "Bit 5 - UsageFault exception coprocessor access Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_nocperr(&mut self) -> VC_NOCPERR_W {
        VC_NOCPERR_W::new(self)
    }
    #[doc = "Bit 6 - UsageFault exception checking error Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_chkerr(&mut self) -> VC_CHKERR_W {
        VC_CHKERR_W::new(self)
    }
    #[doc = "Bit 7 - UsageFault exception state information error Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_staterr(&mut self) -> VC_STATERR_W {
        VC_STATERR_W::new(self)
    }
    #[doc = "Bit 8 - BusFault exception Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_buserr(&mut self) -> VC_BUSERR_W {
        VC_BUSERR_W::new(self)
    }
    #[doc = "Bit 9 - Excception entry and return faults Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_interr(&mut self) -> VC_INTERR_W {
        VC_INTERR_W::new(self)
    }
    #[doc = "Bit 10 - HardFault exception Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_harderr(&mut self) -> VC_HARDERR_W {
        VC_HARDERR_W::new(self)
    }
    #[doc = "Bit 11 - SecureFault exception Halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_sferr(&mut self) -> VC_SFERR_W {
        VC_SFERR_W::new(self)
    }
    #[doc = "Bit 16 - DebugMonitor enable"]
    #[inline(always)]
    pub fn mon_en(&mut self) -> MON_EN_W {
        MON_EN_W::new(self)
    }
    #[doc = "Bit 17 - DebugMonitor pending state"]
    #[inline(always)]
    pub fn mon_pend(&mut self) -> MON_PEND_W {
        MON_PEND_W::new(self)
    }
    #[doc = "Bit 18 - Enable DebugMonitor stepping"]
    #[inline(always)]
    pub fn mon_step(&mut self) -> MON_STEP_W {
        MON_STEP_W::new(self)
    }
    #[doc = "Bit 19 - DebugMonitor semaphore bit"]
    #[inline(always)]
    pub fn mon_req(&mut self) -> MON_REQ_W {
        MON_REQ_W::new(self)
    }
    #[doc = "Bit 20 - Secure DebugMonitor enable"]
    #[inline(always)]
    pub fn sdme(&mut self) -> SDME_W {
        SDME_W::new(self)
    }
    #[doc = "Bit 24 - Global DWT and ITM features enable"]
    #[inline(always)]
    pub fn trcena(&mut self) -> TRCENA_W {
        TRCENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Exception and Monitor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [demcr](index.html) module"]
pub struct DEMCR_SPEC;
impl crate::RegisterSpec for DEMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [demcr::R](R) reader structure"]
impl crate::Readable for DEMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [demcr::W](W) writer structure"]
impl crate::Writable for DEMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEMCR to value 0"]
impl crate::Resettable for DEMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
