#[doc = "Register `CHINTENSET` reader"]
pub struct R(crate::R<CHINTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHINTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHINTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHINTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHINTENSET` writer"]
pub struct W(crate::W<CHINTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHINTENSET_SPEC>;
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
impl From<crate::W<CHINTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHINTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TERR` reader - Channel Transfer Error Interrupt Enable"]
pub type TERR_R = crate::BitReader<bool>;
#[doc = "Field `TERR` writer - Channel Transfer Error Interrupt Enable"]
pub type TERR_W<'a> = crate::BitWriter<'a, u8, CHINTENSET_SPEC, bool, 0>;
#[doc = "Field `TCMPL` reader - Channel Transfer Complete Interrupt Enable"]
pub type TCMPL_R = crate::BitReader<bool>;
#[doc = "Field `TCMPL` writer - Channel Transfer Complete Interrupt Enable"]
pub type TCMPL_W<'a> = crate::BitWriter<'a, u8, CHINTENSET_SPEC, bool, 1>;
#[doc = "Field `SUSP` reader - Channel Suspend Interrupt Enable"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` writer - Channel Suspend Interrupt Enable"]
pub type SUSP_W<'a> = crate::BitWriter<'a, u8, CHINTENSET_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - Channel Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcmpl(&self) -> TCMPL_R {
        TCMPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terr(&mut self) -> TERR_W {
        TERR_W::new(self)
    }
    #[doc = "Bit 1 - Channel Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcmpl(&mut self) -> TCMPL_W {
        TCMPL_W::new(self)
    }
    #[doc = "Bit 2 - Channel Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chintenset](index.html) module"]
pub struct CHINTENSET_SPEC;
impl crate::RegisterSpec for CHINTENSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chintenset::R](R) reader structure"]
impl crate::Readable for CHINTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chintenset::W](W) writer structure"]
impl crate::Writable for CHINTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHINTENSET to value 0"]
impl crate::Resettable for CHINTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
