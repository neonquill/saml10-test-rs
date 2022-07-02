#[doc = "Register `CHCTRLB` reader"]
pub struct R(crate::R<CHCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTRLB` writer"]
pub struct W(crate::W<CHCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTRLB_SPEC>;
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
impl From<crate::W<CHCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Event Input Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVACT_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: Transfer and periodic transfer trigger"]
    TRIG = 1,
    #[doc = "2: Conditional transfer trigger"]
    CTRIG = 2,
    #[doc = "3: Conditional block transfer"]
    CBLOCK = 3,
    #[doc = "4: Channel suspend operation"]
    SUSPEND = 4,
    #[doc = "5: Channel resume operation"]
    RESUME = 5,
    #[doc = "6: Skip next block suspend action"]
    SSKIP = 6,
}
impl From<EVACT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EVACT` reader - Event Input Action"]
pub type EVACT_R = crate::FieldReader<u8, EVACT_A>;
impl EVACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EVACT_A> {
        match self.bits {
            0 => Some(EVACT_A::NOACT),
            1 => Some(EVACT_A::TRIG),
            2 => Some(EVACT_A::CTRIG),
            3 => Some(EVACT_A::CBLOCK),
            4 => Some(EVACT_A::SUSPEND),
            5 => Some(EVACT_A::RESUME),
            6 => Some(EVACT_A::SSKIP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == EVACT_A::NOACT
    }
    #[doc = "Checks if the value of the field is `TRIG`"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == EVACT_A::TRIG
    }
    #[doc = "Checks if the value of the field is `CTRIG`"]
    #[inline(always)]
    pub fn is_ctrig(&self) -> bool {
        *self == EVACT_A::CTRIG
    }
    #[doc = "Checks if the value of the field is `CBLOCK`"]
    #[inline(always)]
    pub fn is_cblock(&self) -> bool {
        *self == EVACT_A::CBLOCK
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == EVACT_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == EVACT_A::RESUME
    }
    #[doc = "Checks if the value of the field is `SSKIP`"]
    #[inline(always)]
    pub fn is_sskip(&self) -> bool {
        *self == EVACT_A::SSKIP
    }
}
#[doc = "Field `EVACT` writer - Event Input Action"]
pub type EVACT_W<'a> =
    crate::FieldWriter<'a, u32, CHCTRLB_SPEC, u8, EVACT_A, 3, 0>;
impl<'a> EVACT_W<'a> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut W {
        self.variant(EVACT_A::NOACT)
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut W {
        self.variant(EVACT_A::TRIG)
    }
    #[doc = "Conditional transfer trigger"]
    #[inline(always)]
    pub fn ctrig(self) -> &'a mut W {
        self.variant(EVACT_A::CTRIG)
    }
    #[doc = "Conditional block transfer"]
    #[inline(always)]
    pub fn cblock(self) -> &'a mut W {
        self.variant(EVACT_A::CBLOCK)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(EVACT_A::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(EVACT_A::RESUME)
    }
    #[doc = "Skip next block suspend action"]
    #[inline(always)]
    pub fn sskip(self) -> &'a mut W {
        self.variant(EVACT_A::SSKIP)
    }
}
#[doc = "Field `EVIE` reader - Channel Event Input Enable"]
pub type EVIE_R = crate::BitReader<bool>;
#[doc = "Field `EVIE` writer - Channel Event Input Enable"]
pub type EVIE_W<'a> = crate::BitWriter<'a, u32, CHCTRLB_SPEC, bool, 3>;
#[doc = "Field `EVOE` reader - Channel Event Output Enable"]
pub type EVOE_R = crate::BitReader<bool>;
#[doc = "Field `EVOE` writer - Channel Event Output Enable"]
pub type EVOE_W<'a> = crate::BitWriter<'a, u32, CHCTRLB_SPEC, bool, 4>;
#[doc = "Field `LVL` reader - Channel Arbitration Level"]
pub type LVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVL` writer - Channel Arbitration Level"]
pub type LVL_W<'a> = crate::FieldWriter<'a, u32, CHCTRLB_SPEC, u8, u8, 2, 5>;
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSRC_A {
    #[doc = "0: Only software/event triggers"]
    DISABLE = 0,
    #[doc = "1: RTC Timestamp Trigger"]
    RTC_TIMESTAMP = 1,
    #[doc = "2: ID for DCC0 register"]
    DSU_DCC0 = 2,
    #[doc = "3: ID for DCC1 register"]
    DSU_DCC1 = 3,
    #[doc = "4: SERCOM0 RX Trigger"]
    SERCOM0_RX = 4,
    #[doc = "5: SERCOM0 TX Trigger"]
    SERCOM0_TX = 5,
    #[doc = "6: SERCOM1 RX Trigger"]
    SERCOM1_RX = 6,
    #[doc = "7: SERCOM1 TX Trigger"]
    SERCOM1_TX = 7,
    #[doc = "8: SERCOM2 RX Trigger"]
    SERCOM2_RX = 8,
    #[doc = "9: SERCOM2 TX Trigger"]
    SERCOM2_TX = 9,
    #[doc = "10: TC0 Overflow Trigger"]
    TC0_OVF = 10,
    #[doc = "11: TC0 Match/Compare 0 Trigger"]
    TC0_MC0 = 11,
    #[doc = "12: TC0 Match/Compare 1 Trigger"]
    TC0_MC1 = 12,
    #[doc = "13: TC1 Overflow Trigger"]
    TC1_OVF = 13,
    #[doc = "14: TC1 Match/Compare 0 Trigger"]
    TC1_MC0 = 14,
    #[doc = "15: TC1 Match/Compare 1 Trigger"]
    TC1_MC1 = 15,
    #[doc = "16: TC2 Overflow Trigger"]
    TC2_OVF = 16,
    #[doc = "17: TC2 Match/Compare 0 Trigger"]
    TC2_MC0 = 17,
    #[doc = "18: TC2 Match/Compare 1 Trigger"]
    TC2_MC1 = 18,
    #[doc = "19: ADC Result Ready Trigger"]
    ADC_RESRDY = 19,
    #[doc = "20: DAC Empty Trigger"]
    DAC_EMPTY = 20,
    #[doc = "21: PTC End of Conversion Trigger"]
    PTC_EOC = 21,
    #[doc = "22: PTC Sequence Trigger"]
    PTC_SEQ = 22,
    #[doc = "23: PTC Window Compare Trigger"]
    PTC_WCOMP = 23,
}
impl From<TRIGSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGSRC` reader - Trigger Source"]
pub type TRIGSRC_R = crate::FieldReader<u8, TRIGSRC_A>;
impl TRIGSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGSRC_A> {
        match self.bits {
            0 => Some(TRIGSRC_A::DISABLE),
            1 => Some(TRIGSRC_A::RTC_TIMESTAMP),
            2 => Some(TRIGSRC_A::DSU_DCC0),
            3 => Some(TRIGSRC_A::DSU_DCC1),
            4 => Some(TRIGSRC_A::SERCOM0_RX),
            5 => Some(TRIGSRC_A::SERCOM0_TX),
            6 => Some(TRIGSRC_A::SERCOM1_RX),
            7 => Some(TRIGSRC_A::SERCOM1_TX),
            8 => Some(TRIGSRC_A::SERCOM2_RX),
            9 => Some(TRIGSRC_A::SERCOM2_TX),
            10 => Some(TRIGSRC_A::TC0_OVF),
            11 => Some(TRIGSRC_A::TC0_MC0),
            12 => Some(TRIGSRC_A::TC0_MC1),
            13 => Some(TRIGSRC_A::TC1_OVF),
            14 => Some(TRIGSRC_A::TC1_MC0),
            15 => Some(TRIGSRC_A::TC1_MC1),
            16 => Some(TRIGSRC_A::TC2_OVF),
            17 => Some(TRIGSRC_A::TC2_MC0),
            18 => Some(TRIGSRC_A::TC2_MC1),
            19 => Some(TRIGSRC_A::ADC_RESRDY),
            20 => Some(TRIGSRC_A::DAC_EMPTY),
            21 => Some(TRIGSRC_A::PTC_EOC),
            22 => Some(TRIGSRC_A::PTC_SEQ),
            23 => Some(TRIGSRC_A::PTC_WCOMP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TRIGSRC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RTC_TIMESTAMP`"]
    #[inline(always)]
    pub fn is_rtc_timestamp(&self) -> bool {
        *self == TRIGSRC_A::RTC_TIMESTAMP
    }
    #[doc = "Checks if the value of the field is `DSU_DCC0`"]
    #[inline(always)]
    pub fn is_dsu_dcc0(&self) -> bool {
        *self == TRIGSRC_A::DSU_DCC0
    }
    #[doc = "Checks if the value of the field is `DSU_DCC1`"]
    #[inline(always)]
    pub fn is_dsu_dcc1(&self) -> bool {
        *self == TRIGSRC_A::DSU_DCC1
    }
    #[doc = "Checks if the value of the field is `SERCOM0_RX`"]
    #[inline(always)]
    pub fn is_sercom0_rx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM0_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM0_TX`"]
    #[inline(always)]
    pub fn is_sercom0_tx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM0_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM1_RX`"]
    #[inline(always)]
    pub fn is_sercom1_rx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM1_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM1_TX`"]
    #[inline(always)]
    pub fn is_sercom1_tx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM1_TX
    }
    #[doc = "Checks if the value of the field is `SERCOM2_RX`"]
    #[inline(always)]
    pub fn is_sercom2_rx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM2_RX
    }
    #[doc = "Checks if the value of the field is `SERCOM2_TX`"]
    #[inline(always)]
    pub fn is_sercom2_tx(&self) -> bool {
        *self == TRIGSRC_A::SERCOM2_TX
    }
    #[doc = "Checks if the value of the field is `TC0_OVF`"]
    #[inline(always)]
    pub fn is_tc0_ovf(&self) -> bool {
        *self == TRIGSRC_A::TC0_OVF
    }
    #[doc = "Checks if the value of the field is `TC0_MC0`"]
    #[inline(always)]
    pub fn is_tc0_mc0(&self) -> bool {
        *self == TRIGSRC_A::TC0_MC0
    }
    #[doc = "Checks if the value of the field is `TC0_MC1`"]
    #[inline(always)]
    pub fn is_tc0_mc1(&self) -> bool {
        *self == TRIGSRC_A::TC0_MC1
    }
    #[doc = "Checks if the value of the field is `TC1_OVF`"]
    #[inline(always)]
    pub fn is_tc1_ovf(&self) -> bool {
        *self == TRIGSRC_A::TC1_OVF
    }
    #[doc = "Checks if the value of the field is `TC1_MC0`"]
    #[inline(always)]
    pub fn is_tc1_mc0(&self) -> bool {
        *self == TRIGSRC_A::TC1_MC0
    }
    #[doc = "Checks if the value of the field is `TC1_MC1`"]
    #[inline(always)]
    pub fn is_tc1_mc1(&self) -> bool {
        *self == TRIGSRC_A::TC1_MC1
    }
    #[doc = "Checks if the value of the field is `TC2_OVF`"]
    #[inline(always)]
    pub fn is_tc2_ovf(&self) -> bool {
        *self == TRIGSRC_A::TC2_OVF
    }
    #[doc = "Checks if the value of the field is `TC2_MC0`"]
    #[inline(always)]
    pub fn is_tc2_mc0(&self) -> bool {
        *self == TRIGSRC_A::TC2_MC0
    }
    #[doc = "Checks if the value of the field is `TC2_MC1`"]
    #[inline(always)]
    pub fn is_tc2_mc1(&self) -> bool {
        *self == TRIGSRC_A::TC2_MC1
    }
    #[doc = "Checks if the value of the field is `ADC_RESRDY`"]
    #[inline(always)]
    pub fn is_adc_resrdy(&self) -> bool {
        *self == TRIGSRC_A::ADC_RESRDY
    }
    #[doc = "Checks if the value of the field is `DAC_EMPTY`"]
    #[inline(always)]
    pub fn is_dac_empty(&self) -> bool {
        *self == TRIGSRC_A::DAC_EMPTY
    }
    #[doc = "Checks if the value of the field is `PTC_EOC`"]
    #[inline(always)]
    pub fn is_ptc_eoc(&self) -> bool {
        *self == TRIGSRC_A::PTC_EOC
    }
    #[doc = "Checks if the value of the field is `PTC_SEQ`"]
    #[inline(always)]
    pub fn is_ptc_seq(&self) -> bool {
        *self == TRIGSRC_A::PTC_SEQ
    }
    #[doc = "Checks if the value of the field is `PTC_WCOMP`"]
    #[inline(always)]
    pub fn is_ptc_wcomp(&self) -> bool {
        *self == TRIGSRC_A::PTC_WCOMP
    }
}
#[doc = "Field `TRIGSRC` writer - Trigger Source"]
pub type TRIGSRC_W<'a> =
    crate::FieldWriter<'a, u32, CHCTRLB_SPEC, u8, TRIGSRC_A, 5, 8>;
impl<'a> TRIGSRC_W<'a> {
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DISABLE)
    }
    #[doc = "RTC Timestamp Trigger"]
    #[inline(always)]
    pub fn rtc_timestamp(self) -> &'a mut W {
        self.variant(TRIGSRC_A::RTC_TIMESTAMP)
    }
    #[doc = "ID for DCC0 register"]
    #[inline(always)]
    pub fn dsu_dcc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DSU_DCC0)
    }
    #[doc = "ID for DCC1 register"]
    #[inline(always)]
    pub fn dsu_dcc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DSU_DCC1)
    }
    #[doc = "SERCOM0 RX Trigger"]
    #[inline(always)]
    pub fn sercom0_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM0_RX)
    }
    #[doc = "SERCOM0 TX Trigger"]
    #[inline(always)]
    pub fn sercom0_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM0_TX)
    }
    #[doc = "SERCOM1 RX Trigger"]
    #[inline(always)]
    pub fn sercom1_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM1_RX)
    }
    #[doc = "SERCOM1 TX Trigger"]
    #[inline(always)]
    pub fn sercom1_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM1_TX)
    }
    #[doc = "SERCOM2 RX Trigger"]
    #[inline(always)]
    pub fn sercom2_rx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM2_RX)
    }
    #[doc = "SERCOM2 TX Trigger"]
    #[inline(always)]
    pub fn sercom2_tx(self) -> &'a mut W {
        self.variant(TRIGSRC_A::SERCOM2_TX)
    }
    #[doc = "TC0 Overflow Trigger"]
    #[inline(always)]
    pub fn tc0_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC0_OVF)
    }
    #[doc = "TC0 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc0_mc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC0_MC0)
    }
    #[doc = "TC0 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc0_mc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC0_MC1)
    }
    #[doc = "TC1 Overflow Trigger"]
    #[inline(always)]
    pub fn tc1_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC1_OVF)
    }
    #[doc = "TC1 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc1_mc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC1_MC0)
    }
    #[doc = "TC1 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc1_mc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC1_MC1)
    }
    #[doc = "TC2 Overflow Trigger"]
    #[inline(always)]
    pub fn tc2_ovf(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC2_OVF)
    }
    #[doc = "TC2 Match/Compare 0 Trigger"]
    #[inline(always)]
    pub fn tc2_mc0(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC2_MC0)
    }
    #[doc = "TC2 Match/Compare 1 Trigger"]
    #[inline(always)]
    pub fn tc2_mc1(self) -> &'a mut W {
        self.variant(TRIGSRC_A::TC2_MC1)
    }
    #[doc = "ADC Result Ready Trigger"]
    #[inline(always)]
    pub fn adc_resrdy(self) -> &'a mut W {
        self.variant(TRIGSRC_A::ADC_RESRDY)
    }
    #[doc = "DAC Empty Trigger"]
    #[inline(always)]
    pub fn dac_empty(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DAC_EMPTY)
    }
    #[doc = "PTC End of Conversion Trigger"]
    #[inline(always)]
    pub fn ptc_eoc(self) -> &'a mut W {
        self.variant(TRIGSRC_A::PTC_EOC)
    }
    #[doc = "PTC Sequence Trigger"]
    #[inline(always)]
    pub fn ptc_seq(self) -> &'a mut W {
        self.variant(TRIGSRC_A::PTC_SEQ)
    }
    #[doc = "PTC Window Compare Trigger"]
    #[inline(always)]
    pub fn ptc_wcomp(self) -> &'a mut W {
        self.variant(TRIGSRC_A::PTC_WCOMP)
    }
}
#[doc = "Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGACT_A {
    #[doc = "0: One trigger required for each block transfer"]
    BLOCK = 0,
    #[doc = "2: One trigger required for each beat transfer"]
    BEAT = 2,
    #[doc = "3: One trigger required for each transaction"]
    TRANSACTION = 3,
}
impl From<TRIGACT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGACT` reader - Trigger Action"]
pub type TRIGACT_R = crate::FieldReader<u8, TRIGACT_A>;
impl TRIGACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGACT_A> {
        match self.bits {
            0 => Some(TRIGACT_A::BLOCK),
            2 => Some(TRIGACT_A::BEAT),
            3 => Some(TRIGACT_A::TRANSACTION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == TRIGACT_A::BLOCK
    }
    #[doc = "Checks if the value of the field is `BEAT`"]
    #[inline(always)]
    pub fn is_beat(&self) -> bool {
        *self == TRIGACT_A::BEAT
    }
    #[doc = "Checks if the value of the field is `TRANSACTION`"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        *self == TRIGACT_A::TRANSACTION
    }
}
#[doc = "Field `TRIGACT` writer - Trigger Action"]
pub type TRIGACT_W<'a> =
    crate::FieldWriter<'a, u32, CHCTRLB_SPEC, u8, TRIGACT_A, 2, 22>;
impl<'a> TRIGACT_W<'a> {
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn block(self) -> &'a mut W {
        self.variant(TRIGACT_A::BLOCK)
    }
    #[doc = "One trigger required for each beat transfer"]
    #[inline(always)]
    pub fn beat(self) -> &'a mut W {
        self.variant(TRIGACT_A::BEAT)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut W {
        self.variant(TRIGACT_A::TRANSACTION)
    }
}
#[doc = "Software Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: Channel suspend operation"]
    SUSPEND = 1,
    #[doc = "2: Channel resume operation"]
    RESUME = 2,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD` reader - Software Command"]
pub type CMD_R = crate::FieldReader<u8, CMD_A>;
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            0 => Some(CMD_A::NOACT),
            1 => Some(CMD_A::SUSPEND),
            2 => Some(CMD_A::RESUME),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == CMD_A::NOACT
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == CMD_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == CMD_A::RESUME
    }
}
#[doc = "Field `CMD` writer - Software Command"]
pub type CMD_W<'a> =
    crate::FieldWriter<'a, u32, CHCTRLB_SPEC, u8, CMD_A, 2, 24>;
impl<'a> CMD_W<'a> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut W {
        self.variant(CMD_A::NOACT)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(CMD_A::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(CMD_A::RESUME)
    }
}
impl R {
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&self) -> EVIE_R {
        EVIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EVOE_R {
        EVOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Channel Arbitration Level"]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TRIGSRC_R {
        TRIGSRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&self) -> TRIGACT_R {
        TRIGACT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline(always)]
    pub fn evact(&mut self) -> EVACT_W {
        EVACT_W::new(self)
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline(always)]
    pub fn evie(&mut self) -> EVIE_W {
        EVIE_W::new(self)
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&mut self) -> EVOE_W {
        EVOE_W::new(self)
    }
    #[doc = "Bits 5:6 - Channel Arbitration Level"]
    #[inline(always)]
    pub fn lvl(&mut self) -> LVL_W {
        LVL_W::new(self)
    }
    #[doc = "Bits 8:12 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&mut self) -> TRIGSRC_W {
        TRIGSRC_W::new(self)
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&mut self) -> TRIGACT_W {
        TRIGACT_W::new(self)
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrlb](index.html) module"]
pub struct CHCTRLB_SPEC;
impl crate::RegisterSpec for CHCTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctrlb::R](R) reader structure"]
impl crate::Readable for CHCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctrlb::W](W) writer structure"]
impl crate::Writable for CHCTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCTRLB to value 0"]
impl crate::Resettable for CHCTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
