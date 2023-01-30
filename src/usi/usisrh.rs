#[doc = "Register `USISRH` reader"]
pub struct R(crate::R<USISRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USISRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USISRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USISRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USISRH` writer"]
pub struct W(crate::W<USISRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USISRH_SPEC>;
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
impl From<crate::W<USISRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USISRH_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USI High Byte Shift Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usisrh](index.html) module"]
pub struct USISRH_SPEC;
impl crate::RegisterSpec for USISRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usisrh::R](R) reader structure"]
impl crate::Readable for USISRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usisrh::W](W) writer structure"]
impl crate::Writable for USISRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USISRH to value 0"]
impl crate::Resettable for USISRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
