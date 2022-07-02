#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, 0>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, 1>;
#[doc = "Field `TAMPERS` reader - Tamper Erase"]
pub type TAMPERS_R = crate::BitReader<bool>;
#[doc = "Field `TAMPERS` writer - Tamper Erase"]
pub type TAMPERS_W<'a> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, 4>;
#[doc = "Field `DRP` reader - Data Remanence Prevention"]
pub type DRP_R = crate::BitReader<bool>;
#[doc = "Field `DRP` writer - Data Remanence Prevention"]
pub type DRP_W<'a> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, 6>;
#[doc = "Field `SILACC` reader - Silent Access"]
pub type SILACC_R = crate::BitReader<bool>;
#[doc = "Field `SILACC` writer - Silent Access"]
pub type SILACC_W<'a> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper Erase"]
    #[inline(always)]
    pub fn tampers(&self) -> TAMPERS_R {
        TAMPERS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Data Remanence Prevention"]
    #[inline(always)]
    pub fn drp(&self) -> DRP_R {
        DRP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Silent Access"]
    #[inline(always)]
    pub fn silacc(&self) -> SILACC_R {
        SILACC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Tamper Erase"]
    #[inline(always)]
    pub fn tampers(&mut self) -> TAMPERS_W {
        TAMPERS_W::new(self)
    }
    #[doc = "Bit 6 - Data Remanence Prevention"]
    #[inline(always)]
    pub fn drp(&mut self) -> DRP_W {
        DRP_W::new(self)
    }
    #[doc = "Bit 7 - Silent Access"]
    #[inline(always)]
    pub fn silacc(&mut self) -> SILACC_W {
        SILACC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
