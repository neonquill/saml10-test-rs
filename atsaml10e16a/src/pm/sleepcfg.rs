#[doc = "Register `SLEEPCFG` reader"]
pub struct R(crate::R<SLEEPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEPCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLEEPCFG` writer"]
pub struct W(crate::W<SLEEPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLEEPCFG_SPEC>;
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
impl From<crate::W<SLEEPCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLEEPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sleep Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLEEPMODE_A {
    #[doc = "2: CPU, AHB, APB clocks are OFF"]
    IDLE = 2,
    #[doc = "4: All Clocks are OFF"]
    STANDBY = 4,
    #[doc = "6: All power domains are powered OFF"]
    OFF = 6,
}
impl From<SLEEPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SLEEPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SLEEPMODE` reader - Sleep Mode"]
pub type SLEEPMODE_R = crate::FieldReader<u8, SLEEPMODE_A>;
impl SLEEPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLEEPMODE_A> {
        match self.bits {
            2 => Some(SLEEPMODE_A::IDLE),
            4 => Some(SLEEPMODE_A::STANDBY),
            6 => Some(SLEEPMODE_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SLEEPMODE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == SLEEPMODE_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SLEEPMODE_A::OFF
    }
}
#[doc = "Field `SLEEPMODE` writer - Sleep Mode"]
pub type SLEEPMODE_W<'a> =
    crate::FieldWriter<'a, u8, SLEEPCFG_SPEC, u8, SLEEPMODE_A, 3, 0>;
impl<'a> SLEEPMODE_W<'a> {
    #[doc = "CPU, AHB, APB clocks are OFF"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::IDLE)
    }
    #[doc = "All Clocks are OFF"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::STANDBY)
    }
    #[doc = "All power domains are powered OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SLEEPMODE_A::OFF)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline(always)]
    pub fn sleepmode(&self) -> SLEEPMODE_R {
        SLEEPMODE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sleep Mode"]
    #[inline(always)]
    pub fn sleepmode(&mut self) -> SLEEPMODE_W {
        SLEEPMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepcfg](index.html) module"]
pub struct SLEEPCFG_SPEC;
impl crate::RegisterSpec for SLEEPCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sleepcfg::R](R) reader structure"]
impl crate::Readable for SLEEPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sleepcfg::W](W) writer structure"]
impl crate::Writable for SLEEPCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLEEPCFG to value 0x02"]
impl crate::Resettable for SLEEPCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
