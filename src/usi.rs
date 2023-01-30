#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USI Control Register 0"]
    pub usictl0: USICTL0,
    #[doc = "0x01 - USI Control Register 1"]
    pub usictl1: USICTL1,
    #[doc = "0x02 - USI Clock Control Register"]
    pub usickctl: USICKCTL,
    #[doc = "0x03 - USI Bit Counter Register"]
    pub usicnt: USICNT,
    #[doc = "0x04 - USI Low Byte Shift Register"]
    pub usisrl: USISRL,
    #[doc = "0x05 - USI High Byte Shift Register"]
    pub usisrh: USISRH,
}
#[doc = "USICTL0 (rw) register accessor: an alias for `Reg<USICTL0_SPEC>`"]
pub type USICTL0 = crate::Reg<usictl0::USICTL0_SPEC>;
#[doc = "USI Control Register 0"]
pub mod usictl0;
#[doc = "USICTL1 (rw) register accessor: an alias for `Reg<USICTL1_SPEC>`"]
pub type USICTL1 = crate::Reg<usictl1::USICTL1_SPEC>;
#[doc = "USI Control Register 1"]
pub mod usictl1;
#[doc = "USICKCTL (rw) register accessor: an alias for `Reg<USICKCTL_SPEC>`"]
pub type USICKCTL = crate::Reg<usickctl::USICKCTL_SPEC>;
#[doc = "USI Clock Control Register"]
pub mod usickctl;
#[doc = "USICNT (rw) register accessor: an alias for `Reg<USICNT_SPEC>`"]
pub type USICNT = crate::Reg<usicnt::USICNT_SPEC>;
#[doc = "USI Bit Counter Register"]
pub mod usicnt;
#[doc = "USISRL (rw) register accessor: an alias for `Reg<USISRL_SPEC>`"]
pub type USISRL = crate::Reg<usisrl::USISRL_SPEC>;
#[doc = "USI Low Byte Shift Register"]
pub mod usisrl;
#[doc = "USISRH (rw) register accessor: an alias for `Reg<USISRH_SPEC>`"]
pub type USISRH = crate::Reg<usisrh::USISRH_SPEC>;
#[doc = "USI High Byte Shift Register"]
pub mod usisrh;
