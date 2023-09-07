use app_service_interface::user::{
    IUserAppService, UserDeleteError, UserDto, UserFindError, UserRegisterError, UserUpdateError,
    UserUpdateOperator,
};
use async_trait::async_trait;
use domain::{
    entity::User,
    object::{
        chrono::Local, email_address::EmailAddress, rusty_ulid::Ulid, url::Url, UserDiscriminator,
        UserName,
    },
    repository::{
        IUserRepository, UserRepositoryAddError, UserRepositoryDeleteError,
        UserRepositoryFindError, UserRepositoryUpdateError,
    },
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
impl<T: IUserRepository + Sync + Send> IUserAppService for UserApplicationService<T> {
    async fn register(
        &self,
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        web_page: Option<Url>,
    ) -> Result<(), UserRegisterError> {
        let ulid = Ulid::generate();
        let now = Local::now();
        let user = User::new(ulid, discriminator, name, email, web_page, now, now);

        let res = self.user_repository.add(user).await;

        match res {
            Err(err) => match err {
                UserRepositoryAddError::DuplicateDiscriminator => {
                    Err(UserRegisterError::DuplicateDiscriminator)
                }
                UserRepositoryAddError::DuplicateEmail => Err(UserRegisterError::DuplicateEmail),
                _ => Err(UserRegisterError::OtherError),
            },
            Ok(_) => Ok(()),
        }
    }

    async fn find_by_discriminator(
        &self,
        discriminator: UserDiscriminator,
    ) -> Result<UserDto, UserFindError> {
        let res = self
            .user_repository
            .find_by_discriminator(discriminator)
            .await;

        match res {
            Err(err) => match err {
                UserRepositoryFindError::NotFound => Err(UserFindError::NotFound),
                _ => Err(UserFindError::OtherError),
            },
            Ok(user) => Ok(user.into()),
        }
    }

    async fn find_by_email(&self, email: EmailAddress) -> Result<UserDto, UserFindError> {
        let res = self.user_repository.find_by_email(email).await;

        match res {
            Err(err) => match err {
                UserRepositoryFindError::NotFound => Err(UserFindError::NotFound),
                _ => Err(UserFindError::OtherError),
            },
            Ok(user) => Ok(user.into()),
        }
    }

    async fn delete(&self, discriminator: UserDiscriminator) -> Result<(), UserDeleteError> {
        let res = self.user_repository.delete(discriminator).await;

        match res {
            Err(err) => match err {
                UserRepositoryDeleteError::NotFound => Err(UserDeleteError::NotFound),
                _ => Err(UserDeleteError::OtherError),
            },
            Ok(_) => Ok(()),
        }
    }

    async fn update(
        &self,
        discriminator: UserDiscriminator,
        update_operator: UserUpdateOperator,
    ) -> Result<(), UserUpdateError> {
        let mut user = self
            .user_repository
            .find_by_discriminator(discriminator)
            .await
            .map_err(|err| match err {
                UserRepositoryFindError::NotFound => UserUpdateError::NotFound,
                _ => UserUpdateError::OtherError,
            })?;

        match update_operator {
            UserUpdateOperator::Discriminator(discriminator) => {
                user.discriminator = discriminator;
            }
            UserUpdateOperator::Name(name) => {
                user.name = name;
            }
            UserUpdateOperator::Email(email) => {
                user.email = email;
            }
            UserUpdateOperator::WebPage(web_page) => {
                user.web_page = web_page;
            }
        }

        let res = self.user_repository.update(user).await;

        match res {
            Err(err) => match err {
                UserRepositoryUpdateError::NotFound => Err(UserUpdateError::NotFound),
                UserRepositoryUpdateError::DuplicateDiscriminator => {
                    Err(UserUpdateError::DuplicateDiscriminator)
                }
                UserRepositoryUpdateError::DuplicateEmail => Err(UserUpdateError::DuplicateEmail),
                _ => Err(UserUpdateError::OtherError),
            },
            Ok(_) => Ok(()),
        }
    }
}
