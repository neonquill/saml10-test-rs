#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x07 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Synchronization Busy Status"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x0c - Data Scramble Control"]
    pub dscc: crate::Reg<dscc::DSCC_SPEC>,
    _reserved7: [u8; 0xf0],
    _reserved_7_ram: [u8; 0x0100],
}
impl RegisterBlock {
    #[doc = "0x100..0x200 - TrustRAM"]
    #[inline(always)]
    pub fn ram_halfword_mode(
        &self,
    ) -> &[crate::Reg<ram_halfword_mode::RAM_HALFWORD_MODE_SPEC>; 64] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(256usize)
                as *const [crate::Reg<
                    ram_halfword_mode::RAM_HALFWORD_MODE_SPEC,
                >; 64])
        }
    }
    #[doc = "0x100..0x200 - TrustRAM"]
    #[inline(always)]
    pub fn ram_byte_mode(
        &self,
    ) -> &[crate::Reg<ram_byte_mode::RAM_BYTE_MODE_SPEC>; 64] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(256usize)
                as *const [crate::Reg<ram_byte_mode::RAM_BYTE_MODE_SPEC>; 64])
        }
    }
    #[doc = "0x100..0x200 - TrustRAM"]
    #[inline(always)]
    pub fn ram(&self) -> &[crate::Reg<ram::RAM_SPEC>; 64] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(256usize)
                as *const [crate::Reg<ram::RAM_SPEC>; 64])
        }
    }
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control"]
pub mod ctrla;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Status"]
pub mod syncbusy;
#[doc = "DSCC register accessor: an alias for `Reg<DSCC_SPEC>`"]
pub type DSCC = crate::Reg<dscc::DSCC_SPEC>;
#[doc = "Data Scramble Control"]
pub mod dscc;
#[doc = "RAM register accessor: an alias for `Reg<RAM_SPEC>`"]
pub type RAM = crate::Reg<ram::RAM_SPEC>;
#[doc = "TrustRAM"]
pub mod ram;
#[doc = "RAM_BYTE_MODE register accessor: an alias for `Reg<RAM_BYTE_MODE_SPEC>`"]
pub type RAM_BYTE_MODE = crate::Reg<ram_byte_mode::RAM_BYTE_MODE_SPEC>;
#[doc = "TrustRAM"]
pub mod ram_byte_mode;
#[doc = "RAM_HALFWORD_MODE register accessor: an alias for `Reg<RAM_HALFWORD_MODE_SPEC>`"]
pub type RAM_HALFWORD_MODE =
    crate::Reg<ram_halfword_mode::RAM_HALFWORD_MODE_SPEC>;
#[doc = "TrustRAM"]
pub mod ram_halfword_mode;
