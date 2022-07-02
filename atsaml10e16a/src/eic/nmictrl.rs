#[doc = "Register `NMICTRL` reader"]
pub struct R(crate::R<NMICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMICTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMICTRL` writer"]
pub struct W(crate::W<NMICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMICTRL_SPEC>;
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
impl From<crate::W<NMICTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Non-Maskable Interrupt Sense Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NMISENSE_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising-edge detection"]
    RISE = 1,
    #[doc = "2: Falling-edge detection"]
    FALL = 2,
    #[doc = "3: Both-edges detection"]
    BOTH = 3,
    #[doc = "4: High-level detection"]
    HIGH = 4,
    #[doc = "5: Low-level detection"]
    LOW = 5,
}
impl From<NMISENSE_A> for u8 {
    #[inline(always)]
    fn from(variant: NMISENSE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NMISENSE` reader - Non-Maskable Interrupt Sense Configuration"]
pub type NMISENSE_R = crate::FieldReader<u8, NMISENSE_A>;
impl NMISENSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NMISENSE_A> {
        match self.bits {
            0 => Some(NMISENSE_A::NONE),
            1 => Some(NMISENSE_A::RISE),
            2 => Some(NMISENSE_A::FALL),
            3 => Some(NMISENSE_A::BOTH),
            4 => Some(NMISENSE_A::HIGH),
            5 => Some(NMISENSE_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NMISENSE_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == NMISENSE_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == NMISENSE_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == NMISENSE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == NMISENSE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == NMISENSE_A::LOW
    }
}
#[doc = "Field `NMISENSE` writer - Non-Maskable Interrupt Sense Configuration"]
pub type NMISENSE_W<'a> =
    crate::FieldWriter<'a, u8, NMICTRL_SPEC, u8, NMISENSE_A, 3, 0>;
impl<'a> NMISENSE_W<'a> {
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(NMISENSE_A::NONE)
    }
    #[doc = "Rising-edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(NMISENSE_A::RISE)
    }
    #[doc = "Falling-edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(NMISENSE_A::FALL)
    }
    #[doc = "Both-edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(NMISENSE_A::BOTH)
    }
    #[doc = "High-level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(NMISENSE_A::HIGH)
    }
    #[doc = "Low-level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(NMISENSE_A::LOW)
    }
}
#[doc = "Field `NMIFILTEN` reader - Non-Maskable Interrupt Filter Enable"]
pub type NMIFILTEN_R = crate::BitReader<bool>;
#[doc = "Field `NMIFILTEN` writer - Non-Maskable Interrupt Filter Enable"]
pub type NMIFILTEN_W<'a> = crate::BitWriter<'a, u8, NMICTRL_SPEC, bool, 3>;
#[doc = "Asynchronous Edge Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIASYNCH_A {
    #[doc = "0: Edge detection is clock synchronously operated"]
    SYNC = 0,
    #[doc = "1: Edge detection is clock asynchronously operated"]
    ASYNC = 1,
}
impl From<NMIASYNCH_A> for bool {
    #[inline(always)]
    fn from(variant: NMIASYNCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIASYNCH` reader - Asynchronous Edge Detection Mode"]
pub type NMIASYNCH_R = crate::BitReader<NMIASYNCH_A>;
impl NMIASYNCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIASYNCH_A {
        match self.bits {
            false => NMIASYNCH_A::SYNC,
            true => NMIASYNCH_A::ASYNC,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == NMIASYNCH_A::SYNC
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == NMIASYNCH_A::ASYNC
    }
}
#[doc = "Field `NMIASYNCH` writer - Asynchronous Edge Detection Mode"]
pub type NMIASYNCH_W<'a> =
    crate::BitWriter<'a, u8, NMICTRL_SPEC, NMIASYNCH_A, 4>;
impl<'a> NMIASYNCH_W<'a> {
    #[doc = "Edge detection is clock synchronously operated"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(NMIASYNCH_A::SYNC)
    }
    #[doc = "Edge detection is clock asynchronously operated"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(NMIASYNCH_A::ASYNC)
    }
}
impl R {
    #[doc = "Bits 0:2 - Non-Maskable Interrupt Sense Configuration"]
    #[inline(always)]
    pub fn nmisense(&self) -> NMISENSE_R {
        NMISENSE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Non-Maskable Interrupt Filter Enable"]
    #[inline(always)]
    pub fn nmifilten(&self) -> NMIFILTEN_R {
        NMIFILTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    pub fn nmiasynch(&self) -> NMIASYNCH_R {
        NMIASYNCH_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Non-Maskable Interrupt Sense Configuration"]
    #[inline(always)]
    pub fn nmisense(&mut self) -> NMISENSE_W {
        NMISENSE_W::new(self)
    }
    #[doc = "Bit 3 - Non-Maskable Interrupt Filter Enable"]
    #[inline(always)]
    pub fn nmifilten(&mut self) -> NMIFILTEN_W {
        NMIFILTEN_W::new(self)
    }
    #[doc = "Bit 4 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    pub fn nmiasynch(&mut self) -> NMIASYNCH_W {
        NMIASYNCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Maskable Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmictrl](index.html) module"]
pub struct NMICTRL_SPEC;
impl crate::RegisterSpec for NMICTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nmictrl::R](R) reader structure"]
impl crate::Readable for NMICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmictrl::W](W) writer structure"]
impl crate::Writable for NMICTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMICTRL to value 0"]
impl crate::Resettable for NMICTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
