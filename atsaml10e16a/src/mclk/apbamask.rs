#[doc = "Register `APBAMASK` reader"]
pub struct R(crate::R<APBAMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBAMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBAMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBAMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBAMASK` writer"]
pub struct W(crate::W<APBAMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBAMASK_SPEC>;
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
impl From<crate::W<APBAMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBAMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAC_` reader - PAC APB Clock Enable"]
pub type PAC__R = crate::BitReader<bool>;
#[doc = "Field `PAC_` writer - PAC APB Clock Enable"]
pub type PAC__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 0>;
#[doc = "Field `PM_` reader - PM APB Clock Enable"]
pub type PM__R = crate::BitReader<bool>;
#[doc = "Field `PM_` writer - PM APB Clock Enable"]
pub type PM__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 1>;
#[doc = "Field `MCLK_` reader - MCLK APB Clock Enable"]
pub type MCLK__R = crate::BitReader<bool>;
#[doc = "Field `MCLK_` writer - MCLK APB Clock Enable"]
pub type MCLK__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 2>;
#[doc = "Field `RSTC_` reader - RSTC APB Clock Enable"]
pub type RSTC__R = crate::BitReader<bool>;
#[doc = "Field `RSTC_` writer - RSTC APB Clock Enable"]
pub type RSTC__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 3>;
#[doc = "Field `OSCCTRL_` reader - OSCCTRL APB Clock Enable"]
pub type OSCCTRL__R = crate::BitReader<bool>;
#[doc = "Field `OSCCTRL_` writer - OSCCTRL APB Clock Enable"]
pub type OSCCTRL__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 4>;
#[doc = "Field `OSC32KCTRL_` reader - OSC32KCTRL APB Clock Enable"]
pub type OSC32KCTRL__R = crate::BitReader<bool>;
#[doc = "Field `OSC32KCTRL_` writer - OSC32KCTRL APB Clock Enable"]
pub type OSC32KCTRL__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 5>;
#[doc = "Field `SUPC_` reader - SUPC APB Clock Enable"]
pub type SUPC__R = crate::BitReader<bool>;
#[doc = "Field `SUPC_` writer - SUPC APB Clock Enable"]
pub type SUPC__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 6>;
#[doc = "Field `GCLK_` reader - GCLK APB Clock Enable"]
pub type GCLK__R = crate::BitReader<bool>;
#[doc = "Field `GCLK_` writer - GCLK APB Clock Enable"]
pub type GCLK__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 7>;
#[doc = "Field `WDT_` reader - WDT APB Clock Enable"]
pub type WDT__R = crate::BitReader<bool>;
#[doc = "Field `WDT_` writer - WDT APB Clock Enable"]
pub type WDT__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 8>;
#[doc = "Field `RTC_` reader - RTC APB Clock Enable"]
pub type RTC__R = crate::BitReader<bool>;
#[doc = "Field `RTC_` writer - RTC APB Clock Enable"]
pub type RTC__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 9>;
#[doc = "Field `EIC_` reader - EIC APB Clock Enable"]
pub type EIC__R = crate::BitReader<bool>;
#[doc = "Field `EIC_` writer - EIC APB Clock Enable"]
pub type EIC__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 10>;
#[doc = "Field `FREQM_` reader - FREQM APB Clock Enable"]
pub type FREQM__R = crate::BitReader<bool>;
#[doc = "Field `FREQM_` writer - FREQM APB Clock Enable"]
pub type FREQM__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 11>;
#[doc = "Field `PORT_` reader - PORT APB Clock Enable"]
pub type PORT__R = crate::BitReader<bool>;
#[doc = "Field `PORT_` writer - PORT APB Clock Enable"]
pub type PORT__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 12>;
#[doc = "Field `AC_` reader - AC APB Clock Enable"]
pub type AC__R = crate::BitReader<bool>;
#[doc = "Field `AC_` writer - AC APB Clock Enable"]
pub type AC__W<'a> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - PAC APB Clock Enable"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCLK APB Clock Enable"]
    #[inline(always)]
    pub fn mclk_(&self) -> MCLK__R {
        MCLK__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSTC APB Clock Enable"]
    #[inline(always)]
    pub fn rstc_(&self) -> RSTC__R {
        RSTC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OSCCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn oscctrl_(&self) -> OSCCTRL__R {
        OSCCTRL__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OSC32KCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn osc32kctrl_(&self) -> OSC32KCTRL__R {
        OSC32KCTRL__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SUPC APB Clock Enable"]
    #[inline(always)]
    pub fn supc_(&self) -> SUPC__R {
        SUPC__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GCLK APB Clock Enable"]
    #[inline(always)]
    pub fn gclk_(&self) -> GCLK__R {
        GCLK__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtc_(&self) -> RTC__R {
        RTC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FREQM APB Clock Enable"]
    #[inline(always)]
    pub fn freqm_(&self) -> FREQM__R {
        FREQM__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PORT APB Clock Enable"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC APB Clock Enable"]
    #[inline(always)]
    pub fn pac_(&mut self) -> PAC__W {
        PAC__W::new(self)
    }
    #[doc = "Bit 1 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&mut self) -> PM__W {
        PM__W::new(self)
    }
    #[doc = "Bit 2 - MCLK APB Clock Enable"]
    #[inline(always)]
    pub fn mclk_(&mut self) -> MCLK__W {
        MCLK__W::new(self)
    }
    #[doc = "Bit 3 - RSTC APB Clock Enable"]
    #[inline(always)]
    pub fn rstc_(&mut self) -> RSTC__W {
        RSTC__W::new(self)
    }
    #[doc = "Bit 4 - OSCCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn oscctrl_(&mut self) -> OSCCTRL__W {
        OSCCTRL__W::new(self)
    }
    #[doc = "Bit 5 - OSC32KCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn osc32kctrl_(&mut self) -> OSC32KCTRL__W {
        OSC32KCTRL__W::new(self)
    }
    #[doc = "Bit 6 - SUPC APB Clock Enable"]
    #[inline(always)]
    pub fn supc_(&mut self) -> SUPC__W {
        SUPC__W::new(self)
    }
    #[doc = "Bit 7 - GCLK APB Clock Enable"]
    #[inline(always)]
    pub fn gclk_(&mut self) -> GCLK__W {
        GCLK__W::new(self)
    }
    #[doc = "Bit 8 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&mut self) -> WDT__W {
        WDT__W::new(self)
    }
    #[doc = "Bit 9 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtc_(&mut self) -> RTC__W {
        RTC__W::new(self)
    }
    #[doc = "Bit 10 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&mut self) -> EIC__W {
        EIC__W::new(self)
    }
    #[doc = "Bit 11 - FREQM APB Clock Enable"]
    #[inline(always)]
    pub fn freqm_(&mut self) -> FREQM__W {
        FREQM__W::new(self)
    }
    #[doc = "Bit 12 - PORT APB Clock Enable"]
    #[inline(always)]
    pub fn port_(&mut self) -> PORT__W {
        PORT__W::new(self)
    }
    #[doc = "Bit 13 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&mut self) -> AC__W {
        AC__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBA Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbamask](index.html) module"]
pub struct APBAMASK_SPEC;
impl crate::RegisterSpec for APBAMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbamask::R](R) reader structure"]
impl crate::Readable for APBAMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbamask::W](W) writer structure"]
impl crate::Writable for APBAMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBAMASK to value 0x7fff"]
impl crate::Resettable for APBAMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff
    }
}
