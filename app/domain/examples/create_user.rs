use std::str::FromStr;
use tracing::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = domain::entity::User::new(
        rusty_ulid::Ulid::generate(),
        domain::object::UserDiscriminator::new("john".to_string())?,
        domain::object::UserName::new("John Doe".to_string()),
        email_address::EmailAddress::from_str("example@example.com")?,
        url::Url::parse("https://example.com")?,
        chrono::Local::now(),
        chrono::Local::now(),
    );
    info!("{:?}", user);
    Ok(())
}
