#[doc = "Register `DPRESCALER` reader"]
pub struct R(crate::R<DPRESCALER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPRESCALER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPRESCALER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPRESCALER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPRESCALER` writer"]
pub struct W(crate::W<DPRESCALER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPRESCALER_SPEC>;
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
impl From<crate::W<DPRESCALER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPRESCALER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Debouncer Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALER0_A {
    #[doc = "0: EIC clock divided by 2"]
    DIV2 = 0,
    #[doc = "1: EIC clock divided by 4"]
    DIV4 = 1,
    #[doc = "2: EIC clock divided by 8"]
    DIV8 = 2,
    #[doc = "3: EIC clock divided by 16"]
    DIV16 = 3,
    #[doc = "4: EIC clock divided by 32"]
    DIV32 = 4,
    #[doc = "5: EIC clock divided by 64"]
    DIV64 = 5,
    #[doc = "6: EIC clock divided by 128"]
    DIV128 = 6,
    #[doc = "7: EIC clock divided by 256"]
    DIV256 = 7,
}
impl From<PRESCALER0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESCALER0` reader - Debouncer Prescaler"]
pub type PRESCALER0_R = crate::FieldReader<u8, PRESCALER0_A>;
impl PRESCALER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALER0_A {
        match self.bits {
            0 => PRESCALER0_A::DIV2,
            1 => PRESCALER0_A::DIV4,
            2 => PRESCALER0_A::DIV8,
            3 => PRESCALER0_A::DIV16,
            4 => PRESCALER0_A::DIV32,
            5 => PRESCALER0_A::DIV64,
            6 => PRESCALER0_A::DIV128,
            7 => PRESCALER0_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER0_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER0_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER0_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER0_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER0_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER0_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER0_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER0_A::DIV256
    }
}
#[doc = "Field `PRESCALER0` writer - Debouncer Prescaler"]
pub type PRESCALER0_W<'a> =
    crate::FieldWriterSafe<'a, u32, DPRESCALER_SPEC, u8, PRESCALER0_A, 3, 0>;
impl<'a> PRESCALER0_W<'a> {
    #[doc = "EIC clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV2)
    }
    #[doc = "EIC clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV4)
    }
    #[doc = "EIC clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV8)
    }
    #[doc = "EIC clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV16)
    }
    #[doc = "EIC clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV32)
    }
    #[doc = "EIC clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV64)
    }
    #[doc = "EIC clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV128)
    }
    #[doc = "EIC clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV256)
    }
}
#[doc = "Field `STATES0` reader - Debouncer number of states"]
pub type STATES0_R = crate::BitReader<bool>;
#[doc = "Field `STATES0` writer - Debouncer number of states"]
pub type STATES0_W<'a> = crate::BitWriter<'a, u32, DPRESCALER_SPEC, bool, 3>;
#[doc = "Field `TICKON` reader - Pin Sampler frequency selection"]
pub type TICKON_R = crate::BitReader<bool>;
#[doc = "Field `TICKON` writer - Pin Sampler frequency selection"]
pub type TICKON_W<'a> = crate::BitWriter<'a, u32, DPRESCALER_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler0(&self) -> PRESCALER0_R {
        PRESCALER0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline(always)]
    pub fn states0(&self) -> STATES0_R {
        STATES0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline(always)]
    pub fn tickon(&self) -> TICKON_R {
        TICKON_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler0(&mut self) -> PRESCALER0_W {
        PRESCALER0_W::new(self)
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline(always)]
    pub fn states0(&mut self) -> STATES0_W {
        STATES0_W::new(self)
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline(always)]
    pub fn tickon(&mut self) -> TICKON_W {
        TICKON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debouncer Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dprescaler](index.html) module"]
pub struct DPRESCALER_SPEC;
impl crate::RegisterSpec for DPRESCALER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dprescaler::R](R) reader structure"]
impl crate::Readable for DPRESCALER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dprescaler::W](W) writer structure"]
impl crate::Writable for DPRESCALER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DPRESCALER to value 0"]
impl crate::Resettable for DPRESCALER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
