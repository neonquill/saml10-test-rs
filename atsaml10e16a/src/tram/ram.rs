#[doc = "Register `RAM[%s]` reader"]
pub struct R(crate::R<RAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM[%s]` writer"]
pub struct W(crate::W<RAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_SPEC>;
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
impl From<crate::W<RAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Trust RAM Data"]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - Trust RAM Data"]
pub type DATA_W<'a> = crate::FieldWriter<'a, u32, RAM_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Trust RAM Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trust RAM Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TrustRAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram](index.html) module"]
pub struct RAM_SPEC;
impl crate::RegisterSpec for RAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram::R](R) reader structure"]
impl crate::Readable for RAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram::W](W) writer structure"]
impl crate::Writable for RAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM[%s]
to value 0"]
impl crate::Resettable for RAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}