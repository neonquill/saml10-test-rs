#[doc = "Register `APBCMASK` reader"]
pub struct R(crate::R<APBCMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBCMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBCMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBCMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBCMASK` writer"]
pub struct W(crate::W<APBCMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBCMASK_SPEC>;
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
impl From<crate::W<APBCMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBCMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVSYS_` reader - EVSYS APB Clock Enable"]
pub type EVSYS__R = crate::BitReader<bool>;
#[doc = "Field `EVSYS_` writer - EVSYS APB Clock Enable"]
pub type EVSYS__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 0>;
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Clock Enable"]
pub type SERCOM0__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM0_` writer - SERCOM0 APB Clock Enable"]
pub type SERCOM0__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 1>;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Clock Enable"]
pub type SERCOM1__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM1_` writer - SERCOM1 APB Clock Enable"]
pub type SERCOM1__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 2>;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Clock Enable"]
pub type SERCOM2__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM2_` writer - SERCOM2 APB Clock Enable"]
pub type SERCOM2__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 3>;
#[doc = "Field `TC0_` reader - TC0 APB Clock Enable"]
pub type TC0__R = crate::BitReader<bool>;
#[doc = "Field `TC0_` writer - TC0 APB Clock Enable"]
pub type TC0__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 4>;
#[doc = "Field `TC1_` reader - TC1 APB Clock Enable"]
pub type TC1__R = crate::BitReader<bool>;
#[doc = "Field `TC1_` writer - TC1 APB Clock Enable"]
pub type TC1__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 5>;
#[doc = "Field `TC2_` reader - TC2 APB Clock Enable"]
pub type TC2__R = crate::BitReader<bool>;
#[doc = "Field `TC2_` writer - TC2 APB Clock Enable"]
pub type TC2__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 6>;
#[doc = "Field `ADC_` reader - ADC APB Clock Enable"]
pub type ADC__R = crate::BitReader<bool>;
#[doc = "Field `ADC_` writer - ADC APB Clock Enable"]
pub type ADC__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 7>;
#[doc = "Field `DAC_` reader - DAC APB Clock Enable"]
pub type DAC__R = crate::BitReader<bool>;
#[doc = "Field `DAC_` writer - DAC APB Clock Enable"]
pub type DAC__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 8>;
#[doc = "Field `PTC_` reader - PTC APB Clock Enable"]
pub type PTC__R = crate::BitReader<bool>;
#[doc = "Field `PTC_` writer - PTC APB Clock Enable"]
pub type PTC__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 9>;
#[doc = "Field `TRNG_` reader - TRNG APB Clock Enable"]
pub type TRNG__R = crate::BitReader<bool>;
#[doc = "Field `TRNG_` writer - TRNG APB Clock Enable"]
pub type TRNG__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 10>;
#[doc = "Field `CCL_` reader - CCL APB Clock Enable"]
pub type CCL__R = crate::BitReader<bool>;
#[doc = "Field `CCL_` writer - CCL APB Clock Enable"]
pub type CCL__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 11>;
#[doc = "Field `OPAMP_` reader - OPAMP APB Clock Enable"]
pub type OPAMP__R = crate::BitReader<bool>;
#[doc = "Field `OPAMP_` writer - OPAMP APB Clock Enable"]
pub type OPAMP__W<'a> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, 12>;
impl R {
    #[doc = "Bit 0 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC APB Clock Enable"]
    #[inline(always)]
    pub fn adc_(&self) -> ADC__R {
        ADC__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC APB Clock Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PTC APB Clock Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CCL APB Clock Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OPAMP APB Clock Enable"]
    #[inline(always)]
    pub fn opamp_(&self) -> OPAMP__R {
        OPAMP__R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&mut self) -> EVSYS__W {
        EVSYS__W::new(self)
    }
    #[doc = "Bit 1 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&mut self) -> SERCOM0__W {
        SERCOM0__W::new(self)
    }
    #[doc = "Bit 2 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&mut self) -> SERCOM1__W {
        SERCOM1__W::new(self)
    }
    #[doc = "Bit 3 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&mut self) -> SERCOM2__W {
        SERCOM2__W::new(self)
    }
    #[doc = "Bit 4 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&mut self) -> TC0__W {
        TC0__W::new(self)
    }
    #[doc = "Bit 5 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&mut self) -> TC1__W {
        TC1__W::new(self)
    }
    #[doc = "Bit 6 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&mut self) -> TC2__W {
        TC2__W::new(self)
    }
    #[doc = "Bit 7 - ADC APB Clock Enable"]
    #[inline(always)]
    pub fn adc_(&mut self) -> ADC__W {
        ADC__W::new(self)
    }
    #[doc = "Bit 8 - DAC APB Clock Enable"]
    #[inline(always)]
    pub fn dac_(&mut self) -> DAC__W {
        DAC__W::new(self)
    }
    #[doc = "Bit 9 - PTC APB Clock Enable"]
    #[inline(always)]
    pub fn ptc_(&mut self) -> PTC__W {
        PTC__W::new(self)
    }
    #[doc = "Bit 10 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&mut self) -> TRNG__W {
        TRNG__W::new(self)
    }
    #[doc = "Bit 11 - CCL APB Clock Enable"]
    #[inline(always)]
    pub fn ccl_(&mut self) -> CCL__W {
        CCL__W::new(self)
    }
    #[doc = "Bit 12 - OPAMP APB Clock Enable"]
    #[inline(always)]
    pub fn opamp_(&mut self) -> OPAMP__W {
        OPAMP__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBC Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbcmask](index.html) module"]
pub struct APBCMASK_SPEC;
impl crate::RegisterSpec for APBCMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbcmask::R](R) reader structure"]
impl crate::Readable for APBCMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbcmask::W](W) writer structure"]
impl crate::Writable for APBCMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBCMASK to value 0x1fff"]
impl crate::Resettable for APBCMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1fff
    }
}
