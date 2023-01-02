use crate::{Absorb, Challenge, Msg, Sampler, Sponge};

use rand_core::{CryptoRng, RngCore};

#[derive(Debug)]
#[repr(transparent)]
pub struct SpongeTranscript<S: Sponge>(S);

pub trait Transcript: Sampler + Sized {
    /// Append message to the trancript
    fn append<A: Absorb>(&mut self, elem: &A);

    /// Generate a challenge
    fn challenge<C: Challenge>(&mut self) -> C;

    fn recv<A: Absorb>(&mut self, msg: Msg<A>) -> A {
        self.append(&msg.0);
        msg.0
    }

    fn send<A: Absorb>(&mut self, elem: A) -> Msg<A> {
        self.append(&elem);
        Msg(elem)
    }
}

impl<S: Sponge> SpongeTranscript<S> {
    pub fn new(sep: &str) -> Self {
        Self(S::new(sep))
    }
}

impl<S: Sponge> RngCore for SpongeTranscript<S> {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.0.try_fill_bytes(dest)
    }
}

impl<S: Sponge> CryptoRng for SpongeTranscript<S> {}

impl<S: Sponge> Sampler for SpongeTranscript<S> {}

impl<S: Sponge> Transcript for SpongeTranscript<S> {
    fn append<T: Absorb>(&mut self, elem: &T) {
        elem.absorb(&mut self.0);
    }

    /// Sends a challenge to the prover
    fn challenge<T: Challenge>(&mut self) -> T {
        T::sample(&mut self.0)
    }
}
