use app_service::implementations::user::UserApplicationService;
use sea_orm::Database;
use tracing_subscriber::{fmt::time::LocalTime, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt()
        .with_timer(LocalTime::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();

    let db = Database::connect("postgres://postgres:postgres@localhost:5432/db").await?;

    api::run(UserApplicationService::new(
        repository::user::UserRepository::new(db.clone()),
    ))
    .await?;
    Ok(())
}
