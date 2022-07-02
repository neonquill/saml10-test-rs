#[doc = "Register `RESCTRL` reader"]
pub struct R(crate::R<RESCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESCTRL` writer"]
pub struct W(crate::W<RESCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESCTRL_SPEC>;
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
impl From<crate::W<RESCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RES2OUT` reader - Resistor ladder To Output"]
pub type RES2OUT_R = crate::BitReader<bool>;
#[doc = "Field `RES2OUT` writer - Resistor ladder To Output"]
pub type RES2OUT_W<'a> = crate::BitWriter<'a, u8, RESCTRL_SPEC, bool, 0>;
#[doc = "Field `RES1EN` reader - Resistor 1 Enable"]
pub type RES1EN_R = crate::BitReader<bool>;
#[doc = "Field `RES1EN` writer - Resistor 1 Enable"]
pub type RES1EN_W<'a> = crate::BitWriter<'a, u8, RESCTRL_SPEC, bool, 1>;
#[doc = "Resistor 1 Mux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RES1MUX_A {
    #[doc = "0: DAC output"]
    DAC = 0,
    #[doc = "1: REFERENCE output"]
    REFBUF = 1,
}
impl From<RES1MUX_A> for bool {
    #[inline(always)]
    fn from(variant: RES1MUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RES1MUX` reader - Resistor 1 Mux"]
pub type RES1MUX_R = crate::BitReader<RES1MUX_A>;
impl RES1MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES1MUX_A {
        match self.bits {
            false => RES1MUX_A::DAC,
            true => RES1MUX_A::REFBUF,
        }
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == RES1MUX_A::DAC
    }
    #[doc = "Checks if the value of the field is `REFBUF`"]
    #[inline(always)]
    pub fn is_refbuf(&self) -> bool {
        *self == RES1MUX_A::REFBUF
    }
}
#[doc = "Field `RES1MUX` writer - Resistor 1 Mux"]
pub type RES1MUX_W<'a> = crate::BitWriter<'a, u8, RESCTRL_SPEC, RES1MUX_A, 2>;
impl<'a> RES1MUX_W<'a> {
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(RES1MUX_A::DAC)
    }
    #[doc = "REFERENCE output"]
    #[inline(always)]
    pub fn refbuf(self) -> &'a mut W {
        self.variant(RES1MUX_A::REFBUF)
    }
}
#[doc = "Potentiometer Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POTMUX_A {
    #[doc = "0: Gain = 0.14"]
    _14R_2R = 0,
    #[doc = "1: Gain = 0.33"]
    _12R_4R = 1,
    #[doc = "2: Gain = 1"]
    _8R_8R = 2,
    #[doc = "3: Gain = 1.67"]
    _6R_10R = 3,
    #[doc = "4: Gain = 3"]
    _4R_12R = 4,
    #[doc = "5: Gain = 4.33"]
    _3R_13R = 5,
    #[doc = "6: Gain = 7"]
    _2R_14R = 6,
    #[doc = "7: Gain = 15"]
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
    crate::FieldWriterSafe<'a, u8, RESCTRL_SPEC, u8, POTMUX_A, 3, 3>;
impl<'a> POTMUX_W<'a> {
    #[doc = "Gain = 0.14"]
    #[inline(always)]
    pub fn _14r_2r(self) -> &'a mut W {
        self.variant(POTMUX_A::_14R_2R)
    }
    #[doc = "Gain = 0.33"]
    #[inline(always)]
    pub fn _12r_4r(self) -> &'a mut W {
        self.variant(POTMUX_A::_12R_4R)
    }
    #[doc = "Gain = 1"]
    #[inline(always)]
    pub fn _8r_8r(self) -> &'a mut W {
        self.variant(POTMUX_A::_8R_8R)
    }
    #[doc = "Gain = 1.67"]
    #[inline(always)]
    pub fn _6r_10r(self) -> &'a mut W {
        self.variant(POTMUX_A::_6R_10R)
    }
    #[doc = "Gain = 3"]
    #[inline(always)]
    pub fn _4r_12r(self) -> &'a mut W {
        self.variant(POTMUX_A::_4R_12R)
    }
    #[doc = "Gain = 4.33"]
    #[inline(always)]
    pub fn _3r_13r(self) -> &'a mut W {
        self.variant(POTMUX_A::_3R_13R)
    }
    #[doc = "Gain = 7"]
    #[inline(always)]
    pub fn _2r_14r(self) -> &'a mut W {
        self.variant(POTMUX_A::_2R_14R)
    }
    #[doc = "Gain = 15"]
    #[inline(always)]
    pub fn r_15r(self) -> &'a mut W {
        self.variant(POTMUX_A::R_15R)
    }
}
#[doc = "Reference output voltage level select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFBUFLEVEL_A {
    #[doc = "0: 1.1v"]
    _1_1V = 0,
    #[doc = "1: 1.25v"]
    _1_25V = 1,
    #[doc = "2: 1.6v"]
    _1_6V = 2,
}
impl From<REFBUFLEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFBUFLEVEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFBUFLEVEL` reader - Reference output voltage level select"]
pub type REFBUFLEVEL_R = crate::FieldReader<u8, REFBUFLEVEL_A>;
impl REFBUFLEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFBUFLEVEL_A> {
        match self.bits {
            0 => Some(REFBUFLEVEL_A::_1_1V),
            1 => Some(REFBUFLEVEL_A::_1_25V),
            2 => Some(REFBUFLEVEL_A::_1_6V),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1_1V`"]
    #[inline(always)]
    pub fn is_1_1v(&self) -> bool {
        *self == REFBUFLEVEL_A::_1_1V
    }
    #[doc = "Checks if the value of the field is `_1_25V`"]
    #[inline(always)]
    pub fn is_1_25v(&self) -> bool {
        *self == REFBUFLEVEL_A::_1_25V
    }
    #[doc = "Checks if the value of the field is `_1_6V`"]
    #[inline(always)]
    pub fn is_1_6v(&self) -> bool {
        *self == REFBUFLEVEL_A::_1_6V
    }
}
#[doc = "Field `REFBUFLEVEL` writer - Reference output voltage level select"]
pub type REFBUFLEVEL_W<'a> =
    crate::FieldWriter<'a, u8, RESCTRL_SPEC, u8, REFBUFLEVEL_A, 2, 6>;
impl<'a> REFBUFLEVEL_W<'a> {
    #[doc = "1.1v"]
    #[inline(always)]
    pub fn _1_1v(self) -> &'a mut W {
        self.variant(REFBUFLEVEL_A::_1_1V)
    }
    #[doc = "1.25v"]
    #[inline(always)]
    pub fn _1_25v(self) -> &'a mut W {
        self.variant(REFBUFLEVEL_A::_1_25V)
    }
    #[doc = "1.6v"]
    #[inline(always)]
    pub fn _1_6v(self) -> &'a mut W {
        self.variant(REFBUFLEVEL_A::_1_6V)
    }
}
impl R {
    #[doc = "Bit 0 - Resistor ladder To Output"]
    #[inline(always)]
    pub fn res2out(&self) -> RES2OUT_R {
        RES2OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resistor 1 Enable"]
    #[inline(always)]
    pub fn res1en(&self) -> RES1EN_R {
        RES1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Resistor 1 Mux"]
    #[inline(always)]
    pub fn res1mux(&self) -> RES1MUX_R {
        RES1MUX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Potentiometer Selection"]
    #[inline(always)]
    pub fn potmux(&self) -> POTMUX_R {
        POTMUX_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Reference output voltage level select"]
    #[inline(always)]
    pub fn refbuflevel(&self) -> REFBUFLEVEL_R {
        REFBUFLEVEL_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Resistor ladder To Output"]
    #[inline(always)]
    pub fn res2out(&mut self) -> RES2OUT_W {
        RES2OUT_W::new(self)
    }
    #[doc = "Bit 1 - Resistor 1 Enable"]
    #[inline(always)]
    pub fn res1en(&mut self) -> RES1EN_W {
        RES1EN_W::new(self)
    }
    #[doc = "Bit 2 - Resistor 1 Mux"]
    #[inline(always)]
    pub fn res1mux(&mut self) -> RES1MUX_W {
        RES1MUX_W::new(self)
    }
    #[doc = "Bits 3:5 - Potentiometer Selection"]
    #[inline(always)]
    pub fn potmux(&mut self) -> POTMUX_W {
        POTMUX_W::new(self)
    }
    #[doc = "Bits 6:7 - Reference output voltage level select"]
    #[inline(always)]
    pub fn refbuflevel(&mut self) -> REFBUFLEVEL_W {
        REFBUFLEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Resister Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resctrl](index.html) module"]
pub struct RESCTRL_SPEC;
impl crate::RegisterSpec for RESCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [resctrl::R](R) reader structure"]
impl crate::Readable for RESCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resctrl::W](W) writer structure"]
impl crate::Writable for RESCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESCTRL to value 0"]
impl crate::Resettable for RESCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
