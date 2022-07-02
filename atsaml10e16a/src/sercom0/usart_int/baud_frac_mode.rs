#[doc = "Register `BAUD_FRAC_MODE` reader"]
pub struct R(crate::R<BAUD_FRAC_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD_FRAC_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD_FRAC_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD_FRAC_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD_FRAC_MODE` writer"]
pub struct W(crate::W<BAUD_FRAC_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD_FRAC_MODE_SPEC>;
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
impl From<crate::W<BAUD_FRAC_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD_FRAC_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUD` reader - Baud Rate Value"]
pub type BAUD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BAUD` writer - Baud Rate Value"]
pub type BAUD_W<'a> =
    crate::FieldWriter<'a, u16, BAUD_FRAC_MODE_SPEC, u16, u16, 13, 0>;
#[doc = "Field `FP` reader - Fractional Part"]
pub type FP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FP` writer - Fractional Part"]
pub type FP_W<'a> =
    crate::FieldWriter<'a, u16, BAUD_FRAC_MODE_SPEC, u8, u8, 3, 13>;
impl R {
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FP_R {
        FP_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&mut self) -> BAUD_W {
        BAUD_W::new(self)
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&mut self) -> FP_W {
        FP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_INT Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud_frac_mode](index.html) module"]
pub struct BAUD_FRAC_MODE_SPEC;
impl crate::RegisterSpec for BAUD_FRAC_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [baud_frac_mode::R](R) reader structure"]
impl crate::Readable for BAUD_FRAC_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud_frac_mode::W](W) writer structure"]
impl crate::Writable for BAUD_FRAC_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUD_FRAC_MODE to value 0"]
impl crate::Resettable for BAUD_FRAC_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
