#[doc = "Register `DPLLPRESC` reader"]
pub struct R(crate::R<DPLLPRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLLPRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLLPRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLLPRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPLLPRESC` writer"]
pub struct W(crate::W<DPLLPRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPLLPRESC_SPEC>;
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
impl From<crate::W<DPLLPRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPLLPRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Output Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: DPLL output is divided by 1"]
    DIV1 = 0,
    #[doc = "1: DPLL output is divided by 2"]
    DIV2 = 1,
    #[doc = "2: DPLL output is divided by 4"]
    DIV4 = 2,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESC` reader - Output Clock Prescaler"]
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::DIV1),
            1 => Some(PRESC_A::DIV2),
            2 => Some(PRESC_A::DIV4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::DIV4
    }
}
#[doc = "Field `PRESC` writer - Output Clock Prescaler"]
pub type PRESC_W<'a> =
    crate::FieldWriter<'a, u8, DPLLPRESC_SPEC, u8, PRESC_A, 2, 0>;
impl<'a> PRESC_W<'a> {
    #[doc = "DPLL output is divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESC_A::DIV1)
    }
    #[doc = "DPLL output is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::DIV2)
    }
    #[doc = "DPLL output is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::DIV4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Output Clock Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output Clock Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DPLL Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllpresc](index.html) module"]
pub struct DPLLPRESC_SPEC;
impl crate::RegisterSpec for DPLLPRESC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpllpresc::R](R) reader structure"]
impl crate::Readable for DPLLPRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpllpresc::W](W) writer structure"]
impl crate::Writable for DPLLPRESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DPLLPRESC to value 0"]
impl crate::Resettable for DPLLPRESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
