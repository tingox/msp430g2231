#[doc = "Register `USICKCTL` reader"]
pub struct R(crate::R<USICKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USICKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USICKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USICKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USICKCTL` writer"]
pub struct W(crate::W<USICKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USICKCTL_SPEC>;
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
impl From<crate::W<USICKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USICKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USISWCLK` reader - USI Software Clock"]
pub type USISWCLK_R = crate::BitReader<bool>;
#[doc = "Field `USISWCLK` writer - USI Software Clock"]
pub type USISWCLK_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICKCTL_SPEC, bool, O>;
#[doc = "Field `USICKPL` reader - USI Clock Polarity 0:Inactive=Low / 1:Inactive=High"]
pub type USICKPL_R = crate::BitReader<bool>;
#[doc = "Field `USICKPL` writer - USI Clock Polarity 0:Inactive=Low / 1:Inactive=High"]
pub type USICKPL_W<'a, const O: u8> = crate::BitWriter<'a, u8, USICKCTL_SPEC, bool, O>;
#[doc = "Field `USISSEL` reader - USI Clock Source Select 2"]
pub type USISSEL_R = crate::FieldReader<u8, USISSEL_A>;
#[doc = "USI Clock Source Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USISSEL_A {
    #[doc = "0: USI Clock Source: 0"]
    USISSEL_0 = 0,
    #[doc = "1: USI Clock Source: 1"]
    USISSEL_1 = 1,
    #[doc = "2: USI Clock Source: 2"]
    USISSEL_2 = 2,
    #[doc = "3: USI Clock Source: 3"]
    USISSEL_3 = 3,
    #[doc = "4: USI Clock Source: 4"]
    USISSEL_4 = 4,
    #[doc = "5: USI Clock Source: 5"]
    USISSEL_5 = 5,
    #[doc = "6: USI Clock Source: 6"]
    USISSEL_6 = 6,
    #[doc = "7: USI Clock Source: 7"]
    USISSEL_7 = 7,
}
impl From<USISSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USISSEL_A) -> Self {
        variant as _
    }
}
impl USISSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USISSEL_A {
        match self.bits {
            0 => USISSEL_A::USISSEL_0,
            1 => USISSEL_A::USISSEL_1,
            2 => USISSEL_A::USISSEL_2,
            3 => USISSEL_A::USISSEL_3,
            4 => USISSEL_A::USISSEL_4,
            5 => USISSEL_A::USISSEL_5,
            6 => USISSEL_A::USISSEL_6,
            7 => USISSEL_A::USISSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USISSEL_0`"]
    #[inline(always)]
    pub fn is_usissel_0(&self) -> bool {
        *self == USISSEL_A::USISSEL_0
    }
    #[doc = "Checks if the value of the field is `USISSEL_1`"]
    #[inline(always)]
    pub fn is_usissel_1(&self) -> bool {
        *self == USISSEL_A::USISSEL_1
    }
    #[doc = "Checks if the value of the field is `USISSEL_2`"]
    #[inline(always)]
    pub fn is_usissel_2(&self) -> bool {
        *self == USISSEL_A::USISSEL_2
    }
    #[doc = "Checks if the value of the field is `USISSEL_3`"]
    #[inline(always)]
    pub fn is_usissel_3(&self) -> bool {
        *self == USISSEL_A::USISSEL_3
    }
    #[doc = "Checks if the value of the field is `USISSEL_4`"]
    #[inline(always)]
    pub fn is_usissel_4(&self) -> bool {
        *self == USISSEL_A::USISSEL_4
    }
    #[doc = "Checks if the value of the field is `USISSEL_5`"]
    #[inline(always)]
    pub fn is_usissel_5(&self) -> bool {
        *self == USISSEL_A::USISSEL_5
    }
    #[doc = "Checks if the value of the field is `USISSEL_6`"]
    #[inline(always)]
    pub fn is_usissel_6(&self) -> bool {
        *self == USISSEL_A::USISSEL_6
    }
    #[doc = "Checks if the value of the field is `USISSEL_7`"]
    #[inline(always)]
    pub fn is_usissel_7(&self) -> bool {
        *self == USISSEL_A::USISSEL_7
    }
}
#[doc = "Field `USISSEL` writer - USI Clock Source Select 2"]
pub type USISSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, USICKCTL_SPEC, u8, USISSEL_A, 3, O>;
impl<'a, const O: u8> USISSEL_W<'a, O> {
    #[doc = "USI Clock Source: 0"]
    #[inline(always)]
    pub fn usissel_0(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_0)
    }
    #[doc = "USI Clock Source: 1"]
    #[inline(always)]
    pub fn usissel_1(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_1)
    }
    #[doc = "USI Clock Source: 2"]
    #[inline(always)]
    pub fn usissel_2(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_2)
    }
    #[doc = "USI Clock Source: 3"]
    #[inline(always)]
    pub fn usissel_3(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_3)
    }
    #[doc = "USI Clock Source: 4"]
    #[inline(always)]
    pub fn usissel_4(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_4)
    }
    #[doc = "USI Clock Source: 5"]
    #[inline(always)]
    pub fn usissel_5(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_5)
    }
    #[doc = "USI Clock Source: 6"]
    #[inline(always)]
    pub fn usissel_6(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_6)
    }
    #[doc = "USI Clock Source: 7"]
    #[inline(always)]
    pub fn usissel_7(self) -> &'a mut W {
        self.variant(USISSEL_A::USISSEL_7)
    }
}
#[doc = "Field `USIDIV` reader - USI Clock Divider 2"]
pub type USIDIV_R = crate::FieldReader<u8, USIDIV_A>;
#[doc = "USI Clock Divider 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USIDIV_A {
    #[doc = "0: USI Clock Divider: 0"]
    USIDIV_0 = 0,
    #[doc = "1: USI Clock Divider: 1"]
    USIDIV_1 = 1,
    #[doc = "2: USI Clock Divider: 2"]
    USIDIV_2 = 2,
    #[doc = "3: USI Clock Divider: 3"]
    USIDIV_3 = 3,
    #[doc = "4: USI Clock Divider: 4"]
    USIDIV_4 = 4,
    #[doc = "5: USI Clock Divider: 5"]
    USIDIV_5 = 5,
    #[doc = "6: USI Clock Divider: 6"]
    USIDIV_6 = 6,
    #[doc = "7: USI Clock Divider: 7"]
    USIDIV_7 = 7,
}
impl From<USIDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: USIDIV_A) -> Self {
        variant as _
    }
}
impl USIDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIDIV_A {
        match self.bits {
            0 => USIDIV_A::USIDIV_0,
            1 => USIDIV_A::USIDIV_1,
            2 => USIDIV_A::USIDIV_2,
            3 => USIDIV_A::USIDIV_3,
            4 => USIDIV_A::USIDIV_4,
            5 => USIDIV_A::USIDIV_5,
            6 => USIDIV_A::USIDIV_6,
            7 => USIDIV_A::USIDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USIDIV_0`"]
    #[inline(always)]
    pub fn is_usidiv_0(&self) -> bool {
        *self == USIDIV_A::USIDIV_0
    }
    #[doc = "Checks if the value of the field is `USIDIV_1`"]
    #[inline(always)]
    pub fn is_usidiv_1(&self) -> bool {
        *self == USIDIV_A::USIDIV_1
    }
    #[doc = "Checks if the value of the field is `USIDIV_2`"]
    #[inline(always)]
    pub fn is_usidiv_2(&self) -> bool {
        *self == USIDIV_A::USIDIV_2
    }
    #[doc = "Checks if the value of the field is `USIDIV_3`"]
    #[inline(always)]
    pub fn is_usidiv_3(&self) -> bool {
        *self == USIDIV_A::USIDIV_3
    }
    #[doc = "Checks if the value of the field is `USIDIV_4`"]
    #[inline(always)]
    pub fn is_usidiv_4(&self) -> bool {
        *self == USIDIV_A::USIDIV_4
    }
    #[doc = "Checks if the value of the field is `USIDIV_5`"]
    #[inline(always)]
    pub fn is_usidiv_5(&self) -> bool {
        *self == USIDIV_A::USIDIV_5
    }
    #[doc = "Checks if the value of the field is `USIDIV_6`"]
    #[inline(always)]
    pub fn is_usidiv_6(&self) -> bool {
        *self == USIDIV_A::USIDIV_6
    }
    #[doc = "Checks if the value of the field is `USIDIV_7`"]
    #[inline(always)]
    pub fn is_usidiv_7(&self) -> bool {
        *self == USIDIV_A::USIDIV_7
    }
}
#[doc = "Field `USIDIV` writer - USI Clock Divider 2"]
pub type USIDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, USICKCTL_SPEC, u8, USIDIV_A, 3, O>;
impl<'a, const O: u8> USIDIV_W<'a, O> {
    #[doc = "USI Clock Divider: 0"]
    #[inline(always)]
    pub fn usidiv_0(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_0)
    }
    #[doc = "USI Clock Divider: 1"]
    #[inline(always)]
    pub fn usidiv_1(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_1)
    }
    #[doc = "USI Clock Divider: 2"]
    #[inline(always)]
    pub fn usidiv_2(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_2)
    }
    #[doc = "USI Clock Divider: 3"]
    #[inline(always)]
    pub fn usidiv_3(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_3)
    }
    #[doc = "USI Clock Divider: 4"]
    #[inline(always)]
    pub fn usidiv_4(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_4)
    }
    #[doc = "USI Clock Divider: 5"]
    #[inline(always)]
    pub fn usidiv_5(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_5)
    }
    #[doc = "USI Clock Divider: 6"]
    #[inline(always)]
    pub fn usidiv_6(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_6)
    }
    #[doc = "USI Clock Divider: 7"]
    #[inline(always)]
    pub fn usidiv_7(self) -> &'a mut W {
        self.variant(USIDIV_A::USIDIV_7)
    }
}
impl R {
    #[doc = "Bit 0 - USI Software Clock"]
    #[inline(always)]
    pub fn usiswclk(&self) -> USISWCLK_R {
        USISWCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USI Clock Polarity 0:Inactive=Low / 1:Inactive=High"]
    #[inline(always)]
    pub fn usickpl(&self) -> USICKPL_R {
        USICKPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - USI Clock Source Select 2"]
    #[inline(always)]
    pub fn usissel(&self) -> USISSEL_R {
        USISSEL_R::new((self.bits >> 2) & 7)
    }
    #[doc = "Bits 5:7 - USI Clock Divider 2"]
    #[inline(always)]
    pub fn usidiv(&self) -> USIDIV_R {
        USIDIV_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - USI Software Clock"]
    #[inline(always)]
    #[must_use]
    pub fn usiswclk(&mut self) -> USISWCLK_W<0> {
        USISWCLK_W::new(self)
    }
    #[doc = "Bit 1 - USI Clock Polarity 0:Inactive=Low / 1:Inactive=High"]
    #[inline(always)]
    #[must_use]
    pub fn usickpl(&mut self) -> USICKPL_W<1> {
        USICKPL_W::new(self)
    }
    #[doc = "Bits 2:4 - USI Clock Source Select 2"]
    #[inline(always)]
    #[must_use]
    pub fn usissel(&mut self) -> USISSEL_W<2> {
        USISSEL_W::new(self)
    }
    #[doc = "Bits 5:7 - USI Clock Divider 2"]
    #[inline(always)]
    #[must_use]
    pub fn usidiv(&mut self) -> USIDIV_W<5> {
        USIDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USI Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usickctl](index.html) module"]
pub struct USICKCTL_SPEC;
impl crate::RegisterSpec for USICKCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usickctl::R](R) reader structure"]
impl crate::Readable for USICKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usickctl::W](W) writer structure"]
impl crate::Writable for USICKCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USICKCTL to value 0"]
impl crate::Resettable for USICKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
