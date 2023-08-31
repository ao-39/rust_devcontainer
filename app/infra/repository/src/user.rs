use async_trait::async_trait;
use domain::repository::{IUserRepository, UserRepositoryAddError};

use entity::implementations::prelude::DbErrUtils;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

pub struct UserRepository {
    db: DatabaseConnection,
}

#[async_trait]
impl IUserRepository for UserRepository {
    fn find_by_id(
        &self,
        user_id: domain::object::rusty_ulid::Ulid,
    ) -> Result<domain::entity::User, Box<dyn std::error::Error>> {
        todo!()
    }
    fn find_by_discriminator(
        &self,
        discriminator: domain::object::UserDiscriminator,
    ) -> Result<domain::entity::User, Box<dyn std::error::Error>> {
        todo!()
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
