use async_trait::async_trait;
use std::str::FromStr;

use app_service::user::user_application_service::IUserApplicationService;
use app_service::user::UserApplicationService;
use domain::{
    entity::User,
    object::{email_address, url, UserDiscriminator, UserName},
    repository::IUserRepository,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_application_service = UserApplicationService::new(ExampleUserRepository);

    user_application_service
        .register(
            UserDiscriminator::new("john".to_string())?,
            UserName::new("John Doe".to_string())?,
            email_address::EmailAddress::from_str("example@example.com")?,
            Some(url::Url::parse("https://example.com")?),
        )
        .await?;

    Ok(())
}

struct ExampleUserRepository;

#[async_trait]
impl IUserRepository for ExampleUserRepository {
    async fn add(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        println!("add user: {:?}", user);
        Ok(())
    }

    fn find_by_id(
        &self,
        _user_id: domain::object::rusty_ulid::Ulid,
    ) -> Result<domain::entity::User, Box<dyn std::error::Error>> {
        todo!()
    }

    fn find_by_discriminator(
        &self,
        _discriminator: domain::object::UserDiscriminator,
    ) -> Result<domain::entity::User, Box<dyn std::error::Error>> {
        todo!()
    }
}
