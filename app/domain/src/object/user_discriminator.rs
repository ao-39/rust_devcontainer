use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDiscriminator(String);

impl UserDiscriminator {
    pub fn new(discriminator: String) -> Result<Self, String> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9-]{2,23}$").unwrap());
        if RE.is_match(&discriminator) {
            return Err("バリデーションエラー".to_string());
        };
        Ok(Self(discriminator))
    }
}
