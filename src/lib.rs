#![no_std]

extern crate bevis_derive;

pub use rand_core::{CryptoRng, RngCore};

mod absorb;
mod challenge;
mod transcript;

// safe interface
#[cfg(feature = "safe")]
mod safe;

// safe interface
#[cfg(feature = "safe")]
pub use safe::{Msg, Tx, Arthur, Proof};

pub use bevis_derive::*;

pub use absorb::{Absorb, Hasher};

pub use transcript::{Transcript, SpongeTranscript};

/// A type which can be sampled uniformly. Provided for convience.
/// Challenges can also be sampled directly using the sponge impl of RngCore.
pub trait Challenge {
    fn sample<S: CryptoRng + RngCore>(ts: &mut S) -> Self;
}



/// A "Sponge" enables both hashing and sampling (squeezing)
pub trait Sponge: Hasher + RngCore + CryptoRng + Sized {
  
}
