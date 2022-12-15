#[macro_use]
extern crate fsffs_derive;

pub use fsffs_derive::*;

use std::hash::{Hash, Hasher};

// just like hotel california: you can check in, but you can never leave...
// It is also not clone/copy to ensure a value is only added to the transcript once.
// Neat.
//
// Msg does not implement:
//
// - Copy
// - Clone
// - PartialEqual
// - Hash
//
// On purpose
// Maybe this should be called round...
pub struct Msg<T> {
    v: T
}

// TODO: implement serialize/deserialize for Msg when T is.
impl <T> From<T> for Msg<T> {
    fn from(v: T) -> Self {
        Self { v }
    }
}

// MUST NOT BE CLONE OR COPY!
pub trait Arthur: Hasher + Sized {
    ///
    /// This is the only way to unpack an Msg.
    fn recv<T: Transcript>(&mut self, elem: Msg<T>) -> T {
        elem.v.read(self); // note: it reads the inner value
        elem.v
    }  
}

pub trait Transcript {
    fn read<A: Arthur>(&self, ts: &mut A);
}

impl <T> Transcript for Msg<T> {
    // all Msg are the same: they have no semantics.
    // receiving them is a no-op. Unpacking them is required.
    #[inline(always)]
    fn read<A: Arthur>(&self, _ts: &mut A) {}
}

impl <T: Transcript> Transcript for Vec<T> {
    // the semantics of a list is its length
    // and the transitive semantics of all its members
    // (in the case of Vec<Msg<_>>) it is just the length
    fn read<A: Arthur>(&self, ts: &mut A) {
        // read the length
        let n = (self.len() as u64).to_le_bytes();
        n.hash(ts);

        // read every element
        for elem in self.iter() {
            elem.read(ts)
        }
    }
}

impl <const N: usize, T: Transcript> Transcript for [T; N] {
    fn read<H: Arthur>(&self, h: &mut H) {
        // read every element
        for elem in self.iter() {
            elem.read(h)
        }
    }
}

// Receiving should stop when it encounters a Msg.
// 
// You can receive:
//
//  - Msg<Vec<T>> // receieves the entire vector, yields Vec<T>
//  - Msg<Vec<Msg<T>> // receives the semantics of the vector, but allows random access to the inner vector, yields Vec<Msg<T>>
//  - Msg<Vec<Msg<Vec<()>>> // yields Vec<Msg<Vec<()>>>
//
// but not:
//
//  - Vec<Msg<T>>
// 
// as behavior of verifier may depend on e.g. Vec::len 
//
// Msg<T> where T: Hash always implements Receieve
//
// Msg is not hash however.
// 
// Implement Receieve for Msg<Vec<T>> where T: Receieve

// Trait provided for convience to the prover
pub trait Merlin: Hasher + Sized {
    fn send<T: Hash>(&mut self, value: T) -> Msg<T> {
        value.hash(self);
        value.into()
    }
}

#[doc(hidden)]
pub mod private {
    use std::hash::Hasher;

    // to prevent crates from implementing Transcript
    pub trait Seal {
        fn check(&self);

        // used by enums to discern the variants
        // not used by any other types
        fn seperate<H: Hasher>(&self) {}
    }
}

pub trait Proof {
    type Statement: Hash; // entire statement is read up-front (i.e. no messages)
    type Proof: Transcript; // a proof consists of multiple messages
    type Error;

    /// Requiring verify to work for any "Arthur" prevents it
    /// from depending on the "Merlin" part which 
    /// is also implemented for the transcript hasher.
    fn verify<A: Arthur>(
        ts: &mut A, 
        st: Self::Statement, 
        pf: Self::Proof
    ) -> Result<(), Self::Error>;
}