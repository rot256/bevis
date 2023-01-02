use rand_core::{CryptoRng, RngCore};

/// A type which can be sampled uniformly. Provided for convience.
/// Challenges can also be sampled directly using the sponge impl of RngCore.
pub trait Challenge<W> {
    fn sample<S: Sampler<W>>(ts: &mut S) -> Self;
}

pub trait Sampler<W> {
    fn read(&mut self) -> W;

    fn fill(&mut self, dst: &mut [W]) {
        for i in 0..dst.len() {
            dst[i] = self.read()
        }
    }
}

impl<T: CryptoRng + RngCore> Sampler<u8> for T {
    fn read(&mut self) -> u8 {
        let mut dst: [u8; 1] = [0u8; 1];
        self.fill_bytes(&mut dst);
        dst[0]
    }

    fn fill(&mut self, dst: &mut [u8]) {
        self.fill_bytes(dst);
    }
}

impl Challenge<u8> for u8 {
    fn sample<S: Sampler<u8>>(ts: &mut S) -> Self {
        ts.read()
    }
}

#[macro_export]
macro_rules! challenge_int_impl {
    ( $t:tt, $n:expr ) => {
        impl Challenge<u8> for $t {
            #[inline(always)]
            fn sample<S: Sampler<u8>>(ts: &mut S) -> Self {
                let mut buf = [0u8; $n];
                ts.fill(&mut buf);
                Self::from_le_bytes(buf)
            }
        }
    };
}

impl<const N: usize, W, T: Challenge<W>> Challenge<W> for [T; N] {
    fn sample<S: Sampler<W>>(ts: &mut S) -> Self {
        [(); N].map(|_| T::sample(ts))
    }
}

// challenge_int_impl!(u8, 1);
challenge_int_impl!(u16, 2);
challenge_int_impl!(u32, 4);
challenge_int_impl!(u64, 8);

challenge_int_impl!(i8, 1);
challenge_int_impl!(i16, 2);
challenge_int_impl!(i32, 4);
challenge_int_impl!(i64, 8);
