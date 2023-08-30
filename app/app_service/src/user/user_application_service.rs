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

#[async_trait]
pub trait IUserApplicationService<T: IUserRepository + Sync + Send> {
    fn new(user_repostitory: T) -> Self;

    async fn register(
        &self,
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        web_page: Option<Url>,
    ) -> Result<User, Box<dyn std::error::Error>>;
}

#[async_trait]
impl<T: IUserRepository + Sync + Send> IUserApplicationService<T> for UserApplicationService<T> {
    fn new(user_repostitory: T) -> Self {
        Self {
            user_repository: user_repostitory,
        }
    }

    async fn register(
        &self,
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        web_page: Option<Url>,
    ) -> Result<User, Box<dyn std::error::Error>> {
        let ulid = Ulid::generate();
        let user = User::new(
            ulid.clone(),
            discriminator,
            name,
            email,
            web_page,
            Local::now(),
            Local::now(),
        );

        self.user_repository.add(user.clone()).await?;
        Ok(user)
    }
}
