#[doc = "Register `STATUSA` reader"]
pub struct R(crate::R<STATUSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUSA` writer"]
pub struct W(crate::W<STATUSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUSA_SPEC>;
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
impl From<crate::W<STATUSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - Done"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - Done"]
pub type DONE_W<'a> = crate::BitWriter<'a, u8, STATUSA_SPEC, bool, 0>;
#[doc = "Field `CRSTEXT` reader - CPU Reset Phase Extension"]
pub type CRSTEXT_R = crate::BitReader<bool>;
#[doc = "Field `CRSTEXT` writer - CPU Reset Phase Extension"]
pub type CRSTEXT_W<'a> = crate::BitWriter<'a, u8, STATUSA_SPEC, bool, 1>;
#[doc = "Field `BERR` reader - Bus Error"]
pub type BERR_R = crate::BitReader<bool>;
#[doc = "Field `BERR` writer - Bus Error"]
pub type BERR_W<'a> = crate::BitWriter<'a, u8, STATUSA_SPEC, bool, 2>;
#[doc = "Field `FAIL` reader - Failure"]
pub type FAIL_R = crate::BitReader<bool>;
#[doc = "Field `FAIL` writer - Failure"]
pub type FAIL_W<'a> = crate::BitWriter<'a, u8, STATUSA_SPEC, bool, 3>;
#[doc = "Field `PERR` reader - Protection Error Detected by the State Machine"]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `PERR` writer - Protection Error Detected by the State Machine"]
pub type PERR_W<'a> = crate::BitWriter<'a, u8, STATUSA_SPEC, bool, 4>;
#[doc = "Field `BREXT` reader - BootRom Phase Extension"]
pub type BREXT_R = crate::BitReader<bool>;
#[doc = "Field `BREXT` writer - BootRom Phase Extension"]
pub type BREXT_W<'a> = crate::BitWriter<'a, u8, STATUSA_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Reset Phase Extension"]
    #[inline(always)]
    pub fn crstext(&self) -> CRSTEXT_R {
        CRSTEXT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Failure"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protection Error Detected by the State Machine"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BootRom Phase Extension"]
    #[inline(always)]
    pub fn brext(&self) -> BREXT_R {
        BREXT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Done"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - CPU Reset Phase Extension"]
    #[inline(always)]
    pub fn crstext(&mut self) -> CRSTEXT_W {
        CRSTEXT_W::new(self)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W::new(self)
    }
    #[doc = "Bit 3 - Failure"]
    #[inline(always)]
    pub fn fail(&mut self) -> FAIL_W {
        FAIL_W::new(self)
    }
    #[doc = "Bit 4 - Protection Error Detected by the State Machine"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W::new(self)
    }
    #[doc = "Bit 5 - BootRom Phase Extension"]
    #[inline(always)]
    pub fn brext(&mut self) -> BREXT_W {
        BREXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusa](index.html) module"]
pub struct STATUSA_SPEC;
impl crate::RegisterSpec for STATUSA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [statusa::R](R) reader structure"]
impl crate::Readable for STATUSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [statusa::W](W) writer structure"]
impl crate::Writable for STATUSA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUSA to value 0"]
impl crate::Resettable for STATUSA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
