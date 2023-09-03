use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct UserDiscriminator(String);

impl UserDiscriminator {
    pub fn new(discriminator: String) -> Result<Self, String> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9-]{2,23}$").unwrap());
        if RE.is_match(&discriminator) {
            Ok(Self(discriminator))
        } else {
            Err("バリデーションエラー".to_string())
        }
    }
}

impl<'de> Deserialize<'de> for UserDiscriminator {
    fn deserialize<D>(deserializer: D) -> Result<UserDiscriminator, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        UserDiscriminator::new(s).map_err(|e| serde::de::Error::custom(e))
    }
}

impl From<UserDiscriminator> for String {
    fn from(value: UserDiscriminator) -> Self {
        value.0
    }
}

impl TryFrom<String> for UserDiscriminator {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check() {
        let valid_discriminator = "abcde".to_string();
        let invalid_discriminator = "abcあ".to_string();
        let invalid_discriminator2 = "ab".to_string();
        assert!(super::UserDiscriminator::new(valid_discriminator).is_ok());
        assert!(super::UserDiscriminator::new(invalid_discriminator).is_err());
        assert!(super::UserDiscriminator::new(invalid_discriminator2).is_err());
    }
}
