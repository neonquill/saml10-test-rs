#[doc = "Register `SECCTRL` reader"]
pub struct R(crate::R<SECCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCTRL` writer"]
pub struct W(crate::W<SECCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCTRL_SPEC>;
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
impl From<crate::W<SECCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMPEEN` reader - Tamper Erase Enable"]
pub type TAMPEEN_R = crate::BitReader<bool>;
#[doc = "Field `TAMPEEN` writer - Tamper Erase Enable"]
pub type TAMPEEN_W<'a> = crate::BitWriter<'a, u32, SECCTRL_SPEC, bool, 0>;
#[doc = "Field `SILACC` reader - Silent Access"]
pub type SILACC_R = crate::BitReader<bool>;
#[doc = "Field `SILACC` writer - Silent Access"]
pub type SILACC_W<'a> = crate::BitWriter<'a, u32, SECCTRL_SPEC, bool, 2>;
#[doc = "Field `DSCEN` reader - Data Scramble Enable"]
pub type DSCEN_R = crate::BitReader<bool>;
#[doc = "Field `DSCEN` writer - Data Scramble Enable"]
pub type DSCEN_W<'a> = crate::BitWriter<'a, u32, SECCTRL_SPEC, bool, 3>;
#[doc = "Field `DXN` reader - Data Flash is eXecute Never"]
pub type DXN_R = crate::BitReader<bool>;
#[doc = "Field `DXN` writer - Data Flash is eXecute Never"]
pub type DXN_W<'a> = crate::BitWriter<'a, u32, SECCTRL_SPEC, bool, 6>;
#[doc = "Field `TEROW` reader - Tamper Rease Row"]
pub type TEROW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEROW` writer - Tamper Rease Row"]
pub type TEROW_W<'a> = crate::FieldWriter<'a, u32, SECCTRL_SPEC, u8, u8, 3, 8>;
#[doc = "Write Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "165: Write Key"]
    KEY = 165,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` reader - Write Key"]
pub type KEY_R = crate::FieldReader<u8, KEY_A>;
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            165 => Some(KEY_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == KEY_A::KEY
    }
}
#[doc = "Field `KEY` writer - Write Key"]
pub type KEY_W<'a> =
    crate::FieldWriter<'a, u32, SECCTRL_SPEC, u8, KEY_A, 8, 24>;
impl<'a> KEY_W<'a> {
    #[doc = "Write Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(KEY_A::KEY)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper Erase Enable"]
    #[inline(always)]
    pub fn tampeen(&self) -> TAMPEEN_R {
        TAMPEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Silent Access"]
    #[inline(always)]
    pub fn silacc(&self) -> SILACC_R {
        SILACC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Scramble Enable"]
    #[inline(always)]
    pub fn dscen(&self) -> DSCEN_R {
        DSCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Data Flash is eXecute Never"]
    #[inline(always)]
    pub fn dxn(&self) -> DXN_R {
        DXN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper Rease Row"]
    #[inline(always)]
    pub fn terow(&self) -> TEROW_R {
        TEROW_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 24:31 - Write Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper Erase Enable"]
    #[inline(always)]
    pub fn tampeen(&mut self) -> TAMPEEN_W {
        TAMPEEN_W::new(self)
    }
    #[doc = "Bit 2 - Silent Access"]
    #[inline(always)]
    pub fn silacc(&mut self) -> SILACC_W {
        SILACC_W::new(self)
    }
    #[doc = "Bit 3 - Data Scramble Enable"]
    #[inline(always)]
    pub fn dscen(&mut self) -> DSCEN_W {
        DSCEN_W::new(self)
    }
    #[doc = "Bit 6 - Data Flash is eXecute Never"]
    #[inline(always)]
    pub fn dxn(&mut self) -> DXN_W {
        DXN_W::new(self)
    }
    #[doc = "Bits 8:10 - Tamper Rease Row"]
    #[inline(always)]
    pub fn terow(&mut self) -> TEROW_W {
        TEROW_W::new(self)
    }
    #[doc = "Bits 24:31 - Write Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secctrl](index.html) module"]
pub struct SECCTRL_SPEC;
impl crate::RegisterSpec for SECCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secctrl::R](R) reader structure"]
impl crate::Readable for SECCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secctrl::W](W) writer structure"]
impl crate::Writable for SECCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECCTRL to value 0x30"]
impl crate::Resettable for SECCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
