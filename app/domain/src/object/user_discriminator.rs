use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDiscriminator(String);

impl UserDiscriminator {
    pub fn new(discriminator: String) -> Self {
        Self(discriminator)
    }
}
