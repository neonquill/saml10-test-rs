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
#[doc = "Field `IBON` reader - Immediate Buffer Overflow Notification"]
pub type IBON_R = crate::BitReader<bool>;
#[doc = "Field `IBON` writer - Immediate Buffer Overflow Notification"]
pub type IBON_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, 8>;
#[doc = "Data Out Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DOPO_A {
    #[doc = "0: DO on PAD\\[0\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    PAD0 = 0,
    #[doc = "1: DO on PAD\\[2\\], SCK on PAD\\[3\\]
and SS on PAD\\[1\\]"]
    PAD1 = 1,
    #[doc = "2: DO on PAD\\[3\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    PAD2 = 2,
    #[doc = "3: DO on PAD\\[0\\], SCK on PAD\\[3\\]
and SS on PAD\\[1\\]"]
    PAD3 = 3,
}
impl From<DOPO_A> for u8 {
    #[inline(always)]
    fn from(variant: DOPO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DOPO` reader - Data Out Pinout"]
pub type DOPO_R = crate::FieldReader<u8, DOPO_A>;
impl DOPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOPO_A {
        match self.bits {
            0 => DOPO_A::PAD0,
            1 => DOPO_A::PAD1,
            2 => DOPO_A::PAD2,
            3 => DOPO_A::PAD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == DOPO_A::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == DOPO_A::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == DOPO_A::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == DOPO_A::PAD3
    }
}
#[doc = "Field `DOPO` writer - Data Out Pinout"]
pub type DOPO_W<'a> =
    crate::FieldWriterSafe<'a, u32, CTRLA_SPEC, u8, DOPO_A, 2, 16>;
impl<'a> DOPO_W<'a> {
    #[doc = "DO on PAD\\[0\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(DOPO_A::PAD0)
    }
    #[doc = "DO on PAD\\[2\\], SCK on PAD\\[3\\]
and SS on PAD\\[1\\]"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut W {
        self.variant(DOPO_A::PAD1)
    }
    #[doc = "DO on PAD\\[3\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(DOPO_A::PAD2)
    }
    #[doc = "DO on PAD\\[0\\], SCK on PAD\\[3\\]
and SS on PAD\\[1\\]"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut W {
        self.variant(DOPO_A::PAD3)
    }
}
#[doc = "Data In Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIPO_A {
    #[doc = "0: SERCOM PAD\\[0\\]"]
    PAD0 = 0,
    #[doc = "1: SERCOM PAD\\[1\\]"]
    PAD1 = 1,
    #[doc = "2: SERCOM PAD\\[2\\]"]
    PAD2 = 2,
    #[doc = "3: SERCOM PAD\\[3\\]"]
    PAD3 = 3,
}
impl From<DIPO_A> for u8 {
    #[inline(always)]
    fn from(variant: DIPO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIPO` reader - Data In Pinout"]
pub type DIPO_R = crate::FieldReader<u8, DIPO_A>;
impl DIPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIPO_A {
        match self.bits {
            0 => DIPO_A::PAD0,
            1 => DIPO_A::PAD1,
            2 => DIPO_A::PAD2,
            3 => DIPO_A::PAD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == DIPO_A::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == DIPO_A::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == DIPO_A::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == DIPO_A::PAD3
    }
}
#[doc = "Field `DIPO` writer - Data In Pinout"]
pub type DIPO_W<'a> =
    crate::FieldWriterSafe<'a, u32, CTRLA_SPEC, u8, DIPO_A, 2, 20>;
impl<'a> DIPO_W<'a> {
    #[doc = "SERCOM PAD\\[0\\]"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(DIPO_A::PAD0)
    }
    #[doc = "SERCOM PAD\\[1\\]"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut W {
        self.variant(DIPO_A::PAD1)
    }
    #[doc = "SERCOM PAD\\[2\\]"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(DIPO_A::PAD2)
    }
    #[doc = "SERCOM PAD\\[3\\]"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut W {
        self.variant(DIPO_A::PAD3)
    }
}
#[doc = "Frame Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORM_A {
    #[doc = "0: SPI Frame"]
    SPI_FRAME = 0,
    #[doc = "2: SPI Frame with Addr"]
    SPI_FRAME_WITH_ADDR = 2,
}
impl From<FORM_A> for u8 {
    #[inline(always)]
    fn from(variant: FORM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FORM` reader - Frame Format"]
pub type FORM_R = crate::FieldReader<u8, FORM_A>;
impl FORM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORM_A> {
        match self.bits {
            0 => Some(FORM_A::SPI_FRAME),
            2 => Some(FORM_A::SPI_FRAME_WITH_ADDR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_FRAME`"]
    #[inline(always)]
    pub fn is_spi_frame(&self) -> bool {
        *self == FORM_A::SPI_FRAME
    }
    #[doc = "Checks if the value of the field is `SPI_FRAME_WITH_ADDR`"]
    #[inline(always)]
    pub fn is_spi_frame_with_addr(&self) -> bool {
        *self == FORM_A::SPI_FRAME_WITH_ADDR
    }
}
#[doc = "Field `FORM` writer - Frame Format"]
pub type FORM_W<'a> =
    crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, FORM_A, 4, 24>;
impl<'a> FORM_W<'a> {
    #[doc = "SPI Frame"]
    #[inline(always)]
    pub fn spi_frame(self) -> &'a mut W {
        self.variant(FORM_A::SPI_FRAME)
    }
    #[doc = "SPI Frame with Addr"]
    #[inline(always)]
    pub fn spi_frame_with_addr(self) -> &'a mut W {
        self.variant(FORM_A::SPI_FRAME_WITH_ADDR)
    }
}
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    LEADING_EDGE = 0,
    #[doc = "1: The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    TRAILING_EDGE = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::LEADING_EDGE,
            true => CPHA_A::TRAILING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEADING_EDGE`"]
    #[inline(always)]
    pub fn is_leading_edge(&self) -> bool {
        *self == CPHA_A::LEADING_EDGE
    }
    #[doc = "Checks if the value of the field is `TRAILING_EDGE`"]
    #[inline(always)]
    pub fn is_trailing_edge(&self) -> bool {
        *self == CPHA_A::TRAILING_EDGE
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CPHA_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, CPHA_A, 28>;
impl<'a> CPHA_W<'a> {
    #[doc = "The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    #[inline(always)]
    pub fn leading_edge(self) -> &'a mut W {
        self.variant(CPHA_A::LEADING_EDGE)
    }
    #[doc = "The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    #[inline(always)]
    pub fn trailing_edge(self) -> &'a mut W {
        self.variant(CPHA_A::TRAILING_EDGE)
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: SCK is low when idle"]
    IDLE_LOW = 0,
    #[doc = "1: SCK is high when idle"]
    IDLE_HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::IDLE_LOW,
            true => CPOL_A::IDLE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_LOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOL_A::IDLE_LOW
    }
    #[doc = "Checks if the value of the field is `IDLE_HIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOL_A::IDLE_HIGH
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, CPOL_A, 29>;
impl<'a> CPOL_W<'a> {
    #[doc = "SCK is low when idle"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOL_A::IDLE_LOW)
    }
    #[doc = "SCK is high when idle"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOL_A::IDLE_HIGH)
    }
}
#[doc = "Data Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DORD_A {
    #[doc = "0: MSB is transferred first"]
    MSB = 0,
    #[doc = "1: LSB is transferred first"]
    LSB = 1,
}
impl From<DORD_A> for bool {
    #[inline(always)]
    fn from(variant: DORD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DORD` reader - Data Order"]
pub type DORD_R = crate::BitReader<DORD_A>;
impl DORD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DORD_A {
        match self.bits {
            false => DORD_A::MSB,
            true => DORD_A::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == DORD_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == DORD_A::LSB
    }
}
#[doc = "Field `DORD` writer - Data Order"]
pub type DORD_W<'a> = crate::BitWriter<'a, u32, CTRLA_SPEC, DORD_A, 30>;
impl<'a> DORD_W<'a> {
    #[doc = "MSB is transferred first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(DORD_A::MSB)
    }
    #[doc = "LSB is transferred first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(DORD_A::LSB)
    }
}
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
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&self) -> IBON_R {
        IBON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    pub fn dopo(&self) -> DOPO_R {
        DOPO_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    pub fn dipo(&self) -> DIPO_R {
        DIPO_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FORM_R {
        FORM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DORD_R {
        DORD_R::new(((self.bits >> 30) & 1) != 0)
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
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&mut self) -> IBON_W {
        IBON_W::new(self)
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    pub fn dopo(&mut self) -> DOPO_W {
        DOPO_W::new(self)
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    pub fn dipo(&mut self) -> DIPO_W {
        DIPO_W::new(self)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&mut self) -> FORM_W {
        FORM_W::new(self)
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W::new(self)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W::new(self)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&mut self) -> DORD_W {
        DORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIS Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
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
