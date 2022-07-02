#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 0>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 1>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: USART with external clock"]
    USART_EXT_CLK = 0,
    #[doc = "1: USART with internal clock"]
    USART_INT_CLK = 1,
    #[doc = "2: SPI in slave operation"]
    SPI_SLAVE = 2,
    #[doc = "3: SPI in master operation"]
    SPI_MASTER = 3,
    #[doc = "4: I2C slave operation"]
    I2C_SLAVE = 4,
    #[doc = "5: I2C master operation"]
    I2C_MASTER = 5,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Operating Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::USART_EXT_CLK),
            1 => Some(MODE_A::USART_INT_CLK),
            2 => Some(MODE_A::SPI_SLAVE),
            3 => Some(MODE_A::SPI_MASTER),
            4 => Some(MODE_A::I2C_SLAVE),
            5 => Some(MODE_A::I2C_MASTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USART_EXT_CLK`"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == MODE_A::USART_EXT_CLK
    }
    #[doc = "Checks if the value of the field is `USART_INT_CLK`"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == MODE_A::USART_INT_CLK
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODE_A::SPI_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == MODE_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE`"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == MODE_A::I2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == MODE_A::I2C_MASTER
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, MODE_A, 3, 2>;
impl<'a> MODE_W<'a> {
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut W {
        self.variant(MODE_A::USART_EXT_CLK)
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut W {
        self.variant(MODE_A::USART_INT_CLK)
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODE_A::SPI_SLAVE)
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODE_A::SPI_MASTER)
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut W {
        self.variant(MODE_A::I2C_SLAVE)
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(MODE_A::I2C_MASTER)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 7>;
#[doc = "Field `PINOUT` reader - Pin Usage"]
pub type PINOUT_R = crate::BitReader<bool>;
#[doc = "Field `PINOUT` writer - Pin Usage"]
pub type PINOUT_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 16>;
#[doc = "SDA Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDAHOLD_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: 50-100ns hold time"]
    _75NS = 1,
    #[doc = "2: 300-600ns hold time"]
    _450NS = 2,
    #[doc = "3: 400-800ns hold time"]
    _600NS = 3,
}
impl From<SDAHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: SDAHOLD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SDAHOLD` reader - SDA Hold Time"]
pub type SDAHOLD_R = crate::FieldReader<u8, SDAHOLD_A>;
impl SDAHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDAHOLD_A {
        match self.bits {
            0 => SDAHOLD_A::DISABLE,
            1 => SDAHOLD_A::_75NS,
            2 => SDAHOLD_A::_450NS,
            3 => SDAHOLD_A::_600NS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDAHOLD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `_75NS`"]
    #[inline(always)]
    pub fn is_75ns(&self) -> bool {
        *self == SDAHOLD_A::_75NS
    }
    #[doc = "Checks if the value of the field is `_450NS`"]
    #[inline(always)]
    pub fn is_450ns(&self) -> bool {
        *self == SDAHOLD_A::_450NS
    }
    #[doc = "Checks if the value of the field is `_600NS`"]
    #[inline(always)]
    pub fn is_600ns(&self) -> bool {
        *self == SDAHOLD_A::_600NS
    }
}
#[doc = "Field `SDAHOLD` writer - SDA Hold Time"]
pub type SDAHOLD_W<'a> =
    crate::FieldWriterSafe<'a, u32, CTRLA_SPEC, u8, SDAHOLD_A, 2, 20>;
impl<'a> SDAHOLD_W<'a> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDAHOLD_A::DISABLE)
    }
    #[doc = "50-100ns hold time"]
    #[inline(always)]
    pub fn _75ns(self) -> &'a mut W {
        self.variant(SDAHOLD_A::_75NS)
    }
    #[doc = "300-600ns hold time"]
    #[inline(always)]
    pub fn _450ns(self) -> &'a mut W {
        self.variant(SDAHOLD_A::_450NS)
    }
    #[doc = "400-800ns hold time"]
    #[inline(always)]
    pub fn _600ns(self) -> &'a mut W {
        self.variant(SDAHOLD_A::_600NS)
    }
}
#[doc = "Field `SEXTTOEN` reader - Slave SCL Low Extend Timeout"]
pub type SEXTTOEN_R = crate::BitReader<bool>;
#[doc = "Field `SEXTTOEN` writer - Slave SCL Low Extend Timeout"]
pub type SEXTTOEN_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 23>;
#[doc = "Transfer Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: Standard Mode(Sm) Upto 100kHz and Fast Mode(Fm) Upto 400kHz"]
    STANDARD_AND_FAST_MODE = 0,
    #[doc = "1: Fast-mode Plus Upto 1MHz"]
    FASTPLUS_MODE = 1,
    #[doc = "2: High-speed mode Upto 3.4MHz"]
    HIGH_SPEED_MODE = 2,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPEED` reader - Transfer Speed"]
pub type SPEED_R = crate::FieldReader<u8, SPEED_A>;
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPEED_A> {
        match self.bits {
            0 => Some(SPEED_A::STANDARD_AND_FAST_MODE),
            1 => Some(SPEED_A::FASTPLUS_MODE),
            2 => Some(SPEED_A::HIGH_SPEED_MODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_AND_FAST_MODE`"]
    #[inline(always)]
    pub fn is_standard_and_fast_mode(&self) -> bool {
        *self == SPEED_A::STANDARD_AND_FAST_MODE
    }
    #[doc = "Checks if the value of the field is `FASTPLUS_MODE`"]
    #[inline(always)]
    pub fn is_fastplus_mode(&self) -> bool {
        *self == SPEED_A::FASTPLUS_MODE
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED_MODE`"]
    #[inline(always)]
    pub fn is_high_speed_mode(&self) -> bool {
        *self == SPEED_A::HIGH_SPEED_MODE
    }
}
#[doc = "Field `SPEED` writer - Transfer Speed"]
pub type SPEED_W<'a> =
    crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, SPEED_A, 2, 24>;
impl<'a> SPEED_W<'a> {
    #[doc = "Standard Mode(Sm) Upto 100kHz and Fast Mode(Fm) Upto 400kHz"]
    #[inline(always)]
    pub fn standard_and_fast_mode(self) -> &'a mut W {
        self.variant(SPEED_A::STANDARD_AND_FAST_MODE)
    }
    #[doc = "Fast-mode Plus Upto 1MHz"]
    #[inline(always)]
    pub fn fastplus_mode(self) -> &'a mut W {
        self.variant(SPEED_A::FASTPLUS_MODE)
    }
    #[doc = "High-speed mode Upto 3.4MHz"]
    #[inline(always)]
    pub fn high_speed_mode(self) -> &'a mut W {
        self.variant(SPEED_A::HIGH_SPEED_MODE)
    }
}
#[doc = "Field `SCLSM` reader - SCL Clock Stretch Mode"]
pub type SCLSM_R = crate::BitReader<bool>;
#[doc = "Field `SCLSM` writer - SCL Clock Stretch Mode"]
pub type SCLSM_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 27>;
#[doc = "Field `LOWTOUTEN` reader - SCL Low Timeout Enable"]
pub type LOWTOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `LOWTOUTEN` writer - SCL Low Timeout Enable"]
pub type LOWTOUTEN_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 30>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    pub fn pinout(&self) -> PINOUT_R {
        PINOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&self) -> SDAHOLD_R {
        SDAHOLD_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn sexttoen(&self) -> SEXTTOEN_R {
        SEXTTOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    pub fn sclsm(&self) -> SCLSM_R {
        SCLSM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline(always)]
    pub fn lowtouten(&self) -> LOWTOUTEN_R {
        LOWTOUTEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    pub fn pinout(&mut self) -> PINOUT_W {
        PINOUT_W::new(self)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&mut self) -> SDAHOLD_W {
        SDAHOLD_W::new(self)
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn sexttoen(&mut self) -> SEXTTOEN_W {
        SEXTTOEN_W::new(self)
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W::new(self)
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    pub fn sclsm(&mut self) -> SCLSM_W {
        SCLSM_W::new(self)
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline(always)]
    pub fn lowtouten(&mut self) -> LOWTOUTEN_W {
        LOWTOUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
