#[doc = "Register `NONSECA` reader"]
pub struct R(crate::R<NONSECA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NONSECA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NONSECA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NONSECA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PAC_` reader - PAC Non-Secure"]
pub type PAC__R = crate::BitReader<bool>;
#[doc = "Field `PM_` reader - PM Non-Secure"]
pub type PM__R = crate::BitReader<bool>;
#[doc = "Field `MCLK_` reader - MCLK Non-Secure"]
pub type MCLK__R = crate::BitReader<bool>;
#[doc = "Field `RSTC_` reader - RSTC Non-Secure"]
pub type RSTC__R = crate::BitReader<bool>;
#[doc = "Field `OSCCTRL_` reader - OSCCTRL Non-Secure"]
pub type OSCCTRL__R = crate::BitReader<bool>;
#[doc = "Field `OSC32KCTRL_` reader - OSC32KCTRL Non-Secure"]
pub type OSC32KCTRL__R = crate::BitReader<bool>;
#[doc = "Field `SUPC_` reader - SUPC Non-Secure"]
pub type SUPC__R = crate::BitReader<bool>;
#[doc = "Field `GCLK_` reader - GCLK Non-Secure"]
pub type GCLK__R = crate::BitReader<bool>;
#[doc = "Field `WDT_` reader - WDT Non-Secure"]
pub type WDT__R = crate::BitReader<bool>;
#[doc = "Field `RTC_` reader - RTC Non-Secure"]
pub type RTC__R = crate::BitReader<bool>;
#[doc = "Field `EIC_` reader - EIC Non-Secure"]
pub type EIC__R = crate::BitReader<bool>;
#[doc = "Field `FREQM_` reader - FREQM Non-Secure"]
pub type FREQM__R = crate::BitReader<bool>;
#[doc = "Field `PORT_` reader - PORT Non-Secure"]
pub type PORT__R = crate::BitReader<bool>;
#[doc = "Field `AC_` reader - AC Non-Secure"]
pub type AC__R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - PAC Non-Secure"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PM Non-Secure"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCLK Non-Secure"]
    #[inline(always)]
    pub fn mclk_(&self) -> MCLK__R {
        MCLK__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSTC Non-Secure"]
    #[inline(always)]
    pub fn rstc_(&self) -> RSTC__R {
        RSTC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OSCCTRL Non-Secure"]
    #[inline(always)]
    pub fn oscctrl_(&self) -> OSCCTRL__R {
        OSCCTRL__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OSC32KCTRL Non-Secure"]
    #[inline(always)]
    pub fn osc32kctrl_(&self) -> OSC32KCTRL__R {
        OSC32KCTRL__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SUPC Non-Secure"]
    #[inline(always)]
    pub fn supc_(&self) -> SUPC__R {
        SUPC__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GCLK Non-Secure"]
    #[inline(always)]
    pub fn gclk_(&self) -> GCLK__R {
        GCLK__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WDT Non-Secure"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC Non-Secure"]
    #[inline(always)]
    pub fn rtc_(&self) -> RTC__R {
        RTC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EIC Non-Secure"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FREQM Non-Secure"]
    #[inline(always)]
    pub fn freqm_(&self) -> FREQM__R {
        FREQM__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PORT Non-Secure"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AC Non-Secure"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Peripheral non-secure status - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nonseca](index.html) module"]
pub struct NONSECA_SPEC;
impl crate::RegisterSpec for NONSECA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nonseca::R](R) reader structure"]
impl crate::Readable for NONSECA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NONSECA to value 0"]
impl crate::Resettable for NONSECA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
