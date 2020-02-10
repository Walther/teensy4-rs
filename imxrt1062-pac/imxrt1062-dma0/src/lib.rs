#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Error Status Register"]
    pub es: ES,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: ERQ,
    _reserved3: [u8; 4usize],
    #[doc = "0x14 - Enable Error Interrupt Register"]
    pub eei: EEI,
    #[doc = "0x18 - Clear Enable Error Interrupt Register"]
    pub ceei: CEEI,
    #[doc = "0x19 - Set Enable Error Interrupt Register"]
    pub seei: SEEI,
    #[doc = "0x1a - Clear Enable Request Register"]
    pub cerq: CERQ,
    #[doc = "0x1b - Set Enable Request Register"]
    pub serq: SERQ,
    #[doc = "0x1c - Clear DONE Status Bit Register"]
    pub cdne: CDNE,
    #[doc = "0x1d - Set START Bit Register"]
    pub ssrt: SSRT,
    #[doc = "0x1e - Clear Error Register"]
    pub cerr: CERR,
    #[doc = "0x1f - Clear Interrupt Request Register"]
    pub cint: CINT,
    _reserved12: [u8; 4usize],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: INT,
    _reserved13: [u8; 4usize],
    #[doc = "0x2c - Error Register"]
    pub err: ERR,
    _reserved14: [u8; 4usize],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: HRS,
    _reserved15: [u8; 12usize],
    #[doc = "0x44 - Enable Asynchronous Request in Stop Register"]
    pub ears: EARS,
    _reserved16: [u8; 184usize],
    #[doc = "0x100 - Channel n Priority Register"]
    pub dchpri3: DCHPRI,
    #[doc = "0x101 - Channel n Priority Register"]
    pub dchpri2: DCHPRI,
    #[doc = "0x102 - Channel n Priority Register"]
    pub dchpri1: DCHPRI,
    #[doc = "0x103 - Channel n Priority Register"]
    pub dchpri0: DCHPRI,
    #[doc = "0x104 - Channel n Priority Register"]
    pub dchpri7: DCHPRI,
    #[doc = "0x105 - Channel n Priority Register"]
    pub dchpri6: DCHPRI,
    #[doc = "0x106 - Channel n Priority Register"]
    pub dchpri5: DCHPRI,
    #[doc = "0x107 - Channel n Priority Register"]
    pub dchpri4: DCHPRI,
    #[doc = "0x108 - Channel n Priority Register"]
    pub dchpri11: DCHPRI,
    #[doc = "0x109 - Channel n Priority Register"]
    pub dchpri10: DCHPRI,
    #[doc = "0x10a - Channel n Priority Register"]
    pub dchpri9: DCHPRI,
    #[doc = "0x10b - Channel n Priority Register"]
    pub dchpri8: DCHPRI,
    #[doc = "0x10c - Channel n Priority Register"]
    pub dchpri15: DCHPRI,
    #[doc = "0x10d - Channel n Priority Register"]
    pub dchpri14: DCHPRI,
    #[doc = "0x10e - Channel n Priority Register"]
    pub dchpri13: DCHPRI,
    #[doc = "0x10f - Channel n Priority Register"]
    pub dchpri12: DCHPRI,
    #[doc = "0x110 - Channel n Priority Register"]
    pub dchpri19: DCHPRI,
    #[doc = "0x111 - Channel n Priority Register"]
    pub dchpri18: DCHPRI,
    #[doc = "0x112 - Channel n Priority Register"]
    pub dchpri17: DCHPRI,
    #[doc = "0x113 - Channel n Priority Register"]
    pub dchpri16: DCHPRI,
    #[doc = "0x114 - Channel n Priority Register"]
    pub dchpri23: DCHPRI,
    #[doc = "0x115 - Channel n Priority Register"]
    pub dchpri22: DCHPRI,
    #[doc = "0x116 - Channel n Priority Register"]
    pub dchpri21: DCHPRI,
    #[doc = "0x117 - Channel n Priority Register"]
    pub dchpri20: DCHPRI,
    #[doc = "0x118 - Channel n Priority Register"]
    pub dchpri27: DCHPRI,
    #[doc = "0x119 - Channel n Priority Register"]
    pub dchpri26: DCHPRI,
    #[doc = "0x11a - Channel n Priority Register"]
    pub dchpri25: DCHPRI,
    #[doc = "0x11b - Channel n Priority Register"]
    pub dchpri24: DCHPRI,
    #[doc = "0x11c - Channel n Priority Register"]
    pub dchpri31: DCHPRI,
    #[doc = "0x11d - Channel n Priority Register"]
    pub dchpri30: DCHPRI,
    #[doc = "0x11e - Channel n Priority Register"]
    pub dchpri29: DCHPRI,
    #[doc = "0x11f - Channel n Priority Register"]
    pub dchpri28: DCHPRI,
    _reserved48: [u8; 3808usize],
    #[doc = "0x1000 - Transfer Control Descriptor"]
    pub tcd: [TCD; 32],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TCD {
    #[doc = "0x00 - TCD Source Address"]
    pub saddr: self::tcd::SADDR,
    #[doc = "0x04 - TCD Signed Source Address Offset"]
    pub soff: self::tcd::SOFF,
    #[doc = "0x06 - TCD Transfer Attributes"]
    pub attr: self::tcd::ATTR,
    _reserved_3_nbytes: [u8; 4usize],
    #[doc = "0x0c - TCD Last Source Address Adjustment"]
    pub slast: self::tcd::SLAST,
    #[doc = "0x10 - TCD Destination Address"]
    pub daddr: self::tcd::DADDR,
    #[doc = "0x14 - TCD Signed Destination Address Offset"]
    pub doff: self::tcd::DOFF,
    _reserved_7_citer: [u8; 2usize],
    #[doc = "0x18 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub dlastsga: self::tcd::DLASTSGA,
    #[doc = "0x1c - TCD Control and Status"]
    pub csr: self::tcd::CSR,
    _reserved_10_biter: [u8; 2usize],
}
impl TCD {
    #[doc = "0x08 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn nbytes_mloffyes(&self) -> &self::tcd::NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const self::tcd::NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x08 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn nbytes_mloffyes_mut(&self) -> &mut self::tcd::NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(8usize)
                as *mut self::tcd::NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x08 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn nbytes_mloffno(&self) -> &self::tcd::NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const self::tcd::NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x08 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn nbytes_mloffno_mut(&self) -> &mut self::tcd::NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut self::tcd::NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x08 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn nbytes_mlno(&self) -> &self::tcd::NBYTES_MLNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const self::tcd::NBYTES_MLNO)
        }
    }
    #[doc = "0x08 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn nbytes_mlno_mut(&self) -> &mut self::tcd::NBYTES_MLNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut self::tcd::NBYTES_MLNO)
        }
    }
    #[doc = "0x16 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn citer_elinkyes(&self) -> &self::tcd::CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(22usize)
                as *const self::tcd::CITER_ELINKYES)
        }
    }
    #[doc = "0x16 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn citer_elinkyes_mut(&self) -> &mut self::tcd::CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(22usize)
                as *mut self::tcd::CITER_ELINKYES)
        }
    }
    #[doc = "0x16 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn citer_elinkno(&self) -> &self::tcd::CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(22usize) as *const self::tcd::CITER_ELINKNO)
        }
    }
    #[doc = "0x16 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn citer_elinkno_mut(&self) -> &mut self::tcd::CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(22usize) as *mut self::tcd::CITER_ELINKNO)
        }
    }
    #[doc = "0x1e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn biter_elinkyes(&self) -> &self::tcd::BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(30usize)
                as *const self::tcd::BITER_ELINKYES)
        }
    }
    #[doc = "0x1e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn biter_elinkyes_mut(&self) -> &mut self::tcd::BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(30usize)
                as *mut self::tcd::BITER_ELINKYES)
        }
    }
    #[doc = "0x1e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn biter_elinkno(&self) -> &self::tcd::BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(30usize) as *const self::tcd::BITER_ELINKNO)
        }
    }
    #[doc = "0x1e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn biter_elinkno_mut(&self) -> &mut self::tcd::BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(30usize) as *mut self::tcd::BITER_ELINKNO)
        }
    }
}
#[doc = r"Register block"]
#[doc = "Transfer Control Descriptor"]
pub mod tcd;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [es](es) module"]
pub type ES = crate::Reg<u32, _ES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ES;
#[doc = "`read()` method returns [es::R](es::R) reader structure"]
impl crate::Readable for ES {}
#[doc = "Error Status Register"]
pub mod es;
#[doc = "Enable Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erq](erq) module"]
pub type ERQ = crate::Reg<u32, _ERQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERQ;
#[doc = "`read()` method returns [erq::R](erq::R) reader structure"]
impl crate::Readable for ERQ {}
#[doc = "`write(|w| ..)` method takes [erq::W](erq::W) writer structure"]
impl crate::Writable for ERQ {}
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "Enable Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eei](eei) module"]
pub type EEI = crate::Reg<u32, _EEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEI;
#[doc = "`read()` method returns [eei::R](eei::R) reader structure"]
impl crate::Readable for EEI {}
#[doc = "`write(|w| ..)` method takes [eei::W](eei::W) writer structure"]
impl crate::Writable for EEI {}
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "Clear Enable Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ceei](ceei) module"]
pub type CEEI = crate::Reg<u8, _CEEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEEI;
#[doc = "`read()` method returns [ceei::R](ceei::R) reader structure"]
impl crate::Readable for CEEI {}
#[doc = "`write(|w| ..)` method takes [ceei::W](ceei::W) writer structure"]
impl crate::Writable for CEEI {}
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "Set Enable Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seei](seei) module"]
pub type SEEI = crate::Reg<u8, _SEEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEEI;
#[doc = "`read()` method returns [seei::R](seei::R) reader structure"]
impl crate::Readable for SEEI {}
#[doc = "`write(|w| ..)` method takes [seei::W](seei::W) writer structure"]
impl crate::Writable for SEEI {}
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "Clear Enable Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerq](cerq) module"]
pub type CERQ = crate::Reg<u8, _CERQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CERQ;
#[doc = "`read()` method returns [cerq::R](cerq::R) reader structure"]
impl crate::Readable for CERQ {}
#[doc = "`write(|w| ..)` method takes [cerq::W](cerq::W) writer structure"]
impl crate::Writable for CERQ {}
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "Set Enable Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [serq](serq) module"]
pub type SERQ = crate::Reg<u8, _SERQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SERQ;
#[doc = "`read()` method returns [serq::R](serq::R) reader structure"]
impl crate::Readable for SERQ {}
#[doc = "`write(|w| ..)` method takes [serq::W](serq::W) writer structure"]
impl crate::Writable for SERQ {}
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "Clear DONE Status Bit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdne](cdne) module"]
pub type CDNE = crate::Reg<u8, _CDNE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDNE;
#[doc = "`read()` method returns [cdne::R](cdne::R) reader structure"]
impl crate::Readable for CDNE {}
#[doc = "`write(|w| ..)` method takes [cdne::W](cdne::W) writer structure"]
impl crate::Writable for CDNE {}
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "Set START Bit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssrt](ssrt) module"]
pub type SSRT = crate::Reg<u8, _SSRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSRT;
#[doc = "`read()` method returns [ssrt::R](ssrt::R) reader structure"]
impl crate::Readable for SSRT {}
#[doc = "`write(|w| ..)` method takes [ssrt::W](ssrt::W) writer structure"]
impl crate::Writable for SSRT {}
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "Clear Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerr](cerr) module"]
pub type CERR = crate::Reg<u8, _CERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CERR;
#[doc = "`read()` method returns [cerr::R](cerr::R) reader structure"]
impl crate::Readable for CERR {}
#[doc = "`write(|w| ..)` method takes [cerr::W](cerr::W) writer structure"]
impl crate::Writable for CERR {}
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "Clear Interrupt Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cint](cint) module"]
pub type CINT = crate::Reg<u8, _CINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CINT;
#[doc = "`read()` method returns [cint::R](cint::R) reader structure"]
impl crate::Readable for CINT {}
#[doc = "`write(|w| ..)` method takes [cint::W](cint::W) writer structure"]
impl crate::Writable for CINT {}
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "Interrupt Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](int) module"]
pub type INT = crate::Reg<u32, _INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT;
#[doc = "`read()` method returns [int::R](int::R) reader structure"]
impl crate::Readable for INT {}
#[doc = "`write(|w| ..)` method takes [int::W](int::W) writer structure"]
impl crate::Writable for INT {}
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err](err) module"]
pub type ERR = crate::Reg<u32, _ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERR;
#[doc = "`read()` method returns [err::R](err::R) reader structure"]
impl crate::Readable for ERR {}
#[doc = "`write(|w| ..)` method takes [err::W](err::W) writer structure"]
impl crate::Writable for ERR {}
#[doc = "Error Register"]
pub mod err;
#[doc = "Hardware Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrs](hrs) module"]
pub type HRS = crate::Reg<u32, _HRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRS;
#[doc = "`read()` method returns [hrs::R](hrs::R) reader structure"]
impl crate::Readable for HRS {}
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "Enable Asynchronous Request in Stop Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ears](ears) module"]
pub type EARS = crate::Reg<u32, _EARS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EARS;
#[doc = "`read()` method returns [ears::R](ears::R) reader structure"]
impl crate::Readable for EARS {}
#[doc = "`write(|w| ..)` method takes [ears::W](ears::W) writer structure"]
impl crate::Writable for EARS {}
#[doc = "Enable Asynchronous Request in Stop Register"]
pub mod ears;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri](dchpri) module"]
pub type DCHPRI = crate::Reg<u8, _DCHPRI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI;
#[doc = "`read()` method returns [dchpri::R](dchpri::R) reader structure"]
impl crate::Readable for DCHPRI {}
#[doc = "`write(|w| ..)` method takes [dchpri::W](dchpri::W) writer structure"]
impl crate::Writable for DCHPRI {}
#[doc = "Channel n Priority Register"]
pub mod dchpri;
