use core::marker::PhantomData;
use core::ops::{Add, Sub};
use typenum::{Add1, Sub1, Unsigned, B1, U0};

use crate::typelevel::Sealed;

mod private {
    use super::*;
    pub trait Increment: Counter {
        type Inc: Counter;
        fn inc(self) -> Self::Inc;
    }
    pub trait Decrement: Counter {
        type Dec: Counter;
        fn dec(self) -> Self::Dec;
    }
}

pub(crate) use private::Decrement as PrivateDecrement;
pub(crate) use private::Increment as PrivateIncrement;

/// TODO
pub trait Increment: PrivateIncrement {}

/// TODO
pub trait Decrement: PrivateDecrement {}

/// TODO
impl<N> PrivateIncrement for N
where
    N: Sealed + Unsigned + Add<B1>,
    Add1<N>: Sealed + Unsigned,
{
    type Inc = Add1<N>;
    fn inc(self) -> Self::Inc {
        Self::Inc::default()
    }
}

/// TODO
impl<N> PrivateDecrement for N
where
    N: Sealed + Unsigned + Sub<B1>,
    Sub1<N>: Sealed + Unsigned,
{
    type Dec = Sub1<N>;
    fn dec(self) -> Self::Dec {
        Self::Dec::default()
    }
}

/// TODO
pub trait Counter: Sealed {}

/// TODO
impl<N: Unsigned + Sealed> Counter for N {}

pub struct Enabled<T, N: Counter>(pub(crate) T, PhantomData<N>);

impl<T> Enabled<T, U0> {
    pub(crate) fn new(t: T) -> Self {
        Self(t, PhantomData)
    }
}

impl<T, N: Counter> Enabled<T, N> {
    // TODO: Rethink if this should be really unsafe. Maybe crate-public is enough.
    // Rename new_unsafe -> create ?
    pub(crate) unsafe fn new_unsafe(t: T) -> Self {
        Enabled(t, PhantomData)
    }
}

impl<T, N: Counter> Counter for Enabled<T, N> {}

impl<T, N: Counter> Sealed for Enabled<T, N> {}

impl<T, N: PrivateIncrement> PrivateIncrement for Enabled<T, N> {
    type Inc = Enabled<T, N::Inc>;

    fn inc(self) -> Self::Inc {
        Enabled(self.0, PhantomData)
    }
}

impl<T, N: PrivateIncrement> Increment for Enabled<T, N> {}

impl<T, N: PrivateDecrement> PrivateDecrement for Enabled<T, N> {
    type Dec = Enabled<T, N::Dec>;

    fn dec(self) -> Self::Dec {
        Enabled(self.0, PhantomData)
    }
}

impl<T, N: PrivateDecrement> Decrement for Enabled<T, N> {}
