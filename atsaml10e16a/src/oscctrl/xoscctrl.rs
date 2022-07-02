#[doc = "Register `XOSCCTRL` reader"]
pub struct R(crate::R<XOSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSCCTRL` writer"]
pub struct W(crate::W<XOSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSCCTRL_SPEC>;
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
impl From<crate::W<XOSCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, 1>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XTALEN_R = crate::BitReader<bool>;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XTALEN_W<'a> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, 2>;
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub type CFDEN_R = crate::BitReader<bool>;
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub type CFDEN_W<'a> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, 3>;
#[doc = "Field `SWBEN` reader - Xosc Clock Switch Enable"]
pub type SWBEN_R = crate::BitReader<bool>;
#[doc = "Field `SWBEN` writer - Xosc Clock Switch Enable"]
pub type SWBEN_W<'a> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, 4>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, 6>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, 7>;
#[doc = "Oscillator Gain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN_A {
    #[doc = "0: 2 MHz"]
    GAIN2 = 0,
    #[doc = "1: 4 MHz"]
    GAIN4 = 1,
    #[doc = "2: 8 MHz"]
    GAIN8 = 2,
    #[doc = "3: 16 MHz"]
    GAIN16 = 3,
    #[doc = "4: 30 MHz"]
    GAIN30 = 4,
}
impl From<GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GAIN` reader - Oscillator Gain"]
pub type GAIN_R = crate::FieldReader<u8, GAIN_A>;
impl GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GAIN_A> {
        match self.bits {
            0 => Some(GAIN_A::GAIN2),
            1 => Some(GAIN_A::GAIN4),
            2 => Some(GAIN_A::GAIN8),
            3 => Some(GAIN_A::GAIN16),
            4 => Some(GAIN_A::GAIN30),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GAIN2`"]
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        *self == GAIN_A::GAIN2
    }
    #[doc = "Checks if the value of the field is `GAIN4`"]
    #[inline(always)]
    pub fn is_gain4(&self) -> bool {
        *self == GAIN_A::GAIN4
    }
    #[doc = "Checks if the value of the field is `GAIN8`"]
    #[inline(always)]
    pub fn is_gain8(&self) -> bool {
        *self == GAIN_A::GAIN8
    }
    #[doc = "Checks if the value of the field is `GAIN16`"]
    #[inline(always)]
    pub fn is_gain16(&self) -> bool {
        *self == GAIN_A::GAIN16
    }
    #[doc = "Checks if the value of the field is `GAIN30`"]
    #[inline(always)]
    pub fn is_gain30(&self) -> bool {
        *self == GAIN_A::GAIN30
    }
}
#[doc = "Field `GAIN` writer - Oscillator Gain"]
pub type GAIN_W<'a> =
    crate::FieldWriter<'a, u16, XOSCCTRL_SPEC, u8, GAIN_A, 3, 8>;
impl<'a> GAIN_W<'a> {
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn gain2(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN2)
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn gain4(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN4)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn gain8(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN8)
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn gain16(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN16)
    }
    #[doc = "30 MHz"]
    #[inline(always)]
    pub fn gain30(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN30)
    }
}
#[doc = "Field `AMPGC` reader - Automatic Amplitude Gain Control"]
pub type AMPGC_R = crate::BitReader<bool>;
#[doc = "Field `AMPGC` writer - Automatic Amplitude Gain Control"]
pub type AMPGC_W<'a> = crate::BitWriter<'a, u16, XOSCCTRL_SPEC, bool, 11>;
#[doc = "Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 31 us"]
    CYCLE1 = 0,
    #[doc = "1: 61 us"]
    CYCLE2 = 1,
    #[doc = "2: 122 us"]
    CYCLE4 = 2,
    #[doc = "3: 244 us"]
    CYCLE8 = 3,
    #[doc = "4: 488 us"]
    CYCLE16 = 4,
    #[doc = "5: 977 us"]
    CYCLE32 = 5,
    #[doc = "6: 1953 us"]
    CYCLE64 = 6,
    #[doc = "7: 3906 us"]
    CYCLE128 = 7,
    #[doc = "8: 7813 us"]
    CYCLE256 = 8,
    #[doc = "9: 15625 us"]
    CYCLE512 = 9,
    #[doc = "10: 31250 us"]
    CYCLE1024 = 10,
    #[doc = "11: 62500 us"]
    CYCLE2048 = 11,
    #[doc = "12: 125000 us"]
    CYCLE4096 = 12,
    #[doc = "13: 250000 us"]
    CYCLE8192 = 13,
    #[doc = "14: 500000 us"]
    CYCLE16384 = 14,
    #[doc = "15: 1000000 us"]
    CYCLE32768 = 15,
}
impl From<STARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STARTUP` reader - Start-Up Time"]
pub type STARTUP_R = crate::FieldReader<u8, STARTUP_A>;
impl STARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTUP_A {
        match self.bits {
            0 => STARTUP_A::CYCLE1,
            1 => STARTUP_A::CYCLE2,
            2 => STARTUP_A::CYCLE4,
            3 => STARTUP_A::CYCLE8,
            4 => STARTUP_A::CYCLE16,
            5 => STARTUP_A::CYCLE32,
            6 => STARTUP_A::CYCLE64,
            7 => STARTUP_A::CYCLE128,
            8 => STARTUP_A::CYCLE256,
            9 => STARTUP_A::CYCLE512,
            10 => STARTUP_A::CYCLE1024,
            11 => STARTUP_A::CYCLE2048,
            12 => STARTUP_A::CYCLE4096,
            13 => STARTUP_A::CYCLE8192,
            14 => STARTUP_A::CYCLE16384,
            15 => STARTUP_A::CYCLE32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE1`"]
    #[inline(always)]
    pub fn is_cycle1(&self) -> bool {
        *self == STARTUP_A::CYCLE1
    }
    #[doc = "Checks if the value of the field is `CYCLE2`"]
    #[inline(always)]
    pub fn is_cycle2(&self) -> bool {
        *self == STARTUP_A::CYCLE2
    }
    #[doc = "Checks if the value of the field is `CYCLE4`"]
    #[inline(always)]
    pub fn is_cycle4(&self) -> bool {
        *self == STARTUP_A::CYCLE4
    }
    #[doc = "Checks if the value of the field is `CYCLE8`"]
    #[inline(always)]
    pub fn is_cycle8(&self) -> bool {
        *self == STARTUP_A::CYCLE8
    }
    #[doc = "Checks if the value of the field is `CYCLE16`"]
    #[inline(always)]
    pub fn is_cycle16(&self) -> bool {
        *self == STARTUP_A::CYCLE16
    }
    #[doc = "Checks if the value of the field is `CYCLE32`"]
    #[inline(always)]
    pub fn is_cycle32(&self) -> bool {
        *self == STARTUP_A::CYCLE32
    }
    #[doc = "Checks if the value of the field is `CYCLE64`"]
    #[inline(always)]
    pub fn is_cycle64(&self) -> bool {
        *self == STARTUP_A::CYCLE64
    }
    #[doc = "Checks if the value of the field is `CYCLE128`"]
    #[inline(always)]
    pub fn is_cycle128(&self) -> bool {
        *self == STARTUP_A::CYCLE128
    }
    #[doc = "Checks if the value of the field is `CYCLE256`"]
    #[inline(always)]
    pub fn is_cycle256(&self) -> bool {
        *self == STARTUP_A::CYCLE256
    }
    #[doc = "Checks if the value of the field is `CYCLE512`"]
    #[inline(always)]
    pub fn is_cycle512(&self) -> bool {
        *self == STARTUP_A::CYCLE512
    }
    #[doc = "Checks if the value of the field is `CYCLE1024`"]
    #[inline(always)]
    pub fn is_cycle1024(&self) -> bool {
        *self == STARTUP_A::CYCLE1024
    }
    #[doc = "Checks if the value of the field is `CYCLE2048`"]
    #[inline(always)]
    pub fn is_cycle2048(&self) -> bool {
        *self == STARTUP_A::CYCLE2048
    }
    #[doc = "Checks if the value of the field is `CYCLE4096`"]
    #[inline(always)]
    pub fn is_cycle4096(&self) -> bool {
        *self == STARTUP_A::CYCLE4096
    }
    #[doc = "Checks if the value of the field is `CYCLE8192`"]
    #[inline(always)]
    pub fn is_cycle8192(&self) -> bool {
        *self == STARTUP_A::CYCLE8192
    }
    #[doc = "Checks if the value of the field is `CYCLE16384`"]
    #[inline(always)]
    pub fn is_cycle16384(&self) -> bool {
        *self == STARTUP_A::CYCLE16384
    }
    #[doc = "Checks if the value of the field is `CYCLE32768`"]
    #[inline(always)]
    pub fn is_cycle32768(&self) -> bool {
        *self == STARTUP_A::CYCLE32768
    }
}
#[doc = "Field `STARTUP` writer - Start-Up Time"]
pub type STARTUP_W<'a> =
    crate::FieldWriterSafe<'a, u16, XOSCCTRL_SPEC, u8, STARTUP_A, 4, 12>;
impl<'a> STARTUP_W<'a> {
    #[doc = "31 us"]
    #[inline(always)]
    pub fn cycle1(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE1)
    }
    #[doc = "61 us"]
    #[inline(always)]
    pub fn cycle2(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE2)
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn cycle4(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE4)
    }
    #[doc = "244 us"]
    #[inline(always)]
    pub fn cycle8(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE8)
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn cycle16(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE16)
    }
    #[doc = "977 us"]
    #[inline(always)]
    pub fn cycle32(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE32)
    }
    #[doc = "1953 us"]
    #[inline(always)]
    pub fn cycle64(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE64)
    }
    #[doc = "3906 us"]
    #[inline(always)]
    pub fn cycle128(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE128)
    }
    #[doc = "7813 us"]
    #[inline(always)]
    pub fn cycle256(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE256)
    }
    #[doc = "15625 us"]
    #[inline(always)]
    pub fn cycle512(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE512)
    }
    #[doc = "31250 us"]
    #[inline(always)]
    pub fn cycle1024(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE1024)
    }
    #[doc = "62500 us"]
    #[inline(always)]
    pub fn cycle2048(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE2048)
    }
    #[doc = "125000 us"]
    #[inline(always)]
    pub fn cycle4096(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE4096)
    }
    #[doc = "250000 us"]
    #[inline(always)]
    pub fn cycle8192(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE8192)
    }
    #[doc = "500000 us"]
    #[inline(always)]
    pub fn cycle16384(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE16384)
    }
    #[doc = "1000000 us"]
    #[inline(always)]
    pub fn cycle32768(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE32768)
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XTALEN_R {
        XTALEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Xosc Clock Switch Enable"]
    #[inline(always)]
    pub fn swben(&self) -> SWBEN_R {
        SWBEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    pub fn ampgc(&self) -> AMPGC_R {
        AMPGC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&mut self) -> XTALEN_W {
        XTALEN_W::new(self)
    }
    #[doc = "Bit 3 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CFDEN_W {
        CFDEN_W::new(self)
    }
    #[doc = "Bit 4 - Xosc Clock Switch Enable"]
    #[inline(always)]
    pub fn swben(&mut self) -> SWBEN_W {
        SWBEN_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W::new(self)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    pub fn ampgc(&mut self) -> AMPGC_W {
        AMPGC_W::new(self)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xoscctrl](index.html) module"]
pub struct XOSCCTRL_SPEC;
impl crate::RegisterSpec for XOSCCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xoscctrl::R](R) reader structure"]
impl crate::Readable for XOSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xoscctrl::W](W) writer structure"]
impl crate::Writable for XOSCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XOSCCTRL to value 0x80"]
impl crate::Resettable for XOSCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
