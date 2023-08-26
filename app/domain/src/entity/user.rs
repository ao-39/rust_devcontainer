use chrono::{DateTime, Local};
use email_address::EmailAddress;
use rusty_ulid::Ulid;
use url::Url;

use serde::{Deserialize, Serialize};

use crate::object::{UserDiscriminator, UserName};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Ulid,
    pub discriminator: UserDiscriminator,
    pub name: UserName,
    pub email: EmailAddress,
    pub web_page: Url,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl User {
    pub fn new(
        id: Ulid,
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        web_page: Url,
        created_at: DateTime<Local>,
        updated_at: DateTime<Local>,
    ) -> Self {
        Self {
            id,
            discriminator,
            name,
            email,
            web_page,
            created_at,
            updated_at,
        }
    }
}
