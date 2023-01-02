use super::{Absorb, Hasher};

impl <W, T: Absorb<W>> Absorb<W> for [T] where usize: Absorb<W> {
    fn absorb<H: Hasher<W>>(&self, h: &mut H) {
        self.len().absorb(h);
        for elem in self.iter() {
            elem.absorb(h)
        }
    }
}

impl <const N: usize, W, T: Absorb<W>> Absorb<W> for [T; N] {
    fn absorb<H: Hasher<W>>(&self, h: &mut H) {
        for elem in self.iter() {
            elem.absorb(h)
        }
    }
}

impl <W> Absorb<W> for W {
    fn absorb<H: Hasher<W>>(&self, h: &mut H) {
        h.add(self)
    }
}

impl Absorb<u8> for &str {
    fn absorb<H: Hasher<u8>>(&self, h: &mut H) {
        self.len().absorb(h);
        h.write(self.as_bytes())
    }
}

impl Absorb<u8> for usize {
    fn absorb<H: Hasher<u8>>(&self, h: &mut H) {
        (*self as u64).absorb(h)
    }
}

#[macro_export]
macro_rules! absorb_int_impl {
    ( $t:tt ) => {
        impl Absorb<u8> for $t {
            fn absorb<H: Hasher<u8>>(&self, h: &mut H) {
                h.write(&self.to_le_bytes())
            }
        }
    };
}

absorb_int_impl!(u16);
absorb_int_impl!(u32);
absorb_int_impl!(u64);
absorb_int_impl!(u128);

absorb_int_impl!(i8);
absorb_int_impl!(i32);
absorb_int_impl!(i64);
absorb_int_impl!(i128);