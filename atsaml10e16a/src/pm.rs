#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Sleep Configuration"]
    pub sleepcfg: crate::Reg<sleepcfg::SLEEPCFG_SPEC>,
    #[doc = "0x02 - Performance Level Configuration"]
    pub plcfg: crate::Reg<plcfg::PLCFG_SPEC>,
    #[doc = "0x03 - Power Configuration"]
    pub pwcfg: crate::Reg<pwcfg::PWCFG_SPEC>,
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x08 - Standby Configuration"]
    pub stdbycfg: crate::Reg<stdbycfg::STDBYCFG_SPEC>,
}
#[doc = "SLEEPCFG register accessor: an alias for `Reg<SLEEPCFG_SPEC>`"]
pub type SLEEPCFG = crate::Reg<sleepcfg::SLEEPCFG_SPEC>;
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
#[doc = "PLCFG register accessor: an alias for `Reg<PLCFG_SPEC>`"]
pub type PLCFG = crate::Reg<plcfg::PLCFG_SPEC>;
#[doc = "Performance Level Configuration"]
pub mod plcfg;
#[doc = "PWCFG register accessor: an alias for `Reg<PWCFG_SPEC>`"]
pub type PWCFG = crate::Reg<pwcfg::PWCFG_SPEC>;
#[doc = "Power Configuration"]
pub mod pwcfg;
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
#[doc = "STDBYCFG register accessor: an alias for `Reg<STDBYCFG_SPEC>`"]
pub type STDBYCFG = crate::Reg<stdbycfg::STDBYCFG_SPEC>;
#[doc = "Standby Configuration"]
pub mod stdbycfg;
