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
    ) -> Result<UserDto, UserFindError>;

    async fn find_by_email(&self, email: EmailAddress) -> Result<UserDto, UserFindError>;

    async fn delete(&self, discriminator: UserDiscriminator) -> Result<(), UserDeleteError>;

    async fn update(
        &self,
        discriminator: UserDiscriminator,
        update_operator: UserUpdateOperator,
    ) -> Result<(), UserUpdateError>;
}

pub struct UserDto {
    pub discriminator: UserDiscriminator,
    pub name: UserName,
    pub email: EmailAddress,
    pub web_page: Option<Url>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            discriminator: user.discriminator,
            name: user.name,
            email: user.email,
            web_page: user.web_page,
        }
    }
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

impl From<UserUpdateOperator> for domain::repository::UserUpdateOperator {
    fn from(value: UserUpdateOperator) -> Self {
        match value {
            UserUpdateOperator::Discriminator(discriminator) => {
                domain::repository::UserUpdateOperator::Discriminator(discriminator)
            }
            UserUpdateOperator::Name(name) => domain::repository::UserUpdateOperator::Name(name),
            UserUpdateOperator::Email(email) => {
                domain::repository::UserUpdateOperator::Email(email)
            }
            UserUpdateOperator::WebPage(web_page) => {
                domain::repository::UserUpdateOperator::WebPage(web_page)
            }
        }
    }
}
