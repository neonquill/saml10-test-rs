#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Software Event"]
    pub swevt: crate::Reg<swevt::SWEVT_SPEC>,
    #[doc = "0x08 - Priority Control"]
    pub prictrl: crate::Reg<prictrl::PRICTRL_SPEC>,
    _reserved3: [u8; 0x07],
    #[doc = "0x10 - Channel Pending Interrupt"]
    pub intpend: crate::Reg<intpend::INTPEND_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - Interrupt Status"]
    pub intstatus: crate::Reg<intstatus::INTSTATUS_SPEC>,
    #[doc = "0x18 - Busy Channels"]
    pub busych: crate::Reg<busych::BUSYCH_SPEC>,
    #[doc = "0x1c - Ready Users"]
    pub readyusr: crate::Reg<readyusr::READYUSR_SPEC>,
    #[doc = "0x20..0x60 - CHANNEL\\[%s\\]"]
    pub channel: [CHANNEL; 8],
    _reserved8: [u8; 0xc0],
    #[doc = "0x120..0x137 - User Multiplexer n"]
    pub user: [crate::Reg<user::USER_SPEC>; 23],
    _reserved9: [u8; 0x9d],
    #[doc = "0x1d4 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x1d5 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x1d6 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    _reserved12: [u8; 0x01],
    #[doc = "0x1d8 - Channels Security Attribution"]
    pub nonsecchan: crate::Reg<nonsecchan::NONSECCHAN_SPEC>,
    #[doc = "0x1dc - Non-Secure Channels Check"]
    pub nschkchan: crate::Reg<nschkchan::NSCHKCHAN_SPEC>,
    #[doc = "0x1e0 - Users Security Attribution"]
    pub nonsecuser: [crate::Reg<nonsecuser::NONSECUSER_SPEC>; 1],
    _reserved15: [u8; 0x0c],
    #[doc = "0x1f0 - Non-Secure Users Check"]
    pub nschkuser: [crate::Reg<nschkuser::NSCHKUSER_SPEC>; 1],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Channel n Control"]
    pub channel: crate::Reg<self::channel::channel::CHANNEL_SPEC>,
    #[doc = "0x04 - Channel n Interrupt Enable Clear"]
    pub chintenclr: crate::Reg<self::channel::chintenclr::CHINTENCLR_SPEC>,
    #[doc = "0x05 - Channel n Interrupt Enable Set"]
    pub chintenset: crate::Reg<self::channel::chintenset::CHINTENSET_SPEC>,
    #[doc = "0x06 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag: crate::Reg<self::channel::chintflag::CHINTFLAG_SPEC>,
    #[doc = "0x07 - Channel n Status"]
    pub chstatus: crate::Reg<self::channel::chstatus::CHSTATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CHANNEL\\[%s\\]"]
pub mod channel;
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control"]
pub mod ctrla;
#[doc = "SWEVT register accessor: an alias for `Reg<SWEVT_SPEC>`"]
pub type SWEVT = crate::Reg<swevt::SWEVT_SPEC>;
#[doc = "Software Event"]
pub mod swevt;
#[doc = "PRICTRL register accessor: an alias for `Reg<PRICTRL_SPEC>`"]
pub type PRICTRL = crate::Reg<prictrl::PRICTRL_SPEC>;
#[doc = "Priority Control"]
pub mod prictrl;
#[doc = "INTPEND register accessor: an alias for `Reg<INTPEND_SPEC>`"]
pub type INTPEND = crate::Reg<intpend::INTPEND_SPEC>;
#[doc = "Channel Pending Interrupt"]
pub mod intpend;
#[doc = "INTSTATUS register accessor: an alias for `Reg<INTSTATUS_SPEC>`"]
pub type INTSTATUS = crate::Reg<intstatus::INTSTATUS_SPEC>;
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "BUSYCH register accessor: an alias for `Reg<BUSYCH_SPEC>`"]
pub type BUSYCH = crate::Reg<busych::BUSYCH_SPEC>;
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "READYUSR register accessor: an alias for `Reg<READYUSR_SPEC>`"]
pub type READYUSR = crate::Reg<readyusr::READYUSR_SPEC>;
#[doc = "Ready Users"]
pub mod readyusr;
#[doc = "USER register accessor: an alias for `Reg<USER_SPEC>`"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "User Multiplexer n"]
pub mod user;
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
#[doc = "NONSECCHAN register accessor: an alias for `Reg<NONSECCHAN_SPEC>`"]
pub type NONSECCHAN = crate::Reg<nonsecchan::NONSECCHAN_SPEC>;
#[doc = "Channels Security Attribution"]
pub mod nonsecchan;
#[doc = "NSCHKCHAN register accessor: an alias for `Reg<NSCHKCHAN_SPEC>`"]
pub type NSCHKCHAN = crate::Reg<nschkchan::NSCHKCHAN_SPEC>;
#[doc = "Non-Secure Channels Check"]
pub mod nschkchan;
#[doc = "NONSECUSER register accessor: an alias for `Reg<NONSECUSER_SPEC>`"]
pub type NONSECUSER = crate::Reg<nonsecuser::NONSECUSER_SPEC>;
#[doc = "Users Security Attribution"]
pub mod nonsecuser;
#[doc = "NSCHKUSER register accessor: an alias for `Reg<NSCHKUSER_SPEC>`"]
pub type NSCHKUSER = crate::Reg<nschkuser::NSCHKUSER_SPEC>;
#[doc = "Non-Secure Users Check"]
pub mod nschkuser;
