#[doc = "Register `CRCCTRL` reader"]
pub struct R(crate::R<CRCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCCTRL` writer"]
pub struct W(crate::W<CRCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCCTRL_SPEC>;
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
impl From<crate::W<CRCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CRC Beat Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRCBEATSIZE_A {
    #[doc = "0: 8-bit bus transfer"]
    BYTE = 0,
    #[doc = "1: 16-bit bus transfer"]
    HWORD = 1,
    #[doc = "2: 32-bit bus transfer"]
    WORD = 2,
}
impl From<CRCBEATSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCBEATSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRCBEATSIZE` reader - CRC Beat Size"]
pub type CRCBEATSIZE_R = crate::FieldReader<u8, CRCBEATSIZE_A>;
impl CRCBEATSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRCBEATSIZE_A> {
        match self.bits {
            0 => Some(CRCBEATSIZE_A::BYTE),
            1 => Some(CRCBEATSIZE_A::HWORD),
            2 => Some(CRCBEATSIZE_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == CRCBEATSIZE_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HWORD`"]
    #[inline(always)]
    pub fn is_hword(&self) -> bool {
        *self == CRCBEATSIZE_A::HWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == CRCBEATSIZE_A::WORD
    }
}
#[doc = "Field `CRCBEATSIZE` writer - CRC Beat Size"]
pub type CRCBEATSIZE_W<'a> =
    crate::FieldWriter<'a, u16, CRCCTRL_SPEC, u8, CRCBEATSIZE_A, 2, 0>;
impl<'a> CRCBEATSIZE_W<'a> {
    #[doc = "8-bit bus transfer"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(CRCBEATSIZE_A::BYTE)
    }
    #[doc = "16-bit bus transfer"]
    #[inline(always)]
    pub fn hword(self) -> &'a mut W {
        self.variant(CRCBEATSIZE_A::HWORD)
    }
    #[doc = "32-bit bus transfer"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(CRCBEATSIZE_A::WORD)
    }
}
#[doc = "CRC Polynomial Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRCPOLY_A {
    #[doc = "0: CRC-16 (CRC-CCITT)"]
    CRC16 = 0,
    #[doc = "1: CRC32 (IEEE 802.3)"]
    CRC32 = 1,
}
impl From<CRCPOLY_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCPOLY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRCPOLY` reader - CRC Polynomial Type"]
pub type CRCPOLY_R = crate::FieldReader<u8, CRCPOLY_A>;
impl CRCPOLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRCPOLY_A> {
        match self.bits {
            0 => Some(CRCPOLY_A::CRC16),
            1 => Some(CRCPOLY_A::CRC32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CRC16`"]
    #[inline(always)]
    pub fn is_crc16(&self) -> bool {
        *self == CRCPOLY_A::CRC16
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == CRCPOLY_A::CRC32
    }
}
#[doc = "Field `CRCPOLY` writer - CRC Polynomial Type"]
pub type CRCPOLY_W<'a> =
    crate::FieldWriter<'a, u16, CRCCTRL_SPEC, u8, CRCPOLY_A, 2, 2>;
impl<'a> CRCPOLY_W<'a> {
    #[doc = "CRC-16 (CRC-CCITT)"]
    #[inline(always)]
    pub fn crc16(self) -> &'a mut W {
        self.variant(CRCPOLY_A::CRC16)
    }
    #[doc = "CRC32 (IEEE 802.3)"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut W {
        self.variant(CRCPOLY_A::CRC32)
    }
}
#[doc = "CRC Input Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRCSRC_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: I/O interface"]
    IO = 1,
    #[doc = "32: DMA Channel 0"]
    CHN0 = 32,
    #[doc = "33: DMA Channel 1"]
    CHN1 = 33,
    #[doc = "34: DMA Channel 2"]
    CHN2 = 34,
    #[doc = "35: DMA Channel 3"]
    CHN3 = 35,
    #[doc = "36: DMA Channel 4"]
    CHN4 = 36,
    #[doc = "37: DMA Channel 5"]
    CHN5 = 37,
    #[doc = "38: DMA Channel 6"]
    CHN6 = 38,
    #[doc = "39: DMA Channel 7"]
    CHN7 = 39,
}
impl From<CRCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRCSRC` reader - CRC Input Source"]
pub type CRCSRC_R = crate::FieldReader<u8, CRCSRC_A>;
impl CRCSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRCSRC_A> {
        match self.bits {
            0 => Some(CRCSRC_A::NOACT),
            1 => Some(CRCSRC_A::IO),
            32 => Some(CRCSRC_A::CHN0),
            33 => Some(CRCSRC_A::CHN1),
            34 => Some(CRCSRC_A::CHN2),
            35 => Some(CRCSRC_A::CHN3),
            36 => Some(CRCSRC_A::CHN4),
            37 => Some(CRCSRC_A::CHN5),
            38 => Some(CRCSRC_A::CHN6),
            39 => Some(CRCSRC_A::CHN7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == CRCSRC_A::NOACT
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == CRCSRC_A::IO
    }
    #[doc = "Checks if the value of the field is `CHN0`"]
    #[inline(always)]
    pub fn is_chn0(&self) -> bool {
        *self == CRCSRC_A::CHN0
    }
    #[doc = "Checks if the value of the field is `CHN1`"]
    #[inline(always)]
    pub fn is_chn1(&self) -> bool {
        *self == CRCSRC_A::CHN1
    }
    #[doc = "Checks if the value of the field is `CHN2`"]
    #[inline(always)]
    pub fn is_chn2(&self) -> bool {
        *self == CRCSRC_A::CHN2
    }
    #[doc = "Checks if the value of the field is `CHN3`"]
    #[inline(always)]
    pub fn is_chn3(&self) -> bool {
        *self == CRCSRC_A::CHN3
    }
    #[doc = "Checks if the value of the field is `CHN4`"]
    #[inline(always)]
    pub fn is_chn4(&self) -> bool {
        *self == CRCSRC_A::CHN4
    }
    #[doc = "Checks if the value of the field is `CHN5`"]
    #[inline(always)]
    pub fn is_chn5(&self) -> bool {
        *self == CRCSRC_A::CHN5
    }
    #[doc = "Checks if the value of the field is `CHN6`"]
    #[inline(always)]
    pub fn is_chn6(&self) -> bool {
        *self == CRCSRC_A::CHN6
    }
    #[doc = "Checks if the value of the field is `CHN7`"]
    #[inline(always)]
    pub fn is_chn7(&self) -> bool {
        *self == CRCSRC_A::CHN7
    }
}
#[doc = "Field `CRCSRC` writer - CRC Input Source"]
pub type CRCSRC_W<'a> =
    crate::FieldWriter<'a, u16, CRCCTRL_SPEC, u8, CRCSRC_A, 6, 8>;
impl<'a> CRCSRC_W<'a> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut W {
        self.variant(CRCSRC_A::NOACT)
    }
    #[doc = "I/O interface"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(CRCSRC_A::IO)
    }
    #[doc = "DMA Channel 0"]
    #[inline(always)]
    pub fn chn0(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN0)
    }
    #[doc = "DMA Channel 1"]
    #[inline(always)]
    pub fn chn1(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN1)
    }
    #[doc = "DMA Channel 2"]
    #[inline(always)]
    pub fn chn2(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN2)
    }
    #[doc = "DMA Channel 3"]
    #[inline(always)]
    pub fn chn3(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN3)
    }
    #[doc = "DMA Channel 4"]
    #[inline(always)]
    pub fn chn4(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN4)
    }
    #[doc = "DMA Channel 5"]
    #[inline(always)]
    pub fn chn5(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN5)
    }
    #[doc = "DMA Channel 6"]
    #[inline(always)]
    pub fn chn6(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN6)
    }
    #[doc = "DMA Channel 7"]
    #[inline(always)]
    pub fn chn7(self) -> &'a mut W {
        self.variant(CRCSRC_A::CHN7)
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC Beat Size"]
    #[inline(always)]
    pub fn crcbeatsize(&self) -> CRCBEATSIZE_R {
        CRCBEATSIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CRC Polynomial Type"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:13 - CRC Input Source"]
    #[inline(always)]
    pub fn crcsrc(&self) -> CRCSRC_R {
        CRCSRC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC Beat Size"]
    #[inline(always)]
    pub fn crcbeatsize(&mut self) -> CRCBEATSIZE_W {
        CRCBEATSIZE_W::new(self)
    }
    #[doc = "Bits 2:3 - CRC Polynomial Type"]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W {
        CRCPOLY_W::new(self)
    }
    #[doc = "Bits 8:13 - CRC Input Source"]
    #[inline(always)]
    pub fn crcsrc(&mut self) -> CRCSRC_W {
        CRCSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcctrl](index.html) module"]
pub struct CRCCTRL_SPEC;
impl crate::RegisterSpec for CRCCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crcctrl::R](R) reader structure"]
impl crate::Readable for CRCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcctrl::W](W) writer structure"]
impl crate::Writable for CRCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCCTRL to value 0"]
impl crate::Resettable for CRCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
