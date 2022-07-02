#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Debug Halting Control and Status Register"]
    pub dhcsr: crate::Reg<dhcsr::DHCSR_SPEC>,
    #[doc = "0x04 - Debug Core Register Select Register"]
    pub dcrsr: crate::Reg<dcrsr::DCRSR_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Debug Exception and Monitor Control Register"]
    pub demcr: crate::Reg<demcr::DEMCR_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x18 - Debug Security Control and Status Register"]
    pub dscsr: crate::Reg<dscsr::DSCSR_SPEC>,
}
#[doc = "DHCSR register accessor: an alias for `Reg<DHCSR_SPEC>`"]
pub type DHCSR = crate::Reg<dhcsr::DHCSR_SPEC>;
#[doc = "Debug Halting Control and Status Register"]
pub mod dhcsr;
#[doc = "DCRSR register accessor: an alias for `Reg<DCRSR_SPEC>`"]
pub type DCRSR = crate::Reg<dcrsr::DCRSR_SPEC>;
#[doc = "Debug Core Register Select Register"]
pub mod dcrsr;
#[doc = "DEMCR register accessor: an alias for `Reg<DEMCR_SPEC>`"]
pub type DEMCR = crate::Reg<demcr::DEMCR_SPEC>;
#[doc = "Debug Exception and Monitor Control Register"]
pub mod demcr;
#[doc = "DSCSR register accessor: an alias for `Reg<DSCSR_SPEC>`"]
pub type DSCSR = crate::Reg<dscsr::DSCSR_SPEC>;
#[doc = "Debug Security Control and Status Register"]
pub mod dscsr;
