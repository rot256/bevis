use crate::{challenge::Sampler, Absorb, Challenge, Safe, Transcript};

use rand_core::{CryptoRng, RngCore};

pub trait Sealed {}

#[repr(transparent)]
pub struct Arthur<'a, T: Transcript>(&'a mut T);

impl<'a, T: Transcript> Arthur<'a, T> {
    #[allow(dead_code)]
    pub(crate) fn new(tx: &'a mut T) -> Self {
        Arthur(tx)
    }
}

impl<'a, T: Transcript> CryptoRng for Arthur<'a, T> {}

impl<'a, T: Transcript> RngCore for Arthur<'a, T> {
    #[inline(always)]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    #[inline(always)]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.0.try_fill_bytes(dest)
    }
}

impl<'a, T: Transcript> Sampler for Arthur<'a, T> {}

impl<'a, T: Transcript> Sealed for Arthur<'a, T> {}

impl<'a, T: Transcript> Safe for Arthur<'a, T> {}

impl<'a, T: Transcript> Transcript for Arthur<'a, T> {
    #[inline(always)]
    fn append<A: Absorb>(&mut self, elem: &A) {
        self.0.append(elem)
    }

    #[inline(always)]
    fn challenge<C: Challenge>(&mut self) -> C {
        self.0.challenge()
    }

    #[inline(always)]
    fn recv<A: Absorb>(&mut self, msg: crate::Msg<A>) -> A {
        self.0.recv(msg)
    }

    #[inline(always)]
    fn send<A: Absorb>(&mut self, elem: A) -> crate::Msg<A> {
        self.0.send(elem)
    }
}
