#![no_std]

#[cfg(feature = "trace")]
extern crate alloc;

extern crate bevis_derive;

pub use rand_core::{CryptoRng, RngCore};

mod absorb;
mod challenge;
mod msg;
mod transcript;

// debugging trace transcript
#[cfg(feature = "trace")]
mod trace;

// debugging trace transcript
#[cfg(feature = "trace")]
pub use trace::TraceTranscript;

// safe-proof interface
#[cfg(feature = "safe")]
mod safe;

// safe-proof interface
#[cfg(feature = "safe")]
pub use safe::{Arthur, Bevis, Proof, Safe, SafeProof, Tx};

pub use bevis_derive::*;

pub use absorb::{Absorb, Hasher};

pub use transcript::{SpongeTranscript, Transcript};

pub use challenge::{Challenge, Sampler};

pub use msg::Msg;

/// A "Sponge" enables both hashing and sampling (squeezing)
pub trait Sponge: Hasher + Sampler {
    fn new(sep: &str) -> Self;
}
