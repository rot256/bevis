use rand_core::{CryptoRng, RngCore};

pub trait Sampler: CryptoRng + RngCore {}

pub trait Challenge {
    fn sample<S: CryptoRng + RngCore>(ts: &mut S) -> Self;
}

#[macro_export]
macro_rules! challenge_int_impl {
    ( $t:tt, $n:expr ) => {
        impl Challenge for $t {
            #[inline(always)]
            #[must_use]
            fn sample<S: CryptoRng + RngCore>(ts: &mut S) -> Self {
                let mut buf = [0u8; $n];
                ts.fill_bytes(&mut buf);
                Self::from_le_bytes(buf)
            }
        }
    };
}

impl<const N: usize, T: Challenge> Challenge for [T; N] {
    fn sample<S: CryptoRng + RngCore>(ts: &mut S) -> Self {
        [(); N].map(|_| T::sample(ts))
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
