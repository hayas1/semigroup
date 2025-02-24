use serde::{Deserialize, Serialize};

use crate::{Access, Coalesced, PriorityAccessor};

impl<C, A> Serialize for Coalesced<C, A>
where
    C: Serialize,
    A: Access<Accessor = PriorityAccessor<A>>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&**self).serialize(serializer)
    }
}

impl<'de, C, A> Deserialize<'de> for Coalesced<C, A>
where
    C: Deserialize<'de>,
    A: Access<Accessor = PriorityAccessor<A>>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let coalesce = C::deserialize(deserializer)?;
        Ok(Coalesced::new(coalesce))
    }
}
