use async_trait::async_trait;
use rusty_ulid::Ulid;

use crate::{entity::User, object::UserDiscriminator};

#[async_trait]
pub trait IUserRepository {
    fn find_by_id(&self, user_id: Ulid) -> Result<User, Box<dyn std::error::Error>>;
    fn find_by_discriminator(
        &self,
        discriminator: UserDiscriminator,
    ) -> Result<User, Box<dyn std::error::Error>>;
    async fn add(&self, user: User) -> Result<(), Box<dyn std::error::Error>>;
}
