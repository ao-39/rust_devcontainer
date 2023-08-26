use domain::{
    entity::User,
    object::{
        chrono::Local, email_address::EmailAddress, rusty_ulid::Ulid, url::Url, UserDiscriminator,
        UserName,
    },
    repository::IUserRepository,
};

pub struct UserApplicationService<T: IUserRepository> {
    user_repository: T,
}

pub trait IUserApplicationService {
    fn register(
        &self,
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        web_page: Option<Url>,
    ) -> Result<User, Box<dyn std::error::Error>>;
}

impl<T: IUserRepository> IUserApplicationService for UserApplicationService<T> {
    fn register(
        &self,
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        web_page: Option<Url>,
    ) -> Result<User, Box<dyn std::error::Error>> {
        let user = User::new(
            Ulid::generate(),
            discriminator,
            name,
            email,
            web_page,
            Local::now(),
            Local::now(),
        );

        self.user_repository.add(user.clone())?;
        Ok(user)
    }
}
