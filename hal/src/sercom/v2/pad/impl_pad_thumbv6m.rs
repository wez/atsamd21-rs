//! Implementations of the [`IsPad`] and [`GetPad`] traits

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
        $PadNum:ident
    ) => {
        #[cfg(feature = "samd11")]
        impl GetPad<$Sercom, $PadNum> for $PinId {
            type PinMode = Alternate<$Cfg>;
        }

        #[cfg(feature = "samd21")]
        impl GetPad<$Sercom> for $PinId {
            type PadNum = $PadNum;
            type PinMode = Alternate<$Cfg>;
        }

        impl IsPad for Pin<$PinId, Alternate<$Cfg>> {
            type Sercom = $Sercom;
            type PadNum = $PadNum;
        }
    };
}

// Feature gates can be placed on individual configurations or on the entire pin
macro_rules! pad_table {
    (
        #[$cfg:meta]
        $PinId:ident {
            $(
                $Cfg:ident: ( $Sercom:ident, $PadNum:ident ),
            )+
        }
    ) => {
        $(
            #[$cfg]
            pad_info!( $PinId, $Cfg, $Sercom, $PadNum );
        )+
    };
    (
        $PinId:ident {
            $(
                $( #[$cfg:meta] )?
                $Cfg:ident: ( $Sercom:ident, $PadNum:ident ),
            )+
        }
    ) => {
        $(
            $( #[$cfg] )?
            pad_info!( $PinId, $Cfg, $Sercom, $PadNum );
        )+
    };
    (
        $(
            $( #[$id_cfg:meta] )?
            $PinId:ident {
                $(
                    $( #[$sercom_cfg:meta] )?
                    $Cfg:ident: ( $Sercom:ident, $PadNum:ident ),
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
                        $Cfg: ( $Sercom, $PadNum ),
                    )+
                }
            );
        )+
    };
}

pad_table!(
    #[cfg(feature = "samd21")]
    PA00 {
        D: (Sercom1, Pad0),
    }
    #[cfg(feature = "samd21")]
    PA01 {
        D: (Sercom1, Pad1),
    }
    PA04 {
        #[cfg(feature = "samd11")]
        C: (Sercom0, Pad2),
        D: (Sercom0, Pad0),
    }
    PA05 {
        #[cfg(feature = "samd11")]
        C: (Sercom0, Pad3),
        D: (Sercom0, Pad1),
    }
    #[cfg(feature = "samd21")]
    PA06 {
        D: (Sercom0, Pad2),
    }
    #[cfg(feature = "samd21")]
    PA07 {
        D: (Sercom0, Pad3),
    }
    PA08 {
        #[cfg(feature = "samd11")]
        C: (Sercom1, Pad2),
        #[cfg(feature = "samd11")]
        D: (Sercom0, Pad2),
        #[cfg(feature = "samd21")]
        C: (Sercom0, Pad0),
        #[cfg(feature = "samd21")]
        D: (Sercom2, Pad0),
    }
    PA09 {
        #[cfg(feature = "samd11")]
        C: (Sercom1, Pad3),
        #[cfg(feature = "samd11")]
        D: (Sercom0, Pad3),
        #[cfg(feature = "samd21")]
        C: (Sercom0, Pad1),
        #[cfg(feature = "samd21")]
        D: (Sercom2, Pad1),
    }
    #[cfg(feature = "samd21")]
    PA10 {
        C: (Sercom0, Pad2),
        D: (Sercom2, Pad2),
    }
    #[cfg(feature = "samd21")]
    PA11 {
        C: (Sercom0, Pad3),
        D: (Sercom2, Pad3),
    }
    #[cfg(feature = "min-samd21g")]
    PA12 {
        C: (Sercom2, Pad0),
        D: (Sercom4, Pad0),
    }
    #[cfg(feature = "min-samd21g")]
    PA13 {
        C: (Sercom2, Pad1),
        D: (Sercom4, Pad1),
    }
    PA14 {
        #[cfg(feature = "samd11")]
        C: (Sercom0, Pad0),
        #[cfg(feature = "samd21")]
        C: (Sercom2, Pad2),
        #[cfg(feature = "min-samd21g")]
        D: (Sercom4, Pad2),
    }
    PA15 {
        #[cfg(feature = "samd11")]
        C: (Sercom0, Pad1),
        #[cfg(feature = "samd21")]
        C: (Sercom2, Pad3),
        #[cfg(feature = "min-samd21g")]
        D: (Sercom4, Pad3),
    }
    #[cfg(feature = "samd21")]
    PA16 {
        C: (Sercom1, Pad0),
        D: (Sercom3, Pad0),
    }
    #[cfg(feature = "samd21")]
    PA17 {
        C: (Sercom1, Pad1),
        D: (Sercom3, Pad1),
    }
    #[cfg(feature = "samd21")]
    PA18 {
        C: (Sercom1, Pad2),
        D: (Sercom3, Pad2),
    }
    #[cfg(feature = "samd21")]
    PA19 {
        C: (Sercom1, Pad3),
        D: (Sercom3, Pad3),
    }
    #[cfg(feature = "min-samd21g")]
    PA20 {
        C: (Sercom5, Pad2),
        D: (Sercom3, Pad2),
    }
    #[cfg(feature = "min-samd21g")]
    PA21 {
        C: (Sercom5, Pad3),
        D: (Sercom3, Pad3),
    }
    PA22 {
        #[cfg(feature = "samd21")]
        C: (Sercom3, Pad0),
        #[cfg(feature = "min-samd21g")]
        D: (Sercom5, Pad0),
    }
    PA23 {
        #[cfg(feature = "samd21")]
        C: (Sercom3, Pad1),
        #[cfg(feature = "min-samd21g")]
        D: (Sercom5, Pad1),
    }
    PA24 {
        #[cfg(feature = "samd11")]
        C: (Sercom1, Pad2),
        #[cfg(feature = "samd21")]
        C: (Sercom3, Pad2),
        #[cfg(feature = "min-samd21g")]
        D: (Sercom5, Pad2),
    }
    PA25 {
        #[cfg(feature = "samd11")]
        C: (Sercom1, Pad3),
        #[cfg(feature = "samd21")]
        C: (Sercom3, Pad3),
        #[cfg(feature = "min-samd21g")]
        D: (Sercom5, Pad3),
    }
    PA30 {
        #[cfg(feature = "samd11")]
        C: (Sercom1, Pad0),
        D: (Sercom1, Pad2),
    }
    PA31 {
        #[cfg(feature = "samd11")]
        C: (Sercom1, Pad1),
        D: (Sercom1, Pad3),
    }
    #[cfg(feature = "min-samd21j")]
    PB00 {
        D: (Sercom5, Pad2),
    }
    #[cfg(feature = "min-samd21j")]
    PB01 {
        D: (Sercom5, Pad3),
    }
    #[cfg(feature = "min-samd21g")]
    PB02 {
        D: (Sercom5, Pad0),
    }
    #[cfg(feature = "min-samd21g")]
    PB03 {
        D: (Sercom5, Pad1),
    }
    #[cfg(feature = "min-samd21g")]
    PB08 {
        D: (Sercom4, Pad0),
    }
    #[cfg(feature = "min-samd21g")]
    PB09 {
        D: (Sercom4, Pad1),
    }
    #[cfg(feature = "min-samd21g")]
    PB10 {
        D: (Sercom4, Pad2),
    }
    #[cfg(feature = "min-samd21g")]
    PB11 {
        D: (Sercom4, Pad3),
    }
    #[cfg(feature = "min-samd21j")]
    PB12 {
        C: (Sercom4, Pad0),
    }
    #[cfg(feature = "min-samd21j")]
    PB13 {
        C: (Sercom4, Pad1),
    }
    #[cfg(feature = "min-samd21j")]
    PB14 {
        C: (Sercom4, Pad2),
    }
    #[cfg(feature = "min-samd21j")]
    PB15 {
        C: (Sercom4, Pad3),
    }
    #[cfg(feature = "min-samd21j")]
    PB16 {
        C: (Sercom5, Pad0),
    }
    #[cfg(feature = "min-samd21j")]
    PB17 {
        C: (Sercom5, Pad1),
    }
    #[cfg(feature = "min-samd21g")]
    PB22 {
        D: (Sercom5, Pad2),
    }
    #[cfg(feature = "min-samd21g")]
    PB23 {
        D: (Sercom5, Pad3),
    }
    #[cfg(feature = "min-samd21j")]
    PB30 {
        D: (Sercom5, Pad0),
    }
    #[cfg(feature = "min-samd21j")]
    PB31 {
        D: (Sercom5, Pad1),
    }
);
