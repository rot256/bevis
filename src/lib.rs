#![no_std]

extern crate bevis_derive;

use challenge::Sampler;
pub use rand_core::{CryptoRng, RngCore};

mod rng;
mod msg;
mod absorb;
mod challenge;
mod transcript;

// safe interface
// #[cfg(feature = "safe")]
mod safe;

// safe interface
#[cfg(feature = "safe")]
pub use safe::{Tx, Arthur, Proof};

pub use rng::AsRng;

pub use bevis_derive::*;

pub use absorb::{Absorb, Hasher};

pub use transcript::{Transcript, SpongeTranscript};

pub use challenge::Challenge;

pub use msg::Msg;

/// A "Sponge" enables both hashing and sampling (squeezing)
pub trait Sponge<W>: Hasher<W> + Sampler<W> {}