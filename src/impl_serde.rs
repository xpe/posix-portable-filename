//! Serde serialization and deserialization with validation.
//!
//! Deserialization fails if the string is not a valid portable filename.

use super::PortableFilename;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for PortableFilename {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for PortableFilename {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        PortableFilename::new(s).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_assert::{Deserializer, Serializer, Token};

    #[test]
    fn serializes_as_string() {
        let valid_str = "okey-dokey";
        let filename = PortableFilename::new(valid_str).unwrap();
        let serializer = Serializer::builder().build();

        let tokens = filename.serialize(&serializer).unwrap();
        assert_eq!(tokens, [Token::Str(valid_str.into())]);
    }

    #[test]
    fn deserializes_from_string() {
        let valid_str = "very_nice.typ";
        let tokens = [Token::Str(valid_str.into())];
        let mut deserializer = Deserializer::builder(tokens).build();
        let filename = PortableFilename::deserialize(&mut deserializer).unwrap();
        assert_eq!(filename.as_str(), valid_str);
    }

    #[test]
    fn rejects_invalid_on_deserialize() {
        let mut deserializer = Deserializer::builder([Token::Str("..".into())]).build();
        assert!(PortableFilename::deserialize(&mut deserializer).is_err());
    }
}
