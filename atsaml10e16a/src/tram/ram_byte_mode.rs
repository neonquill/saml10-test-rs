#[doc = "Register `RAM_BYTE_MODE[%s]` reader"]
pub struct R(crate::R<RAM_BYTE_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_BYTE_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_BYTE_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_BYTE_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM_BYTE_MODE[%s]` writer"]
pub struct W(crate::W<RAM_BYTE_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_BYTE_MODE_SPEC>;
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
impl From<crate::W<RAM_BYTE_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_BYTE_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE0` reader - Trust RAM Data"]
pub type BYTE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTE0` writer - Trust RAM Data"]
pub type BYTE0_W<'a> =
    crate::FieldWriter<'a, u32, RAM_BYTE_MODE_SPEC, u8, u8, 8, 0>;
#[doc = "Field `BYTE1` reader - Trust RAM Data"]
pub type BYTE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTE1` writer - Trust RAM Data"]
pub type BYTE1_W<'a> =
    crate::FieldWriter<'a, u32, RAM_BYTE_MODE_SPEC, u8, u8, 8, 8>;
#[doc = "Field `BYTE2` reader - Trust RAM Data"]
pub type BYTE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTE2` writer - Trust RAM Data"]
pub type BYTE2_W<'a> =
    crate::FieldWriter<'a, u32, RAM_BYTE_MODE_SPEC, u8, u8, 8, 16>;
#[doc = "Field `BYTE3` reader - Trust RAM Data"]
pub type BYTE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTE3` writer - Trust RAM Data"]
pub type BYTE3_W<'a> =
    crate::FieldWriter<'a, u32, RAM_BYTE_MODE_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 0:7 - Trust RAM Data"]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Trust RAM Data"]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Trust RAM Data"]
    #[inline(always)]
    pub fn byte2(&self) -> BYTE2_R {
        BYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Trust RAM Data"]
    #[inline(always)]
    pub fn byte3(&self) -> BYTE3_R {
        BYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trust RAM Data"]
    #[inline(always)]
    pub fn byte0(&mut self) -> BYTE0_W {
        BYTE0_W::new(self)
    }
    #[doc = "Bits 8:15 - Trust RAM Data"]
    #[inline(always)]
    pub fn byte1(&mut self) -> BYTE1_W {
        BYTE1_W::new(self)
    }
    #[doc = "Bits 16:23 - Trust RAM Data"]
    #[inline(always)]
    pub fn byte2(&mut self) -> BYTE2_W {
        BYTE2_W::new(self)
    }
    #[doc = "Bits 24:31 - Trust RAM Data"]
    #[inline(always)]
    pub fn byte3(&mut self) -> BYTE3_W {
        BYTE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TrustRAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_byte_mode](index.html) module"]
pub struct RAM_BYTE_MODE_SPEC;
impl crate::RegisterSpec for RAM_BYTE_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram_byte_mode::R](R) reader structure"]
impl crate::Readable for RAM_BYTE_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram_byte_mode::W](W) writer structure"]
impl crate::Writable for RAM_BYTE_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM_BYTE_MODE[%s]
to value 0"]
impl crate::Resettable for RAM_BYTE_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
