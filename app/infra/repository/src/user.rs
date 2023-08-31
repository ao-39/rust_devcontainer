use async_trait::async_trait;
use domain::{
    object::{rusty_ulid, UserDiscriminator},
    repository::{IUserRepository, UserRepositoryAddError, UserRepositoryFindError},
};

use entity::implementations::prelude::DbErrUtils;
use sea_orm::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct UserRepository {
    db: DatabaseConnection,
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn find_by_id(
        &self,
        user_id: rusty_ulid::Ulid,
    ) -> Result<domain::entity::User, UserRepositoryFindError> {
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
    ) -> Result<domain::entity::User, UserRepositoryFindError> {
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

    async fn add(&self, user: domain::entity::User) -> Result<(), UserRepositoryAddError> {
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
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
