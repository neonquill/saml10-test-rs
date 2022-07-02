#[doc = "Register `DLAR` writer"]
pub struct W(crate::W<DLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLAR_SPEC>;
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
impl From<crate::W<DLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Lock access control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum KEY_AW {
    #[doc = "3316436565: Unlock key value"]
    UNLOCK = 3316436565,
}
impl From<KEY_AW> for u32 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - Lock access control"]
pub type KEY_W<'a> = crate::FieldWriter<'a, u32, DLAR_SPEC, u32, KEY_AW, 32, 0>;
impl<'a> KEY_W<'a> {
    #[doc = "Unlock key value"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(KEY_AW::UNLOCK)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lock access control"]
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
#[doc = "SCS Software Lock Access Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlar](index.html) module"]
pub struct DLAR_SPEC;
impl crate::RegisterSpec for DLAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dlar::W](W) writer structure"]
impl crate::Writable for DLAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLAR to value 0"]
impl crate::Resettable for DLAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
