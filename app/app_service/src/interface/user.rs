use async_trait::async_trait;
use domain::object::{email_address::EmailAddress, url::Url, UserDiscriminator, UserName};

#[async_trait]
pub trait IUserAppService {
    async fn register(
        &self,
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        web_page: Option<Url>,
    ) -> Result<(), UserRegisterError>;
}

pub enum UserRegisterError {
    DuplicateDiscriminator,
    DuplicateEmail,
    OtherError,
}
