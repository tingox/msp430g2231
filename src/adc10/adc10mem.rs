#[doc = "Register `ADC10MEM` reader"]
pub struct R(crate::R<ADC10MEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10MEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10MEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10MEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10MEM` writer"]
pub struct W(crate::W<ADC10MEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10MEM_SPEC>;
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
impl From<crate::W<ADC10MEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10MEM_SPEC>) -> Self {
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
#[doc = "ADC10 Memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10mem](index.html) module"]
pub struct ADC10MEM_SPEC;
impl crate::RegisterSpec for ADC10MEM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10mem::R](R) reader structure"]
impl crate::Readable for ADC10MEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10mem::W](W) writer structure"]
impl crate::Writable for ADC10MEM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC10MEM to value 0"]
impl crate::Resettable for ADC10MEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
