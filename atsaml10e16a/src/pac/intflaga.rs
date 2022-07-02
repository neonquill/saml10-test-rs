#[doc = "Register `INTFLAGA` reader"]
pub struct R(crate::R<INTFLAGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGA` writer"]
pub struct W(crate::W<INTFLAGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGA_SPEC>;
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
impl From<crate::W<INTFLAGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAC_` reader - PAC"]
pub type PAC__R = crate::BitReader<bool>;
#[doc = "Field `PAC_` writer - PAC"]
pub type PAC__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 0>;
#[doc = "Field `PM_` reader - PM"]
pub type PM__R = crate::BitReader<bool>;
#[doc = "Field `PM_` writer - PM"]
pub type PM__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 1>;
#[doc = "Field `MCLK_` reader - MCLK"]
pub type MCLK__R = crate::BitReader<bool>;
#[doc = "Field `MCLK_` writer - MCLK"]
pub type MCLK__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 2>;
#[doc = "Field `RSTC_` reader - RSTC"]
pub type RSTC__R = crate::BitReader<bool>;
#[doc = "Field `RSTC_` writer - RSTC"]
pub type RSTC__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 3>;
#[doc = "Field `OSCCTRL_` reader - OSCCTRL"]
pub type OSCCTRL__R = crate::BitReader<bool>;
#[doc = "Field `OSCCTRL_` writer - OSCCTRL"]
pub type OSCCTRL__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 4>;
#[doc = "Field `OSC32KCTRL_` reader - OSC32KCTRL"]
pub type OSC32KCTRL__R = crate::BitReader<bool>;
#[doc = "Field `OSC32KCTRL_` writer - OSC32KCTRL"]
pub type OSC32KCTRL__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 5>;
#[doc = "Field `SUPC_` reader - SUPC"]
pub type SUPC__R = crate::BitReader<bool>;
#[doc = "Field `SUPC_` writer - SUPC"]
pub type SUPC__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 6>;
#[doc = "Field `GCLK_` reader - GCLK"]
pub type GCLK__R = crate::BitReader<bool>;
#[doc = "Field `GCLK_` writer - GCLK"]
pub type GCLK__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 7>;
#[doc = "Field `WDT_` reader - WDT"]
pub type WDT__R = crate::BitReader<bool>;
#[doc = "Field `WDT_` writer - WDT"]
pub type WDT__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 8>;
#[doc = "Field `RTC_` reader - RTC"]
pub type RTC__R = crate::BitReader<bool>;
#[doc = "Field `RTC_` writer - RTC"]
pub type RTC__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 9>;
#[doc = "Field `EIC_` reader - EIC"]
pub type EIC__R = crate::BitReader<bool>;
#[doc = "Field `EIC_` writer - EIC"]
pub type EIC__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 10>;
#[doc = "Field `FREQM_` reader - FREQM"]
pub type FREQM__R = crate::BitReader<bool>;
#[doc = "Field `FREQM_` writer - FREQM"]
pub type FREQM__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 11>;
#[doc = "Field `PORT_` reader - PORT"]
pub type PORT__R = crate::BitReader<bool>;
#[doc = "Field `PORT_` writer - PORT"]
pub type PORT__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 12>;
#[doc = "Field `AC_` reader - AC"]
pub type AC__R = crate::BitReader<bool>;
#[doc = "Field `AC_` writer - AC"]
pub type AC__W<'a> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - PAC"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PM"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCLK"]
    #[inline(always)]
    pub fn mclk_(&self) -> MCLK__R {
        MCLK__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSTC"]
    #[inline(always)]
    pub fn rstc_(&self) -> RSTC__R {
        RSTC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OSCCTRL"]
    #[inline(always)]
    pub fn oscctrl_(&self) -> OSCCTRL__R {
        OSCCTRL__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OSC32KCTRL"]
    #[inline(always)]
    pub fn osc32kctrl_(&self) -> OSC32KCTRL__R {
        OSC32KCTRL__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SUPC"]
    #[inline(always)]
    pub fn supc_(&self) -> SUPC__R {
        SUPC__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GCLK"]
    #[inline(always)]
    pub fn gclk_(&self) -> GCLK__R {
        GCLK__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WDT"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC"]
    #[inline(always)]
    pub fn rtc_(&self) -> RTC__R {
        RTC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EIC"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FREQM"]
    #[inline(always)]
    pub fn freqm_(&self) -> FREQM__R {
        FREQM__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PORT"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AC"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC"]
    #[inline(always)]
    pub fn pac_(&mut self) -> PAC__W {
        PAC__W::new(self)
    }
    #[doc = "Bit 1 - PM"]
    #[inline(always)]
    pub fn pm_(&mut self) -> PM__W {
        PM__W::new(self)
    }
    #[doc = "Bit 2 - MCLK"]
    #[inline(always)]
    pub fn mclk_(&mut self) -> MCLK__W {
        MCLK__W::new(self)
    }
    #[doc = "Bit 3 - RSTC"]
    #[inline(always)]
    pub fn rstc_(&mut self) -> RSTC__W {
        RSTC__W::new(self)
    }
    #[doc = "Bit 4 - OSCCTRL"]
    #[inline(always)]
    pub fn oscctrl_(&mut self) -> OSCCTRL__W {
        OSCCTRL__W::new(self)
    }
    #[doc = "Bit 5 - OSC32KCTRL"]
    #[inline(always)]
    pub fn osc32kctrl_(&mut self) -> OSC32KCTRL__W {
        OSC32KCTRL__W::new(self)
    }
    #[doc = "Bit 6 - SUPC"]
    #[inline(always)]
    pub fn supc_(&mut self) -> SUPC__W {
        SUPC__W::new(self)
    }
    #[doc = "Bit 7 - GCLK"]
    #[inline(always)]
    pub fn gclk_(&mut self) -> GCLK__W {
        GCLK__W::new(self)
    }
    #[doc = "Bit 8 - WDT"]
    #[inline(always)]
    pub fn wdt_(&mut self) -> WDT__W {
        WDT__W::new(self)
    }
    #[doc = "Bit 9 - RTC"]
    #[inline(always)]
    pub fn rtc_(&mut self) -> RTC__W {
        RTC__W::new(self)
    }
    #[doc = "Bit 10 - EIC"]
    #[inline(always)]
    pub fn eic_(&mut self) -> EIC__W {
        EIC__W::new(self)
    }
    #[doc = "Bit 11 - FREQM"]
    #[inline(always)]
    pub fn freqm_(&mut self) -> FREQM__W {
        FREQM__W::new(self)
    }
    #[doc = "Bit 12 - PORT"]
    #[inline(always)]
    pub fn port_(&mut self) -> PORT__W {
        PORT__W::new(self)
    }
    #[doc = "Bit 13 - AC"]
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
#[doc = "Peripheral interrupt flag status - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflaga](index.html) module"]
pub struct INTFLAGA_SPEC;
impl crate::RegisterSpec for INTFLAGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflaga::R](R) reader structure"]
impl crate::Readable for INTFLAGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflaga::W](W) writer structure"]
impl crate::Writable for INTFLAGA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAGA to value 0"]
impl crate::Resettable for INTFLAGA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
