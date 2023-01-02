use serde::{Deserialize, Serialize};

/// Just like hotel california:
/// you can check in, but you can never leave...
pub struct Msg<T>(pub(crate) T);

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
