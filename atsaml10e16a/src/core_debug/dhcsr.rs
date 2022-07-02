#[doc = "Register `DHCSR` reader"]
pub struct R(crate::R<DHCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DHCSR` writer"]
pub struct W(crate::W<DHCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHCSR_SPEC>;
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
impl From<crate::W<DHCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C_DEBUGEN` reader - Enable Halting debug"]
pub type C_DEBUGEN_R = crate::BitReader<bool>;
#[doc = "Field `C_DEBUGEN` writer - Enable Halting debug"]
pub type C_DEBUGEN_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 0>;
#[doc = "Field `C_HALT` reader - Halt processor"]
pub type C_HALT_R = crate::BitReader<bool>;
#[doc = "Field `C_HALT` writer - Halt processor"]
pub type C_HALT_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 1>;
#[doc = "Field `C_STEP` reader - Enable single step"]
pub type C_STEP_R = crate::BitReader<bool>;
#[doc = "Field `C_STEP` writer - Enable single step"]
pub type C_STEP_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 2>;
#[doc = "Field `C_MASKINTS` reader - Mask PendSV, SysTick and external configurable interrupts"]
pub type C_MASKINTS_R = crate::BitReader<bool>;
#[doc = "Field `C_MASKINTS` writer - Mask PendSV, SysTick and external configurable interrupts"]
pub type C_MASKINTS_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 3>;
#[doc = "Field `S_SNAPSTALL` reader - Snap stall control"]
pub type S_SNAPSTALL_R = crate::BitReader<bool>;
#[doc = "Field `S_SNAPSTALL` writer - Snap stall control"]
pub type S_SNAPSTALL_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 5>;
#[doc = "Field `S_REGRDY` reader - Register ready status"]
pub type S_REGRDY_R = crate::BitReader<bool>;
#[doc = "Field `S_REGRDY` writer - Register ready status"]
pub type S_REGRDY_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 16>;
#[doc = "Field `S_HALT` reader - Halted status"]
pub type S_HALT_R = crate::BitReader<bool>;
#[doc = "Field `S_HALT` writer - Halted status"]
pub type S_HALT_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 17>;
#[doc = "Field `S_SLEEP` reader - Sleeping status"]
pub type S_SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `S_SLEEP` writer - Sleeping status"]
pub type S_SLEEP_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 18>;
#[doc = "Field `S_LOCKUP` reader - Lockup status"]
pub type S_LOCKUP_R = crate::BitReader<bool>;
#[doc = "Field `S_LOCKUP` writer - Lockup status"]
pub type S_LOCKUP_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 19>;
#[doc = "Field `S_SDE` reader - Secure debug enabled"]
pub type S_SDE_R = crate::BitReader<bool>;
#[doc = "Field `S_SDE` writer - Secure debug enabled"]
pub type S_SDE_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 20>;
#[doc = "Field `S_RETIRE_ST` reader - Retire sticky status"]
pub type S_RETIRE_ST_R = crate::BitReader<bool>;
#[doc = "Field `S_RETIRE_ST` writer - Retire sticky status"]
pub type S_RETIRE_ST_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 24>;
#[doc = "Field `S_RESET_ST` reader - Reset sticky status"]
pub type S_RESET_ST_R = crate::BitReader<bool>;
#[doc = "Field `S_RESET_ST` writer - Reset sticky status"]
pub type S_RESET_ST_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 25>;
#[doc = "Field `S_RESTART_ST` reader - Restart sticky status"]
pub type S_RESTART_ST_R = crate::BitReader<bool>;
#[doc = "Field `S_RESTART_ST` writer - Restart sticky status"]
pub type S_RESTART_ST_W<'a> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, 26>;
impl R {
    #[doc = "Bit 0 - Enable Halting debug"]
    #[inline(always)]
    pub fn c_debugen(&self) -> C_DEBUGEN_R {
        C_DEBUGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Halt processor"]
    #[inline(always)]
    pub fn c_halt(&self) -> C_HALT_R {
        C_HALT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable single step"]
    #[inline(always)]
    pub fn c_step(&self) -> C_STEP_R {
        C_STEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask PendSV, SysTick and external configurable interrupts"]
    #[inline(always)]
    pub fn c_maskints(&self) -> C_MASKINTS_R {
        C_MASKINTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Snap stall control"]
    #[inline(always)]
    pub fn s_snapstall(&self) -> S_SNAPSTALL_R {
        S_SNAPSTALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Register ready status"]
    #[inline(always)]
    pub fn s_regrdy(&self) -> S_REGRDY_R {
        S_REGRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Halted status"]
    #[inline(always)]
    pub fn s_halt(&self) -> S_HALT_R {
        S_HALT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Sleeping status"]
    #[inline(always)]
    pub fn s_sleep(&self) -> S_SLEEP_R {
        S_SLEEP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Lockup status"]
    #[inline(always)]
    pub fn s_lockup(&self) -> S_LOCKUP_R {
        S_LOCKUP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Secure debug enabled"]
    #[inline(always)]
    pub fn s_sde(&self) -> S_SDE_R {
        S_SDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Retire sticky status"]
    #[inline(always)]
    pub fn s_retire_st(&self) -> S_RETIRE_ST_R {
        S_RETIRE_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset sticky status"]
    #[inline(always)]
    pub fn s_reset_st(&self) -> S_RESET_ST_R {
        S_RESET_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Restart sticky status"]
    #[inline(always)]
    pub fn s_restart_st(&self) -> S_RESTART_ST_R {
        S_RESTART_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Halting debug"]
    #[inline(always)]
    pub fn c_debugen(&mut self) -> C_DEBUGEN_W {
        C_DEBUGEN_W::new(self)
    }
    #[doc = "Bit 1 - Halt processor"]
    #[inline(always)]
    pub fn c_halt(&mut self) -> C_HALT_W {
        C_HALT_W::new(self)
    }
    #[doc = "Bit 2 - Enable single step"]
    #[inline(always)]
    pub fn c_step(&mut self) -> C_STEP_W {
        C_STEP_W::new(self)
    }
    #[doc = "Bit 3 - Mask PendSV, SysTick and external configurable interrupts"]
    #[inline(always)]
    pub fn c_maskints(&mut self) -> C_MASKINTS_W {
        C_MASKINTS_W::new(self)
    }
    #[doc = "Bit 5 - Snap stall control"]
    #[inline(always)]
    pub fn s_snapstall(&mut self) -> S_SNAPSTALL_W {
        S_SNAPSTALL_W::new(self)
    }
    #[doc = "Bit 16 - Register ready status"]
    #[inline(always)]
    pub fn s_regrdy(&mut self) -> S_REGRDY_W {
        S_REGRDY_W::new(self)
    }
    #[doc = "Bit 17 - Halted status"]
    #[inline(always)]
    pub fn s_halt(&mut self) -> S_HALT_W {
        S_HALT_W::new(self)
    }
    #[doc = "Bit 18 - Sleeping status"]
    #[inline(always)]
    pub fn s_sleep(&mut self) -> S_SLEEP_W {
        S_SLEEP_W::new(self)
    }
    #[doc = "Bit 19 - Lockup status"]
    #[inline(always)]
    pub fn s_lockup(&mut self) -> S_LOCKUP_W {
        S_LOCKUP_W::new(self)
    }
    #[doc = "Bit 20 - Secure debug enabled"]
    #[inline(always)]
    pub fn s_sde(&mut self) -> S_SDE_W {
        S_SDE_W::new(self)
    }
    #[doc = "Bit 24 - Retire sticky status"]
    #[inline(always)]
    pub fn s_retire_st(&mut self) -> S_RETIRE_ST_W {
        S_RETIRE_ST_W::new(self)
    }
    #[doc = "Bit 25 - Reset sticky status"]
    #[inline(always)]
    pub fn s_reset_st(&mut self) -> S_RESET_ST_W {
        S_RESET_ST_W::new(self)
    }
    #[doc = "Bit 26 - Restart sticky status"]
    #[inline(always)]
    pub fn s_restart_st(&mut self) -> S_RESTART_ST_W {
        S_RESTART_ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Halting Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhcsr](index.html) module"]
pub struct DHCSR_SPEC;
impl crate::RegisterSpec for DHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhcsr::R](R) reader structure"]
impl crate::Readable for DHCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dhcsr::W](W) writer structure"]
impl crate::Writable for DHCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DHCSR to value 0"]
impl crate::Resettable for DHCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
