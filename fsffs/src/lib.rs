#[macro_use]
extern crate fsffs_derive;

pub use fsffs_derive::*;

use std::hash::{Hash, Hasher};

// just like hotel california: you can check in, but you can never leave...
// It is also not clone/copy to ensure a value is only added to the transcript once.
// Neat.
pub struct Elem<T> {
    v: T
}

impl <T> From<T> for Elem<T> {
    fn from(v: T) -> Self {
        Self { v }
    }
}

// this gives us a bunch of implementations from the get go
impl <T: Hash> Elem<T> {
    // reading is a destructive act
    fn read<A: Arthur>(self, ts: &mut A) -> T {
        self.v.hash(ts);
        self.v
    }
}

// MUST NOT BE CLONE OR COPY!
pub trait Arthur: Hasher + Sized {
    fn recv<T: Hash>(&mut self, elem: Elem<T>) -> T {
        elem.read(self)
    }
}

pub mod private {
    // to prevent crates from implementing Msg
    pub trait Seal {
        fn check(&self);
    }
}



/// You cannot implement this, only derive it.
pub trait Msg: private::Seal {}

// the fields are covered
impl <T> private::Seal for Elem<T> {
    fn check(&self) {}
}

// the fields are covered
impl <T> Msg for Elem<T> {}

// vectors of messages are messages
impl <M: Msg> private::Seal for Vec<M>  {
    fn check(&self) {}
}

impl <M: Msg> Msg for Vec<M> {}

// arrays of messages are messages
impl <const N: usize, M: Msg> private::Seal for [M; N]  {
    fn check(&self) {}
}

impl <const N: usize, M: Msg> Msg for [M; N] {}

pub trait Proof: Msg {

    fn verify<A: Arthur>(self, ts: &mut A) -> bool;
}