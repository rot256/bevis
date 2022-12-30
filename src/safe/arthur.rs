use crate::{Absorb, Transcript, Challenge, Msg};

use rand_core::{CryptoRng, RngCore};

// prevents impl. of Arthur outside crate
pub trait Sealed {}

pub trait Arthur: Transcript + Sealed {
    fn receive<T: Absorb>(&mut self, elem: Msg<T>) -> T {
        self.append(&elem.0);
        elem.0
    }

    fn send<T: Absorb>(&mut self, elem: T) -> Msg<T> {
        self.append(&elem);
        elem.into()
    }
}

pub(crate) struct ArthurImpl<'a, T: Transcript> {
    pub(crate) tx: &'a mut T,
}

impl <'a, T: Transcript> Transcript for ArthurImpl<'a, T> {
    #[inline(always)]
    fn append<A: Absorb>(&mut self, elem: &A) {
        self.tx.append(elem)
    }

    #[inline(always)]
    fn challenge<C: Challenge>(&mut self) -> C {
        self.tx.challenge()
    }
}

// prevents impl outside crate
impl<'a, T: Transcript> Sealed for ArthurImpl<'a, T> {}

impl<'a, T: Transcript> CryptoRng for ArthurImpl<'a, T> {}

impl<'a, T: Transcript> RngCore for ArthurImpl<'a, T> {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.tx.next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.tx.next_u64()
    }

    #[inline(always)]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.tx.fill_bytes(dest)
    }

    #[inline(always)]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.tx.try_fill_bytes(dest)
    }
}

impl<'a, T: Transcript> Arthur for ArthurImpl<'a, T> {
    fn receive<A: Absorb>(&mut self, elem: Msg<A>) -> A {
        self.append(&elem.0);
        elem.0
    }
}