use async_trait::async_trait;
use domain::{
    entity::User,
    object::{email_address::EmailAddress, url::Url, UserDiscriminator, UserName},
};

#[async_trait]
pub trait IUserAppService {
    async fn register(
        &self,
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        web_page: Option<Url>,
    ) -> Result<(), UserRegisterError>;

    async fn find_by_discriminator(
        &self,
        discriminator: UserDiscriminator,
    ) -> Result<User, UserFindError>;

    async fn find_by_email(&self, email: EmailAddress) -> Result<User, UserFindError>;

    async fn delete(&self, discriminator: UserDiscriminator) -> Result<(), UserDeleteError>;

    async fn update(
        &self,
        discriminator: UserDiscriminator,
        update_operator: UserUpdateOperator,
    ) -> Result<(), UserUpdateError>;
}

pub enum UserRegisterError {
    DuplicateDiscriminator,
    DuplicateEmail,
    OtherError,
}

pub enum UserFindError {
    NotFound,
    OtherError,
}

pub enum UserDeleteError {
    NotFound,
    OtherError,
}

pub enum UserUpdateError {
    NotFound,
    DuplicateDiscriminator,
    DuplicateEmail,
    OtherError,
}

pub enum UserUpdateOperator {
    Discriminator(UserDiscriminator),
    Name(UserName),
    Email(EmailAddress),
    WebPage(Option<Url>),
}
