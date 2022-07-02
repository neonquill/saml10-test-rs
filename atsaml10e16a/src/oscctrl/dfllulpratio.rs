#[doc = "Register `DFLLULPRATIO` reader"]
pub struct R(crate::R<DFLLULPRATIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLULPRATIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLULPRATIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLULPRATIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLULPRATIO` writer"]
pub struct W(crate::W<DFLLULPRATIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLULPRATIO_SPEC>;
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
impl From<crate::W<DFLLULPRATIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLULPRATIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RATIO` reader - Target Tuner Ratio"]
pub type RATIO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RATIO` writer - Target Tuner Ratio"]
pub type RATIO_W<'a> =
    crate::FieldWriter<'a, u32, DFLLULPRATIO_SPEC, u16, u16, 11, 0>;
impl R {
    #[doc = "Bits 0:10 - Target Tuner Ratio"]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Target Tuner Ratio"]
    #[inline(always)]
    pub fn ratio(&mut self) -> RATIO_W {
        RATIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLLULP Target Ratio\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllulpratio](index.html) module"]
pub struct DFLLULPRATIO_SPEC;
impl crate::RegisterSpec for DFLLULPRATIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfllulpratio::R](R) reader structure"]
impl crate::Readable for DFLLULPRATIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllulpratio::W](W) writer structure"]
impl crate::Writable for DFLLULPRATIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFLLULPRATIO to value 0"]
impl crate::Resettable for DFLLULPRATIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
