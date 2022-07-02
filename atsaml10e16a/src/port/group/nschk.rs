#[doc = "Register `NSCHK` reader"]
pub struct R(crate::R<NSCHK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSCHK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSCHK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSCHK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSCHK` writer"]
pub struct W(crate::W<NSCHK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSCHK_SPEC>;
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
impl From<crate::W<NSCHK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSCHK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSCHK` reader - Port Security Attribution Check"]
pub type NSCHK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NSCHK` writer - Port Security Attribution Check"]
pub type NSCHK_W<'a> = crate::FieldWriter<'a, u32, NSCHK_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Port Security Attribution Check"]
    #[inline(always)]
    pub fn nschk(&self) -> NSCHK_R {
        NSCHK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Security Attribution Check"]
    #[inline(always)]
    pub fn nschk(&mut self) -> NSCHK_W {
        NSCHK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Attribution Check\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nschk](index.html) module"]
pub struct NSCHK_SPEC;
impl crate::RegisterSpec for NSCHK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nschk::R](R) reader structure"]
impl crate::Readable for NSCHK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nschk::W](W) writer structure"]
impl crate::Writable for NSCHK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NSCHK to value 0"]
impl crate::Resettable for NSCHK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
