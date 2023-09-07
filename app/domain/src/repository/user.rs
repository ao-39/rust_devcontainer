use async_trait::async_trait;
use email_address::EmailAddress;
use rusty_ulid::Ulid;
use thiserror::Error;

use crate::{
    entity::User,
    object::{url::Url, UserDiscriminator, UserName},
};

#[async_trait]
pub trait IUserRepository {
    async fn find_by_id(&self, user_id: Ulid) -> Result<User, UserRepositoryFindError>;
    async fn find_by_discriminator(
        &self,
        discriminator: UserDiscriminator,
    ) -> Result<User, UserRepositoryFindError>;
    async fn find_by_email(&self, email: EmailAddress) -> Result<User, UserRepositoryFindError>;
    async fn add(&self, user: User) -> Result<(), UserRepositoryAddError>;
    async fn delete(
        &self,
        discriminator: UserDiscriminator,
    ) -> Result<(), UserRepositoryDeleteError>;
    async fn update(
        &self,
        discriminator: UserDiscriminator,
        update_operator: UserUpdateOperator,
    ) -> Result<(), UserRepositoryUpdateError>;
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

#[derive(Error, Debug)]
pub enum UserRepositoryDeleteError {
    #[error("User not found")]
    NotFound,
    #[error("Other error")]
    OtherError,
}

#[derive(Error, Debug)]
pub enum UserRepositoryUpdateError {
    #[error("User not found")]
    NotFound,
    #[error("Duplicate discriminator")]
    DuplicateDiscriminator,
    #[error("Duplicate email")]
    DuplicateEmail,
    #[error("Other error")]
    OtherError,
}

pub enum UserUpdateOperator {
    Discriminator(UserDiscriminator),
    Name(UserName),
    Email(EmailAddress),
    WebPage(Option<Url>),
}
