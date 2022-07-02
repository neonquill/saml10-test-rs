#[doc = "Register `OPAMPCTRL2` reader"]
pub struct R(crate::R<OPAMPCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMPCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMPCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMPCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMPCTRL2` writer"]
pub struct W(crate::W<OPAMPCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMPCTRL2_SPEC>;
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
impl From<crate::W<OPAMPCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMPCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Operational Amplifier Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Operational Amplifier Enable"]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u32, OPAMPCTRL2_SPEC, bool, 1>;
#[doc = "Field `ANAOUT` reader - Analog Output"]
pub type ANAOUT_R = crate::BitReader<bool>;
#[doc = "Field `ANAOUT` writer - Analog Output"]
pub type ANAOUT_W<'a> = crate::BitWriter<'a, u32, OPAMPCTRL2_SPEC, bool, 2>;
#[doc = "Field `BIAS` reader - Bias Selection"]
pub type BIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIAS` writer - Bias Selection"]
pub type BIAS_W<'a> =
    crate::FieldWriter<'a, u32, OPAMPCTRL2_SPEC, u8, u8, 2, 3>;
#[doc = "Field `RES2VCC` reader - Resistor ladder To VCC"]
pub type RES2VCC_R = crate::BitReader<bool>;
#[doc = "Field `RES2VCC` writer - Resistor ladder To VCC"]
pub type RES2VCC_W<'a> = crate::BitWriter<'a, u32, OPAMPCTRL2_SPEC, bool, 5>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a> = crate::BitWriter<'a, u32, OPAMPCTRL2_SPEC, bool, 6>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a> = crate::BitWriter<'a, u32, OPAMPCTRL2_SPEC, bool, 7>;
#[doc = "Field `RES2OUT` reader - Resistor ladder To Output"]
pub type RES2OUT_R = crate::BitReader<bool>;
#[doc = "Field `RES2OUT` writer - Resistor ladder To Output"]
pub type RES2OUT_W<'a> = crate::BitWriter<'a, u32, OPAMPCTRL2_SPEC, bool, 8>;
#[doc = "Field `RES1EN` reader - Resistor 1 Enable"]
pub type RES1EN_R = crate::BitReader<bool>;
#[doc = "Field `RES1EN` writer - Resistor 1 Enable"]
pub type RES1EN_W<'a> = crate::BitWriter<'a, u32, OPAMPCTRL2_SPEC, bool, 9>;
#[doc = "Resistor 1 Mux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES1MUX_A {
    #[doc = "0: Positive inout of OPAMPx"]
    OAXPOS = 0,
    #[doc = "1: Negative input of OPAMPx"]
    OAXNEG = 1,
    #[doc = "2: OPAMP1 output"]
    OA1OUT = 2,
    #[doc = "3: Ground"]
    GND = 3,
}
impl From<RES1MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: RES1MUX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RES1MUX` reader - Resistor 1 Mux"]
pub type RES1MUX_R = crate::FieldReader<u8, RES1MUX_A>;
impl RES1MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RES1MUX_A> {
        match self.bits {
            0 => Some(RES1MUX_A::OAXPOS),
            1 => Some(RES1MUX_A::OAXNEG),
            2 => Some(RES1MUX_A::OA1OUT),
            3 => Some(RES1MUX_A::GND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OAXPOS`"]
    #[inline(always)]
    pub fn is_oax_pos(&self) -> bool {
        *self == RES1MUX_A::OAXPOS
    }
    #[doc = "Checks if the value of the field is `OAXNEG`"]
    #[inline(always)]
    pub fn is_oax_neg(&self) -> bool {
        *self == RES1MUX_A::OAXNEG
    }
    #[doc = "Checks if the value of the field is `OA1OUT`"]
    #[inline(always)]
    pub fn is_oa1out(&self) -> bool {
        *self == RES1MUX_A::OA1OUT
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == RES1MUX_A::GND
    }
}
#[doc = "Field `RES1MUX` writer - Resistor 1 Mux"]
pub type RES1MUX_W<'a> =
    crate::FieldWriter<'a, u32, OPAMPCTRL2_SPEC, u8, RES1MUX_A, 3, 10>;
impl<'a> RES1MUX_W<'a> {
    #[doc = "Positive inout of OPAMPx"]
    #[inline(always)]
    pub fn oax_pos(self) -> &'a mut W {
        self.variant(RES1MUX_A::OAXPOS)
    }
    #[doc = "Negative input of OPAMPx"]
    #[inline(always)]
    pub fn oax_neg(self) -> &'a mut W {
        self.variant(RES1MUX_A::OAXNEG)
    }
    #[doc = "OPAMP1 output"]
    #[inline(always)]
    pub fn oa1out(self) -> &'a mut W {
        self.variant(RES1MUX_A::OA1OUT)
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(RES1MUX_A::GND)
    }
}
#[doc = "Potentiometer Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POTMUX_A {
    #[doc = "0: R1 = 14R, R2 = 2R"]
    _14R_2R = 0,
    #[doc = "1: R1 = 12R, R2 = 4R"]
    _12R_4R = 1,
    #[doc = "2: R1 = 8R, R2 = 8R"]
    _8R_8R = 2,
    #[doc = "3: R1 = 6R, R2 = 10R"]
    _6R_10R = 3,
    #[doc = "4: R1 = 4R, R2 = 12R"]
    _4R_12R = 4,
    #[doc = "5: R1 = 3R, R2 = 13R"]
    _3R_13R = 5,
    #[doc = "6: R1 = 2R, R2 = 14R"]
    _2R_14R = 6,
    #[doc = "7: R1 = 1R, R2 = 15R"]
    R_15R = 7,
}
impl From<POTMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: POTMUX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POTMUX` reader - Potentiometer Selection"]
pub type POTMUX_R = crate::FieldReader<u8, POTMUX_A>;
impl POTMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POTMUX_A {
        match self.bits {
            0 => POTMUX_A::_14R_2R,
            1 => POTMUX_A::_12R_4R,
            2 => POTMUX_A::_8R_8R,
            3 => POTMUX_A::_6R_10R,
            4 => POTMUX_A::_4R_12R,
            5 => POTMUX_A::_3R_13R,
            6 => POTMUX_A::_2R_14R,
            7 => POTMUX_A::R_15R,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_14R_2R`"]
    #[inline(always)]
    pub fn is_14r_2r(&self) -> bool {
        *self == POTMUX_A::_14R_2R
    }
    #[doc = "Checks if the value of the field is `_12R_4R`"]
    #[inline(always)]
    pub fn is_12r_4r(&self) -> bool {
        *self == POTMUX_A::_12R_4R
    }
    #[doc = "Checks if the value of the field is `_8R_8R`"]
    #[inline(always)]
    pub fn is_8r_8r(&self) -> bool {
        *self == POTMUX_A::_8R_8R
    }
    #[doc = "Checks if the value of the field is `_6R_10R`"]
    #[inline(always)]
    pub fn is_6r_10r(&self) -> bool {
        *self == POTMUX_A::_6R_10R
    }
    #[doc = "Checks if the value of the field is `_4R_12R`"]
    #[inline(always)]
    pub fn is_4r_12r(&self) -> bool {
        *self == POTMUX_A::_4R_12R
    }
    #[doc = "Checks if the value of the field is `_3R_13R`"]
    #[inline(always)]
    pub fn is_3r_13r(&self) -> bool {
        *self == POTMUX_A::_3R_13R
    }
    #[doc = "Checks if the value of the field is `_2R_14R`"]
    #[inline(always)]
    pub fn is_2r_14r(&self) -> bool {
        *self == POTMUX_A::_2R_14R
    }
    #[doc = "Checks if the value of the field is `R_15R`"]
    #[inline(always)]
    pub fn is_r_15r(&self) -> bool {
        *self == POTMUX_A::R_15R
    }
}
#[doc = "Field `POTMUX` writer - Potentiometer Selection"]
pub type POTMUX_W<'a> =
    crate::FieldWriterSafe<'a, u32, OPAMPCTRL2_SPEC, u8, POTMUX_A, 3, 13>;
impl<'a> POTMUX_W<'a> {
    #[doc = "R1 = 14R, R2 = 2R"]
    #[inline(always)]
    pub fn _14r_2r(self) -> &'a mut W {
        self.variant(POTMUX_A::_14R_2R)
    }
    #[doc = "R1 = 12R, R2 = 4R"]
    #[inline(always)]
    pub fn _12r_4r(self) -> &'a mut W {
        self.variant(POTMUX_A::_12R_4R)
    }
    #[doc = "R1 = 8R, R2 = 8R"]
    #[inline(always)]
    pub fn _8r_8r(self) -> &'a mut W {
        self.variant(POTMUX_A::_8R_8R)
    }
    #[doc = "R1 = 6R, R2 = 10R"]
    #[inline(always)]
    pub fn _6r_10r(self) -> &'a mut W {
        self.variant(POTMUX_A::_6R_10R)
    }
    #[doc = "R1 = 4R, R2 = 12R"]
    #[inline(always)]
    pub fn _4r_12r(self) -> &'a mut W {
        self.variant(POTMUX_A::_4R_12R)
    }
    #[doc = "R1 = 3R, R2 = 13R"]
    #[inline(always)]
    pub fn _3r_13r(self) -> &'a mut W {
        self.variant(POTMUX_A::_3R_13R)
    }
    #[doc = "R1 = 2R, R2 = 14R"]
    #[inline(always)]
    pub fn _2r_14r(self) -> &'a mut W {
        self.variant(POTMUX_A::_2R_14R)
    }
    #[doc = "R1 = 1R, R2 = 15R"]
    #[inline(always)]
    pub fn r_15r(self) -> &'a mut W {
        self.variant(POTMUX_A::R_15R)
    }
}
#[doc = "Positive Input Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUXPOS_A {
    #[doc = "0: Positive I/O pin"]
    OA2POS = 0,
    #[doc = "1: Resistor ladder 2 taps"]
    OA2TAP = 1,
    #[doc = "2: REFERENCE or DAC output"]
    REFERENCE_DAC = 2,
    #[doc = "3: Ground"]
    GND = 3,
    #[doc = "4: OPAMP1 output"]
    OA1OUT = 4,
    #[doc = "5: Positive I/O pin OPA0"]
    OA0POS = 5,
    #[doc = "6: Positive I/O pin OPA1"]
    OA1POS = 6,
    #[doc = "7: OPAMP0 Resistor Ladder Taps"]
    OA0TAP = 7,
    #[doc = "8: Resistor ladder 3 taps"]
    RES3TAP = 8,
}
impl From<MUXPOS_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXPOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MUXPOS` reader - Positive Input Mux Selection"]
pub type MUXPOS_R = crate::FieldReader<u8, MUXPOS_A>;
impl MUXPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUXPOS_A> {
        match self.bits {
            0 => Some(MUXPOS_A::OA2POS),
            1 => Some(MUXPOS_A::OA2TAP),
            2 => Some(MUXPOS_A::REFERENCE_DAC),
            3 => Some(MUXPOS_A::GND),
            4 => Some(MUXPOS_A::OA1OUT),
            5 => Some(MUXPOS_A::OA0POS),
            6 => Some(MUXPOS_A::OA1POS),
            7 => Some(MUXPOS_A::OA0TAP),
            8 => Some(MUXPOS_A::RES3TAP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OA2POS`"]
    #[inline(always)]
    pub fn is_oa2pos(&self) -> bool {
        *self == MUXPOS_A::OA2POS
    }
    #[doc = "Checks if the value of the field is `OA2TAP`"]
    #[inline(always)]
    pub fn is_oa2tap(&self) -> bool {
        *self == MUXPOS_A::OA2TAP
    }
    #[doc = "Checks if the value of the field is `REFERENCE_DAC`"]
    #[inline(always)]
    pub fn is_reference_dac(&self) -> bool {
        *self == MUXPOS_A::REFERENCE_DAC
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == MUXPOS_A::GND
    }
    #[doc = "Checks if the value of the field is `OA1OUT`"]
    #[inline(always)]
    pub fn is_oa1out(&self) -> bool {
        *self == MUXPOS_A::OA1OUT
    }
    #[doc = "Checks if the value of the field is `OA0POS`"]
    #[inline(always)]
    pub fn is_oa0pos(&self) -> bool {
        *self == MUXPOS_A::OA0POS
    }
    #[doc = "Checks if the value of the field is `OA1POS`"]
    #[inline(always)]
    pub fn is_oa1pos(&self) -> bool {
        *self == MUXPOS_A::OA1POS
    }
    #[doc = "Checks if the value of the field is `OA0TAP`"]
    #[inline(always)]
    pub fn is_oa0tap(&self) -> bool {
        *self == MUXPOS_A::OA0TAP
    }
    #[doc = "Checks if the value of the field is `RES3TAP`"]
    #[inline(always)]
    pub fn is_res3tap(&self) -> bool {
        *self == MUXPOS_A::RES3TAP
    }
}
#[doc = "Field `MUXPOS` writer - Positive Input Mux Selection"]
pub type MUXPOS_W<'a> =
    crate::FieldWriter<'a, u32, OPAMPCTRL2_SPEC, u8, MUXPOS_A, 4, 16>;
impl<'a> MUXPOS_W<'a> {
    #[doc = "Positive I/O pin"]
    #[inline(always)]
    pub fn oa2pos(self) -> &'a mut W {
        self.variant(MUXPOS_A::OA2POS)
    }
    #[doc = "Resistor ladder 2 taps"]
    #[inline(always)]
    pub fn oa2tap(self) -> &'a mut W {
        self.variant(MUXPOS_A::OA2TAP)
    }
    #[doc = "REFERENCE or DAC output"]
    #[inline(always)]
    pub fn reference_dac(self) -> &'a mut W {
        self.variant(MUXPOS_A::REFERENCE_DAC)
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(MUXPOS_A::GND)
    }
    #[doc = "OPAMP1 output"]
    #[inline(always)]
    pub fn oa1out(self) -> &'a mut W {
        self.variant(MUXPOS_A::OA1OUT)
    }
    #[doc = "Positive I/O pin OPA0"]
    #[inline(always)]
    pub fn oa0pos(self) -> &'a mut W {
        self.variant(MUXPOS_A::OA0POS)
    }
    #[doc = "Positive I/O pin OPA1"]
    #[inline(always)]
    pub fn oa1pos(self) -> &'a mut W {
        self.variant(MUXPOS_A::OA1POS)
    }
    #[doc = "OPAMP0 Resistor Ladder Taps"]
    #[inline(always)]
    pub fn oa0tap(self) -> &'a mut W {
        self.variant(MUXPOS_A::OA0TAP)
    }
    #[doc = "Resistor ladder 3 taps"]
    #[inline(always)]
    pub fn res3tap(self) -> &'a mut W {
        self.variant(MUXPOS_A::RES3TAP)
    }
}
#[doc = "Negative Input Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUXNEG_A {
    #[doc = "0: Negative I/O pin"]
    OA0NEG = 0,
    #[doc = "1: Resistor ladder 0 taps"]
    OA0TAP = 1,
    #[doc = "2: REFERENCE or DAC output"]
    REFERENCE_DAC = 2,
    #[doc = "3: OPAMP0 output"]
    OA0OUT = 3,
}
impl From<MUXNEG_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXNEG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MUXNEG` reader - Negative Input Mux Selection"]
pub type MUXNEG_R = crate::FieldReader<u8, MUXNEG_A>;
impl MUXNEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUXNEG_A> {
        match self.bits {
            0 => Some(MUXNEG_A::OA0NEG),
            1 => Some(MUXNEG_A::OA0TAP),
            2 => Some(MUXNEG_A::REFERENCE_DAC),
            3 => Some(MUXNEG_A::OA0OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OA0NEG`"]
    #[inline(always)]
    pub fn is_oa0neg(&self) -> bool {
        *self == MUXNEG_A::OA0NEG
    }
    #[doc = "Checks if the value of the field is `OA0TAP`"]
    #[inline(always)]
    pub fn is_oa0tap(&self) -> bool {
        *self == MUXNEG_A::OA0TAP
    }
    #[doc = "Checks if the value of the field is `REFERENCE_DAC`"]
    #[inline(always)]
    pub fn is_reference_dac(&self) -> bool {
        *self == MUXNEG_A::REFERENCE_DAC
    }
    #[doc = "Checks if the value of the field is `OA0OUT`"]
    #[inline(always)]
    pub fn is_oa0out(&self) -> bool {
        *self == MUXNEG_A::OA0OUT
    }
}
#[doc = "Field `MUXNEG` writer - Negative Input Mux Selection"]
pub type MUXNEG_W<'a> =
    crate::FieldWriter<'a, u32, OPAMPCTRL2_SPEC, u8, MUXNEG_A, 4, 20>;
impl<'a> MUXNEG_W<'a> {
    #[doc = "Negative I/O pin"]
    #[inline(always)]
    pub fn oa0neg(self) -> &'a mut W {
        self.variant(MUXNEG_A::OA0NEG)
    }
    #[doc = "Resistor ladder 0 taps"]
    #[inline(always)]
    pub fn oa0tap(self) -> &'a mut W {
        self.variant(MUXNEG_A::OA0TAP)
    }
    #[doc = "REFERENCE or DAC output"]
    #[inline(always)]
    pub fn reference_dac(self) -> &'a mut W {
        self.variant(MUXNEG_A::REFERENCE_DAC)
    }
    #[doc = "OPAMP0 output"]
    #[inline(always)]
    pub fn oa0out(self) -> &'a mut W {
        self.variant(MUXNEG_A::OA0OUT)
    }
}
impl R {
    #[doc = "Bit 1 - Operational Amplifier Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Output"]
    #[inline(always)]
    pub fn anaout(&self) -> ANAOUT_R {
        ANAOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Bias Selection"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Resistor ladder To VCC"]
    #[inline(always)]
    pub fn res2vcc(&self) -> RES2VCC_R {
        RES2VCC_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 8 - Resistor ladder To Output"]
    #[inline(always)]
    pub fn res2out(&self) -> RES2OUT_R {
        RES2OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Resistor 1 Enable"]
    #[inline(always)]
    pub fn res1en(&self) -> RES1EN_R {
        RES1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Resistor 1 Mux"]
    #[inline(always)]
    pub fn res1mux(&self) -> RES1MUX_R {
        RES1MUX_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Potentiometer Selection"]
    #[inline(always)]
    pub fn potmux(&self) -> POTMUX_R {
        POTMUX_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Operational Amplifier Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Analog Output"]
    #[inline(always)]
    pub fn anaout(&mut self) -> ANAOUT_W {
        ANAOUT_W::new(self)
    }
    #[doc = "Bits 3:4 - Bias Selection"]
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W {
        BIAS_W::new(self)
    }
    #[doc = "Bit 5 - Resistor ladder To VCC"]
    #[inline(always)]
    pub fn res2vcc(&mut self) -> RES2VCC_W {
        RES2VCC_W::new(self)
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
    #[doc = "Bit 8 - Resistor ladder To Output"]
    #[inline(always)]
    pub fn res2out(&mut self) -> RES2OUT_W {
        RES2OUT_W::new(self)
    }
    #[doc = "Bit 9 - Resistor 1 Enable"]
    #[inline(always)]
    pub fn res1en(&mut self) -> RES1EN_W {
        RES1EN_W::new(self)
    }
    #[doc = "Bits 10:12 - Resistor 1 Mux"]
    #[inline(always)]
    pub fn res1mux(&mut self) -> RES1MUX_W {
        RES1MUX_W::new(self)
    }
    #[doc = "Bits 13:15 - Potentiometer Selection"]
    #[inline(always)]
    pub fn potmux(&mut self) -> POTMUX_W {
        POTMUX_W::new(self)
    }
    #[doc = "Bits 16:19 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MUXPOS_W {
        MUXPOS_W::new(self)
    }
    #[doc = "Bits 20:23 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MUXNEG_W {
        MUXNEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP2 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opampctrl2](index.html) module"]
pub struct OPAMPCTRL2_SPEC;
impl crate::RegisterSpec for OPAMPCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opampctrl2::R](R) reader structure"]
impl crate::Readable for OPAMPCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opampctrl2::W](W) writer structure"]
impl crate::Writable for OPAMPCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPAMPCTRL2 to value 0"]
impl crate::Resettable for OPAMPCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
