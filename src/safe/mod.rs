mod proof;
mod arthur;

use crate::Msg;

pub trait Sealed {}

/// Marker trait.
///
/// Transcripts consists of structs where all
/// absorbable base-types are wrapped in Msg.
pub trait Tx {
    fn read(&self);
}

/// Tx is implemented for Msg
/// and can be derived for more complex types.
///
/// When https://github.com/rust-lang/rust/issues/68318
/// lands we can improve this.
impl<T> Tx for Msg<T> {
    #[inline(always)]
    fn read(&self) {}
}

pub use arthur::{Arthur};
pub use proof::{Proof, SafeProof};