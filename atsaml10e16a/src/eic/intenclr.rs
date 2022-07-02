#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTINT` reader - External Interrupt Enable"]
pub type EXTINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTINT` writer - External Interrupt Enable"]
pub type EXTINT_W<'a> =
    crate::FieldWriter<'a, u32, INTENCLR_SPEC, u8, u8, 8, 0>;
#[doc = "Field `NSCHK` reader - Non-secure Check Interrupt Enable"]
pub type NSCHK_R = crate::BitReader<bool>;
#[doc = "Field `NSCHK` writer - Non-secure Check Interrupt Enable"]
pub type NSCHK_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:7 - External Interrupt Enable"]
    #[inline(always)]
    pub fn extint(&self) -> EXTINT_R {
        EXTINT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Non-secure Check Interrupt Enable"]
    #[inline(always)]
    pub fn nschk(&self) -> NSCHK_R {
        NSCHK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - External Interrupt Enable"]
    #[inline(always)]
    pub fn extint(&mut self) -> EXTINT_W {
        EXTINT_W::new(self)
    }
    #[doc = "Bit 31 - Non-secure Check Interrupt Enable"]
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
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
