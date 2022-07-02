#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOEN` reader - External Output Enable"]
pub type EOEN_R = crate::BitReader<bool>;
#[doc = "Field `EOEN` writer - External Output Enable"]
pub type EOEN_W<'a> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, 0>;
#[doc = "Field `IOEN` reader - Internal Output Enable"]
pub type IOEN_R = crate::BitReader<bool>;
#[doc = "Field `IOEN` writer - Internal Output Enable"]
pub type IOEN_W<'a> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, 1>;
#[doc = "Field `LEFTADJ` reader - Left Adjusted Data"]
pub type LEFTADJ_R = crate::BitReader<bool>;
#[doc = "Field `LEFTADJ` writer - Left Adjusted Data"]
pub type LEFTADJ_W<'a> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, 2>;
#[doc = "Field `VPD` reader - Voltage Pump Disable"]
pub type VPD_R = crate::BitReader<bool>;
#[doc = "Field `VPD` writer - Voltage Pump Disable"]
pub type VPD_W<'a> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, 3>;
#[doc = "Field `DITHER` reader - Dither Enable"]
pub type DITHER_R = crate::BitReader<bool>;
#[doc = "Field `DITHER` writer - Dither Enable"]
pub type DITHER_W<'a> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, 5>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal Voltage reference"]
    INTREF = 0,
    #[doc = "1: Analog Voltage Supply"]
    VDDANA = 1,
    #[doc = "2: External reference"]
    VREFA = 2,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::INTREF),
            1 => Some(REFSEL_A::VDDANA),
            2 => Some(REFSEL_A::VREFA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INTREF`"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == REFSEL_A::INTREF
    }
    #[doc = "Checks if the value of the field is `VDDANA`"]
    #[inline(always)]
    pub fn is_vddana(&self) -> bool {
        *self == REFSEL_A::VDDANA
    }
    #[doc = "Checks if the value of the field is `VREFA`"]
    #[inline(always)]
    pub fn is_vrefa(&self) -> bool {
        *self == REFSEL_A::VREFA
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type REFSEL_W<'a> =
    crate::FieldWriter<'a, u8, CTRLB_SPEC, u8, REFSEL_A, 2, 6>;
impl<'a> REFSEL_W<'a> {
    #[doc = "Internal Voltage reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut W {
        self.variant(REFSEL_A::INTREF)
    }
    #[doc = "Analog Voltage Supply"]
    #[inline(always)]
    pub fn vddana(self) -> &'a mut W {
        self.variant(REFSEL_A::VDDANA)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn vrefa(self) -> &'a mut W {
        self.variant(REFSEL_A::VREFA)
    }
}
impl R {
    #[doc = "Bit 0 - External Output Enable"]
    #[inline(always)]
    pub fn eoen(&self) -> EOEN_R {
        EOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Output Enable"]
    #[inline(always)]
    pub fn ioen(&self) -> IOEN_R {
        IOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Pump Disable"]
    #[inline(always)]
    pub fn vpd(&self) -> VPD_R {
        VPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Dither Enable"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Output Enable"]
    #[inline(always)]
    pub fn eoen(&mut self) -> EOEN_W {
        EOEN_W::new(self)
    }
    #[doc = "Bit 1 - Internal Output Enable"]
    #[inline(always)]
    pub fn ioen(&mut self) -> IOEN_W {
        IOEN_W::new(self)
    }
    #[doc = "Bit 2 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&mut self) -> LEFTADJ_W {
        LEFTADJ_W::new(self)
    }
    #[doc = "Bit 3 - Voltage Pump Disable"]
    #[inline(always)]
    pub fn vpd(&mut self) -> VPD_W {
        VPD_W::new(self)
    }
    #[doc = "Bit 5 - Dither Enable"]
    #[inline(always)]
    pub fn dither(&mut self) -> DITHER_W {
        DITHER_W::new(self)
    }
    #[doc = "Bits 6:7 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
