#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - NVM Done Interrupt Enable"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - NVM Done Interrupt Enable"]
pub type DONE_W<'a> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, 0>;
#[doc = "Field `PROGE` reader - Programming Error Status Interrupt Enable"]
pub type PROGE_R = crate::BitReader<bool>;
#[doc = "Field `PROGE` writer - Programming Error Status Interrupt Enable"]
pub type PROGE_W<'a> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, 1>;
#[doc = "Field `LOCKE` reader - Lock Error Status Interrupt Enable"]
pub type LOCKE_R = crate::BitReader<bool>;
#[doc = "Field `LOCKE` writer - Lock Error Status Interrupt Enable"]
pub type LOCKE_W<'a> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, 2>;
#[doc = "Field `NVME` reader - NVM Error Interrupt Enable"]
pub type NVME_R = crate::BitReader<bool>;
#[doc = "Field `NVME` writer - NVM Error Interrupt Enable"]
pub type NVME_W<'a> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, 3>;
#[doc = "Field `KEYE` reader - Key Write Error Interrupt Enable"]
pub type KEYE_R = crate::BitReader<bool>;
#[doc = "Field `KEYE` writer - Key Write Error Interrupt Enable"]
pub type KEYE_W<'a> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, 4>;
#[doc = "Field `NSCHK` reader - NS configuration change detected Interrupt Enable"]
pub type NSCHK_R = crate::BitReader<bool>;
#[doc = "Field `NSCHK` writer - NS configuration change detected Interrupt Enable"]
pub type NSCHK_W<'a> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - NVM Done Interrupt Enable"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Programming Error Status Interrupt Enable"]
    #[inline(always)]
    pub fn proge(&self) -> PROGE_R {
        PROGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Error Status Interrupt Enable"]
    #[inline(always)]
    pub fn locke(&self) -> LOCKE_R {
        LOCKE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NVM Error Interrupt Enable"]
    #[inline(always)]
    pub fn nvme(&self) -> NVME_R {
        NVME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Key Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn keye(&self) -> KEYE_R {
        KEYE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NS configuration change detected Interrupt Enable"]
    #[inline(always)]
    pub fn nschk(&self) -> NSCHK_R {
        NSCHK_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVM Done Interrupt Enable"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - Programming Error Status Interrupt Enable"]
    #[inline(always)]
    pub fn proge(&mut self) -> PROGE_W {
        PROGE_W::new(self)
    }
    #[doc = "Bit 2 - Lock Error Status Interrupt Enable"]
    #[inline(always)]
    pub fn locke(&mut self) -> LOCKE_W {
        LOCKE_W::new(self)
    }
    #[doc = "Bit 3 - NVM Error Interrupt Enable"]
    #[inline(always)]
    pub fn nvme(&mut self) -> NVME_W {
        NVME_W::new(self)
    }
    #[doc = "Bit 4 - Key Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn keye(&mut self) -> KEYE_W {
        KEYE_W::new(self)
    }
    #[doc = "Bit 5 - NS configuration change detected Interrupt Enable"]
    #[inline(always)]
    pub fn nschk(&mut self) -> NSCHK_W {
        NSCHK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
