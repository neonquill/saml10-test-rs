#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCS Software Lock Access Register"]
    pub dlar: crate::Reg<dlar::DLAR_SPEC>,
    #[doc = "0x04 - SCS Software Lock Status Register"]
    pub dlsr: crate::Reg<dlsr::DLSR_SPEC>,
    #[doc = "0x08 - Debug Authentication Status Register"]
    pub dauthstatus: crate::Reg<dauthstatus::DAUTHSTATUS_SPEC>,
    #[doc = "0x0c - SCS Device Architecture Register"]
    pub ddevarch: crate::Reg<ddevarch::DDEVARCH_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x1c - SCS Device Type Register"]
    pub ddevtype: crate::Reg<ddevtype::DDEVTYPE_SPEC>,
    #[doc = "0x20 - SCS Peripheral Identification Register 4"]
    pub dpidr4: crate::Reg<dpidr4::DPIDR4_SPEC>,
    #[doc = "0x24 - SCS Peripheral Identification Register 5"]
    pub dpidr5: crate::Reg<dpidr5::DPIDR5_SPEC>,
    #[doc = "0x28 - SCS Peripheral Identification Register 6"]
    pub dpidr6: crate::Reg<dpidr6::DPIDR6_SPEC>,
    #[doc = "0x2c - SCS Peripheral Identification Register 7"]
    pub dpidr7: crate::Reg<dpidr7::DPIDR7_SPEC>,
    #[doc = "0x30 - SCS Peripheral Identification Register 0"]
    pub dpidr0: crate::Reg<dpidr0::DPIDR0_SPEC>,
    #[doc = "0x34 - SCS Peripheral Identification Register 1"]
    pub dpidr1: crate::Reg<dpidr1::DPIDR1_SPEC>,
    #[doc = "0x38 - SCS Peripheral Identification Register 2"]
    pub dpidr2: crate::Reg<dpidr2::DPIDR2_SPEC>,
    #[doc = "0x3c - SCS Peripheral Identification Register 3"]
    pub dpidr3: crate::Reg<dpidr3::DPIDR3_SPEC>,
    #[doc = "0x40 - SCS Component Identification Register 0"]
    pub dcidr0: crate::Reg<dcidr0::DCIDR0_SPEC>,
    #[doc = "0x44 - SCS Component Identification Register 1"]
    pub dcidr1: crate::Reg<dcidr1::DCIDR1_SPEC>,
    #[doc = "0x48 - SCS Component Identification Register 2"]
    pub dcidr2: crate::Reg<dcidr2::DCIDR2_SPEC>,
    #[doc = "0x4c - SCS Component Identification Register 3"]
    pub dcidr3: crate::Reg<dcidr3::DCIDR3_SPEC>,
}
#[doc = "DLAR register accessor: an alias for `Reg<DLAR_SPEC>`"]
pub type DLAR = crate::Reg<dlar::DLAR_SPEC>;
#[doc = "SCS Software Lock Access Register"]
pub mod dlar;
#[doc = "DLSR register accessor: an alias for `Reg<DLSR_SPEC>`"]
pub type DLSR = crate::Reg<dlsr::DLSR_SPEC>;
#[doc = "SCS Software Lock Status Register"]
pub mod dlsr;
#[doc = "DAUTHSTATUS register accessor: an alias for `Reg<DAUTHSTATUS_SPEC>`"]
pub type DAUTHSTATUS = crate::Reg<dauthstatus::DAUTHSTATUS_SPEC>;
#[doc = "Debug Authentication Status Register"]
pub mod dauthstatus;
#[doc = "DDEVARCH register accessor: an alias for `Reg<DDEVARCH_SPEC>`"]
pub type DDEVARCH = crate::Reg<ddevarch::DDEVARCH_SPEC>;
#[doc = "SCS Device Architecture Register"]
pub mod ddevarch;
#[doc = "DDEVTYPE register accessor: an alias for `Reg<DDEVTYPE_SPEC>`"]
pub type DDEVTYPE = crate::Reg<ddevtype::DDEVTYPE_SPEC>;
#[doc = "SCS Device Type Register"]
pub mod ddevtype;
#[doc = "DPIDR4 register accessor: an alias for `Reg<DPIDR4_SPEC>`"]
pub type DPIDR4 = crate::Reg<dpidr4::DPIDR4_SPEC>;
#[doc = "SCS Peripheral Identification Register 4"]
pub mod dpidr4;
#[doc = "DPIDR5 register accessor: an alias for `Reg<DPIDR5_SPEC>`"]
pub type DPIDR5 = crate::Reg<dpidr5::DPIDR5_SPEC>;
#[doc = "SCS Peripheral Identification Register 5"]
pub mod dpidr5;
#[doc = "DPIDR6 register accessor: an alias for `Reg<DPIDR6_SPEC>`"]
pub type DPIDR6 = crate::Reg<dpidr6::DPIDR6_SPEC>;
#[doc = "SCS Peripheral Identification Register 6"]
pub mod dpidr6;
#[doc = "DPIDR7 register accessor: an alias for `Reg<DPIDR7_SPEC>`"]
pub type DPIDR7 = crate::Reg<dpidr7::DPIDR7_SPEC>;
#[doc = "SCS Peripheral Identification Register 7"]
pub mod dpidr7;
#[doc = "DPIDR0 register accessor: an alias for `Reg<DPIDR0_SPEC>`"]
pub type DPIDR0 = crate::Reg<dpidr0::DPIDR0_SPEC>;
#[doc = "SCS Peripheral Identification Register 0"]
pub mod dpidr0;
#[doc = "DPIDR1 register accessor: an alias for `Reg<DPIDR1_SPEC>`"]
pub type DPIDR1 = crate::Reg<dpidr1::DPIDR1_SPEC>;
#[doc = "SCS Peripheral Identification Register 1"]
pub mod dpidr1;
#[doc = "DPIDR2 register accessor: an alias for `Reg<DPIDR2_SPEC>`"]
pub type DPIDR2 = crate::Reg<dpidr2::DPIDR2_SPEC>;
#[doc = "SCS Peripheral Identification Register 2"]
pub mod dpidr2;
#[doc = "DPIDR3 register accessor: an alias for `Reg<DPIDR3_SPEC>`"]
pub type DPIDR3 = crate::Reg<dpidr3::DPIDR3_SPEC>;
#[doc = "SCS Peripheral Identification Register 3"]
pub mod dpidr3;
#[doc = "DCIDR0 register accessor: an alias for `Reg<DCIDR0_SPEC>`"]
pub type DCIDR0 = crate::Reg<dcidr0::DCIDR0_SPEC>;
#[doc = "SCS Component Identification Register 0"]
pub mod dcidr0;
#[doc = "DCIDR1 register accessor: an alias for `Reg<DCIDR1_SPEC>`"]
pub type DCIDR1 = crate::Reg<dcidr1::DCIDR1_SPEC>;
#[doc = "SCS Component Identification Register 1"]
pub mod dcidr1;
#[doc = "DCIDR2 register accessor: an alias for `Reg<DCIDR2_SPEC>`"]
pub type DCIDR2 = crate::Reg<dcidr2::DCIDR2_SPEC>;
#[doc = "SCS Component Identification Register 2"]
pub mod dcidr2;
#[doc = "DCIDR3 register accessor: an alias for `Reg<DCIDR3_SPEC>`"]
pub type DCIDR3 = crate::Reg<dcidr3::DCIDR3_SPEC>;
#[doc = "SCS Component Identification Register 3"]
pub mod dcidr3;
