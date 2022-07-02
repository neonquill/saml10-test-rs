#[doc = "Register `STATUSB` reader"]
pub struct R(crate::R<STATUSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Debugger Access Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAL_A {
    #[doc = "0: `0`"]
    SECURED = 0,
    #[doc = "2: `10`"]
    FULL_DEBUG = 2,
}
impl From<DAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DAL` reader - Debugger Access Level"]
pub type DAL_R = crate::FieldReader<u8, DAL_A>;
impl DAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAL_A> {
        match self.bits {
            0 => Some(DAL_A::SECURED),
            2 => Some(DAL_A::FULL_DEBUG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SECURED`"]
    #[inline(always)]
    pub fn is_secured(&self) -> bool {
        *self == DAL_A::SECURED
    }
    #[doc = "Checks if the value of the field is `FULL_DEBUG`"]
    #[inline(always)]
    pub fn is_full_debug(&self) -> bool {
        *self == DAL_A::FULL_DEBUG
    }
}
#[doc = "Field `DBGPRES` reader - Debugger Present"]
pub type DBGPRES_R = crate::BitReader<bool>;
#[doc = "Field `HPE` reader - Hot-Plugging Enable"]
pub type HPE_R = crate::BitReader<bool>;
#[doc = "Field `DCCD0` reader - Debug Communication Channel 0 Dirty"]
pub type DCCD0_R = crate::BitReader<bool>;
#[doc = "Field `DCCD1` reader - Debug Communication Channel 1 Dirty"]
pub type DCCD1_R = crate::BitReader<bool>;
#[doc = "Field `BCCD0` reader - Boot ROM Communication Channel 0 Dirty"]
pub type BCCD0_R = crate::BitReader<bool>;
#[doc = "Field `BCCD1` reader - Boot ROM Communication Channel 1 Dirty"]
pub type BCCD1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - Debugger Access Level"]
    #[inline(always)]
    pub fn dal(&self) -> DAL_R {
        DAL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Debugger Present"]
    #[inline(always)]
    pub fn dbgpres(&self) -> DBGPRES_R {
        DBGPRES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Hot-Plugging Enable"]
    #[inline(always)]
    pub fn hpe(&self) -> HPE_R {
        HPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Communication Channel 0 Dirty"]
    #[inline(always)]
    pub fn dccd0(&self) -> DCCD0_R {
        DCCD0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Debug Communication Channel 1 Dirty"]
    #[inline(always)]
    pub fn dccd1(&self) -> DCCD1_R {
        DCCD1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Boot ROM Communication Channel 0 Dirty"]
    #[inline(always)]
    pub fn bccd0(&self) -> BCCD0_R {
        BCCD0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Boot ROM Communication Channel 1 Dirty"]
    #[inline(always)]
    pub fn bccd1(&self) -> BCCD1_R {
        BCCD1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusb](index.html) module"]
pub struct STATUSB_SPEC;
impl crate::RegisterSpec for STATUSB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [statusb::R](R) reader structure"]
impl crate::Readable for STATUSB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSB to value 0"]
impl crate::Resettable for STATUSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
