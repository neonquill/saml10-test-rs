#[doc = "Register `WRCTRL` reader"]
pub struct R(crate::R<WRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRCTRL` writer"]
pub struct W(crate::W<WRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRCTRL_SPEC>;
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
impl From<crate::W<WRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERID` reader - Peripheral identifier"]
pub type PERID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERID` writer - Peripheral identifier"]
pub type PERID_W<'a> =
    crate::FieldWriter<'a, u32, WRCTRL_SPEC, u16, u16, 16, 0>;
#[doc = "Peripheral access control key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "0: No action"]
    OFF = 0,
    #[doc = "1: Clear protection"]
    CLEAR = 1,
    #[doc = "2: Set protection"]
    SET = 2,
    #[doc = "3: Set and lock protection"]
    LOCK = 3,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` reader - Peripheral access control key"]
pub type KEY_R = crate::FieldReader<u8, KEY_A>;
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            0 => Some(KEY_A::OFF),
            1 => Some(KEY_A::CLEAR),
            2 => Some(KEY_A::SET),
            3 => Some(KEY_A::LOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == KEY_A::OFF
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == KEY_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == KEY_A::SET
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == KEY_A::LOCK
    }
}
#[doc = "Field `KEY` writer - Peripheral access control key"]
pub type KEY_W<'a> = crate::FieldWriter<'a, u32, WRCTRL_SPEC, u8, KEY_A, 8, 16>;
impl<'a> KEY_W<'a> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(KEY_A::OFF)
    }
    #[doc = "Clear protection"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(KEY_A::CLEAR)
    }
    #[doc = "Set protection"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(KEY_A::SET)
    }
    #[doc = "Set and lock protection"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(KEY_A::LOCK)
    }
}
impl R {
    #[doc = "Bits 0:15 - Peripheral identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PERID_R {
        PERID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Peripheral access control key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Peripheral identifier"]
    #[inline(always)]
    pub fn perid(&mut self) -> PERID_W {
        PERID_W::new(self)
    }
    #[doc = "Bits 16:23 - Peripheral access control key"]
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
#[doc = "Write control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrctrl](index.html) module"]
pub struct WRCTRL_SPEC;
impl crate::RegisterSpec for WRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrctrl::R](R) reader structure"]
impl crate::Readable for WRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrctrl::W](W) writer structure"]
impl crate::Writable for WRCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRCTRL to value 0"]
impl crate::Resettable for WRCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
