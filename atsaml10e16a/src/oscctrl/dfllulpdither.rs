#[doc = "Register `DFLLULPDITHER` reader"]
pub struct R(crate::R<DFLLULPDITHER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLULPDITHER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLULPDITHER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLULPDITHER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLULPDITHER` writer"]
pub struct W(crate::W<DFLLULPDITHER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLULPDITHER_SPEC>;
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
impl From<crate::W<DFLLULPDITHER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLULPDITHER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Dither Step\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STEP_A {
    #[doc = "0: Dither Step = 1"]
    STEP1 = 0,
    #[doc = "1: Dither Step = 2"]
    STEP2 = 1,
    #[doc = "2: Dither Step = 4"]
    STEP4 = 2,
    #[doc = "3: Dither Step = 8"]
    STEP8 = 3,
}
impl From<STEP_A> for u8 {
    #[inline(always)]
    fn from(variant: STEP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STEP` reader - Dither Step"]
pub type STEP_R = crate::FieldReader<u8, STEP_A>;
impl STEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STEP_A> {
        match self.bits {
            0 => Some(STEP_A::STEP1),
            1 => Some(STEP_A::STEP2),
            2 => Some(STEP_A::STEP4),
            3 => Some(STEP_A::STEP8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STEP1`"]
    #[inline(always)]
    pub fn is_step1(&self) -> bool {
        *self == STEP_A::STEP1
    }
    #[doc = "Checks if the value of the field is `STEP2`"]
    #[inline(always)]
    pub fn is_step2(&self) -> bool {
        *self == STEP_A::STEP2
    }
    #[doc = "Checks if the value of the field is `STEP4`"]
    #[inline(always)]
    pub fn is_step4(&self) -> bool {
        *self == STEP_A::STEP4
    }
    #[doc = "Checks if the value of the field is `STEP8`"]
    #[inline(always)]
    pub fn is_step8(&self) -> bool {
        *self == STEP_A::STEP8
    }
}
#[doc = "Field `STEP` writer - Dither Step"]
pub type STEP_W<'a> =
    crate::FieldWriter<'a, u8, DFLLULPDITHER_SPEC, u8, STEP_A, 3, 0>;
impl<'a> STEP_W<'a> {
    #[doc = "Dither Step = 1"]
    #[inline(always)]
    pub fn step1(self) -> &'a mut W {
        self.variant(STEP_A::STEP1)
    }
    #[doc = "Dither Step = 2"]
    #[inline(always)]
    pub fn step2(self) -> &'a mut W {
        self.variant(STEP_A::STEP2)
    }
    #[doc = "Dither Step = 4"]
    #[inline(always)]
    pub fn step4(self) -> &'a mut W {
        self.variant(STEP_A::STEP4)
    }
    #[doc = "Dither Step = 8"]
    #[inline(always)]
    pub fn step8(self) -> &'a mut W {
        self.variant(STEP_A::STEP8)
    }
}
#[doc = "Dither Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PER_A {
    #[doc = "0: Dither Over 1 Reference Clock Period"]
    PER1 = 0,
    #[doc = "1: Dither Over 2 Reference Clock Period"]
    PER2 = 1,
    #[doc = "2: Dither Over 4 Reference Clock Period"]
    PER4 = 2,
    #[doc = "3: Dither Over 8 Reference Clock Period"]
    PER8 = 3,
    #[doc = "4: Dither Over 16 Reference Clock Period"]
    PER16 = 4,
    #[doc = "5: Dither Over 32 Reference Clock Period"]
    PER32 = 5,
}
impl From<PER_A> for u8 {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PER` reader - Dither Period"]
pub type PER_R = crate::FieldReader<u8, PER_A>;
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PER_A> {
        match self.bits {
            0 => Some(PER_A::PER1),
            1 => Some(PER_A::PER2),
            2 => Some(PER_A::PER4),
            3 => Some(PER_A::PER8),
            4 => Some(PER_A::PER16),
            5 => Some(PER_A::PER32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PER1`"]
    #[inline(always)]
    pub fn is_per1(&self) -> bool {
        *self == PER_A::PER1
    }
    #[doc = "Checks if the value of the field is `PER2`"]
    #[inline(always)]
    pub fn is_per2(&self) -> bool {
        *self == PER_A::PER2
    }
    #[doc = "Checks if the value of the field is `PER4`"]
    #[inline(always)]
    pub fn is_per4(&self) -> bool {
        *self == PER_A::PER4
    }
    #[doc = "Checks if the value of the field is `PER8`"]
    #[inline(always)]
    pub fn is_per8(&self) -> bool {
        *self == PER_A::PER8
    }
    #[doc = "Checks if the value of the field is `PER16`"]
    #[inline(always)]
    pub fn is_per16(&self) -> bool {
        *self == PER_A::PER16
    }
    #[doc = "Checks if the value of the field is `PER32`"]
    #[inline(always)]
    pub fn is_per32(&self) -> bool {
        *self == PER_A::PER32
    }
}
#[doc = "Field `PER` writer - Dither Period"]
pub type PER_W<'a> =
    crate::FieldWriter<'a, u8, DFLLULPDITHER_SPEC, u8, PER_A, 3, 4>;
impl<'a> PER_W<'a> {
    #[doc = "Dither Over 1 Reference Clock Period"]
    #[inline(always)]
    pub fn per1(self) -> &'a mut W {
        self.variant(PER_A::PER1)
    }
    #[doc = "Dither Over 2 Reference Clock Period"]
    #[inline(always)]
    pub fn per2(self) -> &'a mut W {
        self.variant(PER_A::PER2)
    }
    #[doc = "Dither Over 4 Reference Clock Period"]
    #[inline(always)]
    pub fn per4(self) -> &'a mut W {
        self.variant(PER_A::PER4)
    }
    #[doc = "Dither Over 8 Reference Clock Period"]
    #[inline(always)]
    pub fn per8(self) -> &'a mut W {
        self.variant(PER_A::PER8)
    }
    #[doc = "Dither Over 16 Reference Clock Period"]
    #[inline(always)]
    pub fn per16(self) -> &'a mut W {
        self.variant(PER_A::PER16)
    }
    #[doc = "Dither Over 32 Reference Clock Period"]
    #[inline(always)]
    pub fn per32(self) -> &'a mut W {
        self.variant(PER_A::PER32)
    }
}
impl R {
    #[doc = "Bits 0:2 - Dither Step"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Dither Period"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Dither Step"]
    #[inline(always)]
    pub fn step(&mut self) -> STEP_W {
        STEP_W::new(self)
    }
    #[doc = "Bits 4:6 - Dither Period"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLLULP Dither Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllulpdither](index.html) module"]
pub struct DFLLULPDITHER_SPEC;
impl crate::RegisterSpec for DFLLULPDITHER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dfllulpdither::R](R) reader structure"]
impl crate::Readable for DFLLULPDITHER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllulpdither::W](W) writer structure"]
impl crate::Writable for DFLLULPDITHER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFLLULPDITHER to value 0"]
impl crate::Resettable for DFLLULPDITHER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
