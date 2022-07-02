#[doc = "Register `NONSECC` reader"]
pub struct R(crate::R<NONSECC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NONSECC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NONSECC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NONSECC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVSYS_` reader - EVSYS Non-Secure"]
pub type EVSYS__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM0_` reader - SERCOM0 Non-Secure"]
pub type SERCOM0__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM1_` reader - SERCOM1 Non-Secure"]
pub type SERCOM1__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM2_` reader - SERCOM2 Non-Secure"]
pub type SERCOM2__R = crate::BitReader<bool>;
#[doc = "Field `TC0_` reader - TC0 Non-Secure"]
pub type TC0__R = crate::BitReader<bool>;
#[doc = "Field `TC1_` reader - TC1 Non-Secure"]
pub type TC1__R = crate::BitReader<bool>;
#[doc = "Field `TC2_` reader - TC2 Non-Secure"]
pub type TC2__R = crate::BitReader<bool>;
#[doc = "Field `ADC_` reader - ADC Non-Secure"]
pub type ADC__R = crate::BitReader<bool>;
#[doc = "Field `DAC_` reader - DAC Non-Secure"]
pub type DAC__R = crate::BitReader<bool>;
#[doc = "Field `PTC_` reader - PTC Non-Secure"]
pub type PTC__R = crate::BitReader<bool>;
#[doc = "Field `TRNG_` reader - TRNG Non-Secure"]
pub type TRNG__R = crate::BitReader<bool>;
#[doc = "Field `CCL_` reader - CCL Non-Secure"]
pub type CCL__R = crate::BitReader<bool>;
#[doc = "Field `OPAMP_` reader - OPAMP Non-Secure"]
pub type OPAMP__R = crate::BitReader<bool>;
#[doc = "Field `TRAM_` reader - TRAM Non-Secure"]
pub type TRAM__R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - EVSYS Non-Secure"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM0 Non-Secure"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM1 Non-Secure"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM2 Non-Secure"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TC0 Non-Secure"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC1 Non-Secure"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC2 Non-Secure"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC Non-Secure"]
    #[inline(always)]
    pub fn adc_(&self) -> ADC__R {
        ADC__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC Non-Secure"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PTC Non-Secure"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TRNG Non-Secure"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CCL Non-Secure"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OPAMP Non-Secure"]
    #[inline(always)]
    pub fn opamp_(&self) -> OPAMP__R {
        OPAMP__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TRAM Non-Secure"]
    #[inline(always)]
    pub fn tram_(&self) -> TRAM__R {
        TRAM__R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Peripheral non-secure status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nonsecc](index.html) module"]
pub struct NONSECC_SPEC;
impl crate::RegisterSpec for NONSECC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nonsecc::R](R) reader structure"]
impl crate::Readable for NONSECC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NONSECC to value 0"]
impl crate::Resettable for NONSECC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
