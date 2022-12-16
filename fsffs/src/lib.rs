#[macro_use]
extern crate fsffs_derive;

pub use fsffs_derive::*;

use std::hash::{Hash, Hasher};

mod transcript;

/// You should implement this only for primitive types.
/// (e.g. curve points or field elements)
pub trait Absorb {
    fn absorb<A: Arthur>(&self, ts: &mut A);
}

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
    v: T,
}

/// Transcripts consists of structs where all 
/// absorbable base-types are wrapped in one/more Msg.
pub trait Tx {
    fn read<A: Arthur>(&self, ts: &mut A);
}

impl<T: Absorb> Tx for Msg<T> {
    // absorbing a message is a no-op: 
    // the content is ignored until it is unwrapped.
    fn read<A: Arthur>(&self, _ts: &mut A) {}
}

// TODO: implement serialize/deserialize for Msg when T is.
impl<T> From<T> for Msg<T> {
    fn from(v: T) -> Self {
        Self { v }
    }
}

// MUST NOT BE CLONE OR COPY!
pub trait Arthur: Hasher + Sized {
    ///
    /// This is the only way to unpack an Msg.
    fn recv<T: Absorb>(&mut self, elem: Msg<T>) -> T {
        elem.v.absorb(self); // note: it reads the inner value
        elem.v
    }
}

//

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

pub trait Proof {
    type Statement: Hash; // entire statement is read up-front (i.e. no messages)
    type Proof: Tx; // a proof consists of multiple messages
    type Error;

    /// Requiring verify to work for any "Arthur" prevents it
    /// from depending on the "Merlin" part which
    /// is also implemented for the transcript hasher.
    fn verify<A: Arthur>(
        ts: &mut A,
        st: Msg<Self::Statement>,
        pf: Self::Proof,
    ) -> Result<(), Self::Error>;
}
