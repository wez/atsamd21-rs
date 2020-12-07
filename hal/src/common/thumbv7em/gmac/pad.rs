use super::private;
use crate::gpio::*;

#[cfg(feature = "min-samd51n")]
pub struct MiiPadout<IOSet> {
    _gtxck: Pa14<PfL>,
    _gtxen: Pa17<PfL>,
    _gtx0: Pa18<PfL>,
    _gtx1: Pa19<PfL>,
    _gtx2: Pc16<PfL>,
    _gtx3: Pc17<PfL>,
    _gtxer: Pc19<PfL>,
    _grxck: Pc18<PfL>,
    _grxdv: Pc20<PfL>,
    _grx0: Pa13<PfL>,
    _grx1: Pa12<PfL>,
    _grx2: Pc15<PfL>,
    _grx3: Pc14<PfL>,
    _grxer: Pa15<PfL>,
    _gcrs: Pa16<PfL>,
    _gcol: Pc21<PfL>,
    _ioset: IOSet, // GMDC, GMDIO
}

impl<IOSet> MiiPadout<IOSet> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        port: &mut Port,
        _gtxck: Pa14<Input<Floating>>,
        _gtxen: Pa17<Input<Floating>>,
        _gtx0: Pa18<Input<Floating>>,
        _gtx1: Pa19<Input<Floating>>,
        _gtx2: Pc16<Input<Floating>>,
        _gtx3: Pc17<Input<Floating>>,
        _gtxer: Pc19<Input<Floating>>,
        _grxck: Pc18<Input<Floating>>,
        _grxdv: Pc20<Input<Floating>>,
        _grx0: Pa13<Input<Floating>>,
        _grx1: Pa12<Input<Floating>>,
        _grx2: Pc15<Input<Floating>>,
        _grx3: Pc14<Input<Floating>>,
        _grxer: Pa15<Input<Floating>>,
        _gcrs: Pa16<Input<Floating>>,
        _gcol: Pc21<Input<Floating>>,
        _ioset: IOSet,
    ) -> Self {
        MiiPadout {
            _gtxck: _gtxck.into_function(port),
            _gtxen: _gtxen.into_function(port),
            _gtx0: _gtx0.into_function(port),
            _gtx1: _gtx1.into_function(port),
            _gtx2: _gtx2.into_function(port),
            _gtx3: _gtx3.into_function(port),
            _gtxer: _gtxer.into_function(port),
            _grxck: _grxck.into_function(port),
            _grxdv: _grxdv.into_function(port),
            _grx0: _grx0.into_function(port),
            _grx1: _grx1.into_function(port),
            _grx2: _grx2.into_function(port),
            _grx3: _grx3.into_function(port),
            _grxer: _grxer.into_function(port),
            _gcrs: _gcrs.into_function(port),
            _gcol: _gcol.into_function(port),
            _ioset,
        }
    }
}

pub struct RmiiPadout<IOSet> {
    _gtxck: Pa14<PfL>,
    _gtxen: Pa17<PfL>,
    _gtx0: Pa18<PfL>,
    _gtx1: Pa19<PfL>,
    #[cfg(feature = "same53j")]
    _grxdv: Pa16<PfL>,
    #[cfg(not(feature = "same53j"))]
    _grxdv: Pc20<PfL>,
    _grx0: Pa13<PfL>,
    _grx1: Pa12<PfL>,
    _grxer: Pa15<PfL>,
    _ioset: IOSet,
}

#[allow(clippy::too_many_arguments)]
impl<IOSet> RmiiPadout<IOSet> {
    #[cfg(feature = "same53j")]
    pub fn new(
        port: &mut Port,
        _gtxck: Pa14<Input<Floating>>,
        _gtxen: Pa17<Input<Floating>>,
        _gtx0: Pa18<Input<Floating>>,
        _gtx1: Pa19<Input<Floating>>,
        _grxdv: Pa16<Input<Floating>>,
        _grx0: Pa13<Input<Floating>>,
        _grx1: Pa12<Input<Floating>>,
        _grxer: Pa15<Input<Floating>>,
        _ioset: IOSet,
    ) -> Self {
        RmiiPadout {
            _gtxck: _gtxck.into_function(port),
            _gtxen: _gtxen.into_function(port),
            _gtx0: _gtx0.into_function(port),
            _gtx1: _gtx1.into_function(port),
            _grxdv: _grxdv.into_function(port),
            _grx0: _grx0.into_function(port),
            _grx1: _grx1.into_function(port),
            _grxer: _grxer.into_function(port),
            _ioset,
        }
    }
    #[cfg(not(feature = "same53j"))]
    pub fn new(
        port: &mut Port,
        _gtxck: Pa14<Input<Floating>>,
        _gtxen: Pa17<Input<Floating>>,
        _gtx0: Pa18<Input<Floating>>,
        _gtx1: Pa19<Input<Floating>>,
        _grxdv: Pc20<Input<Floating>>,
        _grx0: Pa13<Input<Floating>>,
        _grx1: Pa12<Input<Floating>>,
        _grxer: Pa15<Input<Floating>>,
        _ioset: IOSet,
    ) -> Self {
        RmiiPadout {
            _gtxck: _gtxck.into_function(port),
            _gtxen: _gtxen.into_function(port),
            _gtx0: _gtx0.into_function(port),
            _gtx1: _gtx1.into_function(port),
            _grxdv: _grxdv.into_function(port),
            _grx0: _grx0.into_function(port),
            _grx1: _grx1.into_function(port),
            _grxer: _grxer.into_function(port),
            _ioset,
        }
    }
}

pub struct Ioset<GMDC, GMDIO>(GMDC, GMDIO);
pub trait IOSet: private::Sealed {}
impl<GMDC, GMDIO> private::Sealed for Ioset<GMDC, GMDIO> {}
impl<GMDC, GMDIO> IOSet for Ioset<GMDC, GMDIO> {}

impl<GMDC, GMDIO> Ioset<GMDC, GMDIO>
where
    GMDC: IntoFunction<Pb14<PfL>>,
    GMDIO: IntoFunction<Pb15<PfL>>,
{
    pub fn new1(gmdc: GMDC, gmdio: GMDIO) -> Self {
        Ioset(gmdc, gmdio)
    }
}
impl<GMDC, GMDIO> Ioset<GMDC, GMDIO>
where
    GMDC: IntoFunction<Pc11<PfL>>,
    GMDIO: IntoFunction<Pc12<PfL>>,
{
    pub fn new2(gmdc: GMDC, gmdio: GMDIO) -> Self {
        Ioset(gmdc, gmdio)
    }
}
impl<GMDC, GMDIO> Ioset<GMDC, GMDIO>
where
    GMDC: IntoFunction<Pc22<PfL>>,
    GMDIO: IntoFunction<Pc23<PfL>>,
{
    pub fn new3(gmdc: GMDC, gmdio: GMDIO) -> Self {
        Ioset(gmdc, gmdio)
    }
}
impl<GMDC, GMDIO> Ioset<GMDC, GMDIO>
where
    GMDC: IntoFunction<Pa20<PfL>>,
    GMDIO: IntoFunction<Pa21<PfL>>,
{
    pub fn new4(gmdc: GMDC, gmdio: GMDIO) -> Self {
        Ioset(gmdc, gmdio)
    }
}

// IOSET    1       2       3       4
// GMDC     PB14    PC11    PC22    PA20
// GMDIO    PB15    PC12    PC23    PA21

pub enum GmacMode {
    Rmii = 0,
    Mii = 1,
}

pub trait GmacPadout {
    const MODE: GmacMode;
}
impl<IOSet> GmacPadout for MiiPadout<IOSet> {
    const MODE: GmacMode = GmacMode::Mii;
}
impl<IOSet> GmacPadout for RmiiPadout<IOSet> {
    const MODE: GmacMode = GmacMode::Rmii;
}
