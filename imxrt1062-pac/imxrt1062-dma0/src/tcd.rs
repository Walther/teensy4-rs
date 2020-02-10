#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddr](saddr) module"]
pub type SADDR = crate::Reg<u32, _SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SADDR;
#[doc = "`read()` method returns [saddr::R](saddr::R) reader structure"]
impl crate::Readable for SADDR {}
#[doc = "`write(|w| ..)` method takes [saddr::W](saddr::W) writer structure"]
impl crate::Writable for SADDR {}
#[doc = "TCD Source Address"]
pub mod saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soff](soff) module"]
pub type SOFF = crate::Reg<u16, _SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOFF;
#[doc = "`read()` method returns [soff::R](soff::R) reader structure"]
impl crate::Readable for SOFF {}
#[doc = "`write(|w| ..)` method takes [soff::W](soff::W) writer structure"]
impl crate::Writable for SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [attr](attr) module"]
pub type ATTR = crate::Reg<u16, _ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATTR;
#[doc = "`read()` method returns [attr::R](attr::R) reader structure"]
impl crate::Readable for ATTR {}
#[doc = "`write(|w| ..)` method takes [attr::W](attr::W) writer structure"]
impl crate::Writable for ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nbytes_mlno](nbytes_mlno) module"]
pub type NBYTES_MLNO = crate::Reg<u32, _NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NBYTES_MLNO;
#[doc = "`read()` method returns [nbytes_mlno::R](nbytes_mlno::R) reader structure"]
impl crate::Readable for NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [nbytes_mlno::W](nbytes_mlno::W) writer structure"]
impl crate::Writable for NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nbytes_mloffno](nbytes_mloffno) module"]
pub type NBYTES_MLOFFNO = crate::Reg<u32, _NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NBYTES_MLOFFNO;
#[doc = "`read()` method returns [nbytes_mloffno::R](nbytes_mloffno::R) reader structure"]
impl crate::Readable for NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [nbytes_mloffno::W](nbytes_mloffno::W) writer structure"]
impl crate::Writable for NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nbytes_mloffyes](nbytes_mloffyes) module"]
pub type NBYTES_MLOFFYES = crate::Reg<u32, _NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NBYTES_MLOFFYES;
#[doc = "`read()` method returns [nbytes_mloffyes::R](nbytes_mloffyes::R) reader structure"]
impl crate::Readable for NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [nbytes_mloffyes::W](nbytes_mloffyes::W) writer structure"]
impl crate::Writable for NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slast](slast) module"]
pub type SLAST = crate::Reg<u32, _SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAST;
#[doc = "`read()` method returns [slast::R](slast::R) reader structure"]
impl crate::Readable for SLAST {}
#[doc = "`write(|w| ..)` method takes [slast::W](slast::W) writer structure"]
impl crate::Writable for SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daddr](daddr) module"]
pub type DADDR = crate::Reg<u32, _DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADDR;
#[doc = "`read()` method returns [daddr::R](daddr::R) reader structure"]
impl crate::Readable for DADDR {}
#[doc = "`write(|w| ..)` method takes [daddr::W](daddr::W) writer structure"]
impl crate::Writable for DADDR {}
#[doc = "TCD Destination Address"]
pub mod daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doff](doff) module"]
pub type DOFF = crate::Reg<u16, _DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOFF;
#[doc = "`read()` method returns [doff::R](doff::R) reader structure"]
impl crate::Readable for DOFF {}
#[doc = "`write(|w| ..)` method takes [doff::W](doff::W) writer structure"]
impl crate::Writable for DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [citer_elinkno](citer_elinkno) module"]
pub type CITER_ELINKNO = crate::Reg<u16, _CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CITER_ELINKNO;
#[doc = "`read()` method returns [citer_elinkno::R](citer_elinkno::R) reader structure"]
impl crate::Readable for CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [citer_elinkno::W](citer_elinkno::W) writer structure"]
impl crate::Writable for CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [citer_elinkyes](citer_elinkyes) module"]
pub type CITER_ELINKYES = crate::Reg<u16, _CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CITER_ELINKYES;
#[doc = "`read()` method returns [citer_elinkyes::R](citer_elinkyes::R) reader structure"]
impl crate::Readable for CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [citer_elinkyes::W](citer_elinkyes::W) writer structure"]
impl crate::Writable for CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlastsga](dlastsga) module"]
pub type DLASTSGA = crate::Reg<u32, _DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLASTSGA;
#[doc = "`read()` method returns [dlastsga::R](dlastsga::R) reader structure"]
impl crate::Readable for DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [dlastsga::W](dlastsga::W) writer structure"]
impl crate::Writable for DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u16, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "TCD Control and Status"]
pub mod csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biter_elinkno](biter_elinkno) module"]
pub type BITER_ELINKNO = crate::Reg<u16, _BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BITER_ELINKNO;
#[doc = "`read()` method returns [biter_elinkno::R](biter_elinkno::R) reader structure"]
impl crate::Readable for BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [biter_elinkno::W](biter_elinkno::W) writer structure"]
impl crate::Writable for BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biter_elinkyes](biter_elinkyes) module"]
pub type BITER_ELINKYES = crate::Reg<u16, _BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BITER_ELINKYES;
#[doc = "`read()` method returns [biter_elinkyes::R](biter_elinkyes::R) reader structure"]
impl crate::Readable for BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [biter_elinkyes::W](biter_elinkyes::W) writer structure"]
impl crate::Writable for BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod biter_elinkyes;
