#[doc = "Register `WRCONFIG` writer"]
pub struct W(crate::W<WRCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRCONFIG_SPEC>;
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
impl From<crate::W<WRCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINMASK` writer - Pin Mask for Multiple Pin Configuration"]
pub type PINMASK_W<'a> =
    crate::FieldWriter<'a, u32, WRCONFIG_SPEC, u16, u16, 16, 0>;
#[doc = "Field `PMUXEN` writer - Peripheral Multiplexer Enable"]
pub type PMUXEN_W<'a> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, 16>;
#[doc = "Field `INEN` writer - Input Enable"]
pub type INEN_W<'a> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, 17>;
#[doc = "Field `PULLEN` writer - Pull Enable"]
pub type PULLEN_W<'a> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, 18>;
#[doc = "Field `DRVSTR` writer - Output Driver Strength Selection"]
pub type DRVSTR_W<'a> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, 22>;
#[doc = "Field `PMUX` writer - Peripheral Multiplexing"]
pub type PMUX_W<'a> = crate::FieldWriter<'a, u32, WRCONFIG_SPEC, u8, u8, 4, 24>;
#[doc = "Field `WRPMUX` writer - Write PMUX"]
pub type WRPMUX_W<'a> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, 28>;
#[doc = "Field `WRPINCFG` writer - Write PINCFG"]
pub type WRPINCFG_W<'a> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, 30>;
#[doc = "Field `HWSEL` writer - Half-Word Select"]
pub type HWSEL_W<'a> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, 31>;
impl W {
    #[doc = "Bits 0:15 - Pin Mask for Multiple Pin Configuration"]
    #[inline(always)]
    pub fn pinmask(&mut self) -> PINMASK_W {
        PINMASK_W::new(self)
    }
    #[doc = "Bit 16 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PMUXEN_W {
        PMUXEN_W::new(self)
    }
    #[doc = "Bit 17 - Input Enable"]
    #[inline(always)]
    pub fn inen(&mut self) -> INEN_W {
        INEN_W::new(self)
    }
    #[doc = "Bit 18 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&mut self) -> PULLEN_W {
        PULLEN_W::new(self)
    }
    #[doc = "Bit 22 - Output Driver Strength Selection"]
    #[inline(always)]
    pub fn drvstr(&mut self) -> DRVSTR_W {
        DRVSTR_W::new(self)
    }
    #[doc = "Bits 24:27 - Peripheral Multiplexing"]
    #[inline(always)]
    pub fn pmux(&mut self) -> PMUX_W {
        PMUX_W::new(self)
    }
    #[doc = "Bit 28 - Write PMUX"]
    #[inline(always)]
    pub fn wrpmux(&mut self) -> WRPMUX_W {
        WRPMUX_W::new(self)
    }
    #[doc = "Bit 30 - Write PINCFG"]
    #[inline(always)]
    pub fn wrpincfg(&mut self) -> WRPINCFG_W {
        WRPINCFG_W::new(self)
    }
    #[doc = "Bit 31 - Half-Word Select"]
    #[inline(always)]
    pub fn hwsel(&mut self) -> HWSEL_W {
        HWSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Configuration\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrconfig](index.html) module"]
pub struct WRCONFIG_SPEC;
impl crate::RegisterSpec for WRCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wrconfig::W](W) writer structure"]
impl crate::Writable for WRCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRCONFIG to value 0"]
impl crate::Resettable for WRCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
