use async_trait::async_trait;
use domain::repository::IUserRepository;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub struct UserRepository<'a> {
    db: &'a DatabaseConnection,
}

#[async_trait]
impl IUserRepository for UserRepository<'_> {
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

    async fn add(&self, user: domain::entity::User) -> Result<(), Box<dyn std::error::Error>> {
        entity::user::ActiveModel {
            id: Set(user.id.to_string()),
            discriminator: Set(user.discriminator.into()),
            name: Set(user.name.into()),
            email: Set(user.email.as_str().to_owned()),
            web_page: Set(user.web_page.map(|url| url.to_string())),
            created_at: Set(user.created_at.into()),
            updated_at: Set(user.updated_at.into()),
        }
        .save(self.db)
        .await?;
        Ok(())
    }
}
