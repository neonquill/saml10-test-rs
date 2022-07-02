#[doc = "Register `SWEVT` writer"]
pub struct W(crate::W<SWEVT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWEVT_SPEC>;
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
impl From<crate::W<SWEVT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWEVT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANNEL0` writer - Channel 0 Software Selection"]
pub type CHANNEL0_W<'a> = crate::BitWriter<'a, u32, SWEVT_SPEC, bool, 0>;
#[doc = "Field `CHANNEL1` writer - Channel 1 Software Selection"]
pub type CHANNEL1_W<'a> = crate::BitWriter<'a, u32, SWEVT_SPEC, bool, 1>;
#[doc = "Field `CHANNEL2` writer - Channel 2 Software Selection"]
pub type CHANNEL2_W<'a> = crate::BitWriter<'a, u32, SWEVT_SPEC, bool, 2>;
#[doc = "Field `CHANNEL3` writer - Channel 3 Software Selection"]
pub type CHANNEL3_W<'a> = crate::BitWriter<'a, u32, SWEVT_SPEC, bool, 3>;
#[doc = "Field `CHANNEL4` writer - Channel 4 Software Selection"]
pub type CHANNEL4_W<'a> = crate::BitWriter<'a, u32, SWEVT_SPEC, bool, 4>;
#[doc = "Field `CHANNEL5` writer - Channel 5 Software Selection"]
pub type CHANNEL5_W<'a> = crate::BitWriter<'a, u32, SWEVT_SPEC, bool, 5>;
#[doc = "Field `CHANNEL6` writer - Channel 6 Software Selection"]
pub type CHANNEL6_W<'a> = crate::BitWriter<'a, u32, SWEVT_SPEC, bool, 6>;
#[doc = "Field `CHANNEL7` writer - Channel 7 Software Selection"]
pub type CHANNEL7_W<'a> = crate::BitWriter<'a, u32, SWEVT_SPEC, bool, 7>;
impl W {
    #[doc = "Bit 0 - Channel 0 Software Selection"]
    #[inline(always)]
    pub fn channel0(&mut self) -> CHANNEL0_W {
        CHANNEL0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Software Selection"]
    #[inline(always)]
    pub fn channel1(&mut self) -> CHANNEL1_W {
        CHANNEL1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Software Selection"]
    #[inline(always)]
    pub fn channel2(&mut self) -> CHANNEL2_W {
        CHANNEL2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Software Selection"]
    #[inline(always)]
    pub fn channel3(&mut self) -> CHANNEL3_W {
        CHANNEL3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Software Selection"]
    #[inline(always)]
    pub fn channel4(&mut self) -> CHANNEL4_W {
        CHANNEL4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Software Selection"]
    #[inline(always)]
    pub fn channel5(&mut self) -> CHANNEL5_W {
        CHANNEL5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Software Selection"]
    #[inline(always)]
    pub fn channel6(&mut self) -> CHANNEL6_W {
        CHANNEL6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Software Selection"]
    #[inline(always)]
    pub fn channel7(&mut self) -> CHANNEL7_W {
        CHANNEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Event\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevt](index.html) module"]
pub struct SWEVT_SPEC;
impl crate::RegisterSpec for SWEVT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swevt::W](W) writer structure"]
impl crate::Writable for SWEVT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWEVT to value 0"]
impl crate::Resettable for SWEVT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
