#[doc = "Register `ADC10CTL1` reader"]
pub struct R(crate::R<ADC10CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10CTL1` writer"]
pub struct W(crate::W<ADC10CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10CTL1_SPEC>;
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
impl From<crate::W<ADC10CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10BUSY` reader - ADC10 BUSY"]
pub type ADC10BUSY_R = crate::BitReader<bool>;
#[doc = "Field `ADC10BUSY` writer - ADC10 BUSY"]
pub type ADC10BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC10CTL1_SPEC, bool, O>;
#[doc = "Field `CONSEQ` reader - ADC10 Conversion Sequence Select 0"]
pub type CONSEQ_R = crate::FieldReader<u8, CONSEQ_A>;
#[doc = "ADC10 Conversion Sequence Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONSEQ_A {
    #[doc = "0: Single channel single conversion"]
    CONSEQ_0 = 0,
    #[doc = "1: Sequence of channels"]
    CONSEQ_1 = 1,
    #[doc = "2: Repeat single channel"]
    CONSEQ_2 = 2,
    #[doc = "3: Repeat sequence of channels"]
    CONSEQ_3 = 3,
}
impl From<CONSEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: CONSEQ_A) -> Self {
        variant as _
    }
}
impl CONSEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONSEQ_A {
        match self.bits {
            0 => CONSEQ_A::CONSEQ_0,
            1 => CONSEQ_A::CONSEQ_1,
            2 => CONSEQ_A::CONSEQ_2,
            3 => CONSEQ_A::CONSEQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSEQ_0`"]
    #[inline(always)]
    pub fn is_conseq_0(&self) -> bool {
        *self == CONSEQ_A::CONSEQ_0
    }
    #[doc = "Checks if the value of the field is `CONSEQ_1`"]
    #[inline(always)]
    pub fn is_conseq_1(&self) -> bool {
        *self == CONSEQ_A::CONSEQ_1
    }
    #[doc = "Checks if the value of the field is `CONSEQ_2`"]
    #[inline(always)]
    pub fn is_conseq_2(&self) -> bool {
        *self == CONSEQ_A::CONSEQ_2
    }
    #[doc = "Checks if the value of the field is `CONSEQ_3`"]
    #[inline(always)]
    pub fn is_conseq_3(&self) -> bool {
        *self == CONSEQ_A::CONSEQ_3
    }
}
#[doc = "Field `CONSEQ` writer - ADC10 Conversion Sequence Select 0"]
pub type CONSEQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADC10CTL1_SPEC, u8, CONSEQ_A, 2, O>;
impl<'a, const O: u8> CONSEQ_W<'a, O> {
    #[doc = "Single channel single conversion"]
    #[inline(always)]
    pub fn conseq_0(self) -> &'a mut W {
        self.variant(CONSEQ_A::CONSEQ_0)
    }
    #[doc = "Sequence of channels"]
    #[inline(always)]
    pub fn conseq_1(self) -> &'a mut W {
        self.variant(CONSEQ_A::CONSEQ_1)
    }
    #[doc = "Repeat single channel"]
    #[inline(always)]
    pub fn conseq_2(self) -> &'a mut W {
        self.variant(CONSEQ_A::CONSEQ_2)
    }
    #[doc = "Repeat sequence of channels"]
    #[inline(always)]
    pub fn conseq_3(self) -> &'a mut W {
        self.variant(CONSEQ_A::CONSEQ_3)
    }
}
#[doc = "Field `ADC10SSEL` reader - ADC10 Clock Source Select Bit: 0"]
pub type ADC10SSEL_R = crate::FieldReader<u8, ADC10SSEL_A>;
#[doc = "ADC10 Clock Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC10SSEL_A {
    #[doc = "0: ADC10OSC"]
    ADC10SSEL_0 = 0,
    #[doc = "1: ACLK"]
    ADC10SSEL_1 = 1,
    #[doc = "2: MCLK"]
    ADC10SSEL_2 = 2,
    #[doc = "3: SMCLK"]
    ADC10SSEL_3 = 3,
}
impl From<ADC10SSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10SSEL_A) -> Self {
        variant as _
    }
}
impl ADC10SSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC10SSEL_A {
        match self.bits {
            0 => ADC10SSEL_A::ADC10SSEL_0,
            1 => ADC10SSEL_A::ADC10SSEL_1,
            2 => ADC10SSEL_A::ADC10SSEL_2,
            3 => ADC10SSEL_A::ADC10SSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC10SSEL_0`"]
    #[inline(always)]
    pub fn is_adc10ssel_0(&self) -> bool {
        *self == ADC10SSEL_A::ADC10SSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC10SSEL_1`"]
    #[inline(always)]
    pub fn is_adc10ssel_1(&self) -> bool {
        *self == ADC10SSEL_A::ADC10SSEL_1
    }
    #[doc = "Checks if the value of the field is `ADC10SSEL_2`"]
    #[inline(always)]
    pub fn is_adc10ssel_2(&self) -> bool {
        *self == ADC10SSEL_A::ADC10SSEL_2
    }
    #[doc = "Checks if the value of the field is `ADC10SSEL_3`"]
    #[inline(always)]
    pub fn is_adc10ssel_3(&self) -> bool {
        *self == ADC10SSEL_A::ADC10SSEL_3
    }
}
#[doc = "Field `ADC10SSEL` writer - ADC10 Clock Source Select Bit: 0"]
pub type ADC10SSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADC10CTL1_SPEC, u8, ADC10SSEL_A, 2, O>;
impl<'a, const O: u8> ADC10SSEL_W<'a, O> {
    #[doc = "ADC10OSC"]
    #[inline(always)]
    pub fn adc10ssel_0(self) -> &'a mut W {
        self.variant(ADC10SSEL_A::ADC10SSEL_0)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn adc10ssel_1(self) -> &'a mut W {
        self.variant(ADC10SSEL_A::ADC10SSEL_1)
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn adc10ssel_2(self) -> &'a mut W {
        self.variant(ADC10SSEL_A::ADC10SSEL_2)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn adc10ssel_3(self) -> &'a mut W {
        self.variant(ADC10SSEL_A::ADC10SSEL_3)
    }
}
#[doc = "Field `ADC10DIV` reader - ADC10 Clock Divider Select Bit: 0"]
pub type ADC10DIV_R = crate::FieldReader<u8, ADC10DIV_A>;
#[doc = "ADC10 Clock Divider Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC10DIV_A {
    #[doc = "0: ADC10 Clock Divider Select 0"]
    ADC10DIV_0 = 0,
    #[doc = "1: ADC10 Clock Divider Select 1"]
    ADC10DIV_1 = 1,
    #[doc = "2: ADC10 Clock Divider Select 2"]
    ADC10DIV_2 = 2,
    #[doc = "3: ADC10 Clock Divider Select 3"]
    ADC10DIV_3 = 3,
    #[doc = "4: ADC10 Clock Divider Select 4"]
    ADC10DIV_4 = 4,
    #[doc = "5: ADC10 Clock Divider Select 5"]
    ADC10DIV_5 = 5,
    #[doc = "6: ADC10 Clock Divider Select 6"]
    ADC10DIV_6 = 6,
    #[doc = "7: ADC10 Clock Divider Select 7"]
    ADC10DIV_7 = 7,
}
impl From<ADC10DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10DIV_A) -> Self {
        variant as _
    }
}
impl ADC10DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC10DIV_A {
        match self.bits {
            0 => ADC10DIV_A::ADC10DIV_0,
            1 => ADC10DIV_A::ADC10DIV_1,
            2 => ADC10DIV_A::ADC10DIV_2,
            3 => ADC10DIV_A::ADC10DIV_3,
            4 => ADC10DIV_A::ADC10DIV_4,
            5 => ADC10DIV_A::ADC10DIV_5,
            6 => ADC10DIV_A::ADC10DIV_6,
            7 => ADC10DIV_A::ADC10DIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_0`"]
    #[inline(always)]
    pub fn is_adc10div_0(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_0
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_1`"]
    #[inline(always)]
    pub fn is_adc10div_1(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_1
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_2`"]
    #[inline(always)]
    pub fn is_adc10div_2(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_2
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_3`"]
    #[inline(always)]
    pub fn is_adc10div_3(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_3
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_4`"]
    #[inline(always)]
    pub fn is_adc10div_4(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_4
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_5`"]
    #[inline(always)]
    pub fn is_adc10div_5(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_5
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_6`"]
    #[inline(always)]
    pub fn is_adc10div_6(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_6
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_7`"]
    #[inline(always)]
    pub fn is_adc10div_7(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_7
    }
}
#[doc = "Field `ADC10DIV` writer - ADC10 Clock Divider Select Bit: 0"]
pub type ADC10DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADC10CTL1_SPEC, u8, ADC10DIV_A, 3, O>;
impl<'a, const O: u8> ADC10DIV_W<'a, O> {
    #[doc = "ADC10 Clock Divider Select 0"]
    #[inline(always)]
    pub fn adc10div_0(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_0)
    }
    #[doc = "ADC10 Clock Divider Select 1"]
    #[inline(always)]
    pub fn adc10div_1(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_1)
    }
    #[doc = "ADC10 Clock Divider Select 2"]
    #[inline(always)]
    pub fn adc10div_2(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_2)
    }
    #[doc = "ADC10 Clock Divider Select 3"]
    #[inline(always)]
    pub fn adc10div_3(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_3)
    }
    #[doc = "ADC10 Clock Divider Select 4"]
    #[inline(always)]
    pub fn adc10div_4(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_4)
    }
    #[doc = "ADC10 Clock Divider Select 5"]
    #[inline(always)]
    pub fn adc10div_5(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_5)
    }
    #[doc = "ADC10 Clock Divider Select 6"]
    #[inline(always)]
    pub fn adc10div_6(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_6)
    }
    #[doc = "ADC10 Clock Divider Select 7"]
    #[inline(always)]
    pub fn adc10div_7(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_7)
    }
}
#[doc = "Field `ISSH` reader - ADC10 Invert Sample Hold Signal"]
pub type ISSH_R = crate::BitReader<bool>;
#[doc = "Field `ISSH` writer - ADC10 Invert Sample Hold Signal"]
pub type ISSH_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC10CTL1_SPEC, bool, O>;
#[doc = "Field `ADC10DF` reader - ADC10 Data Format 0:binary 1:2's complement"]
pub type ADC10DF_R = crate::BitReader<bool>;
#[doc = "Field `ADC10DF` writer - ADC10 Data Format 0:binary 1:2's complement"]
pub type ADC10DF_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC10CTL1_SPEC, bool, O>;
#[doc = "Field `SHS` reader - ADC10 Sample/Hold Source Bit: 0"]
pub type SHS_R = crate::FieldReader<u8, SHS_A>;
#[doc = "ADC10 Sample/Hold Source Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHS_A {
    #[doc = "0: ADC10SC"]
    SHS_0 = 0,
    #[doc = "1: TA3 OUT1"]
    SHS_1 = 1,
    #[doc = "2: TA3 OUT0"]
    SHS_2 = 2,
    #[doc = "3: TA3 OUT2"]
    SHS_3 = 3,
}
impl From<SHS_A> for u8 {
    #[inline(always)]
    fn from(variant: SHS_A) -> Self {
        variant as _
    }
}
impl SHS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHS_A {
        match self.bits {
            0 => SHS_A::SHS_0,
            1 => SHS_A::SHS_1,
            2 => SHS_A::SHS_2,
            3 => SHS_A::SHS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SHS_0`"]
    #[inline(always)]
    pub fn is_shs_0(&self) -> bool {
        *self == SHS_A::SHS_0
    }
    #[doc = "Checks if the value of the field is `SHS_1`"]
    #[inline(always)]
    pub fn is_shs_1(&self) -> bool {
        *self == SHS_A::SHS_1
    }
    #[doc = "Checks if the value of the field is `SHS_2`"]
    #[inline(always)]
    pub fn is_shs_2(&self) -> bool {
        *self == SHS_A::SHS_2
    }
    #[doc = "Checks if the value of the field is `SHS_3`"]
    #[inline(always)]
    pub fn is_shs_3(&self) -> bool {
        *self == SHS_A::SHS_3
    }
}
#[doc = "Field `SHS` writer - ADC10 Sample/Hold Source Bit: 0"]
pub type SHS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, ADC10CTL1_SPEC, u8, SHS_A, 2, O>;
impl<'a, const O: u8> SHS_W<'a, O> {
    #[doc = "ADC10SC"]
    #[inline(always)]
    pub fn shs_0(self) -> &'a mut W {
        self.variant(SHS_A::SHS_0)
    }
    #[doc = "TA3 OUT1"]
    #[inline(always)]
    pub fn shs_1(self) -> &'a mut W {
        self.variant(SHS_A::SHS_1)
    }
    #[doc = "TA3 OUT0"]
    #[inline(always)]
    pub fn shs_2(self) -> &'a mut W {
        self.variant(SHS_A::SHS_2)
    }
    #[doc = "TA3 OUT2"]
    #[inline(always)]
    pub fn shs_3(self) -> &'a mut W {
        self.variant(SHS_A::SHS_3)
    }
}
#[doc = "Field `INCH` reader - ADC10 Input Channel Select Bit: 0"]
pub type INCH_R = crate::FieldReader<u8, INCH_A>;
#[doc = "ADC10 Input Channel Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INCH_A {
    #[doc = "0: Selects Channel 0"]
    INCH_0 = 0,
    #[doc = "1: Selects Channel 1"]
    INCH_1 = 1,
    #[doc = "2: Selects Channel 2"]
    INCH_2 = 2,
    #[doc = "3: Selects Channel 3"]
    INCH_3 = 3,
    #[doc = "4: Selects Channel 4"]
    INCH_4 = 4,
    #[doc = "5: Selects Channel 5"]
    INCH_5 = 5,
    #[doc = "6: Selects Channel 6"]
    INCH_6 = 6,
    #[doc = "7: Selects Channel 7"]
    INCH_7 = 7,
    #[doc = "8: Selects Channel 8"]
    INCH_8 = 8,
    #[doc = "9: Selects Channel 9"]
    INCH_9 = 9,
    #[doc = "10: Selects Channel 10"]
    INCH_10 = 10,
    #[doc = "11: Selects Channel 11"]
    INCH_11 = 11,
    #[doc = "12: Selects Channel 12"]
    INCH_12 = 12,
    #[doc = "13: Selects Channel 13"]
    INCH_13 = 13,
    #[doc = "14: Selects Channel 14"]
    INCH_14 = 14,
    #[doc = "15: Selects Channel 15"]
    INCH_15 = 15,
}
impl From<INCH_A> for u8 {
    #[inline(always)]
    fn from(variant: INCH_A) -> Self {
        variant as _
    }
}
impl INCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INCH_A {
        match self.bits {
            0 => INCH_A::INCH_0,
            1 => INCH_A::INCH_1,
            2 => INCH_A::INCH_2,
            3 => INCH_A::INCH_3,
            4 => INCH_A::INCH_4,
            5 => INCH_A::INCH_5,
            6 => INCH_A::INCH_6,
            7 => INCH_A::INCH_7,
            8 => INCH_A::INCH_8,
            9 => INCH_A::INCH_9,
            10 => INCH_A::INCH_10,
            11 => INCH_A::INCH_11,
            12 => INCH_A::INCH_12,
            13 => INCH_A::INCH_13,
            14 => INCH_A::INCH_14,
            15 => INCH_A::INCH_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INCH_0`"]
    #[inline(always)]
    pub fn is_inch_0(&self) -> bool {
        *self == INCH_A::INCH_0
    }
    #[doc = "Checks if the value of the field is `INCH_1`"]
    #[inline(always)]
    pub fn is_inch_1(&self) -> bool {
        *self == INCH_A::INCH_1
    }
    #[doc = "Checks if the value of the field is `INCH_2`"]
    #[inline(always)]
    pub fn is_inch_2(&self) -> bool {
        *self == INCH_A::INCH_2
    }
    #[doc = "Checks if the value of the field is `INCH_3`"]
    #[inline(always)]
    pub fn is_inch_3(&self) -> bool {
        *self == INCH_A::INCH_3
    }
    #[doc = "Checks if the value of the field is `INCH_4`"]
    #[inline(always)]
    pub fn is_inch_4(&self) -> bool {
        *self == INCH_A::INCH_4
    }
    #[doc = "Checks if the value of the field is `INCH_5`"]
    #[inline(always)]
    pub fn is_inch_5(&self) -> bool {
        *self == INCH_A::INCH_5
    }
    #[doc = "Checks if the value of the field is `INCH_6`"]
    #[inline(always)]
    pub fn is_inch_6(&self) -> bool {
        *self == INCH_A::INCH_6
    }
    #[doc = "Checks if the value of the field is `INCH_7`"]
    #[inline(always)]
    pub fn is_inch_7(&self) -> bool {
        *self == INCH_A::INCH_7
    }
    #[doc = "Checks if the value of the field is `INCH_8`"]
    #[inline(always)]
    pub fn is_inch_8(&self) -> bool {
        *self == INCH_A::INCH_8
    }
    #[doc = "Checks if the value of the field is `INCH_9`"]
    #[inline(always)]
    pub fn is_inch_9(&self) -> bool {
        *self == INCH_A::INCH_9
    }
    #[doc = "Checks if the value of the field is `INCH_10`"]
    #[inline(always)]
    pub fn is_inch_10(&self) -> bool {
        *self == INCH_A::INCH_10
    }
    #[doc = "Checks if the value of the field is `INCH_11`"]
    #[inline(always)]
    pub fn is_inch_11(&self) -> bool {
        *self == INCH_A::INCH_11
    }
    #[doc = "Checks if the value of the field is `INCH_12`"]
    #[inline(always)]
    pub fn is_inch_12(&self) -> bool {
        *self == INCH_A::INCH_12
    }
    #[doc = "Checks if the value of the field is `INCH_13`"]
    #[inline(always)]
    pub fn is_inch_13(&self) -> bool {
        *self == INCH_A::INCH_13
    }
    #[doc = "Checks if the value of the field is `INCH_14`"]
    #[inline(always)]
    pub fn is_inch_14(&self) -> bool {
        *self == INCH_A::INCH_14
    }
    #[doc = "Checks if the value of the field is `INCH_15`"]
    #[inline(always)]
    pub fn is_inch_15(&self) -> bool {
        *self == INCH_A::INCH_15
    }
}
#[doc = "Field `INCH` writer - ADC10 Input Channel Select Bit: 0"]
pub type INCH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADC10CTL1_SPEC, u8, INCH_A, 4, O>;
impl<'a, const O: u8> INCH_W<'a, O> {
    #[doc = "Selects Channel 0"]
    #[inline(always)]
    pub fn inch_0(self) -> &'a mut W {
        self.variant(INCH_A::INCH_0)
    }
    #[doc = "Selects Channel 1"]
    #[inline(always)]
    pub fn inch_1(self) -> &'a mut W {
        self.variant(INCH_A::INCH_1)
    }
    #[doc = "Selects Channel 2"]
    #[inline(always)]
    pub fn inch_2(self) -> &'a mut W {
        self.variant(INCH_A::INCH_2)
    }
    #[doc = "Selects Channel 3"]
    #[inline(always)]
    pub fn inch_3(self) -> &'a mut W {
        self.variant(INCH_A::INCH_3)
    }
    #[doc = "Selects Channel 4"]
    #[inline(always)]
    pub fn inch_4(self) -> &'a mut W {
        self.variant(INCH_A::INCH_4)
    }
    #[doc = "Selects Channel 5"]
    #[inline(always)]
    pub fn inch_5(self) -> &'a mut W {
        self.variant(INCH_A::INCH_5)
    }
    #[doc = "Selects Channel 6"]
    #[inline(always)]
    pub fn inch_6(self) -> &'a mut W {
        self.variant(INCH_A::INCH_6)
    }
    #[doc = "Selects Channel 7"]
    #[inline(always)]
    pub fn inch_7(self) -> &'a mut W {
        self.variant(INCH_A::INCH_7)
    }
    #[doc = "Selects Channel 8"]
    #[inline(always)]
    pub fn inch_8(self) -> &'a mut W {
        self.variant(INCH_A::INCH_8)
    }
    #[doc = "Selects Channel 9"]
    #[inline(always)]
    pub fn inch_9(self) -> &'a mut W {
        self.variant(INCH_A::INCH_9)
    }
    #[doc = "Selects Channel 10"]
    #[inline(always)]
    pub fn inch_10(self) -> &'a mut W {
        self.variant(INCH_A::INCH_10)
    }
    #[doc = "Selects Channel 11"]
    #[inline(always)]
    pub fn inch_11(self) -> &'a mut W {
        self.variant(INCH_A::INCH_11)
    }
    #[doc = "Selects Channel 12"]
    #[inline(always)]
    pub fn inch_12(self) -> &'a mut W {
        self.variant(INCH_A::INCH_12)
    }
    #[doc = "Selects Channel 13"]
    #[inline(always)]
    pub fn inch_13(self) -> &'a mut W {
        self.variant(INCH_A::INCH_13)
    }
    #[doc = "Selects Channel 14"]
    #[inline(always)]
    pub fn inch_14(self) -> &'a mut W {
        self.variant(INCH_A::INCH_14)
    }
    #[doc = "Selects Channel 15"]
    #[inline(always)]
    pub fn inch_15(self) -> &'a mut W {
        self.variant(INCH_A::INCH_15)
    }
}
impl R {
    #[doc = "Bit 0 - ADC10 BUSY"]
    #[inline(always)]
    pub fn adc10busy(&self) -> ADC10BUSY_R {
        ADC10BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - ADC10 Conversion Sequence Select 0"]
    #[inline(always)]
    pub fn conseq(&self) -> CONSEQ_R {
        CONSEQ_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - ADC10 Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn adc10ssel(&self) -> ADC10SSEL_R {
        ADC10SSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - ADC10 Clock Divider Select Bit: 0"]
    #[inline(always)]
    pub fn adc10div(&self) -> ADC10DIV_R {
        ADC10DIV_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - ADC10 Invert Sample Hold Signal"]
    #[inline(always)]
    pub fn issh(&self) -> ISSH_R {
        ISSH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC10 Data Format 0:binary 1:2's complement"]
    #[inline(always)]
    pub fn adc10df(&self) -> ADC10DF_R {
        ADC10DF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - ADC10 Sample/Hold Source Bit: 0"]
    #[inline(always)]
    pub fn shs(&self) -> SHS_R {
        SHS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - ADC10 Input Channel Select Bit: 0"]
    #[inline(always)]
    pub fn inch(&self) -> INCH_R {
        INCH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC10 BUSY"]
    #[inline(always)]
    #[must_use]
    pub fn adc10busy(&mut self) -> ADC10BUSY_W<0> {
        ADC10BUSY_W::new(self)
    }
    #[doc = "Bits 1:2 - ADC10 Conversion Sequence Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn conseq(&mut self) -> CONSEQ_W<1> {
        CONSEQ_W::new(self)
    }
    #[doc = "Bits 3:4 - ADC10 Clock Source Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc10ssel(&mut self) -> ADC10SSEL_W<3> {
        ADC10SSEL_W::new(self)
    }
    #[doc = "Bits 5:7 - ADC10 Clock Divider Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc10div(&mut self) -> ADC10DIV_W<5> {
        ADC10DIV_W::new(self)
    }
    #[doc = "Bit 8 - ADC10 Invert Sample Hold Signal"]
    #[inline(always)]
    #[must_use]
    pub fn issh(&mut self) -> ISSH_W<8> {
        ISSH_W::new(self)
    }
    #[doc = "Bit 9 - ADC10 Data Format 0:binary 1:2's complement"]
    #[inline(always)]
    #[must_use]
    pub fn adc10df(&mut self) -> ADC10DF_W<9> {
        ADC10DF_W::new(self)
    }
    #[doc = "Bits 10:11 - ADC10 Sample/Hold Source Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn shs(&mut self) -> SHS_W<10> {
        SHS_W::new(self)
    }
    #[doc = "Bits 12:15 - ADC10 Input Channel Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn inch(&mut self) -> INCH_W<12> {
        INCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10ctl1](index.html) module"]
pub struct ADC10CTL1_SPEC;
impl crate::RegisterSpec for ADC10CTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10ctl1::R](R) reader structure"]
impl crate::Readable for ADC10CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10ctl1::W](W) writer structure"]
impl crate::Writable for ADC10CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC10CTL1 to value 0"]
impl crate::Resettable for ADC10CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
