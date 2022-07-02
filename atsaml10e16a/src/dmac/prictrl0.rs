#[doc = "Register `PRICTRL0` reader"]
pub struct R(crate::R<PRICTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRICTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRICTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRICTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRICTRL0` writer"]
pub struct W(crate::W<PRICTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRICTRL0_SPEC>;
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
impl From<crate::W<PRICTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRICTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVLPRI0` reader - Level 0 Channel Priority Number"]
pub type LVLPRI0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVLPRI0` writer - Level 0 Channel Priority Number"]
pub type LVLPRI0_W<'a> =
    crate::FieldWriter<'a, u32, PRICTRL0_SPEC, u8, u8, 3, 0>;
#[doc = "Field `RRLVLEN0` reader - Level 0 Round-Robin Scheduling Enable"]
pub type RRLVLEN0_R = crate::BitReader<bool>;
#[doc = "Field `RRLVLEN0` writer - Level 0 Round-Robin Scheduling Enable"]
pub type RRLVLEN0_W<'a> = crate::BitWriter<'a, u32, PRICTRL0_SPEC, bool, 7>;
#[doc = "Field `LVLPRI1` reader - Level 1 Channel Priority Number"]
pub type LVLPRI1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVLPRI1` writer - Level 1 Channel Priority Number"]
pub type LVLPRI1_W<'a> =
    crate::FieldWriter<'a, u32, PRICTRL0_SPEC, u8, u8, 3, 8>;
#[doc = "Field `RRLVLEN1` reader - Level 1 Round-Robin Scheduling Enable"]
pub type RRLVLEN1_R = crate::BitReader<bool>;
#[doc = "Field `RRLVLEN1` writer - Level 1 Round-Robin Scheduling Enable"]
pub type RRLVLEN1_W<'a> = crate::BitWriter<'a, u32, PRICTRL0_SPEC, bool, 15>;
#[doc = "Field `LVLPRI2` reader - Level 2 Channel Priority Number"]
pub type LVLPRI2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVLPRI2` writer - Level 2 Channel Priority Number"]
pub type LVLPRI2_W<'a> =
    crate::FieldWriter<'a, u32, PRICTRL0_SPEC, u8, u8, 3, 16>;
#[doc = "Field `RRLVLEN2` reader - Level 2 Round-Robin Scheduling Enable"]
pub type RRLVLEN2_R = crate::BitReader<bool>;
#[doc = "Field `RRLVLEN2` writer - Level 2 Round-Robin Scheduling Enable"]
pub type RRLVLEN2_W<'a> = crate::BitWriter<'a, u32, PRICTRL0_SPEC, bool, 23>;
#[doc = "Field `LVLPRI3` reader - Level 3 Channel Priority Number"]
pub type LVLPRI3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVLPRI3` writer - Level 3 Channel Priority Number"]
pub type LVLPRI3_W<'a> =
    crate::FieldWriter<'a, u32, PRICTRL0_SPEC, u8, u8, 3, 24>;
#[doc = "Field `RRLVLEN3` reader - Level 3 Round-Robin Scheduling Enable"]
pub type RRLVLEN3_R = crate::BitReader<bool>;
#[doc = "Field `RRLVLEN3` writer - Level 3 Round-Robin Scheduling Enable"]
pub type RRLVLEN3_W<'a> = crate::BitWriter<'a, u32, PRICTRL0_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:2 - Level 0 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri0(&self) -> LVLPRI0_R {
        LVLPRI0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen0(&self) -> RRLVLEN0_R {
        RRLVLEN0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Level 1 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri1(&self) -> LVLPRI1_R {
        LVLPRI1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen1(&self) -> RRLVLEN1_R {
        RRLVLEN1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Level 2 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri2(&self) -> LVLPRI2_R {
        LVLPRI2_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen2(&self) -> RRLVLEN2_R {
        RRLVLEN2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Level 3 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri3(&self) -> LVLPRI3_R {
        LVLPRI3_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen3(&self) -> RRLVLEN3_R {
        RRLVLEN3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Level 0 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri0(&mut self) -> LVLPRI0_W {
        LVLPRI0_W::new(self)
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen0(&mut self) -> RRLVLEN0_W {
        RRLVLEN0_W::new(self)
    }
    #[doc = "Bits 8:10 - Level 1 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri1(&mut self) -> LVLPRI1_W {
        LVLPRI1_W::new(self)
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen1(&mut self) -> RRLVLEN1_W {
        RRLVLEN1_W::new(self)
    }
    #[doc = "Bits 16:18 - Level 2 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri2(&mut self) -> LVLPRI2_W {
        LVLPRI2_W::new(self)
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen2(&mut self) -> RRLVLEN2_W {
        RRLVLEN2_W::new(self)
    }
    #[doc = "Bits 24:26 - Level 3 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri3(&mut self) -> LVLPRI3_W {
        LVLPRI3_W::new(self)
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen3(&mut self) -> RRLVLEN3_W {
        RRLVLEN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prictrl0](index.html) module"]
pub struct PRICTRL0_SPEC;
impl crate::RegisterSpec for PRICTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prictrl0::R](R) reader structure"]
impl crate::Readable for PRICTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prictrl0::W](W) writer structure"]
impl crate::Writable for PRICTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRICTRL0 to value 0"]
impl crate::Resettable for PRICTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
