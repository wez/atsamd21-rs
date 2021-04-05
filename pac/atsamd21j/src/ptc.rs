#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control B"]
    pub ctrlb: CTRLB,
    _reserved2: [u8; 2usize],
    #[doc = "0x04 - Unknown Register 0x42004C04"]
    pub unk4c04: UNK4C04,
    #[doc = "0x05 - Control C"]
    pub ctrlc: CTRLC,
    _reserved4: [u8; 2usize],
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x09 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved7: [u8; 1usize],
    #[doc = "0x0c - Frequency Control"]
    pub freqctrl: FREQCTRL,
    #[doc = "0x0d - Conversion control"]
    pub convctrl: CONVCTRL,
    _reserved9: [u8; 2usize],
    #[doc = "0x10 - Select Y line"]
    pub yselect: YSELECT,
    #[doc = "0x12 - Select X line"]
    pub xselect: XSELECT,
    #[doc = "0x14 - Enable Y lines"]
    pub yselecten: YSELECTEN,
    #[doc = "0x16 - Enable X lines"]
    pub xselecten: XSELECTEN,
    #[doc = "0x18 - Compensation capacitor value"]
    pub compcap: COMPCAP,
    #[doc = "0x1a - Internal capacitor value"]
    pub intcap: INTCAP,
    #[doc = "0x1b - Series resistor for PTC measurements"]
    pub serres: SERRES,
    #[doc = "0x1c - Conversion result"]
    pub result: RESULT,
    _reserved17: [u8; 2usize],
    #[doc = "0x20 - Enable burst or clear to send low power mode"]
    pub burstmode: BURSTMODE,
    #[doc = "0x21 - Set WCO mode"]
    pub wcomode: WCOMODE,
    _reserved19: [u8; 2usize],
    #[doc = "0x24 - Set lower WCO threshold for port A"]
    pub wcothresholdal: WCOTHRESHOLDAL,
    #[doc = "0x25 - Set upper WCO threshold for port A"]
    pub wcothresholdah: WCOTHRESHOLDAH,
    #[doc = "0x26 - Set lower WCO threshold for port B"]
    pub wcothresholdbl: WCOTHRESHOLDBL,
    #[doc = "0x27 - Set upper WCO threshold for port B"]
    pub wcothresholdbh: WCOTHRESHOLDBH,
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u8, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](ctrlb) module"]
pub type CTRLB = crate::Reg<u8, _CTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB;
#[doc = "`read()` method returns [ctrlb::R](ctrlb::R) reader structure"]
impl crate::Readable for CTRLB {}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](ctrlb::W) writer structure"]
impl crate::Writable for CTRLB {}
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "Unknown Register 0x42004C04\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unk4c04](unk4c04) module"]
pub type UNK4C04 = crate::Reg<u8, _UNK4C04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNK4C04;
#[doc = "`read()` method returns [unk4c04::R](unk4c04::R) reader structure"]
impl crate::Readable for UNK4C04 {}
#[doc = "`write(|w| ..)` method takes [unk4c04::W](unk4c04::W) writer structure"]
impl crate::Writable for UNK4C04 {}
#[doc = "Unknown Register 0x42004C04"]
pub mod unk4c04;
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](ctrlc) module"]
pub type CTRLC = crate::Reg<u8, _CTRLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLC;
#[doc = "`read()` method returns [ctrlc::R](ctrlc::R) reader structure"]
impl crate::Readable for CTRLC {}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](ctrlc::W) writer structure"]
impl crate::Writable for CTRLC {}
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u8, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u8, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u8, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Frequency Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqctrl](freqctrl) module"]
pub type FREQCTRL = crate::Reg<u8, _FREQCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQCTRL;
#[doc = "`read()` method returns [freqctrl::R](freqctrl::R) reader structure"]
impl crate::Readable for FREQCTRL {}
#[doc = "`write(|w| ..)` method takes [freqctrl::W](freqctrl::W) writer structure"]
impl crate::Writable for FREQCTRL {}
#[doc = "Frequency Control"]
pub mod freqctrl;
#[doc = "Conversion control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [convctrl](convctrl) module"]
pub type CONVCTRL = crate::Reg<u8, _CONVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONVCTRL;
#[doc = "`read()` method returns [convctrl::R](convctrl::R) reader structure"]
impl crate::Readable for CONVCTRL {}
#[doc = "`write(|w| ..)` method takes [convctrl::W](convctrl::W) writer structure"]
impl crate::Writable for CONVCTRL {}
#[doc = "Conversion control"]
pub mod convctrl;
#[doc = "Select Y line\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [yselect](yselect) module"]
pub type YSELECT = crate::Reg<u16, _YSELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _YSELECT;
#[doc = "`read()` method returns [yselect::R](yselect::R) reader structure"]
impl crate::Readable for YSELECT {}
#[doc = "`write(|w| ..)` method takes [yselect::W](yselect::W) writer structure"]
impl crate::Writable for YSELECT {}
#[doc = "Select Y line"]
pub mod yselect;
#[doc = "Select X line\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xselect](xselect) module"]
pub type XSELECT = crate::Reg<u16, _XSELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XSELECT;
#[doc = "`read()` method returns [xselect::R](xselect::R) reader structure"]
impl crate::Readable for XSELECT {}
#[doc = "`write(|w| ..)` method takes [xselect::W](xselect::W) writer structure"]
impl crate::Writable for XSELECT {}
#[doc = "Select X line"]
pub mod xselect;
#[doc = "Enable Y lines\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [yselecten](yselecten) module"]
pub type YSELECTEN = crate::Reg<u16, _YSELECTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _YSELECTEN;
#[doc = "`read()` method returns [yselecten::R](yselecten::R) reader structure"]
impl crate::Readable for YSELECTEN {}
#[doc = "`write(|w| ..)` method takes [yselecten::W](yselecten::W) writer structure"]
impl crate::Writable for YSELECTEN {}
#[doc = "Enable Y lines"]
pub mod yselecten;
#[doc = "Enable X lines\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xselecten](xselecten) module"]
pub type XSELECTEN = crate::Reg<u16, _XSELECTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XSELECTEN;
#[doc = "`read()` method returns [xselecten::R](xselecten::R) reader structure"]
impl crate::Readable for XSELECTEN {}
#[doc = "`write(|w| ..)` method takes [xselecten::W](xselecten::W) writer structure"]
impl crate::Writable for XSELECTEN {}
#[doc = "Enable X lines"]
pub mod xselecten;
#[doc = "Compensation capacitor value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compcap](compcap) module"]
pub type COMPCAP = crate::Reg<u16, _COMPCAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPCAP;
#[doc = "`read()` method returns [compcap::R](compcap::R) reader structure"]
impl crate::Readable for COMPCAP {}
#[doc = "`write(|w| ..)` method takes [compcap::W](compcap::W) writer structure"]
impl crate::Writable for COMPCAP {}
#[doc = "Compensation capacitor value"]
pub mod compcap;
#[doc = "Internal capacitor value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcap](intcap) module"]
pub type INTCAP = crate::Reg<u8, _INTCAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCAP;
#[doc = "`read()` method returns [intcap::R](intcap::R) reader structure"]
impl crate::Readable for INTCAP {}
#[doc = "`write(|w| ..)` method takes [intcap::W](intcap::W) writer structure"]
impl crate::Writable for INTCAP {}
#[doc = "Internal capacitor value"]
pub mod intcap;
#[doc = "Series resistor for PTC measurements\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [serres](serres) module"]
pub type SERRES = crate::Reg<u8, _SERRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SERRES;
#[doc = "`read()` method returns [serres::R](serres::R) reader structure"]
impl crate::Readable for SERRES {}
#[doc = "`write(|w| ..)` method takes [serres::W](serres::W) writer structure"]
impl crate::Writable for SERRES {}
#[doc = "Series resistor for PTC measurements"]
pub mod serres;
#[doc = "Conversion result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](result) module"]
pub type RESULT = crate::Reg<u16, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [result::R](result::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "Conversion result"]
pub mod result;
#[doc = "Enable burst or clear to send low power mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [burstmode](burstmode) module"]
pub type BURSTMODE = crate::Reg<u8, _BURSTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BURSTMODE;
#[doc = "`read()` method returns [burstmode::R](burstmode::R) reader structure"]
impl crate::Readable for BURSTMODE {}
#[doc = "`write(|w| ..)` method takes [burstmode::W](burstmode::W) writer structure"]
impl crate::Writable for BURSTMODE {}
#[doc = "Enable burst or clear to send low power mode"]
pub mod burstmode;
#[doc = "Set WCO mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcomode](wcomode) module"]
pub type WCOMODE = crate::Reg<u8, _WCOMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCOMODE;
#[doc = "`read()` method returns [wcomode::R](wcomode::R) reader structure"]
impl crate::Readable for WCOMODE {}
#[doc = "`write(|w| ..)` method takes [wcomode::W](wcomode::W) writer structure"]
impl crate::Writable for WCOMODE {}
#[doc = "Set WCO mode"]
pub mod wcomode;
#[doc = "Set lower WCO threshold for port A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcothresholdal](wcothresholdal) module"]
pub type WCOTHRESHOLDAL = crate::Reg<u8, _WCOTHRESHOLDAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCOTHRESHOLDAL;
#[doc = "`read()` method returns [wcothresholdal::R](wcothresholdal::R) reader structure"]
impl crate::Readable for WCOTHRESHOLDAL {}
#[doc = "`write(|w| ..)` method takes [wcothresholdal::W](wcothresholdal::W) writer structure"]
impl crate::Writable for WCOTHRESHOLDAL {}
#[doc = "Set lower WCO threshold for port A"]
pub mod wcothresholdal;
#[doc = "Set upper WCO threshold for port A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcothresholdah](wcothresholdah) module"]
pub type WCOTHRESHOLDAH = crate::Reg<u8, _WCOTHRESHOLDAH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCOTHRESHOLDAH;
#[doc = "`read()` method returns [wcothresholdah::R](wcothresholdah::R) reader structure"]
impl crate::Readable for WCOTHRESHOLDAH {}
#[doc = "`write(|w| ..)` method takes [wcothresholdah::W](wcothresholdah::W) writer structure"]
impl crate::Writable for WCOTHRESHOLDAH {}
#[doc = "Set upper WCO threshold for port A"]
pub mod wcothresholdah;
#[doc = "Set lower WCO threshold for port B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcothresholdbl](wcothresholdbl) module"]
pub type WCOTHRESHOLDBL = crate::Reg<u8, _WCOTHRESHOLDBL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCOTHRESHOLDBL;
#[doc = "`read()` method returns [wcothresholdbl::R](wcothresholdbl::R) reader structure"]
impl crate::Readable for WCOTHRESHOLDBL {}
#[doc = "`write(|w| ..)` method takes [wcothresholdbl::W](wcothresholdbl::W) writer structure"]
impl crate::Writable for WCOTHRESHOLDBL {}
#[doc = "Set lower WCO threshold for port B"]
pub mod wcothresholdbl;
#[doc = "Set upper WCO threshold for port B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcothresholdbh](wcothresholdbh) module"]
pub type WCOTHRESHOLDBH = crate::Reg<u8, _WCOTHRESHOLDBH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCOTHRESHOLDBH;
#[doc = "`read()` method returns [wcothresholdbh::R](wcothresholdbh::R) reader structure"]
impl crate::Readable for WCOTHRESHOLDBH {}
#[doc = "`write(|w| ..)` method takes [wcothresholdbh::W](wcothresholdbh::W) writer structure"]
impl crate::Writable for WCOTHRESHOLDBH {}
#[doc = "Set upper WCO threshold for port B"]
pub mod wcothresholdbh;
