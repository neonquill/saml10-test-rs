#[doc = "Register `ADDR` reader"]
pub struct R(crate::R<ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR` writer"]
pub struct W(crate::W<ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_SPEC>;
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
impl From<crate::W<ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Address Value"]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - Address Value"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, ADDR_SPEC, u16, u16, 11, 0>;
#[doc = "Field `LENEN` reader - Length Enable"]
pub type LENEN_R = crate::BitReader<bool>;
#[doc = "Field `LENEN` writer - Length Enable"]
pub type LENEN_W<'a> = crate::BitWriter<'a, u32, ADDR_SPEC, bool, 13>;
#[doc = "Field `HS` reader - High Speed Mode"]
pub type HS_R = crate::BitReader<bool>;
#[doc = "Field `HS` writer - High Speed Mode"]
pub type HS_W<'a> = crate::BitWriter<'a, u32, ADDR_SPEC, bool, 14>;
#[doc = "Field `TENBITEN` reader - Ten Bit Addressing Enable"]
pub type TENBITEN_R = crate::BitReader<bool>;
#[doc = "Field `TENBITEN` writer - Ten Bit Addressing Enable"]
pub type TENBITEN_W<'a> = crate::BitWriter<'a, u32, ADDR_SPEC, bool, 15>;
#[doc = "Field `LEN` reader - Length"]
pub type LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEN` writer - Length"]
pub type LEN_W<'a> = crate::FieldWriter<'a, u32, ADDR_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:10 - Address Value"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 13 - Length Enable"]
    #[inline(always)]
    pub fn lenen(&self) -> LENEN_R {
        LENEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - High Speed Mode"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    pub fn tenbiten(&self) -> TENBITEN_R {
        TENBITEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Length"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Address Value"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Bit 13 - Length Enable"]
    #[inline(always)]
    pub fn lenen(&mut self) -> LENEN_W {
        LENEN_W::new(self)
    }
    #[doc = "Bit 14 - High Speed Mode"]
    #[inline(always)]
    pub fn hs(&mut self) -> HS_W {
        HS_W::new(self)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    pub fn tenbiten(&mut self) -> TENBITEN_W {
        TENBITEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Length"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CM Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](index.html) module"]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr::R](R) reader structure"]
impl crate::Readable for ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr::W](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
