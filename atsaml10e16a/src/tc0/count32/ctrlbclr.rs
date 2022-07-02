#[doc = "Register `CTRLBCLR` reader"]
pub struct R(crate::R<CTRLBCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLBCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLBCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLBCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLBCLR` writer"]
pub struct W(crate::W<CTRLBCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLBCLR_SPEC>;
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
impl From<crate::W<CTRLBCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLBCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - Counter Direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Counter Direction"]
pub type DIR_W<'a> = crate::BitWriter<'a, u8, CTRLBCLR_SPEC, bool, 0>;
#[doc = "Field `LUPD` reader - Lock Update"]
pub type LUPD_R = crate::BitReader<bool>;
#[doc = "Field `LUPD` writer - Lock Update"]
pub type LUPD_W<'a> = crate::BitWriter<'a, u8, CTRLBCLR_SPEC, bool, 1>;
#[doc = "Field `ONESHOT` reader - One-Shot on Counter"]
pub type ONESHOT_R = crate::BitReader<bool>;
#[doc = "Field `ONESHOT` writer - One-Shot on Counter"]
pub type ONESHOT_W<'a> = crate::BitWriter<'a, u8, CTRLBCLR_SPEC, bool, 2>;
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a> = crate::FieldWriter<'a, u8, CTRLBCLR_SPEC, u8, u8, 3, 5>;
impl R {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&self) -> LUPD_R {
        LUPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - One-Shot on Counter"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W::new(self)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&mut self) -> LUPD_W {
        LUPD_W::new(self)
    }
    #[doc = "Bit 2 - One-Shot on Counter"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> ONESHOT_W {
        ONESHOT_W::new(self)
    }
    #[doc = "Bits 5:7 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlbclr](index.html) module"]
pub struct CTRLBCLR_SPEC;
impl crate::RegisterSpec for CTRLBCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlbclr::R](R) reader structure"]
impl crate::Readable for CTRLBCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlbclr::W](W) writer structure"]
impl crate::Writable for CTRLBCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLBCLR to value 0"]
impl crate::Resettable for CTRLBCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
