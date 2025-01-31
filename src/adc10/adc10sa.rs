#[doc = "Register `ADC10SA` reader"]
pub struct R(crate::R<ADC10SA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10SA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10SA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10SA` writer"]
pub struct W(crate::W<ADC10SA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10SA_SPEC>;
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
impl From<crate::W<ADC10SA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10SA_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Data Transfer Start Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10sa](index.html) module"]
pub struct ADC10SA_SPEC;
impl crate::RegisterSpec for ADC10SA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10sa::R](R) reader structure"]
impl crate::Readable for ADC10SA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10sa::W](W) writer structure"]
impl crate::Writable for ADC10SA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC10SA to value 0"]
impl crate::Resettable for ADC10SA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
