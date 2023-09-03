use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> Result<Self, String> {
        let len = name.chars().count();
        if len >= 3 && len <= 80 {
            Ok(Self(name))
        } else {
            return Err("バリデーションエラー".to_string());
        }
    }
}

impl<'de> Deserialize<'de> for UserName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(s).map_err(|e| serde::de::Error::custom(e))
    }
}

impl From<UserName> for String {
    fn from(value: UserName) -> Self {
        value.0
    }
}

impl TryFrom<String> for UserName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn check() {
        let valid_name = "abcde".to_string();
        let invalid_name = "初音".to_string();
        assert!(super::UserName::new(valid_name).is_ok());
        assert!(super::UserName::new(invalid_name).is_err());
    }
}
