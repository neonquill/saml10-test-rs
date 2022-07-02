#[doc = "Register `INTPEND` reader"]
pub struct R(crate::R<INTPEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTPEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTPEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTPEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTPEND` writer"]
pub struct W(crate::W<INTPEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTPEND_SPEC>;
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
impl From<crate::W<INTPEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTPEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Channel ID"]
pub type ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ID` writer - Channel ID"]
pub type ID_W<'a> = crate::FieldWriter<'a, u16, INTPEND_SPEC, u8, u8, 2, 0>;
#[doc = "Field `OVR` reader - Channel Overrun"]
pub type OVR_R = crate::BitReader<bool>;
#[doc = "Field `OVR` writer - Channel Overrun"]
pub type OVR_W<'a> = crate::BitWriter<'a, u16, INTPEND_SPEC, bool, 8>;
#[doc = "Field `EVD` reader - Channel Event Detected"]
pub type EVD_R = crate::BitReader<bool>;
#[doc = "Field `EVD` writer - Channel Event Detected"]
pub type EVD_W<'a> = crate::BitWriter<'a, u16, INTPEND_SPEC, bool, 9>;
#[doc = "Field `READY` reader - Ready"]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Field `READY` writer - Ready"]
pub type READY_W<'a> = crate::BitWriter<'a, u16, INTPEND_SPEC, bool, 14>;
#[doc = "Field `BUSY` reader - Busy"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - Busy"]
pub type BUSY_W<'a> = crate::BitWriter<'a, u16, INTPEND_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:1 - Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Channel Overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Event Detected"]
    #[inline(always)]
    pub fn evd(&self) -> EVD_R {
        EVD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel ID"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W::new(self)
    }
    #[doc = "Bit 8 - Channel Overrun"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W::new(self)
    }
    #[doc = "Bit 9 - Channel Event Detected"]
    #[inline(always)]
    pub fn evd(&mut self) -> EVD_W {
        EVD_W::new(self)
    }
    #[doc = "Bit 14 - Ready"]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W {
        READY_W::new(self)
    }
    #[doc = "Bit 15 - Busy"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Pending Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intpend](index.html) module"]
pub struct INTPEND_SPEC;
impl crate::RegisterSpec for INTPEND_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [intpend::R](R) reader structure"]
impl crate::Readable for INTPEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intpend::W](W) writer structure"]
impl crate::Writable for INTPEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTPEND to value 0x4000"]
impl crate::Resettable for INTPEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
