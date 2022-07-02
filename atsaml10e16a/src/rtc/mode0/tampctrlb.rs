#[doc = "Register `TAMPCTRLB` reader"]
pub struct R(crate::R<TAMPCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMPCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMPCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMPCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMPCTRLB` writer"]
pub struct W(crate::W<TAMPCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMPCTRLB_SPEC>;
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
impl From<crate::W<TAMPCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMPCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALSI0` reader - Active Layer Select Internal 0"]
pub type ALSI0_R = crate::BitReader<bool>;
#[doc = "Field `ALSI0` writer - Active Layer Select Internal 0"]
pub type ALSI0_W<'a> = crate::BitWriter<'a, u32, TAMPCTRLB_SPEC, bool, 0>;
#[doc = "Field `ALSI1` reader - Active Layer Select Internal 1"]
pub type ALSI1_R = crate::BitReader<bool>;
#[doc = "Field `ALSI1` writer - Active Layer Select Internal 1"]
pub type ALSI1_W<'a> = crate::BitWriter<'a, u32, TAMPCTRLB_SPEC, bool, 1>;
#[doc = "Field `ALSI2` reader - Active Layer Select Internal 2"]
pub type ALSI2_R = crate::BitReader<bool>;
#[doc = "Field `ALSI2` writer - Active Layer Select Internal 2"]
pub type ALSI2_W<'a> = crate::BitWriter<'a, u32, TAMPCTRLB_SPEC, bool, 2>;
#[doc = "Field `ALSI3` reader - Active Layer Select Internal 3"]
pub type ALSI3_R = crate::BitReader<bool>;
#[doc = "Field `ALSI3` writer - Active Layer Select Internal 3"]
pub type ALSI3_W<'a> = crate::BitWriter<'a, u32, TAMPCTRLB_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Active Layer Select Internal 0"]
    #[inline(always)]
    pub fn alsi0(&self) -> ALSI0_R {
        ALSI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active Layer Select Internal 1"]
    #[inline(always)]
    pub fn alsi1(&self) -> ALSI1_R {
        ALSI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active Layer Select Internal 2"]
    #[inline(always)]
    pub fn alsi2(&self) -> ALSI2_R {
        ALSI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Active Layer Select Internal 3"]
    #[inline(always)]
    pub fn alsi3(&self) -> ALSI3_R {
        ALSI3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active Layer Select Internal 0"]
    #[inline(always)]
    pub fn alsi0(&mut self) -> ALSI0_W {
        ALSI0_W::new(self)
    }
    #[doc = "Bit 1 - Active Layer Select Internal 1"]
    #[inline(always)]
    pub fn alsi1(&mut self) -> ALSI1_W {
        ALSI1_W::new(self)
    }
    #[doc = "Bit 2 - Active Layer Select Internal 2"]
    #[inline(always)]
    pub fn alsi2(&mut self) -> ALSI2_W {
        ALSI2_W::new(self)
    }
    #[doc = "Bit 3 - Active Layer Select Internal 3"]
    #[inline(always)]
    pub fn alsi3(&mut self) -> ALSI3_W {
        ALSI3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tampctrlb](index.html) module"]
pub struct TAMPCTRLB_SPEC;
impl crate::RegisterSpec for TAMPCTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tampctrlb::R](R) reader structure"]
impl crate::Readable for TAMPCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tampctrlb::W](W) writer structure"]
impl crate::Writable for TAMPCTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMPCTRLB to value 0"]
impl crate::Resettable for TAMPCTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
