#[doc = "Register `DAUTHSTATUS` reader"]
pub struct R(crate::R<DAUTHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAUTHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAUTHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAUTHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SID_A {
    #[doc = "0: Security Extension not implemented"]
    NOSEC = 0,
    #[doc = "2: Secure invasive debug prohibited"]
    NO = 2,
    #[doc = "3: Secure invasive debug allowed"]
    YES = 3,
}
impl From<SID_A> for u8 {
    #[inline(always)]
    fn from(variant: SID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SID` reader - "]
pub type SID_R = crate::FieldReader<u8, SID_A>;
impl SID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SID_A> {
        match self.bits {
            0 => Some(SID_A::NOSEC),
            2 => Some(SID_A::NO),
            3 => Some(SID_A::YES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOSEC`"]
    #[inline(always)]
    pub fn is_nosec(&self) -> bool {
        *self == SID_A::NOSEC
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SID_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SID_A::YES
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SNID_A {
    #[doc = "0: Security Extension not implemented"]
    NOSEC = 0,
    #[doc = "2: Secure non-invasive debug prohibited"]
    NO = 2,
    #[doc = "3: Secure non-invasive debug allowed"]
    YES = 3,
}
impl From<SNID_A> for u8 {
    #[inline(always)]
    fn from(variant: SNID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SNID` reader - "]
pub type SNID_R = crate::FieldReader<u8, SNID_A>;
impl SNID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SNID_A> {
        match self.bits {
            0 => Some(SNID_A::NOSEC),
            2 => Some(SNID_A::NO),
            3 => Some(SNID_A::YES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOSEC`"]
    #[inline(always)]
    pub fn is_nosec(&self) -> bool {
        *self == SNID_A::NOSEC
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SNID_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SNID_A::YES
    }
}
impl R {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn snid(&self) -> SNID_R {
        SNID_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Debug Authentication Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dauthstatus](index.html) module"]
pub struct DAUTHSTATUS_SPEC;
impl crate::RegisterSpec for DAUTHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dauthstatus::R](R) reader structure"]
impl crate::Readable for DAUTHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAUTHSTATUS to value 0"]
impl crate::Resettable for DAUTHSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
