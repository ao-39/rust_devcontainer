use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}
