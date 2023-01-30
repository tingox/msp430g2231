#[doc = "Register `USICNT` reader"]
pub struct R(crate::R<USICNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USICNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USICNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USICNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USICNT` writer"]
pub struct W(crate::W<USICNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USICNT_SPEC>;
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
impl From<crate::W<USICNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USICNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USICNT0` reader - USI Bit Count 0"]
pub type USICNT0_R = crate::BitReader<bool>;
#[doc = "Field `USICNT0` writer - USI Bit Count 0"]
pub type USICNT0_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICNT_SPEC, bool, O>;
#[doc = "Field `USICNT1` reader - USI Bit Count 1"]
pub type USICNT1_R = crate::BitReader<bool>;
#[doc = "Field `USICNT1` writer - USI Bit Count 1"]
pub type USICNT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICNT_SPEC, bool, O>;
#[doc = "Field `USICNT2` reader - USI Bit Count 2"]
pub type USICNT2_R = crate::BitReader<bool>;
#[doc = "Field `USICNT2` writer - USI Bit Count 2"]
pub type USICNT2_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICNT_SPEC, bool, O>;
#[doc = "Field `USICNT3` reader - USI Bit Count 3"]
pub type USICNT3_R = crate::BitReader<bool>;
#[doc = "Field `USICNT3` writer - USI Bit Count 3"]
pub type USICNT3_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICNT_SPEC, bool, O>;
#[doc = "Field `USICNT4` reader - USI Bit Count 4"]
pub type USICNT4_R = crate::BitReader<bool>;
#[doc = "Field `USICNT4` writer - USI Bit Count 4"]
pub type USICNT4_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICNT_SPEC, bool, O>;
#[doc = "Field `USIIFGCC` reader - USI Interrupt Flag Clear Control"]
pub type USIIFGCC_R = crate::BitReader<bool>;
#[doc = "Field `USIIFGCC` writer - USI Interrupt Flag Clear Control"]
pub type USIIFGCC_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICNT_SPEC, bool, O>;
#[doc = "Field `USI16B` reader - USI 16 Bit Shift Register Enable"]
pub type USI16B_R = crate::BitReader<bool>;
#[doc = "Field `USI16B` writer - USI 16 Bit Shift Register Enable"]
pub type USI16B_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICNT_SPEC, bool, O>;
#[doc = "Field `USISCLREL` reader - USI SCL Released"]
pub type USISCLREL_R = crate::BitReader<bool>;
#[doc = "Field `USISCLREL` writer - USI SCL Released"]
pub type USISCLREL_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICNT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USI Bit Count 0"]
    #[inline(always)]
    pub fn usicnt0(&self) -> USICNT0_R {
        USICNT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USI Bit Count 1"]
    #[inline(always)]
    pub fn usicnt1(&self) -> USICNT1_R {
        USICNT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USI Bit Count 2"]
    #[inline(always)]
    pub fn usicnt2(&self) -> USICNT2_R {
        USICNT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USI Bit Count 3"]
    #[inline(always)]
    pub fn usicnt3(&self) -> USICNT3_R {
        USICNT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USI Bit Count 4"]
    #[inline(always)]
    pub fn usicnt4(&self) -> USICNT4_R {
        USICNT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USI Interrupt Flag Clear Control"]
    #[inline(always)]
    pub fn usiifgcc(&self) -> USIIFGCC_R {
        USIIFGCC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USI 16 Bit Shift Register Enable"]
    #[inline(always)]
    pub fn usi16b(&self) -> USI16B_R {
        USI16B_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USI SCL Released"]
    #[inline(always)]
    pub fn usisclrel(&self) -> USISCLREL_R {
        USISCLREL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USI Bit Count 0"]
    #[inline(always)]
    #[must_use]
    pub fn usicnt0(&mut self) -> USICNT0_W<0> {
        USICNT0_W::new(self)
    }
    #[doc = "Bit 1 - USI Bit Count 1"]
    #[inline(always)]
    #[must_use]
    pub fn usicnt1(&mut self) -> USICNT1_W<1> {
        USICNT1_W::new(self)
    }
    #[doc = "Bit 2 - USI Bit Count 2"]
    #[inline(always)]
    #[must_use]
    pub fn usicnt2(&mut self) -> USICNT2_W<2> {
        USICNT2_W::new(self)
    }
    #[doc = "Bit 3 - USI Bit Count 3"]
    #[inline(always)]
    #[must_use]
    pub fn usicnt3(&mut self) -> USICNT3_W<3> {
        USICNT3_W::new(self)
    }
    #[doc = "Bit 4 - USI Bit Count 4"]
    #[inline(always)]
    #[must_use]
    pub fn usicnt4(&mut self) -> USICNT4_W<4> {
        USICNT4_W::new(self)
    }
    #[doc = "Bit 5 - USI Interrupt Flag Clear Control"]
    #[inline(always)]
    #[must_use]
    pub fn usiifgcc(&mut self) -> USIIFGCC_W<5> {
        USIIFGCC_W::new(self)
    }
    #[doc = "Bit 6 - USI 16 Bit Shift Register Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usi16b(&mut self) -> USI16B_W<6> {
        USI16B_W::new(self)
    }
    #[doc = "Bit 7 - USI SCL Released"]
    #[inline(always)]
    #[must_use]
    pub fn usisclrel(&mut self) -> USISCLREL_W<7> {
        USISCLREL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "USI Bit Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usicnt](index.html) module"]
pub struct USICNT_SPEC;
impl crate::RegisterSpec for USICNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usicnt::R](R) reader structure"]
impl crate::Readable for USICNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usicnt::W](W) writer structure"]
impl crate::Writable for USICNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USICNT to value 0"]
impl crate::Resettable for USICNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
