#[doc = "Register `USICTL0` reader"]
pub struct R(crate::R<USICTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USICTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USICTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USICTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USICTL0` writer"]
pub struct W(crate::W<USICTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USICTL0_SPEC>;
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
impl From<crate::W<USICTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USICTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USISWRST` reader - USI Software Reset"]
pub type USISWRST_R = crate::BitReader<bool>;
#[doc = "Field `USISWRST` writer - USI Software Reset"]
pub type USISWRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICTL0_SPEC, bool, O>;
#[doc = "Field `USIOE` reader - USI Output Enable"]
pub type USIOE_R = crate::BitReader<bool>;
#[doc = "Field `USIOE` writer - USI Output Enable"]
pub type USIOE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICTL0_SPEC, bool, O>;
#[doc = "Field `USIGE` reader - USI General Output Enable Latch"]
pub type USIGE_R = crate::BitReader<bool>;
#[doc = "Field `USIGE` writer - USI General Output Enable Latch"]
pub type USIGE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICTL0_SPEC, bool, O>;
#[doc = "Field `USIMST` reader - USI Master Select 0:Slave / 1:Master"]
pub type USIMST_R = crate::BitReader<bool>;
#[doc = "Field `USIMST` writer - USI Master Select 0:Slave / 1:Master"]
pub type USIMST_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICTL0_SPEC, bool, O>;
#[doc = "Field `USILSB` reader - USI LSB first 1:LSB / 0:MSB"]
pub type USILSB_R = crate::BitReader<bool>;
#[doc = "Field `USILSB` writer - USI LSB first 1:LSB / 0:MSB"]
pub type USILSB_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICTL0_SPEC, bool, O>;
#[doc = "Field `USIPE5` reader - USI Port Enable Px.5"]
pub type USIPE5_R = crate::BitReader<bool>;
#[doc = "Field `USIPE5` writer - USI Port Enable Px.5"]
pub type USIPE5_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICTL0_SPEC, bool, O>;
#[doc = "Field `USIPE6` reader - USI Port Enable Px.6"]
pub type USIPE6_R = crate::BitReader<bool>;
#[doc = "Field `USIPE6` writer - USI Port Enable Px.6"]
pub type USIPE6_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICTL0_SPEC, bool, O>;
#[doc = "Field `USIPE7` reader - USI Port Enable Px.7"]
pub type USIPE7_R = crate::BitReader<bool>;
#[doc = "Field `USIPE7` writer - USI Port Enable Px.7"]
pub type USIPE7_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USI Software Reset"]
    #[inline(always)]
    pub fn usiswrst(&self) -> USISWRST_R {
        USISWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USI Output Enable"]
    #[inline(always)]
    pub fn usioe(&self) -> USIOE_R {
        USIOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USI General Output Enable Latch"]
    #[inline(always)]
    pub fn usige(&self) -> USIGE_R {
        USIGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USI Master Select 0:Slave / 1:Master"]
    #[inline(always)]
    pub fn usimst(&self) -> USIMST_R {
        USIMST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USI LSB first 1:LSB / 0:MSB"]
    #[inline(always)]
    pub fn usilsb(&self) -> USILSB_R {
        USILSB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USI Port Enable Px.5"]
    #[inline(always)]
    pub fn usipe5(&self) -> USIPE5_R {
        USIPE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USI Port Enable Px.6"]
    #[inline(always)]
    pub fn usipe6(&self) -> USIPE6_R {
        USIPE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USI Port Enable Px.7"]
    #[inline(always)]
    pub fn usipe7(&self) -> USIPE7_R {
        USIPE7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USI Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usiswrst(&mut self) -> USISWRST_W<0> {
        USISWRST_W::new(self)
    }
    #[doc = "Bit 1 - USI Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usioe(&mut self) -> USIOE_W<1> {
        USIOE_W::new(self)
    }
    #[doc = "Bit 2 - USI General Output Enable Latch"]
    #[inline(always)]
    #[must_use]
    pub fn usige(&mut self) -> USIGE_W<2> {
        USIGE_W::new(self)
    }
    #[doc = "Bit 3 - USI Master Select 0:Slave / 1:Master"]
    #[inline(always)]
    #[must_use]
    pub fn usimst(&mut self) -> USIMST_W<3> {
        USIMST_W::new(self)
    }
    #[doc = "Bit 4 - USI LSB first 1:LSB / 0:MSB"]
    #[inline(always)]
    #[must_use]
    pub fn usilsb(&mut self) -> USILSB_W<4> {
        USILSB_W::new(self)
    }
    #[doc = "Bit 5 - USI Port Enable Px.5"]
    #[inline(always)]
    #[must_use]
    pub fn usipe5(&mut self) -> USIPE5_W<5> {
        USIPE5_W::new(self)
    }
    #[doc = "Bit 6 - USI Port Enable Px.6"]
    #[inline(always)]
    #[must_use]
    pub fn usipe6(&mut self) -> USIPE6_W<6> {
        USIPE6_W::new(self)
    }
    #[doc = "Bit 7 - USI Port Enable Px.7"]
    #[inline(always)]
    #[must_use]
    pub fn usipe7(&mut self) -> USIPE7_W<7> {
        USIPE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "USI Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usictl0](index.html) module"]
pub struct USICTL0_SPEC;
impl crate::RegisterSpec for USICTL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usictl0::R](R) reader structure"]
impl crate::Readable for USICTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usictl0::W](W) writer structure"]
impl crate::Writable for USICTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USICTL0 to value 0"]
impl crate::Resettable for USICTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
