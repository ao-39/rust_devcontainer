use async_trait::async_trait;
use domain::{
    entity::User,
    object::{
        chrono::Local, email_address::EmailAddress, rusty_ulid::Ulid, url::Url, UserDiscriminator,
        UserName,
    },
    repository::IUserRepository,
};

pub struct UserApplicationService<T: IUserRepository + Sync + Send> {
    user_repository: T,
}

impl<T: IUserRepository + Sync + Send> UserApplicationService<T> {
    pub fn new(user_repostitory: T) -> Self {
        Self {
            user_repository: user_repostitory,
        }
    }
}

#[async_trait]
pub trait IUserApplicationService {
    async fn register(
        &self,
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        web_page: Option<Url>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl<T: IUserRepository + Sync + Send> IUserApplicationService for UserApplicationService<T> {
    async fn register(
        &self,
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        web_page: Option<Url>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ulid = Ulid::generate();
        let now = Local::now();
        let user = User::new(ulid, discriminator, name, email, web_page, now, now);

        self.user_repository.add(user).await?;
        Ok(())
    }
}

pub enum UserRegisterError {
    DuplicateDiscriminator,
    DuplicateEmailAddress,
    OtherError,
}
