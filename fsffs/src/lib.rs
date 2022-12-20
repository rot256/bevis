extern crate fsffs_derive;

pub use fsffs_derive::*;

pub use core::hash::Hasher;

#[cfg(feature = "enable_serde")]
use serde::{Serialize, Deserialize};

mod transcript;
mod challenge;

/// You should implement this only for primitive types.
/// (e.g. curve points or field elements)
/// 
/// Absorb is a machine indepent encoding,
/// e.g. the bytes written to the Hasher must not depent 
/// on the machine endianness.
pub trait Absorb {
    fn absorb<H: Hasher>(&self, h: &mut H);
}

/// A type which can be generated from the Sponge:
/// a message from the verifier.
/// 
/// You should derive this for more complex types.
pub trait Challenge {
    fn sample<S: Sponge>(ts: &mut S) -> Self; 
}

/// Just like hotel california: 
/// you can check in, but you can never leave...
pub struct Msg<T: Absorb>(T);

// Messages serialize without overhead
#[cfg(feature = "enable_serde")]
impl <T: Serialize + Absorb> Serialize for Msg<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.0.serialize(serializer)
    }
}

// Messages deserialize without overhead
#[cfg(feature = "enable_serde")]
impl <'de, T: Deserialize<'de> + Absorb> Deserialize<'de> for Msg<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
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

// TODO: implement serialize/deserialize for Msg when T is.
impl<T: Absorb> From<T> for Msg<T> {
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
    pub fn verify<P: Proof>(
        &mut self,
        st: P::Statement,
        pf: P::Proof
    ) -> Result<(), <P as Proof>::Error> {
        // absorb the statement
        st.absorb(self);

        // consume the interaction
        P::interact(self, &st, pf)
    }
}

impl <S: Sponge> Hasher for Arthur<S> {
    fn write(&mut self, bytes: &[u8]) {
        self.sponge.write(bytes)
    }

    fn finish(&self) -> u64 {
        unimplemented!()
    }
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
    type Statement: Absorb; // entire statement is read up-front (i.e. no messages)
    type Proof: Tx; // a proof consists of multiple messages
    type Error;

    /// Requiring verify to work for any "Arthur" prevents it
    /// from depending on the "Merlin" part which
    /// is also implemented for the transcript hasher.
    /// 
    /// You should not invoke this method directly.
    /// Since "recv" takes ownership of its input,
    /// it is not possible to receieve partx of the statement:
    /// (it must be fixed up-front)
    fn interact<S: Sponge>(
        ts: &mut Arthur<S>,
        st: &Self::Statement,
        pf: Self::Proof,
    ) -> Result<(), Self::Error>;
}


