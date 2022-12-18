extern crate fsffs_derive;

pub use fsffs_derive::*;

use serde::{Serialize, Deserialize};
use std::{hash::{Hash, Hasher}};

mod transcript;
mod challenge;

/// You should implement this only for primitive types.
/// (e.g. curve points or field elements)
pub trait Absorb {
    fn absorb<S: Sponge>(&self, ts: &mut S);
}

/// A type which can be generated from the Sponge:
/// a message from the verifier.
/// 
/// You should derive this for more complex types.
pub trait Challenge {
    fn sample<S: Sponge>(ts: &mut S) -> Self; 
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
pub struct Msg<T>(T);

// Messages serialize without overhead
impl <T: Serialize> Serialize for Msg<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.0.serialize(serializer)
    }
}

// Messages deserialize without overhead
impl <'de, T: Deserialize<'de>> Deserialize<'de> for Msg<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        T::deserialize(deserializer).map(|v| Msg(v))
    }
}

/// Transcripts consists of structs where all 
/// absorbable base-types are wrapped in one/more Msg.
pub trait Tx {
    fn read<S: Sponge>(&self, ts: &mut S);
}

impl<T: Absorb> Tx for Msg<T> {
    // absorbing a message is a no-op: 
    // the content is ignored until it is unwrapped.
    fn read<S: Sponge>(&self, _ts: &mut S) {}
}

// TODO: implement serialize/deserialize for Msg when T is.
impl<T> From<T> for Msg<T> {
    fn from(v: T) -> Self {
        Self(v)
    }
}

pub trait Sponge: Hasher {
    fn read(&mut self) -> u8;
}

pub struct Arthur<S: Sponge> {
    sponge: S
}

impl <S: Sponge> Arthur<S> {
    pub fn recv<T: Absorb>(&mut self, elem: Msg<T>) -> T {
        elem.0.absorb(&mut self.sponge); // note: it reads the inner value
        elem.0
    }

    pub fn send<T: Challenge>(&mut self) -> T {
        T::sample(&mut self.sponge)
    }
}

/*
// Trait provided for convience to the prover
pub trait Merlin: Hasher + Sized {
    fn send<T: Absorb>(&mut self, value: T) -> Msg<T> {
        value.absorb(self);
        value.into()
    }
}
*/

pub trait Proof {
    type Statement: Hash; // entire statement is read up-front (i.e. no messages)
    type Proof: Tx; // a proof consists of multiple messages
    type Error;

    /// Requiring verify to work for any "Arthur" prevents it
    /// from depending on the "Merlin" part which
    /// is also implemented for the transcript hasher.
    fn verify<S: Sponge>(
        ts: &mut Arthur<S>,
        st: Msg<Self::Statement>,
        pf: Self::Proof,
    ) -> Result<(), Self::Error>;
}
