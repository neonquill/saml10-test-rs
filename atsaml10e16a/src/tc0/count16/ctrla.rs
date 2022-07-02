#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 0>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 1>;
#[doc = "Timer Counter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Counter in 16-bit mode"]
    COUNT16 = 0,
    #[doc = "1: Counter in 8-bit mode"]
    COUNT8 = 1,
    #[doc = "2: Counter in 32-bit mode"]
    COUNT32 = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Timer Counter Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::COUNT16),
            1 => Some(MODE_A::COUNT8),
            2 => Some(MODE_A::COUNT32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COUNT16`"]
    #[inline(always)]
    pub fn is_count16(&self) -> bool {
        *self == MODE_A::COUNT16
    }
    #[doc = "Checks if the value of the field is `COUNT8`"]
    #[inline(always)]
    pub fn is_count8(&self) -> bool {
        *self == MODE_A::COUNT8
    }
    #[doc = "Checks if the value of the field is `COUNT32`"]
    #[inline(always)]
    pub fn is_count32(&self) -> bool {
        *self == MODE_A::COUNT32
    }
}
#[doc = "Field `MODE` writer - Timer Counter Mode"]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, MODE_A, 2, 2>;
impl<'a> MODE_W<'a> {
    #[doc = "Counter in 16-bit mode"]
    #[inline(always)]
    pub fn count16(self) -> &'a mut W {
        self.variant(MODE_A::COUNT16)
    }
    #[doc = "Counter in 8-bit mode"]
    #[inline(always)]
    pub fn count8(self) -> &'a mut W {
        self.variant(MODE_A::COUNT8)
    }
    #[doc = "Counter in 32-bit mode"]
    #[inline(always)]
    pub fn count32(self) -> &'a mut W {
        self.variant(MODE_A::COUNT32)
    }
}
#[doc = "Prescaler and Counter Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCSYNC_A {
    #[doc = "0: Reload or reset the counter on next generic clock"]
    GCLK = 0,
    #[doc = "1: Reload or reset the counter on next prescaler clock"]
    PRESC = 1,
    #[doc = "2: Reload or reset the counter on next generic clock and reset the prescaler counter"]
    RESYNC = 2,
}
impl From<PRESCSYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCSYNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESCSYNC` reader - Prescaler and Counter Synchronization"]
pub type PRESCSYNC_R = crate::FieldReader<u8, PRESCSYNC_A>;
impl PRESCSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCSYNC_A> {
        match self.bits {
            0 => Some(PRESCSYNC_A::GCLK),
            1 => Some(PRESCSYNC_A::PRESC),
            2 => Some(PRESCSYNC_A::RESYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == PRESCSYNC_A::GCLK
    }
    #[doc = "Checks if the value of the field is `PRESC`"]
    #[inline(always)]
    pub fn is_presc(&self) -> bool {
        *self == PRESCSYNC_A::PRESC
    }
    #[doc = "Checks if the value of the field is `RESYNC`"]
    #[inline(always)]
    pub fn is_resync(&self) -> bool {
        *self == PRESCSYNC_A::RESYNC
    }
}
#[doc = "Field `PRESCSYNC` writer - Prescaler and Counter Synchronization"]
pub type PRESCSYNC_W<'a> =
    crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, PRESCSYNC_A, 2, 4>;
impl<'a> PRESCSYNC_W<'a> {
    #[doc = "Reload or reset the counter on next generic clock"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(PRESCSYNC_A::GCLK)
    }
    #[doc = "Reload or reset the counter on next prescaler clock"]
    #[inline(always)]
    pub fn presc(self) -> &'a mut W {
        self.variant(PRESCSYNC_A::PRESC)
    }
    #[doc = "Reload or reset the counter on next generic clock and reset the prescaler counter"]
    #[inline(always)]
    pub fn resync(self) -> &'a mut W {
        self.variant(PRESCSYNC_A::RESYNC)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 6>;
#[doc = "Field `ONDEMAND` reader - Clock On Demand"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - Clock On Demand"]
pub type ONDEMAND_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 7>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: Prescaler: GCLK_TC"]
    DIV1 = 0,
    #[doc = "1: Prescaler: GCLK_TC/2"]
    DIV2 = 1,
    #[doc = "2: Prescaler: GCLK_TC/4"]
    DIV4 = 2,
    #[doc = "3: Prescaler: GCLK_TC/8"]
    DIV8 = 3,
    #[doc = "4: Prescaler: GCLK_TC/16"]
    DIV16 = 4,
    #[doc = "5: Prescaler: GCLK_TC/64"]
    DIV64 = 5,
    #[doc = "6: Prescaler: GCLK_TC/256"]
    DIV256 = 6,
    #[doc = "7: Prescaler: GCLK_TC/1024"]
    DIV1024 = 7,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESCALER` reader - Prescaler"]
pub type PRESCALER_R = crate::FieldReader<u8, PRESCALER_A>;
impl PRESCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALER_A {
        match self.bits {
            0 => PRESCALER_A::DIV1,
            1 => PRESCALER_A::DIV2,
            2 => PRESCALER_A::DIV4,
            3 => PRESCALER_A::DIV8,
            4 => PRESCALER_A::DIV16,
            5 => PRESCALER_A::DIV64,
            6 => PRESCALER_A::DIV256,
            7 => PRESCALER_A::DIV1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALER_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCALER_A::DIV1024
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler"]
pub type PRESCALER_W<'a> =
    crate::FieldWriterSafe<'a, u32, CTRLA_SPEC, u8, PRESCALER_A, 3, 8>;
impl<'a> PRESCALER_W<'a> {
    #[doc = "Prescaler: GCLK_TC"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV1)
    }
    #[doc = "Prescaler: GCLK_TC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV2)
    }
    #[doc = "Prescaler: GCLK_TC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV4)
    }
    #[doc = "Prescaler: GCLK_TC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV8)
    }
    #[doc = "Prescaler: GCLK_TC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV16)
    }
    #[doc = "Prescaler: GCLK_TC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV64)
    }
    #[doc = "Prescaler: GCLK_TC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV256)
    }
    #[doc = "Prescaler: GCLK_TC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV1024)
    }
}
#[doc = "Field `ALOCK` reader - Auto Lock"]
pub type ALOCK_R = crate::BitReader<bool>;
#[doc = "Field `ALOCK` writer - Auto Lock"]
pub type ALOCK_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 11>;
#[doc = "Field `CAPTEN0` reader - Capture Channel 0 Enable"]
pub type CAPTEN0_R = crate::BitReader<bool>;
#[doc = "Field `CAPTEN0` writer - Capture Channel 0 Enable"]
pub type CAPTEN0_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 16>;
#[doc = "Field `CAPTEN1` reader - Capture Channel 1 Enable"]
pub type CAPTEN1_R = crate::BitReader<bool>;
#[doc = "Field `CAPTEN1` writer - Capture Channel 1 Enable"]
pub type CAPTEN1_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 17>;
#[doc = "Field `COPEN0` reader - Capture On Pin 0 Enable"]
pub type COPEN0_R = crate::BitReader<bool>;
#[doc = "Field `COPEN0` writer - Capture On Pin 0 Enable"]
pub type COPEN0_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 20>;
#[doc = "Field `COPEN1` reader - Capture On Pin 1 Enable"]
pub type COPEN1_R = crate::BitReader<bool>;
#[doc = "Field `COPEN1` writer - Capture On Pin 1 Enable"]
pub type COPEN1_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 21>;
#[doc = "Capture Mode Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPTMODE0_A {
    #[doc = "0: Default capture"]
    DEFAULT = 0,
    #[doc = "1: Minimum capture"]
    CAPTMIN = 1,
    #[doc = "2: Maximum capture"]
    CAPTMAX = 2,
}
impl From<CAPTMODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTMODE0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPTMODE0` reader - Capture Mode Channel 0"]
pub type CAPTMODE0_R = crate::FieldReader<u8, CAPTMODE0_A>;
impl CAPTMODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPTMODE0_A> {
        match self.bits {
            0 => Some(CAPTMODE0_A::DEFAULT),
            1 => Some(CAPTMODE0_A::CAPTMIN),
            2 => Some(CAPTMODE0_A::CAPTMAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == CAPTMODE0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `CAPTMIN`"]
    #[inline(always)]
    pub fn is_captmin(&self) -> bool {
        *self == CAPTMODE0_A::CAPTMIN
    }
    #[doc = "Checks if the value of the field is `CAPTMAX`"]
    #[inline(always)]
    pub fn is_captmax(&self) -> bool {
        *self == CAPTMODE0_A::CAPTMAX
    }
}
#[doc = "Field `CAPTMODE0` writer - Capture Mode Channel 0"]
pub type CAPTMODE0_W<'a> =
    crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, CAPTMODE0_A, 2, 24>;
impl<'a> CAPTMODE0_W<'a> {
    #[doc = "Default capture"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(CAPTMODE0_A::DEFAULT)
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn captmin(self) -> &'a mut W {
        self.variant(CAPTMODE0_A::CAPTMIN)
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn captmax(self) -> &'a mut W {
        self.variant(CAPTMODE0_A::CAPTMAX)
    }
}
#[doc = "Capture mode Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPTMODE1_A {
    #[doc = "0: Default capture"]
    DEFAULT = 0,
    #[doc = "1: Minimum capture"]
    CAPTMIN = 1,
    #[doc = "2: Maximum capture"]
    CAPTMAX = 2,
}
impl From<CAPTMODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTMODE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPTMODE1` reader - Capture mode Channel 1"]
pub type CAPTMODE1_R = crate::FieldReader<u8, CAPTMODE1_A>;
impl CAPTMODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPTMODE1_A> {
        match self.bits {
            0 => Some(CAPTMODE1_A::DEFAULT),
            1 => Some(CAPTMODE1_A::CAPTMIN),
            2 => Some(CAPTMODE1_A::CAPTMAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == CAPTMODE1_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `CAPTMIN`"]
    #[inline(always)]
    pub fn is_captmin(&self) -> bool {
        *self == CAPTMODE1_A::CAPTMIN
    }
    #[doc = "Checks if the value of the field is `CAPTMAX`"]
    #[inline(always)]
    pub fn is_captmax(&self) -> bool {
        *self == CAPTMODE1_A::CAPTMAX
    }
}
#[doc = "Field `CAPTMODE1` writer - Capture mode Channel 1"]
pub type CAPTMODE1_W<'a> =
    crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, CAPTMODE1_A, 2, 27>;
impl<'a> CAPTMODE1_W<'a> {
    #[doc = "Default capture"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(CAPTMODE1_A::DEFAULT)
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn captmin(self) -> &'a mut W {
        self.variant(CAPTMODE1_A::CAPTMIN)
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn captmax(self) -> &'a mut W {
        self.variant(CAPTMODE1_A::CAPTMAX)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Timer Counter Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Prescaler and Counter Synchronization"]
    #[inline(always)]
    pub fn prescsync(&self) -> PRESCSYNC_R {
        PRESCSYNC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock On Demand"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&self) -> ALOCK_R {
        ALOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn capten0(&self) -> CAPTEN0_R {
        CAPTEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn capten1(&self) -> CAPTEN1_R {
        CAPTEN1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Capture On Pin 0 Enable"]
    #[inline(always)]
    pub fn copen0(&self) -> COPEN0_R {
        COPEN0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Capture On Pin 1 Enable"]
    #[inline(always)]
    pub fn copen1(&self) -> COPEN1_R {
        COPEN1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Capture Mode Channel 0"]
    #[inline(always)]
    pub fn captmode0(&self) -> CAPTMODE0_R {
        CAPTMODE0_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 27:28 - Capture mode Channel 1"]
    #[inline(always)]
    pub fn captmode1(&self) -> CAPTMODE1_R {
        CAPTMODE1_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Timer Counter Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Prescaler and Counter Synchronization"]
    #[inline(always)]
    pub fn prescsync(&mut self) -> PRESCSYNC_W {
        PRESCSYNC_W::new(self)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - Clock On Demand"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W::new(self)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&mut self) -> ALOCK_W {
        ALOCK_W::new(self)
    }
    #[doc = "Bit 16 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn capten0(&mut self) -> CAPTEN0_W {
        CAPTEN0_W::new(self)
    }
    #[doc = "Bit 17 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn capten1(&mut self) -> CAPTEN1_W {
        CAPTEN1_W::new(self)
    }
    #[doc = "Bit 20 - Capture On Pin 0 Enable"]
    #[inline(always)]
    pub fn copen0(&mut self) -> COPEN0_W {
        COPEN0_W::new(self)
    }
    #[doc = "Bit 21 - Capture On Pin 1 Enable"]
    #[inline(always)]
    pub fn copen1(&mut self) -> COPEN1_W {
        COPEN1_W::new(self)
    }
    #[doc = "Bits 24:25 - Capture Mode Channel 0"]
    #[inline(always)]
    pub fn captmode0(&mut self) -> CAPTMODE0_W {
        CAPTMODE0_W::new(self)
    }
    #[doc = "Bits 27:28 - Capture mode Channel 1"]
    #[inline(always)]
    pub fn captmode1(&mut self) -> CAPTMODE1_W {
        CAPTMODE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
