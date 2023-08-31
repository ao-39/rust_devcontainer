use async_trait::async_trait;
use rusty_ulid::Ulid;
use thiserror::Error;

use crate::{entity::User, object::UserDiscriminator};

#[async_trait]
pub trait IUserRepository {
    async fn find_by_id(&self, user_id: Ulid) -> Result<User, UserRepositoryFindError>;
    async fn find_by_discriminator(
        &self,
        discriminator: UserDiscriminator,
    ) -> Result<User, UserRepositoryFindError>;
    async fn add(&self, user: User) -> Result<(), UserRepositoryAddError>;
}

#[derive(Error, Debug)]
pub enum UserRepositoryAddError {
    #[error("Duplicate discriminator")]
    DuplicateDiscriminator,
    #[error("Duplicate email")]
    DuplicateEmail,
    #[error("Other error")]
    OtherError,
}

#[derive(Error, Debug)]
pub enum UserRepositoryFindError {
    #[error("User not found")]
    NotFound,
    #[error("Deserialize error")]
    DeserializeError,
    #[error("Other error")]
    OtherError,
}
