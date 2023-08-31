use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> Result<Self, String> {
        if name.len() >= 3 && name.len() <= 80 {
            Ok(Self(name))
        } else {
            return Err("バリデーションエラー".to_string());
        }
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
