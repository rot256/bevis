#![no_std]

#[cfg(feature = "trace")]
extern crate alloc;

extern crate bevis_derive;

pub use rand_core::{CryptoRng, RngCore};

mod absorb;
mod challenge;
mod msg;
mod transcript;

#[cfg(feature = "trace")]
mod trace;

// safe-proof interface
#[cfg(feature = "safe")]
mod safe;

// safe-proof interface
#[cfg(feature = "safe")]
pub use safe::{Arthur, Proof, Tx};

pub use bevis_derive::*;

pub use absorb::{Absorb, Hasher};

pub use transcript::{SpongeTranscript, Transcript};

#[cfg(feature = "trace")]
pub use trace::{TraceTranscript};

pub use challenge::{Challenge, Sampler};

pub use msg::Msg;

/// A "Sponge" enables both hashing and sampling (squeezing)
pub trait Sponge: Hasher + Sampler {
    fn new(sep: &str) -> Self;
}