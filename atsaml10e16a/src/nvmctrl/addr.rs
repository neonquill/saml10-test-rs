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
#[doc = "Field `AOFFSET` reader - NVM Address Offset In The Selected Array"]
pub type AOFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AOFFSET` writer - NVM Address Offset In The Selected Array"]
pub type AOFFSET_W<'a> =
    crate::FieldWriter<'a, u32, ADDR_SPEC, u16, u16, 16, 0>;
#[doc = "Array Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ARRAY_A {
    #[doc = "0: FLASH Array"]
    FLASH = 0,
    #[doc = "1: DATA FLASH Array"]
    DATAFLASH = 1,
    #[doc = "2: NVM Rows"]
    NVMROWS = 2,
}
impl From<ARRAY_A> for u8 {
    #[inline(always)]
    fn from(variant: ARRAY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ARRAY` reader - Array Select"]
pub type ARRAY_R = crate::FieldReader<u8, ARRAY_A>;
impl ARRAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARRAY_A> {
        match self.bits {
            0 => Some(ARRAY_A::FLASH),
            1 => Some(ARRAY_A::DATAFLASH),
            2 => Some(ARRAY_A::NVMROWS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == ARRAY_A::FLASH
    }
    #[doc = "Checks if the value of the field is `DATAFLASH`"]
    #[inline(always)]
    pub fn is_dataflash(&self) -> bool {
        *self == ARRAY_A::DATAFLASH
    }
    #[doc = "Checks if the value of the field is `NVMROWS`"]
    #[inline(always)]
    pub fn is_nvmrows(&self) -> bool {
        *self == ARRAY_A::NVMROWS
    }
}
#[doc = "Field `ARRAY` writer - Array Select"]
pub type ARRAY_W<'a> =
    crate::FieldWriter<'a, u32, ADDR_SPEC, u8, ARRAY_A, 2, 22>;
impl<'a> ARRAY_W<'a> {
    #[doc = "FLASH Array"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut W {
        self.variant(ARRAY_A::FLASH)
    }
    #[doc = "DATA FLASH Array"]
    #[inline(always)]
    pub fn dataflash(self) -> &'a mut W {
        self.variant(ARRAY_A::DATAFLASH)
    }
    #[doc = "NVM Rows"]
    #[inline(always)]
    pub fn nvmrows(self) -> &'a mut W {
        self.variant(ARRAY_A::NVMROWS)
    }
}
impl R {
    #[doc = "Bits 0:15 - NVM Address Offset In The Selected Array"]
    #[inline(always)]
    pub fn aoffset(&self) -> AOFFSET_R {
        AOFFSET_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 22:23 - Array Select"]
    #[inline(always)]
    pub fn array(&self) -> ARRAY_R {
        ARRAY_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - NVM Address Offset In The Selected Array"]
    #[inline(always)]
    pub fn aoffset(&mut self) -> AOFFSET_W {
        AOFFSET_W::new(self)
    }
    #[doc = "Bits 22:23 - Array Select"]
    #[inline(always)]
    pub fn array(&mut self) -> ARRAY_W {
        ARRAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](index.html) module"]
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
