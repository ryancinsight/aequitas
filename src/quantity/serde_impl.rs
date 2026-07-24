use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Quantity;

impl<T, D> Serialize for Quantity<T, D>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_base().serialize(serializer)
    }
}

impl<'de, T, D> Deserialize<'de> for Quantity<T, D>
where
    T: Deserialize<'de>,
{
    fn deserialize<Ds>(deserializer: Ds) -> Result<Self, Ds::Error>
    where
        Ds: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Self::from_base)
    }
}
