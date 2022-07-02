#[doc = "Register `DFLLULPDLY` reader"]
pub struct R(crate::R<DFLLULPDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLULPDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLULPDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLULPDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLULPDLY` writer"]
pub struct W(crate::W<DFLLULPDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLULPDLY_SPEC>;
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
impl From<crate::W<DFLLULPDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLULPDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELAY` reader - Delay Value"]
pub type DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELAY` writer - Delay Value"]
pub type DELAY_W<'a> =
    crate::FieldWriter<'a, u32, DFLLULPDLY_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Delay Value"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay Value"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLLULP Delay Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllulpdly](index.html) module"]
pub struct DFLLULPDLY_SPEC;
impl crate::RegisterSpec for DFLLULPDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfllulpdly::R](R) reader structure"]
impl crate::Readable for DFLLULPDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllulpdly::W](W) writer structure"]
impl crate::Writable for DFLLULPDLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFLLULPDLY to value 0x80"]
impl crate::Resettable for DFLLULPDLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
