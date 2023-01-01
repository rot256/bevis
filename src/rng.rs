use crate::Sampler;

use rand_core::{CryptoRng, RngCore, impls};

pub struct RngImpl<'a, S: Sampler<u8>>(&'a mut S);

/// Enables the use of Samplers over bytes as regular Rngs
pub trait AsRng<'a, S: Sampler<u8>> {
    fn rng(&'a mut self) -> RngImpl<'a, S>;
}

impl <'a, S: Sampler<u8>> AsRng<'a, S> for S {
    fn rng(&'a mut self) -> RngImpl<'a, S> {
        RngImpl(self)
    }
}

impl <'a, S: Sampler<u8>> CryptoRng for RngImpl<'a, S>  {}

impl <'a, S: Sampler<u8>> RngCore for RngImpl<'a, S> {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        impls::next_u32_via_fill(self)
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_fill(self)
    }

    #[inline(always)]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill(dest)
    }

    #[inline(always)]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}