use crate::{Absorb, Hasher, Tx};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[macro_export]
macro_rules! absorb_int_impl {
    ( $t:tt ) => {
        impl Absorb for $t {
            #[inline(always)]
            fn absorb<H: Hasher>(&self, h: &mut H) {
                h.write(&self.to_le_bytes())
            }
        }
    };
}

absorb_int_impl!(i8);
absorb_int_impl!(i16);
absorb_int_impl!(i32);
absorb_int_impl!(i64);
absorb_int_impl!(i128);

absorb_int_impl!(u8);
absorb_int_impl!(u16);
absorb_int_impl!(u32);
absorb_int_impl!(u64);
absorb_int_impl!(u128);

impl<T: Tx> Absorb for T {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        self.read(h)
    }
}

impl Absorb for () {
    fn absorb<H: Hasher>(&self, _h: &mut H) {}
}

impl Absorb for bool {
    #[inline(always)]
    fn absorb<H: Hasher>(&self, h: &mut H) {
        let bit: u8 = if *self { 1 } else { 0 };
        h.write(&[bit]);
    }
}

#[cfg(feature = "alloc")]
impl<T: Absorb> Absorb for Vec<T> {
    #[inline(always)]
    fn absorb<H: Hasher>(&self, h: &mut H) {
        let s: &[T] = &self[..];
        s.absorb(h)
    }
}

impl<T: Absorb> Absorb for [T] {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        // read the length
        // TOOD: wait for https://github.com/rust-lang/rust/issues/96762
        let n = (self.len() as u64).to_le_bytes();
        n.absorb(h);

        // read every element
        for elem in self.iter() {
            elem.absorb(h)
        }
    }
}

impl<T: Absorb> Absorb for Option<T> {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        // read if Some/None
        self.is_some().absorb(h);

        // read inner value (if present)
        if let Some(v) = self {
            v.absorb(h)
        }
    }
}

impl<A: Absorb, B: Absorb> Absorb for Result<A, B> {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        // read if Ok/Err
        self.is_ok().absorb(h);

        // read inner value (if present)
        match self {
            Ok(v) => v.absorb(h),
            Err(e) => e.absorb(h),
        }
    }
}

impl<const N: usize, T: Absorb> Absorb for [T; N] {
    fn absorb<H: Hasher>(&self, h: &mut H) {
        // read every element 
        // (the length is fixed so no need to include it)
        for elem in self.iter() {
            elem.absorb(h)
        }
    }
}
