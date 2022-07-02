#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved2: [u8; 0x01],
    #[doc = "0x04 - OPAMP0 Control"]
    pub opampctrl0: crate::Reg<opampctrl0::OPAMPCTRL0_SPEC>,
    #[doc = "0x08 - OPAMP1 Control"]
    pub opampctrl1: crate::Reg<opampctrl1::OPAMPCTRL1_SPEC>,
    #[doc = "0x0c - OPAMP2 Control"]
    pub opampctrl2: crate::Reg<opampctrl2::OPAMPCTRL2_SPEC>,
    #[doc = "0x10 - Resister Control"]
    pub resctrl: crate::Reg<resctrl::RESCTRL_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "OPAMPCTRL0 register accessor: an alias for `Reg<OPAMPCTRL0_SPEC>`"]
pub type OPAMPCTRL0 = crate::Reg<opampctrl0::OPAMPCTRL0_SPEC>;
#[doc = "OPAMP0 Control"]
pub mod opampctrl0;
#[doc = "OPAMPCTRL1 register accessor: an alias for `Reg<OPAMPCTRL1_SPEC>`"]
pub type OPAMPCTRL1 = crate::Reg<opampctrl1::OPAMPCTRL1_SPEC>;
#[doc = "OPAMP1 Control"]
pub mod opampctrl1;
#[doc = "OPAMPCTRL2 register accessor: an alias for `Reg<OPAMPCTRL2_SPEC>`"]
pub type OPAMPCTRL2 = crate::Reg<opampctrl2::OPAMPCTRL2_SPEC>;
#[doc = "OPAMP2 Control"]
pub mod opampctrl2;
#[doc = "RESCTRL register accessor: an alias for `Reg<RESCTRL_SPEC>`"]
pub type RESCTRL = crate::Reg<resctrl::RESCTRL_SPEC>;
#[doc = "Resister Control"]
pub mod resctrl;
