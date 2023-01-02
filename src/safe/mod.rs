mod arthur;
mod proof;

pub use arthur::Arthur;
pub use proof::{Proof, Bevis, SafeProof};

use core::fmt::Debug;

use crate::Msg;

/// Guarantees that the transcript occurs only in a 
/// context in which it is bound to the statement.
pub trait Safe: arthur::Sealed {}

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
impl<T: Debug> Tx for Msg<T> {
    #[inline(always)]
    fn read(&self) {}
}
