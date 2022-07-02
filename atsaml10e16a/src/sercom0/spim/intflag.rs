#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
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
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRE` reader - Data Register Empty Interrupt"]
pub type DRE_R = crate::BitReader<bool>;
#[doc = "Field `DRE` writer - Data Register Empty Interrupt"]
pub type DRE_W<'a> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, 0>;
#[doc = "Field `TXC` reader - Transmit Complete Interrupt"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - Transmit Complete Interrupt"]
pub type TXC_W<'a> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, 1>;
#[doc = "Field `RXC` reader - Receive Complete Interrupt"]
pub type RXC_R = crate::BitReader<bool>;
#[doc = "Field `RXC` writer - Receive Complete Interrupt"]
pub type RXC_W<'a> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, 2>;
#[doc = "Field `SSL` reader - Slave Select Low Interrupt Flag"]
pub type SSL_R = crate::BitReader<bool>;
#[doc = "Field `SSL` writer - Slave Select Low Interrupt Flag"]
pub type SSL_W<'a> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, 3>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Combined Error Interrupt"]
pub type ERROR_W<'a> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Data Register Empty Interrupt"]
    #[inline(always)]
    pub fn dre(&self) -> DRE_R {
        DRE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Complete Interrupt"]
    #[inline(always)]
    pub fn rxc(&self) -> RXC_R {
        RXC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Select Low Interrupt Flag"]
    #[inline(always)]
    pub fn ssl(&self) -> SSL_R {
        SSL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Register Empty Interrupt"]
    #[inline(always)]
    pub fn dre(&mut self) -> DRE_W {
        DRE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W::new(self)
    }
    #[doc = "Bit 2 - Receive Complete Interrupt"]
    #[inline(always)]
    pub fn rxc(&mut self) -> RXC_W {
        RXC_W::new(self)
    }
    #[doc = "Bit 3 - Slave Select Low Interrupt Flag"]
    #[inline(always)]
    pub fn ssl(&mut self) -> SSL_W {
        SSL_W::new(self)
    }
    #[doc = "Bit 7 - Combined Error Interrupt"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIM Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
