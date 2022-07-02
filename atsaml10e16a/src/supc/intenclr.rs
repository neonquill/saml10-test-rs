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
#[doc = "Field `BOD33RDY` reader - BOD33 Ready"]
pub type BOD33RDY_R = crate::BitReader<bool>;
#[doc = "Field `BOD33RDY` writer - BOD33 Ready"]
pub type BOD33RDY_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 0>;
#[doc = "Field `BOD33DET` reader - BOD33 Detection"]
pub type BOD33DET_R = crate::BitReader<bool>;
#[doc = "Field `BOD33DET` writer - BOD33 Detection"]
pub type BOD33DET_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 1>;
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready"]
pub type B33SRDY_R = crate::BitReader<bool>;
#[doc = "Field `B33SRDY` writer - BOD33 Synchronization Ready"]
pub type B33SRDY_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 2>;
#[doc = "Field `VREGRDY` reader - Voltage Regulator Ready"]
pub type VREGRDY_R = crate::BitReader<bool>;
#[doc = "Field `VREGRDY` writer - Voltage Regulator Ready"]
pub type VREGRDY_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 8>;
#[doc = "Field `VCORERDY` reader - VDDCORE Ready"]
pub type VCORERDY_R = crate::BitReader<bool>;
#[doc = "Field `VCORERDY` writer - VDDCORE Ready"]
pub type VCORERDY_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 10>;
#[doc = "Field `ULPVREFRDY` reader - ULPVREF Voltage Reference Ready"]
pub type ULPVREFRDY_R = crate::BitReader<bool>;
#[doc = "Field `ULPVREFRDY` writer - ULPVREF Voltage Reference Ready"]
pub type ULPVREFRDY_W<'a> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> BOD33RDY_R {
        BOD33RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33SRDY_R {
        B33SRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    pub fn vregrdy(&self) -> VREGRDY_R {
        VREGRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline(always)]
    pub fn vcorerdy(&self) -> VCORERDY_R {
        VCORERDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ULPVREF Voltage Reference Ready"]
    #[inline(always)]
    pub fn ulpvrefrdy(&self) -> ULPVREFRDY_R {
        ULPVREFRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&mut self) -> BOD33RDY_W {
        BOD33RDY_W::new(self)
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&mut self) -> BOD33DET_W {
        BOD33DET_W::new(self)
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&mut self) -> B33SRDY_W {
        B33SRDY_W::new(self)
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    pub fn vregrdy(&mut self) -> VREGRDY_W {
        VREGRDY_W::new(self)
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline(always)]
    pub fn vcorerdy(&mut self) -> VCORERDY_W {
        VCORERDY_W::new(self)
    }
    #[doc = "Bit 11 - ULPVREF Voltage Reference Ready"]
    #[inline(always)]
    pub fn ulpvrefrdy(&mut self) -> ULPVREFRDY_W {
        ULPVREFRDY_W::new(self)
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
