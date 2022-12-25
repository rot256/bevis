#![no_std]

// used to impl. Absorb for Vec<_>
#[cfg(feature = "alloc")]
extern crate alloc;

extern crate bevis_derive;

pub use bevis_derive::*;

pub use core::hash::Hasher;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod challenge;

mod transcript;

/// You should implement this only for primitive types.
/// (e.g. curve points or field elements)
///
/// Absorb is a machine indepent encoding,
/// e.g. the bytes written to the Hasher must not depent
/// on the machine endianness.
pub trait Absorb {
    fn absorb<H: Hasher>(&self, h: &mut H);
}

/// You should derive this for more complex types.
pub trait Challenge {
    fn sample<S: Sampler>(ts: &mut S) -> Self;
}

/// Just like hotel california:
/// you can check in, but you can never leave...
pub struct Msg<T: Absorb>(T);

// Clone is provided for convience, however,
// if you find yourself cloning proofs
// you are probably doing something wrong.
impl<T: Clone + Absorb> Clone for Msg<T> {
    fn clone(&self) -> Self {
        Msg(self.0.clone())
    }
}

// Messages serialize without overhead
#[cfg(feature = "serde")]
impl<T: Serialize + Absorb> Serialize for Msg<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// Messages deserialize without overhead
#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de> + Absorb> Deserialize<'de> for Msg<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|v| Msg(v))
    }
}

/// Transcripts consists of structs where all
/// absorbable base-types are wrapped in one/more Msg.
pub trait Tx {
    fn read<H: Hasher>(&self, ts: &mut H);
}

impl<T: Absorb> Tx for Msg<T> {
    // absorbing a message is a no-op:
    // the content is ignored until it is unwrapped.
    fn read<H: Hasher>(&self, _ts: &mut H) {}
}

/// Any type T can be converted into Msg<T>
impl<T: Absorb> From<T> for Msg<T> {
    fn from(v: T) -> Self {
        Self(v)
    }
}

pub trait Sampler {
    fn sample(&mut self) -> u8;
}

/// A "Sponge" enables both hashing and sampling (squeezing)
pub trait Sponge: Hasher + Sampler + Sized {
    ///
    /// An implementation overwriting this
    ///
    /// In-order to verify a statement it must be absorbable,
    /// note that sub-protocols do not need absorable statements.
    ///
    /// This method cannot be overwritten since Arthur has no public constructor.
    fn verify<P: Proof>(
        &mut self,
        st: &P::Statement,
        pf: P,
    ) -> Result<P::Result, <P as Proof>::Error>
    where
        P::Statement: Absorb,
    {
        // oracle seperation
        P::NAME.absorb(self);

        // read the statement
        st.absorb(self);

        // create Arthur instance
        // (enabling the interaction to receive Msg<T>)
        let mut arthur = Arthur { sponge: self };

        // run the interaction
        pf.interact(st, &mut arthur)
    }

    /// Provide for convience:
    /// Makes it easier to simulate the interaction with the verifier to create a proof.
    ///
    /// This method cannot be overwritten since Merlin has no public constructor.
    fn prove<'a, P: Proof, F: FnOnce(&P::Statement, &mut Merlin<'a, Self>) -> P>(
        &'a mut self,
        st: &P::Statement,
        prover: F,
    ) -> P
    where
        P::Statement: Absorb,
    {
        // oracle seperation
        P::NAME.absorb(self);

        // read the statement
        st.absorb(self);

        // create Merlin instance
        // (providing a more convient way to construct Msg<T>)
        let mut merlin = Merlin { sponge: self };

        // run the prover to obtain the proof
        prover(st, &mut merlin)
    }
}

/// Representing the verifiers view.
pub struct Arthur<'a, S: Sponge> {
    sponge: &'a mut S,
}

/// Representing the provers view.
pub struct Merlin<'a, S: Sponge> {
    sponge: &'a mut S,
}

impl<'a, S: Sponge> Arthur<'a, S> {
    /// Receives a message from the prover
    pub fn recv<T: Absorb>(&mut self, elem: Msg<T>) -> T {
        elem.0.absorb(self.sponge);
        elem.0
    }

    /// Sends a challenge to the prover
    pub fn send<T: Challenge>(&mut self) -> T {
        T::sample(self.sponge)
    }
}

impl<'a, S: Sponge> Merlin<'a, S> {
    /// Sends a message to the verifier
    pub fn send<T: Absorb>(&mut self, elem: T) -> Msg<T> {
        elem.absorb(self.sponge);
        elem.into()
    }

    /// Receives a challenge from the verifier
    pub fn recv<T: Challenge>(&mut self) -> T {
        T::sample(self.sponge)
    }
}

/// A proof is a transcript, meaning it consists of:
///
/// - Msg's messages.
/// - Other transcripts.
pub trait Proof: Tx {
    type Statement;
    type Error;
    type Result;

    /// Every protocol should have a unique identifier.
    /// It can be chosen arbitarily.
    const NAME: &'static [u8];

    /// You CANNOT invoke this method directly
    /// (because Arthur does not have a public constructor),
    /// instead you must use .verify.
    ///
    /// However, you CAN invoke this "interact" method from other "interact" methods.
    /// This enables safely composing sub-protocols without comitting to the intermediate statements
    /// and domain seperators.
    ///
    /// Note that the statement is a reference.
    fn interact<S: Sponge>(
        self,
        st: &Self::Statement,
        ts: &mut Arthur<'_, S>,
    ) -> Result<Self::Result, Self::Error>;
}
