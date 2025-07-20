//! TODO doc

use serde::{Deserialize, Serialize};

use crate::{
    coalesce::Coalesce,
    coalesced::{Coalesced, IntoHistory},
};

impl<T> Serialize for Coalesced<T>
where
    T: Coalesce + Clone + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.into_cloned().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Coalesced<T>
where
    T: Coalesce + Clone + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(T::deserialize(deserializer)?.into_history())
    }
}

#[cfg(test)]
mod tests {
    use coalesced::{strategy::overwrite::Overwrite, Coalesce};

    use super::*;

    #[derive(Coalesce, Serialize, Deserialize)]
    struct Config {
        name: Overwrite<String>,
        number: Option<i64>,
    }

    #[test]
    fn test_deserialize_without_history() {
        let file: Config = serde_json::from_str(r#"{"name":"file","number":1}"#).unwrap();
        let env: Config = serde_json::from_str(r#"{"name":"env","number":10}"#).unwrap();
        let cli: Config = serde_json::from_str(r#"{"name":"cli"}"#).unwrap();

        let cfg = file.prior(env).prior(cli);

        assert_eq!(cfg.name, Overwrite("cli".to_string()));
        assert_eq!(cfg.number, Some(10));
    }

    #[test]
    fn test_deserialize_with_history() {
        // TODO
    }
}
