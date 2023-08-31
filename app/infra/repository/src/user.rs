use async_trait::async_trait;
use domain::repository::{IUserRepository, UserRepositoryAddError};

use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

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
        let res = entity::user::ActiveModel {
            id: Set(user.id.to_string()),
            discriminator: Set(user.discriminator.into()),
            name: Set(user.name.into()),
            email: Set(user.email.as_str().to_owned()),
            web_page: Set(user.web_page.map(|url| url.to_string())),
            created_at: Set(user.created_at.into()),
            updated_at: Set(user.updated_at.into()),
        }
        .insert(&self.db)
        .await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                let constrint = get_db_constrint_err(&err);
                match constrint {
                    Some(constrint) => match constrint.as_str() {
                        "user_discriminator_key" => Err(
                            UserRepositoryAddError::DuplicateDiscriminator(Box::new(err)),
                        ),
                        "user_email_key" => {
                            Err(UserRepositoryAddError::DuplicateEmail(Box::new(err)))
                        }
                        _ => Err(UserRepositoryAddError::OtherError(Box::new(err))),
                    },
                    None => Err(UserRepositoryAddError::OtherError(Box::new(err))),
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

fn get_db_constrint_err(err: &DbErr) -> Option<String> {
    match err {
        sea_orm::error::DbErr::Query(runtime_err) => match runtime_err {
            sea_orm::error::RuntimeErr::SqlxError(sqlx_err) => match sqlx_err {
                sqlx::error::Error::Database(db_err) => db_err.constraint().map(|s| s.to_owned()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}
