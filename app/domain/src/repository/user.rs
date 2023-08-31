use async_trait::async_trait;
use rusty_ulid::Ulid;
use thiserror::Error;

use crate::{entity::User, object::UserDiscriminator};

#[async_trait]
pub trait IUserRepository {
    fn find_by_id(&self, user_id: Ulid) -> Result<User, Box<dyn std::error::Error>>;
    fn find_by_discriminator(
        &self,
        discriminator: UserDiscriminator,
    ) -> Result<User, Box<dyn std::error::Error>>;
    async fn add(&self, user: User) -> Result<(), UserRepositoryAddError>;
}

#[derive(Error, Debug)]
pub enum UserRepositoryAddError {
    #[error("Duplicate discriminator")]
    DuplicateDiscriminator(Box<dyn std::error::Error>),
    #[error("Duplicate email")]
    DuplicateEmail(Box<dyn std::error::Error>),
    #[error("Other error")]
    OtherError(Box<dyn std::error::Error>),
}
