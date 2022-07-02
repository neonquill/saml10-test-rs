#[doc = "Register `SCFGAD` reader"]
pub struct R(crate::R<SCFGAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFGAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFGAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFGAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFGAD` writer"]
pub struct W(crate::W<SCFGAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFGAD_SPEC>;
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
impl From<crate::W<SCFGAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFGAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `URWEN` reader - User Row Write Enable"]
pub type URWEN_R = crate::BitReader<bool>;
#[doc = "Field `URWEN` writer - User Row Write Enable"]
pub type URWEN_W<'a> = crate::BitWriter<'a, u32, SCFGAD_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - User Row Write Enable"]
    #[inline(always)]
    pub fn urwen(&self) -> URWEN_R {
        URWEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User Row Write Enable"]
    #[inline(always)]
    pub fn urwen(&mut self) -> URWEN_W {
        URWEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure Application and Data Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfgad](index.html) module"]
pub struct SCFGAD_SPEC;
impl crate::RegisterSpec for SCFGAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scfgad::R](R) reader structure"]
impl crate::Readable for SCFGAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfgad::W](W) writer structure"]
impl crate::Writable for SCFGAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCFGAD to value 0x01"]
impl crate::Resettable for SCFGAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
