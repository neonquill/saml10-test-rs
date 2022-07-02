#[doc = "Register `DFLLULPRREQ` reader"]
pub struct R(crate::R<DFLLULPRREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLULPRREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLULPRREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLULPRREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLULPRREQ` writer"]
pub struct W(crate::W<DFLLULPRREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLULPRREQ_SPEC>;
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
impl From<crate::W<DFLLULPRREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLULPRREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RREQ` reader - Read Request"]
pub type RREQ_R = crate::BitReader<bool>;
#[doc = "Field `RREQ` writer - Read Request"]
pub type RREQ_W<'a> = crate::BitWriter<'a, u8, DFLLULPRREQ_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 7 - Read Request"]
    #[inline(always)]
    pub fn rreq(&self) -> RREQ_R {
        RREQ_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Read Request"]
    #[inline(always)]
    pub fn rreq(&mut self) -> RREQ_W {
        RREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLLULP Read Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllulprreq](index.html) module"]
pub struct DFLLULPRREQ_SPEC;
impl crate::RegisterSpec for DFLLULPRREQ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dfllulprreq::R](R) reader structure"]
impl crate::Readable for DFLLULPRREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllulprreq::W](W) writer structure"]
impl crate::Writable for DFLLULPRREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFLLULPRREQ to value 0"]
impl crate::Resettable for DFLLULPRREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
