use crate::{Challenge, Absorb, Sponge};

use rand_core::{CryptoRng, RngCore};

#[derive(Debug)]
pub struct SpongeTranscript<S: Sponge> {
    sponge: S
}

pub trait Transcript: RngCore + CryptoRng + Sized {    
    ///
    fn append<T: Absorb>(&mut self, elem: &T);

    /// Sends a challenge to the prover
    fn challenge<T: Challenge>(&mut self) -> T;
}

impl <S: Sponge> Transcript for S {
    fn append<T: Absorb>(&mut self, elem: &T) {
        elem.absorb(self)
    }

    fn challenge<T: Challenge>(&mut self) -> T {
        T::sample(self)
    }
}

impl<S: Sponge> CryptoRng for SpongeTranscript<S> {}

impl<S: Sponge> RngCore for SpongeTranscript<S> {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.sponge.next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.sponge.next_u64()
    }

    #[inline(always)]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.sponge.fill_bytes(dest)
    }

    #[inline(always)]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.sponge.try_fill_bytes(dest)
    }
}

impl<S: Sponge> SpongeTranscript<S> {
    pub fn new(sponge: S) -> Self {
        Self { sponge }
    }
}

impl<S: Sponge> Transcript for SpongeTranscript<S> {
    fn append<T: Absorb>(&mut self, elem: &T) {
        elem.absorb(&mut self.sponge);
    }

    /// Sends a challenge to the prover
    fn challenge<T: Challenge>(&mut self) -> T {
        T::sample(&mut self.sponge)
    }
}