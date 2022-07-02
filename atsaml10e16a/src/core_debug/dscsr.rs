#[doc = "Register `DSCSR` reader"]
pub struct R(crate::R<DSCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSCSR` writer"]
pub struct W(crate::W<DSCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSCSR_SPEC>;
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
impl From<crate::W<DSCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBRSELEN` reader - Secure Banked register select enable"]
pub type SBRSELEN_R = crate::BitReader<bool>;
#[doc = "Field `SBRSELEN` writer - Secure Banked register select enable"]
pub type SBRSELEN_W<'a> = crate::BitWriter<'a, u32, DSCSR_SPEC, bool, 0>;
#[doc = "Field `SBRSEL` reader - Secure Banked register select"]
pub type SBRSEL_R = crate::BitReader<bool>;
#[doc = "Field `SBRSEL` writer - Secure Banked register select"]
pub type SBRSEL_W<'a> = crate::BitWriter<'a, u32, DSCSR_SPEC, bool, 1>;
#[doc = "Field `CDS` reader - Current domain Secure"]
pub type CDS_R = crate::BitReader<bool>;
#[doc = "Field `CDS` writer - Current domain Secure"]
pub type CDS_W<'a> = crate::BitWriter<'a, u32, DSCSR_SPEC, bool, 16>;
#[doc = "Field `CDSKEY` reader - CDS field write-enable key"]
pub type CDSKEY_R = crate::BitReader<bool>;
#[doc = "Field `CDSKEY` writer - CDS field write-enable key"]
pub type CDSKEY_W<'a> = crate::BitWriter<'a, u32, DSCSR_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 0 - Secure Banked register select enable"]
    #[inline(always)]
    pub fn sbrselen(&self) -> SBRSELEN_R {
        SBRSELEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure Banked register select"]
    #[inline(always)]
    pub fn sbrsel(&self) -> SBRSEL_R {
        SBRSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Current domain Secure"]
    #[inline(always)]
    pub fn cds(&self) -> CDS_R {
        CDS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CDS field write-enable key"]
    #[inline(always)]
    pub fn cdskey(&self) -> CDSKEY_R {
        CDSKEY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Banked register select enable"]
    #[inline(always)]
    pub fn sbrselen(&mut self) -> SBRSELEN_W {
        SBRSELEN_W::new(self)
    }
    #[doc = "Bit 1 - Secure Banked register select"]
    #[inline(always)]
    pub fn sbrsel(&mut self) -> SBRSEL_W {
        SBRSEL_W::new(self)
    }
    #[doc = "Bit 16 - Current domain Secure"]
    #[inline(always)]
    pub fn cds(&mut self) -> CDS_W {
        CDS_W::new(self)
    }
    #[doc = "Bit 17 - CDS field write-enable key"]
    #[inline(always)]
    pub fn cdskey(&mut self) -> CDSKEY_W {
        CDSKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Security Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscsr](index.html) module"]
pub struct DSCSR_SPEC;
impl crate::RegisterSpec for DSCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dscsr::R](R) reader structure"]
impl crate::Readable for DSCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dscsr::W](W) writer structure"]
impl crate::Writable for DSCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSCSR to value 0"]
impl crate::Resettable for DSCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
