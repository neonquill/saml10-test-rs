#[doc = "Register `NONSEC` reader"]
pub struct R(crate::R<NONSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NONSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NONSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NONSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NONSEC` writer"]
pub struct W(crate::W<NONSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NONSEC_SPEC>;
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
impl From<crate::W<NONSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NONSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTINT` reader - External Interrupt Nonsecure Enable"]
pub type EXTINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTINT` writer - External Interrupt Nonsecure Enable"]
pub type EXTINT_W<'a> = crate::FieldWriter<'a, u32, NONSEC_SPEC, u8, u8, 8, 0>;
#[doc = "Field `NMI` reader - Non-Maskable Interrupt Nonsecure Enable"]
pub type NMI_R = crate::BitReader<bool>;
#[doc = "Field `NMI` writer - Non-Maskable Interrupt Nonsecure Enable"]
pub type NMI_W<'a> = crate::BitWriter<'a, u32, NONSEC_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:7 - External Interrupt Nonsecure Enable"]
    #[inline(always)]
    pub fn extint(&self) -> EXTINT_R {
        EXTINT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Non-Maskable Interrupt Nonsecure Enable"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - External Interrupt Nonsecure Enable"]
    #[inline(always)]
    pub fn extint(&mut self) -> EXTINT_W {
        EXTINT_W::new(self)
    }
    #[doc = "Bit 31 - Non-Maskable Interrupt Nonsecure Enable"]
    #[inline(always)]
    pub fn nmi(&mut self) -> NMI_W {
        NMI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-secure Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nonsec](index.html) module"]
pub struct NONSEC_SPEC;
impl crate::RegisterSpec for NONSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nonsec::R](R) reader structure"]
impl crate::Readable for NONSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nonsec::W](W) writer structure"]
impl crate::Writable for NONSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NONSEC to value 0"]
impl crate::Resettable for NONSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
