#[doc = "Register `RAM_HALFWORD_MODE[%s]` reader"]
pub struct R(crate::R<RAM_HALFWORD_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_HALFWORD_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_HALFWORD_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_HALFWORD_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM_HALFWORD_MODE[%s]` writer"]
pub struct W(crate::W<RAM_HALFWORD_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_HALFWORD_MODE_SPEC>;
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
impl From<crate::W<RAM_HALFWORD_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_HALFWORD_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HWORD0` reader - Trust RAM Halfword Data"]
pub type HWORD0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HWORD0` writer - Trust RAM Halfword Data"]
pub type HWORD0_W<'a> =
    crate::FieldWriter<'a, u32, RAM_HALFWORD_MODE_SPEC, u16, u16, 16, 0>;
#[doc = "Field `HWORD1` reader - Trust RAM Halfword Data"]
pub type HWORD1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HWORD1` writer - Trust RAM Halfword Data"]
pub type HWORD1_W<'a> =
    crate::FieldWriter<'a, u32, RAM_HALFWORD_MODE_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:15 - Trust RAM Halfword Data"]
    #[inline(always)]
    pub fn hword0(&self) -> HWORD0_R {
        HWORD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Trust RAM Halfword Data"]
    #[inline(always)]
    pub fn hword1(&self) -> HWORD1_R {
        HWORD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Trust RAM Halfword Data"]
    #[inline(always)]
    pub fn hword0(&mut self) -> HWORD0_W {
        HWORD0_W::new(self)
    }
    #[doc = "Bits 16:31 - Trust RAM Halfword Data"]
    #[inline(always)]
    pub fn hword1(&mut self) -> HWORD1_W {
        HWORD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TrustRAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_halfword_mode](index.html) module"]
pub struct RAM_HALFWORD_MODE_SPEC;
impl crate::RegisterSpec for RAM_HALFWORD_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram_halfword_mode::R](R) reader structure"]
impl crate::Readable for RAM_HALFWORD_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram_halfword_mode::W](W) writer structure"]
impl crate::Writable for RAM_HALFWORD_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM_HALFWORD_MODE[%s]
to value 0"]
impl crate::Resettable for RAM_HALFWORD_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
