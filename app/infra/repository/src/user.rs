use async_trait::async_trait;
use domain::{
    entity::User,
    object::{email_address, rusty_ulid, UserDiscriminator},
    repository::{
        IUserRepository, UserRepositoryAddError, UserRepositoryDeleteError,
        UserRepositoryFindError, UserRepositoryUpdateError,
    },
};

use entity::implementations::prelude::DbErrUtils;
use sea_orm::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct UserRepository {
    db: DatabaseConnection,
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn find_by_id(&self, user_id: rusty_ulid::Ulid) -> Result<User, UserRepositoryFindError> {
        let res = entity::user::Entity::find_by_id(user_id.to_string())
            .one(&self.db)
            .await;

        match res {
            Err(_) => Err(UserRepositoryFindError::OtherError),
            Ok(None) => Err(UserRepositoryFindError::NotFound),
            Ok(Some(user)) => user
                .try_into()
                .map_err(|_| UserRepositoryFindError::DeserializeError),
        }
    }

    async fn find_by_discriminator(
        &self,
        discriminator: UserDiscriminator,
    ) -> Result<User, UserRepositoryFindError> {
        let res = entity::user::Entity::find()
            .filter(entity::user::Column::Discriminator.eq(Into::<String>::into(discriminator)))
            .one(&self.db)
            .await;

        match res {
            Err(_) => Err(UserRepositoryFindError::OtherError),
            Ok(None) => Err(UserRepositoryFindError::NotFound),
            Ok(Some(user)) => user
                .try_into()
                .map_err(|_| UserRepositoryFindError::DeserializeError),
        }
    }

    async fn find_by_email(
        &self,
        email: email_address::EmailAddress,
    ) -> Result<User, UserRepositoryFindError> {
        let res = entity::user::Entity::find()
            .filter(entity::user::Column::Email.eq(email.to_string()))
            .one(&self.db)
            .await;

        match res {
            Err(_) => Err(UserRepositoryFindError::OtherError),
            Ok(None) => Err(UserRepositoryFindError::NotFound),
            Ok(Some(user)) => user
                .try_into()
                .map_err(|_| UserRepositoryFindError::DeserializeError),
        }
    }

    async fn add(&self, user: User) -> Result<(), UserRepositoryAddError> {
        let res = entity::user::ActiveModel::from(user).insert(&self.db).await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                let constrint = err.get_db_constrint_err();
                match constrint {
                    Some(constrint) => match constrint.as_str() {
                        "user_discriminator_key" => {
                            Err(UserRepositoryAddError::DuplicateDiscriminator)
                        }
                        "user_email_key" => Err(UserRepositoryAddError::DuplicateEmail),
                        _ => Err(UserRepositoryAddError::OtherError),
                    },
                    None => Err(UserRepositoryAddError::OtherError),
                }
            }
        }
    }

    async fn delete(
        &self,
        discriminator: UserDiscriminator,
    ) -> Result<(), UserRepositoryDeleteError> {
        let res = entity::user::Entity::find()
            .filter(entity::user::Column::Discriminator.eq(Into::<String>::into(discriminator)))
            .one(&self.db)
            .await;

        match res {
            Err(_) => Err(UserRepositoryDeleteError::OtherError),
            Ok(None) => Err(UserRepositoryDeleteError::NotFound),
            Ok(Some(user)) => {
                let res = user.delete(&self.db).await;

                match res {
                    Ok(_) => Ok(()),
                    Err(_) => Err(domain::repository::UserRepositoryDeleteError::OtherError),
                }
            }
        }
    }

    async fn update(&self, user: User) -> Result<(), UserRepositoryUpdateError> {
        let res = entity::user::Entity::find_by_id(user.id.to_string())
            .one(&self.db)
            .await;

        match res {
            Err(_) => Err(UserRepositoryUpdateError::OtherError),
            Ok(None) => Err(UserRepositoryUpdateError::NotFound),
            Ok(Some(found_user)) => {
                let res = Into::<entity::user::ActiveModel>::into(user)
                    .update(&self.db)
                    .await;

                match res {
                    Ok(_) => Ok(()),
                    Err(e) => match e.get_db_constrint_err() {
                        Some(constrint) => match constrint.as_str() {
                            "user_discriminator_key" => {
                                Err(UserRepositoryUpdateError::DuplicateDiscriminator)
                            }
                            "user_email_key" => Err(UserRepositoryUpdateError::DuplicateEmail),
                            _ => Err(UserRepositoryUpdateError::OtherError),
                        },
                        None => Err(UserRepositoryUpdateError::OtherError),
                    },
                }
            }
        }
    }
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
