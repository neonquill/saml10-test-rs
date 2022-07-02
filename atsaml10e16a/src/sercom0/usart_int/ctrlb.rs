#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHSIZE_A {
    #[doc = "0: 8 Bits"]
    _8_BIT = 0,
    #[doc = "1: 9 Bits"]
    _9_BIT = 1,
    #[doc = "5: 5 Bits"]
    _5_BIT = 5,
    #[doc = "6: 6 Bits"]
    _6_BIT = 6,
    #[doc = "7: 7 Bits"]
    _7_BIT = 7,
}
impl From<CHSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type CHSIZE_R = crate::FieldReader<u8, CHSIZE_A>;
impl CHSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHSIZE_A> {
        match self.bits {
            0 => Some(CHSIZE_A::_8_BIT),
            1 => Some(CHSIZE_A::_9_BIT),
            5 => Some(CHSIZE_A::_5_BIT),
            6 => Some(CHSIZE_A::_6_BIT),
            7 => Some(CHSIZE_A::_7_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == CHSIZE_A::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_9_BIT`"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == CHSIZE_A::_9_BIT
    }
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == CHSIZE_A::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == CHSIZE_A::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == CHSIZE_A::_7_BIT
    }
}
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type CHSIZE_W<'a> =
    crate::FieldWriter<'a, u32, CTRLB_SPEC, u8, CHSIZE_A, 3, 0>;
impl<'a> CHSIZE_W<'a> {
    #[doc = "8 Bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(CHSIZE_A::_8_BIT)
    }
    #[doc = "9 Bits"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut W {
        self.variant(CHSIZE_A::_9_BIT)
    }
    #[doc = "5 Bits"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(CHSIZE_A::_5_BIT)
    }
    #[doc = "6 Bits"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(CHSIZE_A::_6_BIT)
    }
    #[doc = "7 Bits"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(CHSIZE_A::_7_BIT)
    }
}
#[doc = "Stop Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBMODE_A {
    #[doc = "0: One Stop Bit"]
    _1_BIT = 0,
    #[doc = "1: Two Stop Bits"]
    _2_BIT = 1,
}
impl From<SBMODE_A> for bool {
    #[inline(always)]
    fn from(variant: SBMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBMODE` reader - Stop Bit Mode"]
pub type SBMODE_R = crate::BitReader<SBMODE_A>;
impl SBMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBMODE_A {
        match self.bits {
            false => SBMODE_A::_1_BIT,
            true => SBMODE_A::_2_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_1_BIT`"]
    #[inline(always)]
    pub fn is_1_bit(&self) -> bool {
        *self == SBMODE_A::_1_BIT
    }
    #[doc = "Checks if the value of the field is `_2_BIT`"]
    #[inline(always)]
    pub fn is_2_bit(&self) -> bool {
        *self == SBMODE_A::_2_BIT
    }
}
#[doc = "Field `SBMODE` writer - Stop Bit Mode"]
pub type SBMODE_W<'a> = crate::BitWriter<'a, u32, CTRLB_SPEC, SBMODE_A, 6>;
impl<'a> SBMODE_W<'a> {
    #[doc = "One Stop Bit"]
    #[inline(always)]
    pub fn _1_bit(self) -> &'a mut W {
        self.variant(SBMODE_A::_1_BIT)
    }
    #[doc = "Two Stop Bits"]
    #[inline(always)]
    pub fn _2_bit(self) -> &'a mut W {
        self.variant(SBMODE_A::_2_BIT)
    }
}
#[doc = "Field `COLDEN` reader - Collision Detection Enable"]
pub type COLDEN_R = crate::BitReader<bool>;
#[doc = "Field `COLDEN` writer - Collision Detection Enable"]
pub type COLDEN_W<'a> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, 8>;
#[doc = "Field `SFDE` reader - Start of Frame Detection Enable"]
pub type SFDE_R = crate::BitReader<bool>;
#[doc = "Field `SFDE` writer - Start of Frame Detection Enable"]
pub type SFDE_W<'a> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, 9>;
#[doc = "Field `ENC` reader - Encoding Format"]
pub type ENC_R = crate::BitReader<bool>;
#[doc = "Field `ENC` writer - Encoding Format"]
pub type ENC_W<'a> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, 10>;
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMODE_A {
    #[doc = "0: Even Parity"]
    EVEN = 0,
    #[doc = "1: Odd Parity"]
    ODD = 1,
}
impl From<PMODE_A> for bool {
    #[inline(always)]
    fn from(variant: PMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMODE` reader - Parity Mode"]
pub type PMODE_R = crate::BitReader<PMODE_A>;
impl PMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMODE_A {
        match self.bits {
            false => PMODE_A::EVEN,
            true => PMODE_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PMODE_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PMODE_A::ODD
    }
}
#[doc = "Field `PMODE` writer - Parity Mode"]
pub type PMODE_W<'a> = crate::BitWriter<'a, u32, CTRLB_SPEC, PMODE_A, 13>;
impl<'a> PMODE_W<'a> {
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PMODE_A::EVEN)
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PMODE_A::ODD)
    }
}
#[doc = "Field `TXEN` reader - Transmitter Enable"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, 16>;
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a> = crate::BitWriter<'a, u32, CTRLB_SPEC, bool, 17>;
impl R {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&self) -> CHSIZE_R {
        CHSIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline(always)]
    pub fn sbmode(&self) -> SBMODE_R {
        SBMODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline(always)]
    pub fn colden(&self) -> COLDEN_R {
        COLDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde(&self) -> SFDE_R {
        SFDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&mut self) -> CHSIZE_W {
        CHSIZE_W::new(self)
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline(always)]
    pub fn sbmode(&mut self) -> SBMODE_W {
        SBMODE_W::new(self)
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline(always)]
    pub fn colden(&mut self) -> COLDEN_W {
        COLDEN_W::new(self)
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde(&mut self) -> SFDE_W {
        SFDE_W::new(self)
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W::new(self)
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline(always)]
    pub fn pmode(&mut self) -> PMODE_W {
        PMODE_W::new(self)
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W::new(self)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_INT Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
