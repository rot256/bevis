#![no_std]

extern crate bevis_derive;

pub use rand_core::{CryptoRng, RngCore};

mod absorb;
mod challenge;
mod msg;
mod rng;
mod transcript;

// safe-proof interface
#[cfg(feature = "safe")]
mod safe;

// safe-proof interface
#[cfg(feature = "safe")]
pub use safe::{Arthur, Proof, Tx};

pub use rng::AsRng;

pub use bevis_derive::*;

pub use absorb::{Absorb, Hasher};

pub use transcript::{SpongeTranscript, Transcript};

pub use challenge::{Challenge, Sampler};

pub use msg::Msg;

/// A "Sponge" enables both hashing and sampling (squeezing)
pub trait Sponge<W>: Hasher<W> + Sampler<W> {}
