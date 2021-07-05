//! Implementations of the [`IsPad`], [`GetPad`] and [`InIoSet`] traits

use crate::gpio::v2::*;
use crate::sercom::v2::*;

//==============================================================================
//  Pad definitions
//==============================================================================

macro_rules! pad_info {
    (
        $PinId:ident,
        $Cfg:ident,
        $Sercom:ident,
        $PadNum:ident,
        $( $IoSet:ident ),+
    ) => {
        impl GetPad<$Sercom> for $PinId {
            type PadNum = $PadNum;
            type PinMode = Alternate<$Cfg>;
        }
        $(
            impl InIoSet<$IoSet> for Pin<$PinId, Alternate<$Cfg>> {}
        )+
        impl IsPad for Pin<$PinId, Alternate<$Cfg>> {
            type Sercom = $Sercom;
            type PadNum = $PadNum;
        }
    };
}

macro_rules! pad_table {
    (
        #[$cfg:meta]
        $PinId:ident {
            $(
                $Cfg:ident: ( $Sercom:ident, $PadNum:ident, $( $IoSet:ident ),+ ),
            )+
        }
    ) => {
        $(
            #[$cfg]
            pad_info!( $PinId, $Cfg, $Sercom, $PadNum, $( $IoSet ),+ );
        )+
    };
    (
        $PinId:ident {
            $(
                $( #[$cfg:meta] )?
                $Cfg:ident: ( $Sercom:ident, $PadNum:ident, $( $IoSet:ident ),+ ),
            )+
        }
    ) => {
        $(
            $( #[$cfg] )?
            pad_info!( $PinId, $Cfg, $Sercom, $PadNum, $( $IoSet ),+ );
        )+
    };
    (
        $(
            $( #[$id_cfg:meta] )?
            $PinId:ident {
                $(
                    $( #[$sercom_cfg:meta] )?
                    $Cfg:ident: ( $Sercom:ident, $PadNum:ident, $( $IoSet:ident ),+ ),
                )+
            }
        )+
    ) => {
        $(
            pad_table!(
                $( #[$id_cfg] )?
                $PinId{
                    $(
                        $( #[$sercom_cfg] )?
                        $Cfg: ( $Sercom, $PadNum, $( $IoSet),+ ),
                    )+
                }
            );
        )+
    };
}

pad_table!(
    PA00 {
        D: (Sercom1, Pad0, IoSet4),
    }
    PA01 {
        D: (Sercom1, Pad1, IoSet4),
    }
    PA04 {
        D: (Sercom0, Pad0, IoSet3),
    }
    PA05 {
        D: (Sercom0, Pad1, IoSet3),
    }
    PA06 {
        D: (Sercom0, Pad2, IoSet3),
    }
    PA07 {
        D: (Sercom0, Pad3, IoSet3),
    }
    PA08 {
        C: (Sercom0, Pad0, IoSet1),
        D: (Sercom2, Pad1, IoSet3),
    }
    PA09 {
        C: (Sercom0, Pad1, IoSet1),
        D: (Sercom2, Pad0, IoSet3),
    }
    PA10 {
        C: (Sercom0, Pad2, IoSet1),
        D: (Sercom2, Pad2, IoSet3),
    }
    PA11 {
        C: (Sercom0, Pad3, IoSet1),
        D: (Sercom2, Pad3, IoSet3),
    }
    PA12 {
        C: (Sercom2, Pad0, IoSet1),
        D: (Sercom4, Pad1, IoSet3),
    }
    PA13 {
        C: (Sercom2, Pad1, IoSet1),
        D: (Sercom4, Pad0, IoSet3),
    }
    PA14 {
        C: (Sercom2, Pad2, IoSet1),
        D: (Sercom4, Pad2, IoSet3),
    }
    PA15 {
        C: (Sercom2, Pad3, IoSet1),
        D: (Sercom4, Pad3, IoSet3),
    }
    PA16 {
        C: (Sercom1, Pad0, IoSet1),
        D: (Sercom3, Pad1, IoSet3),
    }
    PA17 {
        C: (Sercom1, Pad1, IoSet1),
        D: (Sercom3, Pad0, IoSet3),
    }
    PA18 {
        C: (Sercom1, Pad2, IoSet1),
        D: (Sercom3, Pad2, IoSet3),
    }
    PA19 {
        C: (Sercom1, Pad3, IoSet1),
        D: (Sercom3, Pad3, IoSet3),
    }
    PA20 {
        C: (Sercom5, Pad2, IoSet2),
        D: (Sercom3, Pad2, IoSet2),
    }
    PA21 {
        C: (Sercom5, Pad3, IoSet2),
        D: (Sercom3, Pad3, IoSet2),
    }
    PA22 {
        C: (Sercom3, Pad0, IoSet1),
        D: (Sercom5, Pad1, IoSet2, IoSet3, IoSet4),
    }
    PA23 {
        C: (Sercom3, Pad1, IoSet1),
        D: (Sercom5, Pad0, IoSet2, IoSet3, IoSet4),
    }
    PA24 {
        C: (Sercom3, Pad2, IoSet1),
        D: (Sercom5, Pad2, IoSet3),
    }
    PA25 {
        C: (Sercom3, Pad3, IoSet1),
        D: (Sercom5, Pad3, IoSet3),
    }
    PA30 {
        #[cfg(feature = "min-samd51n")]
        C: (Sercom7, Pad2, IoSet5),
        D: (Sercom1, Pad2, IoSet4),
    }
    PA31 {
        #[cfg(feature = "min-samd51n")]
        C: (Sercom7, Pad3, IoSet5),
        D: (Sercom1, Pad3, IoSet4),
    }
    #[cfg(feature = "min-samd51j")]
    PB00 {
        D: (Sercom5, Pad2, IoSet5, IoSet6),
    }
    #[cfg(feature = "min-samd51j")]
    PB01 {
        D: (Sercom5, Pad3, IoSet5, IoSet6),
    }
    PB02 {
        D: (Sercom5, Pad0, IoSet6),
    }
    PB03 {
        D: (Sercom5, Pad1, IoSet6),
    }
    PB08 {
        D: (Sercom4, Pad0, IoSet2),
    }
    PB09 {
        D: (Sercom4, Pad1, IoSet2),
    }
    PB10 {
        D: (Sercom4, Pad2, IoSet2),
    }
    PB11 {
        D: (Sercom4, Pad3, IoSet2),
    }
    #[cfg(feature = "min-samd51j")]
    PB12 {
        C: (Sercom4, Pad0, IoSet1),
    }
    #[cfg(feature = "min-samd51j")]
    PB13 {
        C: (Sercom4, Pad1, IoSet1),
    }
    #[cfg(feature = "min-samd51j")]
    PB14 {
        C: (Sercom4, Pad2, IoSet1),
    }
    #[cfg(feature = "min-samd51j")]
    PB15 {
        C: (Sercom4, Pad3, IoSet1),
    }
    #[cfg(feature = "min-samd51j")]
    PB16 {
        C: (Sercom5, Pad0, IoSet1),
    }
    #[cfg(feature = "min-samd51j")]
    PB17 {
        C: (Sercom5, Pad1, IoSet1),
    }
    #[cfg(feature = "min-samd51n")]
    PB18 {
        C: (Sercom5, Pad2, IoSet1),
        D: (Sercom7, Pad2, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PB19 {
        C: (Sercom5, Pad3, IoSet1),
        D: (Sercom7, Pad3, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PB20 {
        C: (Sercom3, Pad0, IoSet2),
        D: (Sercom7, Pad1, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PB21 {
        C: (Sercom3, Pad1, IoSet2),
        D: (Sercom7, Pad0, IoSet4),
    }
    PB22 {
        C: (Sercom1, Pad2, IoSet3),
        D: (Sercom5, Pad2, IoSet4),
    }
    PB23 {
        C: (Sercom1, Pad3, IoSet3),
        D: (Sercom5, Pad3, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PB24 {
        C: (Sercom0, Pad0, IoSet2),
        D: (Sercom2, Pad1, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PB25 {
        C: (Sercom0, Pad1, IoSet2),
        D: (Sercom2, Pad0, IoSet4),
    }
    #[cfg(feature = "min-samd51p")]
    PB26 {
        C: (Sercom2, Pad0, IoSet2),
        D: (Sercom4, Pad1, IoSet4),
    }
    #[cfg(feature = "min-samd51p")]
    PB27 {
        C: (Sercom2, Pad1, IoSet2),
        D: (Sercom4, Pad0, IoSet4),
    }
    #[cfg(feature = "min-samd51p")]
    PB28 {
        C: (Sercom2, Pad2, IoSet2),
        D: (Sercom4, Pad2, IoSet4),
    }
    #[cfg(feature = "min-samd51p")]
    PB29 {
        C: (Sercom2, Pad3, IoSet2),
        D: (Sercom4, Pad3, IoSet4),
    }
    PB30 {
        #[cfg(feature = "min-samd51n")]
        C: (Sercom7, Pad0, IoSet5),
        #[cfg(feature = "min-samd51j")]
        D: (Sercom5, Pad1, IoSet5),
    }
    PB31 {
        #[cfg(feature = "min-samd51n")]
        C: (Sercom7, Pad1, IoSet5),
        #[cfg(feature = "min-samd51j")]
        D: (Sercom5, Pad0, IoSet5),
    }
    #[cfg(feature = "min-samd51p")]
    PC04 {
        C: (Sercom6, Pad0, IoSet2),
    }
    #[cfg(feature = "min-samd51n")]
    PC05 {
        C: (Sercom6, Pad1, IoSet2),
    }
    #[cfg(feature = "min-samd51n")]
    PC06 {
        C: (Sercom6, Pad2, IoSet2),
    }
    #[cfg(feature = "min-samd51n")]
    PC07 {
        C: (Sercom6, Pad3, IoSet2),
    }
    #[cfg(feature = "min-samd51n")]
    PC10 {
        C: (Sercom6, Pad2, IoSet5),
        D: (Sercom7, Pad2, IoSet3),
    }
    #[cfg(feature = "min-samd51n")]
    PC11 {
        C: (Sercom6, Pad3, IoSet5),
        D: (Sercom7, Pad3, IoSet3),
    }
    #[cfg(feature = "min-samd51n")]
    PC12 {
        C: (Sercom7, Pad0, IoSet1, IoSet3),
        D: (Sercom6, Pad1, IoSet4, IoSet5),
    }
    #[cfg(feature = "min-samd51n")]
    PC13 {
        C: (Sercom7, Pad1, IoSet1, IoSet3),
        D: (Sercom6, Pad0, IoSet4, IoSet5),
    }
    #[cfg(feature = "min-samd51n")]
    PC14 {
        C: (Sercom7, Pad2, IoSet1),
        D: (Sercom6, Pad2, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PC15 {
        C: (Sercom7, Pad3, IoSet1),
        D: (Sercom6, Pad3, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PC16 {
        C: (Sercom6, Pad0, IoSet1),
        D: (Sercom0, Pad1, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PC17 {
        C: (Sercom6, Pad1, IoSet1),
        D: (Sercom0, Pad0, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PC18 {
        C: (Sercom6, Pad2, IoSet1),
        D: (Sercom0, Pad2, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PC19 {
        C: (Sercom6, Pad3, IoSet1),
        D: (Sercom0, Pad3, IoSet4),
    }
    #[cfg(feature = "min-samd51p")]
    PC22 {
        C: (Sercom1, Pad0, IoSet2),
        D: (Sercom3, Pad1, IoSet4),
    }
    #[cfg(feature = "min-samd51p")]
    PC23 {
        C: (Sercom1, Pad1, IoSet2),
        D: (Sercom3, Pad0, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PC24 {
        C: (Sercom0, Pad2, IoSet2),
        D: (Sercom2, Pad2, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PC25 {
        C: (Sercom0, Pad3, IoSet2),
        D: (Sercom2, Pad3, IoSet4),
    }
    #[cfg(feature = "min-samd51n")]
    PC27 {
        C: (Sercom1, Pad0, IoSet3),
    }
    #[cfg(feature = "min-samd51n")]
    PC28 {
        C: (Sercom1, Pad1, IoSet3),
    }
    #[cfg(feature = "min-samd51p")]
    PD08 {
        C: (Sercom7, Pad0, IoSet2),
        D: (Sercom6, Pad1, IoSet3),
    }
    #[cfg(feature = "min-samd51p")]
    PD09 {
        C: (Sercom7, Pad1, IoSet2),
        D: (Sercom6, Pad0, IoSet3),
    }
    #[cfg(feature = "min-samd51p")]
    PD10 {
        C: (Sercom7, Pad2, IoSet2),
        D: (Sercom6, Pad2, IoSet3),
    }
    #[cfg(feature = "min-samd51p")]
    PD11 {
        C: (Sercom7, Pad3, IoSet2),
        D: (Sercom6, Pad3, IoSet3),
    }
    #[cfg(feature = "min-samd51p")]
    PD20 {
        C: (Sercom1, Pad2, IoSet2),
        D: (Sercom3, Pad2, IoSet4),
    }
    #[cfg(feature = "min-samd51p")]
    PD21 {
        C: (Sercom1, Pad3, IoSet2),
        D: (Sercom3, Pad3, IoSet4),
    }
);

// Implement an undocumented `IoSet` for PA16, PA17, PB22 & PB23 configured for
// `Sercom1`. The pygamer uses this combination, but it is not listed as valid
// in the datasheet.
impl InIoSet<UndocIoSet1> for Pin<PA16, Alternate<C>> {}
impl InIoSet<UndocIoSet1> for Pin<PA17, Alternate<C>> {}
impl InIoSet<UndocIoSet1> for Pin<PB22, Alternate<C>> {}
impl InIoSet<UndocIoSet1> for Pin<PB23, Alternate<C>> {}
