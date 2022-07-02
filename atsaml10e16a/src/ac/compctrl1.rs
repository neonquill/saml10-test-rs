#[doc = "Register `COMPCTRL1` reader"]
pub struct R(crate::R<COMPCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPCTRL1` writer"]
pub struct W(crate::W<COMPCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPCTRL1_SPEC>;
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
impl From<crate::W<COMPCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u32, COMPCTRL1_SPEC, bool, 1>;
#[doc = "Field `SINGLE` reader - Single-Shot Mode"]
pub type SINGLE_R = crate::BitReader<bool>;
#[doc = "Field `SINGLE` writer - Single-Shot Mode"]
pub type SINGLE_W<'a> = crate::BitWriter<'a, u32, COMPCTRL1_SPEC, bool, 2>;
#[doc = "Interrupt Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTSEL_A {
    #[doc = "0: Interrupt on comparator output toggle"]
    TOGGLE = 0,
    #[doc = "1: Interrupt on comparator output rising"]
    RISING = 1,
    #[doc = "2: Interrupt on comparator output falling"]
    FALLING = 2,
    #[doc = "3: Interrupt on end of comparison (single-shot mode only)"]
    EOC = 3,
}
impl From<INTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTSEL` reader - Interrupt Selection"]
pub type INTSEL_R = crate::FieldReader<u8, INTSEL_A>;
impl INTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSEL_A {
        match self.bits {
            0 => INTSEL_A::TOGGLE,
            1 => INTSEL_A::RISING,
            2 => INTSEL_A::FALLING,
            3 => INTSEL_A::EOC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == INTSEL_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTSEL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTSEL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EOC`"]
    #[inline(always)]
    pub fn is_eoc(&self) -> bool {
        *self == INTSEL_A::EOC
    }
}
#[doc = "Field `INTSEL` writer - Interrupt Selection"]
pub type INTSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, COMPCTRL1_SPEC, u8, INTSEL_A, 2, 3>;
impl<'a> INTSEL_W<'a> {
    #[doc = "Interrupt on comparator output toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(INTSEL_A::TOGGLE)
    }
    #[doc = "Interrupt on comparator output rising"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTSEL_A::RISING)
    }
    #[doc = "Interrupt on comparator output falling"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTSEL_A::FALLING)
    }
    #[doc = "Interrupt on end of comparison (single-shot mode only)"]
    #[inline(always)]
    pub fn eoc(self) -> &'a mut W {
        self.variant(INTSEL_A::EOC)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a> = crate::BitWriter<'a, u32, COMPCTRL1_SPEC, bool, 6>;
#[doc = "Negative Input Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUXNEG_A {
    #[doc = "0: I/O pin 0"]
    PIN0 = 0,
    #[doc = "1: I/O pin 1"]
    PIN1 = 1,
    #[doc = "2: I/O pin 2"]
    PIN2 = 2,
    #[doc = "3: I/O pin 3"]
    PIN3 = 3,
    #[doc = "4: Ground"]
    GND = 4,
    #[doc = "5: VDD scaler"]
    VSCALE = 5,
    #[doc = "6: Internal bandgap voltage"]
    BANDGAP = 6,
    #[doc = "7: OPAMP output (on AC1)"]
    OPAMP = 7,
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
    pub fn variant(&self) -> MUXNEG_A {
        match self.bits {
            0 => MUXNEG_A::PIN0,
            1 => MUXNEG_A::PIN1,
            2 => MUXNEG_A::PIN2,
            3 => MUXNEG_A::PIN3,
            4 => MUXNEG_A::GND,
            5 => MUXNEG_A::VSCALE,
            6 => MUXNEG_A::BANDGAP,
            7 => MUXNEG_A::OPAMP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXNEG_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXNEG_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == MUXNEG_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == MUXNEG_A::PIN3
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == MUXNEG_A::GND
    }
    #[doc = "Checks if the value of the field is `VSCALE`"]
    #[inline(always)]
    pub fn is_vscale(&self) -> bool {
        *self == MUXNEG_A::VSCALE
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == MUXNEG_A::BANDGAP
    }
    #[doc = "Checks if the value of the field is `OPAMP`"]
    #[inline(always)]
    pub fn is_opamp(&self) -> bool {
        *self == MUXNEG_A::OPAMP
    }
}
#[doc = "Field `MUXNEG` writer - Negative Input Mux Selection"]
pub type MUXNEG_W<'a> =
    crate::FieldWriterSafe<'a, u32, COMPCTRL1_SPEC, u8, MUXNEG_A, 3, 8>;
impl<'a> MUXNEG_W<'a> {
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN0)
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN1)
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN2)
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXNEG_A::PIN3)
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(MUXNEG_A::GND)
    }
    #[doc = "VDD scaler"]
    #[inline(always)]
    pub fn vscale(self) -> &'a mut W {
        self.variant(MUXNEG_A::VSCALE)
    }
    #[doc = "Internal bandgap voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(MUXNEG_A::BANDGAP)
    }
    #[doc = "OPAMP output (on AC1)"]
    #[inline(always)]
    pub fn opamp(self) -> &'a mut W {
        self.variant(MUXNEG_A::OPAMP)
    }
}
#[doc = "Positive Input Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUXPOS_A {
    #[doc = "0: I/O pin 0"]
    PIN0 = 0,
    #[doc = "1: I/O pin 1"]
    PIN1 = 1,
    #[doc = "2: I/O pin 2"]
    PIN2 = 2,
    #[doc = "3: I/O pin 3"]
    PIN3 = 3,
    #[doc = "4: VDD Scaler"]
    VSCALE = 4,
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
            0 => Some(MUXPOS_A::PIN0),
            1 => Some(MUXPOS_A::PIN1),
            2 => Some(MUXPOS_A::PIN2),
            3 => Some(MUXPOS_A::PIN3),
            4 => Some(MUXPOS_A::VSCALE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == MUXPOS_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == MUXPOS_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == MUXPOS_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == MUXPOS_A::PIN3
    }
    #[doc = "Checks if the value of the field is `VSCALE`"]
    #[inline(always)]
    pub fn is_vscale(&self) -> bool {
        *self == MUXPOS_A::VSCALE
    }
}
#[doc = "Field `MUXPOS` writer - Positive Input Mux Selection"]
pub type MUXPOS_W<'a> =
    crate::FieldWriter<'a, u32, COMPCTRL1_SPEC, u8, MUXPOS_A, 3, 12>;
impl<'a> MUXPOS_W<'a> {
    #[doc = "I/O pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN0)
    }
    #[doc = "I/O pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN1)
    }
    #[doc = "I/O pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN2)
    }
    #[doc = "I/O pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(MUXPOS_A::PIN3)
    }
    #[doc = "VDD Scaler"]
    #[inline(always)]
    pub fn vscale(self) -> &'a mut W {
        self.variant(MUXPOS_A::VSCALE)
    }
}
#[doc = "Field `SWAP` reader - Swap Inputs and Invert"]
pub type SWAP_R = crate::BitReader<bool>;
#[doc = "Field `SWAP` writer - Swap Inputs and Invert"]
pub type SWAP_W<'a> = crate::BitWriter<'a, u32, COMPCTRL1_SPEC, bool, 15>;
#[doc = "Speed Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: Low speed"]
    LOW = 0,
    #[doc = "1: Medium low speed"]
    MEDLOW = 1,
    #[doc = "2: Medium high speed"]
    MEDHIGH = 2,
    #[doc = "3: High speed"]
    HIGH = 3,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPEED` reader - Speed Selection"]
pub type SPEED_R = crate::FieldReader<u8, SPEED_A>;
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPEED_A {
        match self.bits {
            0 => SPEED_A::LOW,
            1 => SPEED_A::MEDLOW,
            2 => SPEED_A::MEDHIGH,
            3 => SPEED_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPEED_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDLOW`"]
    #[inline(always)]
    pub fn is_medlow(&self) -> bool {
        *self == SPEED_A::MEDLOW
    }
    #[doc = "Checks if the value of the field is `MEDHIGH`"]
    #[inline(always)]
    pub fn is_medhigh(&self) -> bool {
        *self == SPEED_A::MEDHIGH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPEED_A::HIGH
    }
}
#[doc = "Field `SPEED` writer - Speed Selection"]
pub type SPEED_W<'a> =
    crate::FieldWriterSafe<'a, u32, COMPCTRL1_SPEC, u8, SPEED_A, 2, 16>;
impl<'a> SPEED_W<'a> {
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPEED_A::LOW)
    }
    #[doc = "Medium low speed"]
    #[inline(always)]
    pub fn medlow(self) -> &'a mut W {
        self.variant(SPEED_A::MEDLOW)
    }
    #[doc = "Medium high speed"]
    #[inline(always)]
    pub fn medhigh(self) -> &'a mut W {
        self.variant(SPEED_A::MEDHIGH)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPEED_A::HIGH)
    }
}
#[doc = "Field `HYSTEN` reader - Hysteresis Enable"]
pub type HYSTEN_R = crate::BitReader<bool>;
#[doc = "Field `HYSTEN` writer - Hysteresis Enable"]
pub type HYSTEN_W<'a> = crate::BitWriter<'a, u32, COMPCTRL1_SPEC, bool, 19>;
#[doc = "Hysteresis Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HYST_A {
    #[doc = "0: 50mV"]
    HYST50 = 0,
    #[doc = "1: 70mV"]
    HYST70 = 1,
    #[doc = "2: 90mV"]
    HYST90 = 2,
    #[doc = "3: 110mV"]
    HYST110 = 3,
}
impl From<HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HYST` reader - Hysteresis Level"]
pub type HYST_R = crate::FieldReader<u8, HYST_A>;
impl HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            0 => HYST_A::HYST50,
            1 => HYST_A::HYST70,
            2 => HYST_A::HYST90,
            3 => HYST_A::HYST110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYST50`"]
    #[inline(always)]
    pub fn is_hyst50(&self) -> bool {
        *self == HYST_A::HYST50
    }
    #[doc = "Checks if the value of the field is `HYST70`"]
    #[inline(always)]
    pub fn is_hyst70(&self) -> bool {
        *self == HYST_A::HYST70
    }
    #[doc = "Checks if the value of the field is `HYST90`"]
    #[inline(always)]
    pub fn is_hyst90(&self) -> bool {
        *self == HYST_A::HYST90
    }
    #[doc = "Checks if the value of the field is `HYST110`"]
    #[inline(always)]
    pub fn is_hyst110(&self) -> bool {
        *self == HYST_A::HYST110
    }
}
#[doc = "Field `HYST` writer - Hysteresis Level"]
pub type HYST_W<'a> =
    crate::FieldWriterSafe<'a, u32, COMPCTRL1_SPEC, u8, HYST_A, 2, 20>;
impl<'a> HYST_W<'a> {
    #[doc = "50mV"]
    #[inline(always)]
    pub fn hyst50(self) -> &'a mut W {
        self.variant(HYST_A::HYST50)
    }
    #[doc = "70mV"]
    #[inline(always)]
    pub fn hyst70(self) -> &'a mut W {
        self.variant(HYST_A::HYST70)
    }
    #[doc = "90mV"]
    #[inline(always)]
    pub fn hyst90(self) -> &'a mut W {
        self.variant(HYST_A::HYST90)
    }
    #[doc = "110mV"]
    #[inline(always)]
    pub fn hyst110(self) -> &'a mut W {
        self.variant(HYST_A::HYST110)
    }
}
#[doc = "Filter Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEN_A {
    #[doc = "0: No filtering"]
    OFF = 0,
    #[doc = "1: 3-bit majority function (2 of 3)"]
    MAJ3 = 1,
    #[doc = "2: 5-bit majority function (3 of 5)"]
    MAJ5 = 2,
}
impl From<FLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLEN` reader - Filter Length"]
pub type FLEN_R = crate::FieldReader<u8, FLEN_A>;
impl FLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLEN_A> {
        match self.bits {
            0 => Some(FLEN_A::OFF),
            1 => Some(FLEN_A::MAJ3),
            2 => Some(FLEN_A::MAJ5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FLEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `MAJ3`"]
    #[inline(always)]
    pub fn is_maj3(&self) -> bool {
        *self == FLEN_A::MAJ3
    }
    #[doc = "Checks if the value of the field is `MAJ5`"]
    #[inline(always)]
    pub fn is_maj5(&self) -> bool {
        *self == FLEN_A::MAJ5
    }
}
#[doc = "Field `FLEN` writer - Filter Length"]
pub type FLEN_W<'a> =
    crate::FieldWriter<'a, u32, COMPCTRL1_SPEC, u8, FLEN_A, 3, 24>;
impl<'a> FLEN_W<'a> {
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FLEN_A::OFF)
    }
    #[doc = "3-bit majority function (2 of 3)"]
    #[inline(always)]
    pub fn maj3(self) -> &'a mut W {
        self.variant(FLEN_A::MAJ3)
    }
    #[doc = "5-bit majority function (3 of 5)"]
    #[inline(always)]
    pub fn maj5(self) -> &'a mut W {
        self.variant(FLEN_A::MAJ5)
    }
}
#[doc = "Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUT_A {
    #[doc = "0: The output of COMPn is not routed to the COMPn I/O port"]
    OFF = 0,
    #[doc = "1: The asynchronous output of COMPn is routed to the COMPn I/O port"]
    ASYNC = 1,
    #[doc = "2: The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    SYNC = 2,
}
impl From<OUT_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OUT` reader - Output"]
pub type OUT_R = crate::FieldReader<u8, OUT_A>;
impl OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUT_A> {
        match self.bits {
            0 => Some(OUT_A::OFF),
            1 => Some(OUT_A::ASYNC),
            2 => Some(OUT_A::SYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == OUT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == OUT_A::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == OUT_A::SYNC
    }
}
#[doc = "Field `OUT` writer - Output"]
pub type OUT_W<'a> =
    crate::FieldWriter<'a, u32, COMPCTRL1_SPEC, u8, OUT_A, 2, 28>;
impl<'a> OUT_W<'a> {
    #[doc = "The output of COMPn is not routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(OUT_A::OFF)
    }
    #[doc = "The asynchronous output of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(OUT_A::ASYNC)
    }
    #[doc = "The synchronous output (including filtering) of COMPn is routed to the COMPn I/O port"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(OUT_A::SYNC)
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single-Shot Mode"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&self) -> INTSEL_R {
        INTSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Speed Selection"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&self) -> HYSTEN_R {
        HYSTEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Hysteresis Level"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline(always)]
    pub fn flen(&self) -> FLEN_R {
        FLEN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Output"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Single-Shot Mode"]
    #[inline(always)]
    pub fn single(&mut self) -> SINGLE_W {
        SINGLE_W::new(self)
    }
    #[doc = "Bits 3:4 - Interrupt Selection"]
    #[inline(always)]
    pub fn intsel(&mut self) -> INTSEL_W {
        INTSEL_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bits 8:10 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MUXNEG_W {
        MUXNEG_W::new(self)
    }
    #[doc = "Bits 12:14 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MUXPOS_W {
        MUXPOS_W::new(self)
    }
    #[doc = "Bit 15 - Swap Inputs and Invert"]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W {
        SWAP_W::new(self)
    }
    #[doc = "Bits 16:17 - Speed Selection"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W::new(self)
    }
    #[doc = "Bit 19 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&mut self) -> HYSTEN_W {
        HYSTEN_W::new(self)
    }
    #[doc = "Bits 20:21 - Hysteresis Level"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W::new(self)
    }
    #[doc = "Bits 24:26 - Filter Length"]
    #[inline(always)]
    pub fn flen(&mut self) -> FLEN_W {
        FLEN_W::new(self)
    }
    #[doc = "Bits 28:29 - Output"]
    #[inline(always)]
    pub fn out(&mut self) -> OUT_W {
        OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compctrl1](index.html) module"]
pub struct COMPCTRL1_SPEC;
impl crate::RegisterSpec for COMPCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [compctrl1::R](R) reader structure"]
impl crate::Readable for COMPCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compctrl1::W](W) writer structure"]
impl crate::Writable for COMPCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMPCTRL1 to value 0"]
impl crate::Resettable for COMPCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
