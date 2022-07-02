#[doc = "Register `STDBYCFG` reader"]
pub struct R(crate::R<STDBYCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STDBYCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STDBYCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STDBYCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STDBYCFG` writer"]
pub struct W(crate::W<STDBYCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STDBYCFG_SPEC>;
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
impl From<crate::W<STDBYCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STDBYCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power Domain Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDCFG_A {
    #[doc = "0: PDSW power domain switching is handled by hardware."]
    DEFAULT = 0,
    #[doc = "1: PDSW is forced ACTIVE."]
    PDSW = 1,
}
impl From<PDCFG_A> for bool {
    #[inline(always)]
    fn from(variant: PDCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDCFG` reader - Power Domain Configuration"]
pub type PDCFG_R = crate::BitReader<PDCFG_A>;
impl PDCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDCFG_A {
        match self.bits {
            false => PDCFG_A::DEFAULT,
            true => PDCFG_A::PDSW,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == PDCFG_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `PDSW`"]
    #[inline(always)]
    pub fn is_pdsw(&self) -> bool {
        *self == PDCFG_A::PDSW
    }
}
#[doc = "Field `PDCFG` writer - Power Domain Configuration"]
pub type PDCFG_W<'a> = crate::BitWriter<'a, u16, STDBYCFG_SPEC, PDCFG_A, 0>;
impl<'a> PDCFG_W<'a> {
    #[doc = "PDSW power domain switching is handled by hardware."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(PDCFG_A::DEFAULT)
    }
    #[doc = "PDSW is forced ACTIVE."]
    #[inline(always)]
    pub fn pdsw(self) -> &'a mut W {
        self.variant(PDCFG_A::PDSW)
    }
}
#[doc = "Dynamic Power Gating for PDSW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPGPDSW_A {
    #[doc = "0: Dynamic Power Gating disabled"]
    _0 = 0,
    #[doc = "1: Dynamic Power Gating enabled"]
    _1 = 1,
}
impl From<DPGPDSW_A> for bool {
    #[inline(always)]
    fn from(variant: DPGPDSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPGPDSW` reader - Dynamic Power Gating for PDSW"]
pub type DPGPDSW_R = crate::BitReader<DPGPDSW_A>;
impl DPGPDSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPGPDSW_A {
        match self.bits {
            false => DPGPDSW_A::_0,
            true => DPGPDSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPGPDSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPGPDSW_A::_1
    }
}
#[doc = "Field `DPGPDSW` writer - Dynamic Power Gating for PDSW"]
pub type DPGPDSW_W<'a> = crate::BitWriter<'a, u16, STDBYCFG_SPEC, DPGPDSW_A, 4>;
impl<'a> DPGPDSW_W<'a> {
    #[doc = "Dynamic Power Gating disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPGPDSW_A::_0)
    }
    #[doc = "Dynamic Power Gating enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPGPDSW_A::_1)
    }
}
#[doc = "Voltage Regulator Standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VREGSMOD_A {
    #[doc = "0: Automatic mode"]
    AUTO = 0,
    #[doc = "1: Performance oriented"]
    PERFORMANCE = 1,
    #[doc = "2: Low Power oriented"]
    LP = 2,
}
impl From<VREGSMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: VREGSMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VREGSMOD` reader - Voltage Regulator Standby mode"]
pub type VREGSMOD_R = crate::FieldReader<u8, VREGSMOD_A>;
impl VREGSMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VREGSMOD_A> {
        match self.bits {
            0 => Some(VREGSMOD_A::AUTO),
            1 => Some(VREGSMOD_A::PERFORMANCE),
            2 => Some(VREGSMOD_A::LP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == VREGSMOD_A::AUTO
    }
    #[doc = "Checks if the value of the field is `PERFORMANCE`"]
    #[inline(always)]
    pub fn is_performance(&self) -> bool {
        *self == VREGSMOD_A::PERFORMANCE
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == VREGSMOD_A::LP
    }
}
#[doc = "Field `VREGSMOD` writer - Voltage Regulator Standby mode"]
pub type VREGSMOD_W<'a> =
    crate::FieldWriter<'a, u16, STDBYCFG_SPEC, u8, VREGSMOD_A, 2, 6>;
impl<'a> VREGSMOD_W<'a> {
    #[doc = "Automatic mode"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(VREGSMOD_A::AUTO)
    }
    #[doc = "Performance oriented"]
    #[inline(always)]
    pub fn performance(self) -> &'a mut W {
        self.variant(VREGSMOD_A::PERFORMANCE)
    }
    #[doc = "Low Power oriented"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(VREGSMOD_A::LP)
    }
}
#[doc = "Field `BBIASHS` reader - Back Bias for HSRAM"]
pub type BBIASHS_R = crate::BitReader<bool>;
#[doc = "Field `BBIASHS` writer - Back Bias for HSRAM"]
pub type BBIASHS_W<'a> = crate::BitWriter<'a, u16, STDBYCFG_SPEC, bool, 10>;
#[doc = "Field `BBIASTR` reader - Back Bias for Trust RAM"]
pub type BBIASTR_R = crate::BitReader<bool>;
#[doc = "Field `BBIASTR` writer - Back Bias for Trust RAM"]
pub type BBIASTR_W<'a> = crate::BitWriter<'a, u16, STDBYCFG_SPEC, bool, 12>;
impl R {
    #[doc = "Bit 0 - Power Domain Configuration"]
    #[inline(always)]
    pub fn pdcfg(&self) -> PDCFG_R {
        PDCFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Dynamic Power Gating for PDSW"]
    #[inline(always)]
    pub fn dpgpdsw(&self) -> DPGPDSW_R {
        DPGPDSW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline(always)]
    pub fn vregsmod(&self) -> VREGSMOD_R {
        VREGSMOD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 10 - Back Bias for HSRAM"]
    #[inline(always)]
    pub fn bbiashs(&self) -> BBIASHS_R {
        BBIASHS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Back Bias for Trust RAM"]
    #[inline(always)]
    pub fn bbiastr(&self) -> BBIASTR_R {
        BBIASTR_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Domain Configuration"]
    #[inline(always)]
    pub fn pdcfg(&mut self) -> PDCFG_W {
        PDCFG_W::new(self)
    }
    #[doc = "Bit 4 - Dynamic Power Gating for PDSW"]
    #[inline(always)]
    pub fn dpgpdsw(&mut self) -> DPGPDSW_W {
        DPGPDSW_W::new(self)
    }
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline(always)]
    pub fn vregsmod(&mut self) -> VREGSMOD_W {
        VREGSMOD_W::new(self)
    }
    #[doc = "Bit 10 - Back Bias for HSRAM"]
    #[inline(always)]
    pub fn bbiashs(&mut self) -> BBIASHS_W {
        BBIASHS_W::new(self)
    }
    #[doc = "Bit 12 - Back Bias for Trust RAM"]
    #[inline(always)]
    pub fn bbiastr(&mut self) -> BBIASTR_W {
        BBIASTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standby Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stdbycfg](index.html) module"]
pub struct STDBYCFG_SPEC;
impl crate::RegisterSpec for STDBYCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [stdbycfg::R](R) reader structure"]
impl crate::Readable for STDBYCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stdbycfg::W](W) writer structure"]
impl crate::Writable for STDBYCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STDBYCFG to value 0"]
impl crate::Resettable for STDBYCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
