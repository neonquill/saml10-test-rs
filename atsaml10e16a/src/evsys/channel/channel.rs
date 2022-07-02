#[doc = "Register `CHANNEL` reader"]
pub struct R(crate::R<CHANNEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL` writer"]
pub struct W(crate::W<CHANNEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_SPEC>;
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
impl From<crate::W<CHANNEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVGEN` reader - Event Generator Selection"]
pub type EVGEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVGEN` writer - Event Generator Selection"]
pub type EVGEN_W<'a> = crate::FieldWriter<'a, u32, CHANNEL_SPEC, u8, u8, 6, 0>;
#[doc = "Path Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PATH_A {
    #[doc = "0: Synchronous path"]
    SYNCHRONOUS = 0,
    #[doc = "1: Resynchronized path"]
    RESYNCHRONIZED = 1,
    #[doc = "2: Asynchronous path"]
    ASYNCHRONOUS = 2,
}
impl From<PATH_A> for u8 {
    #[inline(always)]
    fn from(variant: PATH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PATH` reader - Path Selection"]
pub type PATH_R = crate::FieldReader<u8, PATH_A>;
impl PATH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PATH_A> {
        match self.bits {
            0 => Some(PATH_A::SYNCHRONOUS),
            1 => Some(PATH_A::RESYNCHRONIZED),
            2 => Some(PATH_A::ASYNCHRONOUS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == PATH_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `RESYNCHRONIZED`"]
    #[inline(always)]
    pub fn is_resynchronized(&self) -> bool {
        *self == PATH_A::RESYNCHRONIZED
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == PATH_A::ASYNCHRONOUS
    }
}
#[doc = "Field `PATH` writer - Path Selection"]
pub type PATH_W<'a> =
    crate::FieldWriter<'a, u32, CHANNEL_SPEC, u8, PATH_A, 2, 8>;
impl<'a> PATH_W<'a> {
    #[doc = "Synchronous path"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(PATH_A::SYNCHRONOUS)
    }
    #[doc = "Resynchronized path"]
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut W {
        self.variant(PATH_A::RESYNCHRONIZED)
    }
    #[doc = "Asynchronous path"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(PATH_A::ASYNCHRONOUS)
    }
}
#[doc = "Edge Detection Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGSEL_A {
    #[doc = "0: No event output when using the resynchronized or synchronous path"]
    NO_EVT_OUTPUT = 0,
    #[doc = "1: Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    RISING_EDGE = 1,
    #[doc = "2: Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    FALLING_EDGE = 2,
    #[doc = "3: Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    BOTH_EDGES = 3,
}
impl From<EDGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EDGSEL` reader - Edge Detection Selection"]
pub type EDGSEL_R = crate::FieldReader<u8, EDGSEL_A>;
impl EDGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGSEL_A {
        match self.bits {
            0 => EDGSEL_A::NO_EVT_OUTPUT,
            1 => EDGSEL_A::RISING_EDGE,
            2 => EDGSEL_A::FALLING_EDGE,
            3 => EDGSEL_A::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVT_OUTPUT`"]
    #[inline(always)]
    pub fn is_no_evt_output(&self) -> bool {
        *self == EDGSEL_A::NO_EVT_OUTPUT
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EDGSEL_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EDGSEL_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EDGSEL_A::BOTH_EDGES
    }
}
#[doc = "Field `EDGSEL` writer - Edge Detection Selection"]
pub type EDGSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, CHANNEL_SPEC, u8, EDGSEL_A, 2, 10>;
impl<'a> EDGSEL_W<'a> {
    #[doc = "No event output when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn no_evt_output(self) -> &'a mut W {
        self.variant(EDGSEL_A::NO_EVT_OUTPUT)
    }
    #[doc = "Event detection only on the rising edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EDGSEL_A::RISING_EDGE)
    }
    #[doc = "Event detection only on the falling edge of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EDGSEL_A::FALLING_EDGE)
    }
    #[doc = "Event detection on rising and falling edges of the signal from the event generator when using the resynchronized or synchronous path"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EDGSEL_A::BOTH_EDGES)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in standby"]
pub type RUNSTDBY_W<'a> = crate::BitWriter<'a, u32, CHANNEL_SPEC, bool, 14>;
#[doc = "Field `ONDEMAND` reader - Generic Clock On Demand"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - Generic Clock On Demand"]
pub type ONDEMAND_W<'a> = crate::BitWriter<'a, u32, CHANNEL_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:5 - Event Generator Selection"]
    #[inline(always)]
    pub fn evgen(&self) -> EVGEN_R {
        EVGEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - Path Selection"]
    #[inline(always)]
    pub fn path(&self) -> PATH_R {
        PATH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Edge Detection Selection"]
    #[inline(always)]
    pub fn edgsel(&self) -> EDGSEL_R {
        EDGSEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - Run in standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Generic Clock On Demand"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Event Generator Selection"]
    #[inline(always)]
    pub fn evgen(&mut self) -> EVGEN_W {
        EVGEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Path Selection"]
    #[inline(always)]
    pub fn path(&mut self) -> PATH_W {
        PATH_W::new(self)
    }
    #[doc = "Bits 10:11 - Edge Detection Selection"]
    #[inline(always)]
    pub fn edgsel(&mut self) -> EDGSEL_W {
        EDGSEL_W::new(self)
    }
    #[doc = "Bit 14 - Run in standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 15 - Generic Clock On Demand"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel](index.html) module"]
pub struct CHANNEL_SPEC;
impl crate::RegisterSpec for CHANNEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel::R](R) reader structure"]
impl crate::Readable for CHANNEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel::W](W) writer structure"]
impl crate::Writable for CHANNEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHANNEL to value 0x8000"]
impl crate::Resettable for CHANNEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
