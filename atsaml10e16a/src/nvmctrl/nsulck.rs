#[doc = "Register `NSULCK` reader"]
pub struct R(crate::R<NSULCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSULCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSULCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSULCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSULCK` writer"]
pub struct W(crate::W<NSULCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSULCK_SPEC>;
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
impl From<crate::W<NSULCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSULCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNS` reader - Non-Secure Boot Region"]
pub type BNS_R = crate::BitReader<bool>;
#[doc = "Field `BNS` writer - Non-Secure Boot Region"]
pub type BNS_W<'a> = crate::BitWriter<'a, u16, NSULCK_SPEC, bool, 0>;
#[doc = "Field `ANS` reader - Non-Secure Application Region"]
pub type ANS_R = crate::BitReader<bool>;
#[doc = "Field `ANS` writer - Non-Secure Application Region"]
pub type ANS_W<'a> = crate::BitWriter<'a, u16, NSULCK_SPEC, bool, 1>;
#[doc = "Field `DNS` reader - Non-Secure Data Region"]
pub type DNS_R = crate::BitReader<bool>;
#[doc = "Field `DNS` writer - Non-Secure Data Region"]
pub type DNS_W<'a> = crate::BitWriter<'a, u16, NSULCK_SPEC, bool, 2>;
#[doc = "Write Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NSLKEY_A {
    #[doc = "165: Write Key"]
    KEY = 165,
}
impl From<NSLKEY_A> for u8 {
    #[inline(always)]
    fn from(variant: NSLKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NSLKEY` reader - Write Key"]
pub type NSLKEY_R = crate::FieldReader<u8, NSLKEY_A>;
impl NSLKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NSLKEY_A> {
        match self.bits {
            165 => Some(NSLKEY_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == NSLKEY_A::KEY
    }
}
#[doc = "Field `NSLKEY` writer - Write Key"]
pub type NSLKEY_W<'a> =
    crate::FieldWriter<'a, u16, NSULCK_SPEC, u8, NSLKEY_A, 8, 8>;
impl<'a> NSLKEY_W<'a> {
    #[doc = "Write Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(NSLKEY_A::KEY)
    }
}
impl R {
    #[doc = "Bit 0 - Non-Secure Boot Region"]
    #[inline(always)]
    pub fn bns(&self) -> BNS_R {
        BNS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-Secure Application Region"]
    #[inline(always)]
    pub fn ans(&self) -> ANS_R {
        ANS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-Secure Data Region"]
    #[inline(always)]
    pub fn dns(&self) -> DNS_R {
        DNS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Write Key"]
    #[inline(always)]
    pub fn nslkey(&self) -> NSLKEY_R {
        NSLKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Secure Boot Region"]
    #[inline(always)]
    pub fn bns(&mut self) -> BNS_W {
        BNS_W::new(self)
    }
    #[doc = "Bit 1 - Non-Secure Application Region"]
    #[inline(always)]
    pub fn ans(&mut self) -> ANS_W {
        ANS_W::new(self)
    }
    #[doc = "Bit 2 - Non-Secure Data Region"]
    #[inline(always)]
    pub fn dns(&mut self) -> DNS_W {
        DNS_W::new(self)
    }
    #[doc = "Bits 8:15 - Write Key"]
    #[inline(always)]
    pub fn nslkey(&mut self) -> NSLKEY_W {
        NSLKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Secure Unlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsulck](index.html) module"]
pub struct NSULCK_SPEC;
impl crate::RegisterSpec for NSULCK_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nsulck::R](R) reader structure"]
impl crate::Readable for NSULCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nsulck::W](W) writer structure"]
impl crate::Writable for NSULCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NSULCK to value 0"]
impl crate::Resettable for NSULCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
