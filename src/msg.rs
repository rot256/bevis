use core::fmt::Debug;

use serde::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(Debug)]
pub struct Msg<T>(pub(crate) T);

impl <T> From<T> for Msg<T> {
    fn from(value: T) -> Self {
        Msg(value)
    }
}

impl <T: Clone> Clone for Msg<T> {
    fn clone(&self) -> Self {
        Msg(self.0.clone())
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
