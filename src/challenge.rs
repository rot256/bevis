use crate::Challenge;

use rand_core::{CryptoRng, RngCore};

#[macro_export]
macro_rules! challenge_int_impl {
    ( $t:tt, $n:expr ) => {
        impl Challenge for $t {
            #[inline(always)]
            fn sample<S: CryptoRng + RngCore>(ts: &mut S) -> Self {
                let mut buf = [0u8; $n];
                ts.fill_bytes(&mut buf);
                Self::from_le_bytes(buf)
            }
        }
    };
}

impl Challenge for bool {
    fn sample<S: CryptoRng + RngCore>(ts: &mut S) -> Self {
        let v: u8 = u8::sample(ts);
        (v & 1) == 1
    }
}

impl<const N: usize, T: Challenge + Default + Copy> Challenge for [T; N] {
    fn sample<S: CryptoRng + RngCore>(ts: &mut S) -> Self {
        let mut res: [T; N] = [T::default(); N];
        for e in res.iter_mut() {
            *e = T::sample(ts);
        }
        res
    }
}

challenge_int_impl!(u8, 1);
challenge_int_impl!(u16, 2);
challenge_int_impl!(u32, 4);
challenge_int_impl!(u64, 8);

challenge_int_impl!(i8, 1);
challenge_int_impl!(i16, 2);
challenge_int_impl!(i32, 4);
challenge_int_impl!(i64, 8);
