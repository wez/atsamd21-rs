use core::marker::PhantomData;
use core::ops::Deref;
use typenum::U0;

use crate::typelevel::{Counter, Decrement, Increment, PrivateDecrement, PrivateIncrement, Sealed};

pub struct Counted<T, N: Counter>(pub(crate) T, PhantomData<N>);

impl<T> Counted<T, U0> {
    pub(crate) fn new(t: T) -> Self {
        Self(t, PhantomData)
    }
}

impl<T, N: Counter> Counted<T, N> {
    // TODO: Rethink if this should be really unsafe. Maybe crate-public is enough.
    // Rename new_unsafe -> create ?
    pub(crate) unsafe fn new_unsafe(t: T) -> Self {
        Counted(t, PhantomData)
    }
}

impl<T, N: Counter> Counter for Counted<T, N> {}

impl<T, N: Counter> Sealed for Counted<T, N> {}

impl<T, N: PrivateIncrement> PrivateIncrement for Counted<T, N> {
    type Inc = Counted<T, N::Inc>;

    fn inc(self) -> Self::Inc {
        Counted(self.0, PhantomData)
    }
}

impl<T, N: PrivateIncrement> Increment for Counted<T, N> {}

impl<T, N: PrivateDecrement> PrivateDecrement for Counted<T, N> {
    type Dec = Counted<T, N::Dec>;

    fn dec(self) -> Self::Dec {
        Counted(self.0, PhantomData)
    }
}

impl<T, N: PrivateDecrement> Decrement for Counted<T, N> {}

impl<T, N: Counter> Deref for Counted<T, N> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
