#[doc = "Register `ALARM` reader"]
pub struct R(crate::R<ALARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARM` writer"]
pub struct W(crate::W<ALARM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM_SPEC>;
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
impl From<crate::W<ALARM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECOND` reader - Second"]
pub type SECOND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECOND` writer - Second"]
pub type SECOND_W<'a> = crate::FieldWriter<'a, u32, ALARM_SPEC, u8, u8, 6, 0>;
#[doc = "Field `MINUTE` reader - Minute"]
pub type MINUTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINUTE` writer - Minute"]
pub type MINUTE_W<'a> = crate::FieldWriter<'a, u32, ALARM_SPEC, u8, u8, 6, 6>;
#[doc = "Hour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HOUR_A {
    #[doc = "0: Morning hour"]
    AM = 0,
    #[doc = "16: Afternoon hour"]
    PM = 16,
}
impl From<HOUR_A> for u8 {
    #[inline(always)]
    fn from(variant: HOUR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HOUR` reader - Hour"]
pub type HOUR_R = crate::FieldReader<u8, HOUR_A>;
impl HOUR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HOUR_A> {
        match self.bits {
            0 => Some(HOUR_A::AM),
            16 => Some(HOUR_A::PM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AM`"]
    #[inline(always)]
    pub fn is_am(&self) -> bool {
        *self == HOUR_A::AM
    }
    #[doc = "Checks if the value of the field is `PM`"]
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        *self == HOUR_A::PM
    }
}
#[doc = "Field `HOUR` writer - Hour"]
pub type HOUR_W<'a> =
    crate::FieldWriter<'a, u32, ALARM_SPEC, u8, HOUR_A, 5, 12>;
impl<'a> HOUR_W<'a> {
    #[doc = "Morning hour"]
    #[inline(always)]
    pub fn am(self) -> &'a mut W {
        self.variant(HOUR_A::AM)
    }
    #[doc = "Afternoon hour"]
    #[inline(always)]
    pub fn pm(self) -> &'a mut W {
        self.variant(HOUR_A::PM)
    }
}
#[doc = "Field `DAY` reader - Day"]
pub type DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAY` writer - Day"]
pub type DAY_W<'a> = crate::FieldWriter<'a, u32, ALARM_SPEC, u8, u8, 5, 17>;
#[doc = "Field `MONTH` reader - Month"]
pub type MONTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MONTH` writer - Month"]
pub type MONTH_W<'a> = crate::FieldWriter<'a, u32, ALARM_SPEC, u8, u8, 4, 22>;
#[doc = "Field `YEAR` reader - Year"]
pub type YEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEAR` writer - Year"]
pub type YEAR_W<'a> = crate::FieldWriter<'a, u32, ALARM_SPEC, u8, u8, 6, 26>;
impl R {
    #[doc = "Bits 0:5 - Second"]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline(always)]
    pub fn minute(&self) -> MINUTE_R {
        MINUTE_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:16 - Hour"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Second"]
    #[inline(always)]
    pub fn second(&mut self) -> SECOND_W {
        SECOND_W::new(self)
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline(always)]
    pub fn minute(&mut self) -> MINUTE_W {
        MINUTE_W::new(self)
    }
    #[doc = "Bits 12:16 - Hour"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W {
        HOUR_W::new(self)
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W {
        DAY_W::new(self)
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline(always)]
    pub fn month(&mut self) -> MONTH_W {
        MONTH_W::new(self)
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE2_ALARM Alarm n Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm](index.html) module"]
pub struct ALARM_SPEC;
impl crate::RegisterSpec for ALARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm::R](R) reader structure"]
impl crate::Readable for ALARM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm::W](W) writer structure"]
impl crate::Writable for ALARM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALARM to value 0"]
impl crate::Resettable for ALARM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
