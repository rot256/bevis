use core::marker::PhantomData;

use crate::{Challenge, Absorb, Sponge, challenge::Sampler};

#[derive(Debug)]
pub struct SpongeTranscript<W, S: Sponge<W>> {
    _ph: PhantomData<W>,
    sponge: S
} 


///
/// 
/// Transcripts over u8 can additionally be used as RngCore
/// enabling easy sampling of a variety of types in the Rust ecosystem.
pub trait Transcript<W>: Sampler<W> + Sized {
    /// Append message to the trancript
    fn append<A: Absorb<W>>(&mut self, elem: &A);

    /// Generate a challenge
    fn challenge<C: Challenge<W>>(&mut self) -> C;
}

impl<W, S: Sponge<W>> SpongeTranscript<W, S> {
    pub fn new(sponge: S) -> Self {
        Self { _ph: PhantomData, sponge }
    }
}

impl<W, S: Sponge<W>> Sampler<W> for SpongeTranscript<W, S> {
    fn fill(&mut self, dst: &mut [W]) {
        self.sponge.fill(dst)
    }
}

impl<W, S: Sponge<W>> Transcript<W> for SpongeTranscript<W, S> {
    fn append<T: Absorb<W>>(&mut self, elem: &T) {
        elem.absorb(&mut self.sponge);
    }

    /// Sends a challenge to the prover
    fn challenge<T: Challenge<W>>(&mut self) -> T {
        T::sample(&mut self.sponge)
    }
}