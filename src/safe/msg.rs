use serde::{Deserialize, Serialize};

use crate::Absorb;

/// Just like hotel california:
/// you can check in, but you can never leave...
pub struct Msg<T>(pub(crate) T);

/// Any type T can be converted into Msg<T>
impl<T: Absorb> From<T> for Msg<T> {
    #[inline(always)]
    fn from(v: T) -> Self {
        Self(v)
    }
}

/// Messages serialize without overhead
impl<T: Serialize> Serialize for Msg<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

/// Messages deserialize without overhead
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Msg<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|v| Msg(v))
    }
}

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
impl<T: Absorb> Tx for Msg<T> {
    #[inline(always)]
    fn read(&self) {}
}