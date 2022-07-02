#[doc = "Register `EVCTRL` reader"]
pub struct R(crate::R<EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL` writer"]
pub struct W(crate::W<EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRL_SPEC>;
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
impl From<crate::W<EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTEI` reader - Start Conversion Event Input"]
pub type STARTEI_R = crate::BitReader<bool>;
#[doc = "Field `STARTEI` writer - Start Conversion Event Input"]
pub type STARTEI_W<'a> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, 0>;
#[doc = "Field `EMPTYEO` reader - Data Buffer Empty Event Output"]
pub type EMPTYEO_R = crate::BitReader<bool>;
#[doc = "Field `EMPTYEO` writer - Data Buffer Empty Event Output"]
pub type EMPTYEO_W<'a> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, 1>;
#[doc = "Field `INVEI` reader - Invert Event Input"]
pub type INVEI_R = crate::BitReader<bool>;
#[doc = "Field `INVEI` writer - Invert Event Input"]
pub type INVEI_W<'a> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - Start Conversion Event Input"]
    #[inline(always)]
    pub fn startei(&self) -> STARTEI_R {
        STARTEI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Buffer Empty Event Output"]
    #[inline(always)]
    pub fn emptyeo(&self) -> EMPTYEO_R {
        EMPTYEO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invert Event Input"]
    #[inline(always)]
    pub fn invei(&self) -> INVEI_R {
        INVEI_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Event Input"]
    #[inline(always)]
    pub fn startei(&mut self) -> STARTEI_W {
        STARTEI_W::new(self)
    }
    #[doc = "Bit 1 - Data Buffer Empty Event Output"]
    #[inline(always)]
    pub fn emptyeo(&mut self) -> EMPTYEO_W {
        EMPTYEO_W::new(self)
    }
    #[doc = "Bit 2 - Invert Event Input"]
    #[inline(always)]
    pub fn invei(&mut self) -> INVEI_W {
        INVEI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [evctrl::R](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrl::W](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
