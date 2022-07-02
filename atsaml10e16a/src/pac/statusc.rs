#[doc = "Register `STATUSC` reader"]
pub struct R(crate::R<STATUSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVSYS_` reader - EVSYS APB Protect Enable"]
pub type EVSYS__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Protect Enable"]
pub type SERCOM0__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Protect Enable"]
pub type SERCOM1__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Protect Enable"]
pub type SERCOM2__R = crate::BitReader<bool>;
#[doc = "Field `TC0_` reader - TC0 APB Protect Enable"]
pub type TC0__R = crate::BitReader<bool>;
#[doc = "Field `TC1_` reader - TC1 APB Protect Enable"]
pub type TC1__R = crate::BitReader<bool>;
#[doc = "Field `TC2_` reader - TC2 APB Protect Enable"]
pub type TC2__R = crate::BitReader<bool>;
#[doc = "Field `ADC_` reader - ADC APB Protect Enable"]
pub type ADC__R = crate::BitReader<bool>;
#[doc = "Field `DAC_` reader - DAC APB Protect Enable"]
pub type DAC__R = crate::BitReader<bool>;
#[doc = "Field `PTC_` reader - PTC APB Protect Enable"]
pub type PTC__R = crate::BitReader<bool>;
#[doc = "Field `TRNG_` reader - TRNG APB Protect Enable"]
pub type TRNG__R = crate::BitReader<bool>;
#[doc = "Field `CCL_` reader - CCL APB Protect Enable"]
pub type CCL__R = crate::BitReader<bool>;
#[doc = "Field `OPAMP_` reader - OPAMP APB Protect Enable"]
pub type OPAMP__R = crate::BitReader<bool>;
#[doc = "Field `TRAM_` reader - TRAM APB Protect Enable"]
pub type TRAM__R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - EVSYS APB Protect Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM0 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM1 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM2 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC APB Protect Enable"]
    #[inline(always)]
    pub fn adc_(&self) -> ADC__R {
        ADC__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC APB Protect Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PTC APB Protect Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TRNG APB Protect Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CCL APB Protect Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OPAMP APB Protect Enable"]
    #[inline(always)]
    pub fn opamp_(&self) -> OPAMP__R {
        OPAMP__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TRAM APB Protect Enable"]
    #[inline(always)]
    pub fn tram_(&self) -> TRAM__R {
        TRAM__R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusc](index.html) module"]
pub struct STATUSC_SPEC;
impl crate::RegisterSpec for STATUSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusc::R](R) reader structure"]
impl crate::Readable for STATUSC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSC to value 0"]
impl crate::Resettable for STATUSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
