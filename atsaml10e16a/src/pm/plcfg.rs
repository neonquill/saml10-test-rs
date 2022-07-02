#[doc = "Register `PLCFG` reader"]
pub struct R(crate::R<PLCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLCFG` writer"]
pub struct W(crate::W<PLCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLCFG_SPEC>;
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
impl From<crate::W<PLCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Performance Level Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLSEL_A {
    #[doc = "0: Performance Level 0"]
    PL0 = 0,
    #[doc = "2: Performance Level 2"]
    PL2 = 2,
}
impl From<PLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLSEL` reader - Performance Level Select"]
pub type PLSEL_R = crate::FieldReader<u8, PLSEL_A>;
impl PLSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLSEL_A> {
        match self.bits {
            0 => Some(PLSEL_A::PL0),
            2 => Some(PLSEL_A::PL2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PL0`"]
    #[inline(always)]
    pub fn is_pl0(&self) -> bool {
        *self == PLSEL_A::PL0
    }
    #[doc = "Checks if the value of the field is `PL2`"]
    #[inline(always)]
    pub fn is_pl2(&self) -> bool {
        *self == PLSEL_A::PL2
    }
}
#[doc = "Field `PLSEL` writer - Performance Level Select"]
pub type PLSEL_W<'a> =
    crate::FieldWriter<'a, u8, PLCFG_SPEC, u8, PLSEL_A, 2, 0>;
impl<'a> PLSEL_W<'a> {
    #[doc = "Performance Level 0"]
    #[inline(always)]
    pub fn pl0(self) -> &'a mut W {
        self.variant(PLSEL_A::PL0)
    }
    #[doc = "Performance Level 2"]
    #[inline(always)]
    pub fn pl2(self) -> &'a mut W {
        self.variant(PLSEL_A::PL2)
    }
}
#[doc = "Field `PLDIS` reader - Performance Level Disable"]
pub type PLDIS_R = crate::BitReader<bool>;
#[doc = "Field `PLDIS` writer - Performance Level Disable"]
pub type PLDIS_W<'a> = crate::BitWriter<'a, u8, PLCFG_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:1 - Performance Level Select"]
    #[inline(always)]
    pub fn plsel(&self) -> PLSEL_R {
        PLSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - Performance Level Disable"]
    #[inline(always)]
    pub fn pldis(&self) -> PLDIS_R {
        PLDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Performance Level Select"]
    #[inline(always)]
    pub fn plsel(&mut self) -> PLSEL_W {
        PLSEL_W::new(self)
    }
    #[doc = "Bit 7 - Performance Level Disable"]
    #[inline(always)]
    pub fn pldis(&mut self) -> PLDIS_W {
        PLDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Performance Level Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plcfg](index.html) module"]
pub struct PLCFG_SPEC;
impl crate::RegisterSpec for PLCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [plcfg::R](R) reader structure"]
impl crate::Readable for PLCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plcfg::W](W) writer structure"]
impl crate::Writable for PLCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLCFG to value 0"]
impl crate::Resettable for PLCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
