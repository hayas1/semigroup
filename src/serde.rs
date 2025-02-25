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

#[cfg(test)]
mod tests {
    use crate::Prior;

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct Config<'a> {
        name: &'a str,
        number: Coalesced<Option<i64>, Prior>,
    }

    #[test]
    fn test_coalesced_serialize() {
        let file = Config {
            name: "file",
            number: Coalesced::new_prior(Some(1)),
        };
        let env = Config {
            name: "env",
            number: Coalesced::new_prior(Some(10)),
        };
        let cli = Config {
            name: "cli",
            number: Coalesced::new_prior(Some(100)),
        };

        let config = Config {
            name: "config",
            number: file
                .number
                .extend_prior(env.number)
                .extend_prior(cli.number),
        };

        let serialized = serde_json::to_string(&config).unwrap();
        assert_eq!(serialized, r#"{"name":"config","number":100}"#);
    }
    #[test]
    fn test_coalesced_deserialize() {
        let file: Config = serde_json::from_str(r#"{"name":"file","number":1}"#).unwrap();
        let env: Config = serde_json::from_str(r#"{"name":"env","number":10}"#).unwrap();
        let cli: Config = serde_json::from_str(r#"{"name":"cli","number":100}"#).unwrap();

        let config = Config {
            name: "config",
            number: file
                .number
                .extend_prior(env.number)
                .extend_prior(cli.number),
        };

        assert_eq!(config.number.unwrap(), 100);
    }
}
