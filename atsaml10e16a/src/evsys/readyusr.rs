#[doc = "Register `READYUSR` reader"]
pub struct R(crate::R<READYUSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READYUSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READYUSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READYUSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READYUSR0` reader - Ready User for Channel 0"]
pub type READYUSR0_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR1` reader - Ready User for Channel 1"]
pub type READYUSR1_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR2` reader - Ready User for Channel 2"]
pub type READYUSR2_R = crate::BitReader<bool>;
#[doc = "Field `READYUSR3` reader - Ready User for Channel 3"]
pub type READYUSR3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Ready User for Channel 0"]
    #[inline(always)]
    pub fn readyusr0(&self) -> READYUSR0_R {
        READYUSR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ready User for Channel 1"]
    #[inline(always)]
    pub fn readyusr1(&self) -> READYUSR1_R {
        READYUSR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ready User for Channel 2"]
    #[inline(always)]
    pub fn readyusr2(&self) -> READYUSR2_R {
        READYUSR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ready User for Channel 3"]
    #[inline(always)]
    pub fn readyusr3(&self) -> READYUSR3_R {
        READYUSR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Ready Users\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readyusr](index.html) module"]
pub struct READYUSR_SPEC;
impl crate::RegisterSpec for READYUSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readyusr::R](R) reader structure"]
impl crate::Readable for READYUSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets READYUSR to value 0xffff_ffff"]
impl crate::Resettable for READYUSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
