#[doc = "Register `NSCHKUSER[%s]` reader"]
pub struct R(crate::R<NSCHKUSER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSCHKUSER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSCHKUSER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSCHKUSER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSCHKUSER[%s]` writer"]
pub struct W(crate::W<NSCHKUSER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSCHKUSER_SPEC>;
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
impl From<crate::W<NSCHKUSER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSCHKUSER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USER0` reader - User 0 to be checked as non-secured"]
pub type USER0_R = crate::BitReader<bool>;
#[doc = "Field `USER0` writer - User 0 to be checked as non-secured"]
pub type USER0_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 0>;
#[doc = "Field `USER1` reader - User 1 to be checked as non-secured"]
pub type USER1_R = crate::BitReader<bool>;
#[doc = "Field `USER1` writer - User 1 to be checked as non-secured"]
pub type USER1_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 1>;
#[doc = "Field `USER2` reader - User 2 to be checked as non-secured"]
pub type USER2_R = crate::BitReader<bool>;
#[doc = "Field `USER2` writer - User 2 to be checked as non-secured"]
pub type USER2_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 2>;
#[doc = "Field `USER3` reader - User 3 to be checked as non-secured"]
pub type USER3_R = crate::BitReader<bool>;
#[doc = "Field `USER3` writer - User 3 to be checked as non-secured"]
pub type USER3_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 3>;
#[doc = "Field `USER4` reader - User 4 to be checked as non-secured"]
pub type USER4_R = crate::BitReader<bool>;
#[doc = "Field `USER4` writer - User 4 to be checked as non-secured"]
pub type USER4_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 4>;
#[doc = "Field `USER5` reader - User 5 to be checked as non-secured"]
pub type USER5_R = crate::BitReader<bool>;
#[doc = "Field `USER5` writer - User 5 to be checked as non-secured"]
pub type USER5_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 5>;
#[doc = "Field `USER6` reader - User 6 to be checked as non-secured"]
pub type USER6_R = crate::BitReader<bool>;
#[doc = "Field `USER6` writer - User 6 to be checked as non-secured"]
pub type USER6_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 6>;
#[doc = "Field `USER7` reader - User 7 to be checked as non-secured"]
pub type USER7_R = crate::BitReader<bool>;
#[doc = "Field `USER7` writer - User 7 to be checked as non-secured"]
pub type USER7_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 7>;
#[doc = "Field `USER8` reader - User 8 to be checked as non-secured"]
pub type USER8_R = crate::BitReader<bool>;
#[doc = "Field `USER8` writer - User 8 to be checked as non-secured"]
pub type USER8_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 8>;
#[doc = "Field `USER9` reader - User 9 to be checked as non-secured"]
pub type USER9_R = crate::BitReader<bool>;
#[doc = "Field `USER9` writer - User 9 to be checked as non-secured"]
pub type USER9_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 9>;
#[doc = "Field `USER10` reader - User 10 to be checked as non-secured"]
pub type USER10_R = crate::BitReader<bool>;
#[doc = "Field `USER10` writer - User 10 to be checked as non-secured"]
pub type USER10_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 10>;
#[doc = "Field `USER11` reader - User 11 to be checked as non-secured"]
pub type USER11_R = crate::BitReader<bool>;
#[doc = "Field `USER11` writer - User 11 to be checked as non-secured"]
pub type USER11_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 11>;
#[doc = "Field `USER12` reader - User 12 to be checked as non-secured"]
pub type USER12_R = crate::BitReader<bool>;
#[doc = "Field `USER12` writer - User 12 to be checked as non-secured"]
pub type USER12_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 12>;
#[doc = "Field `USER13` reader - User 13 to be checked as non-secured"]
pub type USER13_R = crate::BitReader<bool>;
#[doc = "Field `USER13` writer - User 13 to be checked as non-secured"]
pub type USER13_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 13>;
#[doc = "Field `USER14` reader - User 14 to be checked as non-secured"]
pub type USER14_R = crate::BitReader<bool>;
#[doc = "Field `USER14` writer - User 14 to be checked as non-secured"]
pub type USER14_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 14>;
#[doc = "Field `USER15` reader - User 15 to be checked as non-secured"]
pub type USER15_R = crate::BitReader<bool>;
#[doc = "Field `USER15` writer - User 15 to be checked as non-secured"]
pub type USER15_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 15>;
#[doc = "Field `USER16` reader - User 16 to be checked as non-secured"]
pub type USER16_R = crate::BitReader<bool>;
#[doc = "Field `USER16` writer - User 16 to be checked as non-secured"]
pub type USER16_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 16>;
#[doc = "Field `USER17` reader - User 17 to be checked as non-secured"]
pub type USER17_R = crate::BitReader<bool>;
#[doc = "Field `USER17` writer - User 17 to be checked as non-secured"]
pub type USER17_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 17>;
#[doc = "Field `USER18` reader - User 18 to be checked as non-secured"]
pub type USER18_R = crate::BitReader<bool>;
#[doc = "Field `USER18` writer - User 18 to be checked as non-secured"]
pub type USER18_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 18>;
#[doc = "Field `USER19` reader - User 19 to be checked as non-secured"]
pub type USER19_R = crate::BitReader<bool>;
#[doc = "Field `USER19` writer - User 19 to be checked as non-secured"]
pub type USER19_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 19>;
#[doc = "Field `USER20` reader - User 20 to be checked as non-secured"]
pub type USER20_R = crate::BitReader<bool>;
#[doc = "Field `USER20` writer - User 20 to be checked as non-secured"]
pub type USER20_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 20>;
#[doc = "Field `USER21` reader - User 21 to be checked as non-secured"]
pub type USER21_R = crate::BitReader<bool>;
#[doc = "Field `USER21` writer - User 21 to be checked as non-secured"]
pub type USER21_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 21>;
#[doc = "Field `USER22` reader - User 22 to be checked as non-secured"]
pub type USER22_R = crate::BitReader<bool>;
#[doc = "Field `USER22` writer - User 22 to be checked as non-secured"]
pub type USER22_W<'a> = crate::BitWriter<'a, u32, NSCHKUSER_SPEC, bool, 22>;
impl R {
    #[doc = "Bit 0 - User 0 to be checked as non-secured"]
    #[inline(always)]
    pub fn user0(&self) -> USER0_R {
        USER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - User 1 to be checked as non-secured"]
    #[inline(always)]
    pub fn user1(&self) -> USER1_R {
        USER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - User 2 to be checked as non-secured"]
    #[inline(always)]
    pub fn user2(&self) -> USER2_R {
        USER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - User 3 to be checked as non-secured"]
    #[inline(always)]
    pub fn user3(&self) -> USER3_R {
        USER3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - User 4 to be checked as non-secured"]
    #[inline(always)]
    pub fn user4(&self) -> USER4_R {
        USER4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - User 5 to be checked as non-secured"]
    #[inline(always)]
    pub fn user5(&self) -> USER5_R {
        USER5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - User 6 to be checked as non-secured"]
    #[inline(always)]
    pub fn user6(&self) -> USER6_R {
        USER6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - User 7 to be checked as non-secured"]
    #[inline(always)]
    pub fn user7(&self) -> USER7_R {
        USER7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - User 8 to be checked as non-secured"]
    #[inline(always)]
    pub fn user8(&self) -> USER8_R {
        USER8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - User 9 to be checked as non-secured"]
    #[inline(always)]
    pub fn user9(&self) -> USER9_R {
        USER9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - User 10 to be checked as non-secured"]
    #[inline(always)]
    pub fn user10(&self) -> USER10_R {
        USER10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - User 11 to be checked as non-secured"]
    #[inline(always)]
    pub fn user11(&self) -> USER11_R {
        USER11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - User 12 to be checked as non-secured"]
    #[inline(always)]
    pub fn user12(&self) -> USER12_R {
        USER12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - User 13 to be checked as non-secured"]
    #[inline(always)]
    pub fn user13(&self) -> USER13_R {
        USER13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - User 14 to be checked as non-secured"]
    #[inline(always)]
    pub fn user14(&self) -> USER14_R {
        USER14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - User 15 to be checked as non-secured"]
    #[inline(always)]
    pub fn user15(&self) -> USER15_R {
        USER15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - User 16 to be checked as non-secured"]
    #[inline(always)]
    pub fn user16(&self) -> USER16_R {
        USER16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - User 17 to be checked as non-secured"]
    #[inline(always)]
    pub fn user17(&self) -> USER17_R {
        USER17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - User 18 to be checked as non-secured"]
    #[inline(always)]
    pub fn user18(&self) -> USER18_R {
        USER18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - User 19 to be checked as non-secured"]
    #[inline(always)]
    pub fn user19(&self) -> USER19_R {
        USER19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - User 20 to be checked as non-secured"]
    #[inline(always)]
    pub fn user20(&self) -> USER20_R {
        USER20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - User 21 to be checked as non-secured"]
    #[inline(always)]
    pub fn user21(&self) -> USER21_R {
        USER21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - User 22 to be checked as non-secured"]
    #[inline(always)]
    pub fn user22(&self) -> USER22_R {
        USER22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User 0 to be checked as non-secured"]
    #[inline(always)]
    pub fn user0(&mut self) -> USER0_W {
        USER0_W::new(self)
    }
    #[doc = "Bit 1 - User 1 to be checked as non-secured"]
    #[inline(always)]
    pub fn user1(&mut self) -> USER1_W {
        USER1_W::new(self)
    }
    #[doc = "Bit 2 - User 2 to be checked as non-secured"]
    #[inline(always)]
    pub fn user2(&mut self) -> USER2_W {
        USER2_W::new(self)
    }
    #[doc = "Bit 3 - User 3 to be checked as non-secured"]
    #[inline(always)]
    pub fn user3(&mut self) -> USER3_W {
        USER3_W::new(self)
    }
    #[doc = "Bit 4 - User 4 to be checked as non-secured"]
    #[inline(always)]
    pub fn user4(&mut self) -> USER4_W {
        USER4_W::new(self)
    }
    #[doc = "Bit 5 - User 5 to be checked as non-secured"]
    #[inline(always)]
    pub fn user5(&mut self) -> USER5_W {
        USER5_W::new(self)
    }
    #[doc = "Bit 6 - User 6 to be checked as non-secured"]
    #[inline(always)]
    pub fn user6(&mut self) -> USER6_W {
        USER6_W::new(self)
    }
    #[doc = "Bit 7 - User 7 to be checked as non-secured"]
    #[inline(always)]
    pub fn user7(&mut self) -> USER7_W {
        USER7_W::new(self)
    }
    #[doc = "Bit 8 - User 8 to be checked as non-secured"]
    #[inline(always)]
    pub fn user8(&mut self) -> USER8_W {
        USER8_W::new(self)
    }
    #[doc = "Bit 9 - User 9 to be checked as non-secured"]
    #[inline(always)]
    pub fn user9(&mut self) -> USER9_W {
        USER9_W::new(self)
    }
    #[doc = "Bit 10 - User 10 to be checked as non-secured"]
    #[inline(always)]
    pub fn user10(&mut self) -> USER10_W {
        USER10_W::new(self)
    }
    #[doc = "Bit 11 - User 11 to be checked as non-secured"]
    #[inline(always)]
    pub fn user11(&mut self) -> USER11_W {
        USER11_W::new(self)
    }
    #[doc = "Bit 12 - User 12 to be checked as non-secured"]
    #[inline(always)]
    pub fn user12(&mut self) -> USER12_W {
        USER12_W::new(self)
    }
    #[doc = "Bit 13 - User 13 to be checked as non-secured"]
    #[inline(always)]
    pub fn user13(&mut self) -> USER13_W {
        USER13_W::new(self)
    }
    #[doc = "Bit 14 - User 14 to be checked as non-secured"]
    #[inline(always)]
    pub fn user14(&mut self) -> USER14_W {
        USER14_W::new(self)
    }
    #[doc = "Bit 15 - User 15 to be checked as non-secured"]
    #[inline(always)]
    pub fn user15(&mut self) -> USER15_W {
        USER15_W::new(self)
    }
    #[doc = "Bit 16 - User 16 to be checked as non-secured"]
    #[inline(always)]
    pub fn user16(&mut self) -> USER16_W {
        USER16_W::new(self)
    }
    #[doc = "Bit 17 - User 17 to be checked as non-secured"]
    #[inline(always)]
    pub fn user17(&mut self) -> USER17_W {
        USER17_W::new(self)
    }
    #[doc = "Bit 18 - User 18 to be checked as non-secured"]
    #[inline(always)]
    pub fn user18(&mut self) -> USER18_W {
        USER18_W::new(self)
    }
    #[doc = "Bit 19 - User 19 to be checked as non-secured"]
    #[inline(always)]
    pub fn user19(&mut self) -> USER19_W {
        USER19_W::new(self)
    }
    #[doc = "Bit 20 - User 20 to be checked as non-secured"]
    #[inline(always)]
    pub fn user20(&mut self) -> USER20_W {
        USER20_W::new(self)
    }
    #[doc = "Bit 21 - User 21 to be checked as non-secured"]
    #[inline(always)]
    pub fn user21(&mut self) -> USER21_W {
        USER21_W::new(self)
    }
    #[doc = "Bit 22 - User 22 to be checked as non-secured"]
    #[inline(always)]
    pub fn user22(&mut self) -> USER22_W {
        USER22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Secure Users Check\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nschkuser](index.html) module"]
pub struct NSCHKUSER_SPEC;
impl crate::RegisterSpec for NSCHKUSER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nschkuser::R](R) reader structure"]
impl crate::Readable for NSCHKUSER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nschkuser::W](W) writer structure"]
impl crate::Writable for NSCHKUSER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NSCHKUSER[%s]
to value 0"]
impl crate::Resettable for NSCHKUSER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
