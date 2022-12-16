use crate::{Arthur, Absorb, Msg, Tx};

use std::hash::Hash;

#[macro_export]
macro_rules! int_impl {
    ( $t:tt ) => {
        impl Absorb for $t {
            #[inline(always)]
            fn absorb<A: Arthur>(&self, ts: &mut A) {
                ts.write(&self.to_le_bytes())
            }
        }
    };
}

int_impl!(i8);
int_impl!(i16);
int_impl!(i32);
int_impl!(i64);
int_impl!(i128);

int_impl!(u8);
int_impl!(u16);
int_impl!(u32);
int_impl!(u64);
int_impl!(u128);

impl<T: Tx> Absorb for T {
    fn absorb<A: Arthur>(&self, ts: &mut A) {
        self.read(ts)
    }
}

impl Absorb for bool {
    #[inline(always)]
    fn absorb<A: Arthur>(&self, ts: &mut A) {
        ts.write(&[match self {
            true => 1,
            false => 0,
        }]);
    }
}

impl<T: Absorb> Absorb for Vec<T> {
    // the semantics of a list is its length
    // and the transitive semantics of all its members
    // (in the case of e.g. Vec<Msg<_>>) it is just the length
    fn absorb<A: Arthur>(&self, ts: &mut A) {
        // read the length
        let n = (self.len() as u64).to_le_bytes();
        n.hash(ts);

        // read every element
        for elem in self.iter() {
            elem.absorb(ts)
        }
    }
}

impl<T: Absorb> Absorb for Option<T> {
    fn absorb<A: Arthur>(&self, ts: &mut A) {
        match self {
            None => false.absorb(ts), // indicate none
            Some(value) => {
                true.absorb(ts); // indicate some
                value.absorb(ts); // read the inner value
            }
        }
    }
}

impl<const N: usize, T: Absorb> Absorb for [T; N] {
    fn absorb<H: Arthur>(&self, h: &mut H) {
        // read every element
        for elem in self.iter() {
            elem.absorb(h)
        }
    }
}
