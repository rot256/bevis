use crate::safe::Sealed;
use crate::{challenge::Sampler, Absorb, Challenge, Transcript};

use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

#[repr(transparent)]
pub struct Arthur<'a, W, T: Transcript<W>> {
    _ph: PhantomData<W>,
    tx: &'a mut T,
}

impl<'a, W, T: Transcript<W>> Arthur<'a, W, T> {
    #[allow(dead_code)]
    pub(crate) fn new(tx: &'a mut T) -> Self {
        Arthur {
            _ph: PhantomData,
            tx,
        }
    }
}

impl<'a, W, T: Transcript<W>> Sealed for Arthur<'a, W, T> {}

impl<'a, W, T: Transcript<W>> Deref for Arthur<'a, W, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.tx
    }
}

impl<'a, W, T: Transcript<W>> DerefMut for Arthur<'a, W, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tx
    }
}

impl<'a, W, T: Transcript<W>> Sampler<W> for Arthur<'a, W, T> {
    fn read(&mut self) -> W {
        self.tx.read()
    }
}

impl<'a, W, T: Transcript<W>> Transcript<W> for Arthur<'a, W, T> {
    #[inline(always)]
    fn append<A: Absorb<W>>(&mut self, elem: &A) {
        self.tx.append(elem)
    }

    #[inline(always)]
    fn challenge<C: Challenge<W>>(&mut self) -> C {
        self.tx.challenge()
    }
}
