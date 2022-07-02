#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRM` reader - Power Reduction Mode"]
pub type PRM_R = crate::BitReader<bool>;
#[doc = "Field `LOAD` reader - NVM Page Buffer Active Loading"]
pub type LOAD_R = crate::BitReader<bool>;
#[doc = "Field `READY` reader - NVM Ready"]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Debug Access Level Fuse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DALFUSE_A {
    #[doc = "0: NO DEBUG"]
    DAL0 = 0,
    #[doc = "2: FULL DEBUG"]
    DAL2 = 2,
}
impl From<DALFUSE_A> for u8 {
    #[inline(always)]
    fn from(variant: DALFUSE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DALFUSE` reader - Debug Access Level Fuse"]
pub type DALFUSE_R = crate::FieldReader<u8, DALFUSE_A>;
impl DALFUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DALFUSE_A> {
        match self.bits {
            0 => Some(DALFUSE_A::DAL0),
            2 => Some(DALFUSE_A::DAL2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DAL0`"]
    #[inline(always)]
    pub fn is_dal0(&self) -> bool {
        *self == DALFUSE_A::DAL0
    }
    #[doc = "Checks if the value of the field is `DAL2`"]
    #[inline(always)]
    pub fn is_dal2(&self) -> bool {
        *self == DALFUSE_A::DAL2
    }
}
impl R {
    #[doc = "Bit 0 - Power Reduction Mode"]
    #[inline(always)]
    pub fn prm(&self) -> PRM_R {
        PRM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NVM Page Buffer Active Loading"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVM Ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Debug Access Level Fuse"]
    #[inline(always)]
    pub fn dalfuse(&self) -> DALFUSE_R {
        DALFUSE_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
