#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x08 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x0c - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x10 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x14 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    pub xoscctrl: crate::Reg<xoscctrl::XOSCCTRL_SPEC>,
    #[doc = "0x16 - Clock Failure Detector Prescaler"]
    pub cfdpresc: crate::Reg<cfdpresc::CFDPRESC_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x18 - 16MHz Internal Oscillator (OSC16M) Control"]
    pub osc16mctrl: crate::Reg<osc16mctrl::OSC16MCTRL_SPEC>,
    _reserved8: [u8; 0x03],
    #[doc = "0x1c - DFLLULP Control"]
    pub dfllulpctrl: crate::Reg<dfllulpctrl::DFLLULPCTRL_SPEC>,
    #[doc = "0x1e - DFLLULP Dither Control"]
    pub dfllulpdither: crate::Reg<dfllulpdither::DFLLULPDITHER_SPEC>,
    #[doc = "0x1f - DFLLULP Read Request"]
    pub dfllulprreq: crate::Reg<dfllulprreq::DFLLULPRREQ_SPEC>,
    #[doc = "0x20 - DFLLULP Delay Value"]
    pub dfllulpdly: crate::Reg<dfllulpdly::DFLLULPDLY_SPEC>,
    #[doc = "0x24 - DFLLULP Target Ratio"]
    pub dfllulpratio: crate::Reg<dfllulpratio::DFLLULPRATIO_SPEC>,
    #[doc = "0x28 - DFLLULP Synchronization Busy"]
    pub dfllulpsyncbusy: crate::Reg<dfllulpsyncbusy::DFLLULPSYNCBUSY_SPEC>,
    #[doc = "0x2c - DPLL Control A"]
    pub dpllctrla: crate::Reg<dpllctrla::DPLLCTRLA_SPEC>,
    _reserved15: [u8; 0x03],
    #[doc = "0x30 - DPLL Ratio Control"]
    pub dpllratio: crate::Reg<dpllratio::DPLLRATIO_SPEC>,
    #[doc = "0x34 - DPLL Control B"]
    pub dpllctrlb: crate::Reg<dpllctrlb::DPLLCTRLB_SPEC>,
    #[doc = "0x38 - DPLL Prescaler"]
    pub dpllpresc: crate::Reg<dpllpresc::DPLLPRESC_SPEC>,
    _reserved18: [u8; 0x03],
    #[doc = "0x3c - DPLL Synchronization Busy"]
    pub dpllsyncbusy: crate::Reg<dpllsyncbusy::DPLLSYNCBUSY_SPEC>,
    _reserved19: [u8; 0x03],
    #[doc = "0x40 - DPLL Status"]
    pub dpllstatus: crate::Reg<dpllstatus::DPLLSTATUS_SPEC>,
}
#[doc = "EVCTRL register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "XOSCCTRL register accessor: an alias for `Reg<XOSCCTRL_SPEC>`"]
pub type XOSCCTRL = crate::Reg<xoscctrl::XOSCCTRL_SPEC>;
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xoscctrl;
#[doc = "CFDPRESC register accessor: an alias for `Reg<CFDPRESC_SPEC>`"]
pub type CFDPRESC = crate::Reg<cfdpresc::CFDPRESC_SPEC>;
#[doc = "Clock Failure Detector Prescaler"]
pub mod cfdpresc;
#[doc = "OSC16MCTRL register accessor: an alias for `Reg<OSC16MCTRL_SPEC>`"]
pub type OSC16MCTRL = crate::Reg<osc16mctrl::OSC16MCTRL_SPEC>;
#[doc = "16MHz Internal Oscillator (OSC16M) Control"]
pub mod osc16mctrl;
#[doc = "DFLLULPCTRL register accessor: an alias for `Reg<DFLLULPCTRL_SPEC>`"]
pub type DFLLULPCTRL = crate::Reg<dfllulpctrl::DFLLULPCTRL_SPEC>;
#[doc = "DFLLULP Control"]
pub mod dfllulpctrl;
#[doc = "DFLLULPDITHER register accessor: an alias for `Reg<DFLLULPDITHER_SPEC>`"]
pub type DFLLULPDITHER = crate::Reg<dfllulpdither::DFLLULPDITHER_SPEC>;
#[doc = "DFLLULP Dither Control"]
pub mod dfllulpdither;
#[doc = "DFLLULPRREQ register accessor: an alias for `Reg<DFLLULPRREQ_SPEC>`"]
pub type DFLLULPRREQ = crate::Reg<dfllulprreq::DFLLULPRREQ_SPEC>;
#[doc = "DFLLULP Read Request"]
pub mod dfllulprreq;
#[doc = "DFLLULPDLY register accessor: an alias for `Reg<DFLLULPDLY_SPEC>`"]
pub type DFLLULPDLY = crate::Reg<dfllulpdly::DFLLULPDLY_SPEC>;
#[doc = "DFLLULP Delay Value"]
pub mod dfllulpdly;
#[doc = "DFLLULPRATIO register accessor: an alias for `Reg<DFLLULPRATIO_SPEC>`"]
pub type DFLLULPRATIO = crate::Reg<dfllulpratio::DFLLULPRATIO_SPEC>;
#[doc = "DFLLULP Target Ratio"]
pub mod dfllulpratio;
#[doc = "DFLLULPSYNCBUSY register accessor: an alias for `Reg<DFLLULPSYNCBUSY_SPEC>`"]
pub type DFLLULPSYNCBUSY = crate::Reg<dfllulpsyncbusy::DFLLULPSYNCBUSY_SPEC>;
#[doc = "DFLLULP Synchronization Busy"]
pub mod dfllulpsyncbusy;
#[doc = "DPLLCTRLA register accessor: an alias for `Reg<DPLLCTRLA_SPEC>`"]
pub type DPLLCTRLA = crate::Reg<dpllctrla::DPLLCTRLA_SPEC>;
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLLRATIO register accessor: an alias for `Reg<DPLLRATIO_SPEC>`"]
pub type DPLLRATIO = crate::Reg<dpllratio::DPLLRATIO_SPEC>;
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLLCTRLB register accessor: an alias for `Reg<DPLLCTRLB_SPEC>`"]
pub type DPLLCTRLB = crate::Reg<dpllctrlb::DPLLCTRLB_SPEC>;
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLLPRESC register accessor: an alias for `Reg<DPLLPRESC_SPEC>`"]
pub type DPLLPRESC = crate::Reg<dpllpresc::DPLLPRESC_SPEC>;
#[doc = "DPLL Prescaler"]
pub mod dpllpresc;
#[doc = "DPLLSYNCBUSY register accessor: an alias for `Reg<DPLLSYNCBUSY_SPEC>`"]
pub type DPLLSYNCBUSY = crate::Reg<dpllsyncbusy::DPLLSYNCBUSY_SPEC>;
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLLSTATUS register accessor: an alias for `Reg<DPLLSTATUS_SPEC>`"]
pub type DPLLSTATUS = crate::Reg<dpllstatus::DPLLSTATUS_SPEC>;
#[doc = "DPLL Status"]
pub mod dpllstatus;
