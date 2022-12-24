use crate::{Challenge, Sponge};

/// Have a seperate type for squeezing challenges
/// (takes a mut borrow on Sponge -- to avoid changing state)

#[macro_export]
macro_rules! challenge_int_impl {
    ( $t:tt, $n:expr ) => {
        impl Challenge for $t {
            #[inline(always)]
            fn sample<S: Sponge>(ts: &mut S) -> Self {
                let buf = <[u8; $n]>::sample(ts);
                Self::from_le_bytes(buf)
            }
        }
    };
}

impl Challenge for bool {
    fn sample<S: Sponge>(ts: &mut S) -> Self {
        let v: u8 = u8::sample(ts);
        (v & 1) == 1
    }
}

impl Challenge for u8 {
    fn sample<S: Sponge>(ts: &mut S) -> Self {
        ts.squeeze()
    }
}

impl<const N: usize, T: Challenge + Default + Copy> Challenge for [T; N] {
    #[allow(clippy::needless_range_loop)]
    fn sample<S: Sponge>(ts: &mut S) -> Self {
        let mut res: [T; N] = [T::default(); N];
        for i in 0..N {
            res[i] = T::sample(ts);
        }
        res
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
