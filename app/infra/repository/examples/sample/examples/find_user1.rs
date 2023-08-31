use std::str::FromStr;

use domain::{
    object::{chrono::Local, email_address, rusty_ulid, url, UserDiscriminator, UserName},
    repository::IUserRepository,
};
use repository::user::UserRepository;
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::connect("postgres://postgres:postgres@db:5432/db").await?;
    println!("db connected!");

    let user_repository = UserRepository::new(db.clone());
    // let user = domain::entity::User::new(
    //     rusty_ulid::Ulid::generate(),
    //     UserDiscriminator::new("miku1".to_string())?,
    //     UserName::new("初音ミク".to_string())?,
    //     email_address::EmailAddress::from_str("example@example.com")?,
    //     Some(url::Url::parse("https://example.com")?),
    //     Local::now(),
    //     Local::now(),
    // );

    let res = user_repository
        .find_by_discriminator("miku".to_string().try_into()?)
        .await?;

    println!("res: {:?}", res);
    Ok(())
}
