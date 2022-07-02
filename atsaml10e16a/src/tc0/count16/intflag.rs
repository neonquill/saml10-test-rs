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
#[doc = "Field `OVF` reader - OVF Interrupt Flag"]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `OVF` writer - OVF Interrupt Flag"]
pub type OVF_W<'a> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, 0>;
#[doc = "Field `ERR` reader - ERR Interrupt Flag"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - ERR Interrupt Flag"]
pub type ERR_W<'a> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, 1>;
#[doc = "Field `MC0` reader - MC Interrupt Flag 0"]
pub type MC0_R = crate::BitReader<bool>;
#[doc = "Field `MC0` writer - MC Interrupt Flag 0"]
pub type MC0_W<'a> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, 4>;
#[doc = "Field `MC1` reader - MC Interrupt Flag 1"]
pub type MC1_R = crate::BitReader<bool>;
#[doc = "Field `MC1` writer - MC Interrupt Flag 1"]
pub type MC1_W<'a> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - OVF Interrupt Flag"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ERR Interrupt Flag"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - MC Interrupt Flag 0"]
    #[inline(always)]
    pub fn mc0(&self) -> MC0_R {
        MC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MC Interrupt Flag 1"]
    #[inline(always)]
    pub fn mc1(&self) -> MC1_R {
        MC1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OVF Interrupt Flag"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W {
        OVF_W::new(self)
    }
    #[doc = "Bit 1 - ERR Interrupt Flag"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W::new(self)
    }
    #[doc = "Bit 4 - MC Interrupt Flag 0"]
    #[inline(always)]
    pub fn mc0(&mut self) -> MC0_W {
        MC0_W::new(self)
    }
    #[doc = "Bit 5 - MC Interrupt Flag 1"]
    #[inline(always)]
    pub fn mc1(&mut self) -> MC1_W {
        MC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
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
