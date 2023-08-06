use tracing_subscriber::{fmt::time::LocalTime, EnvFilter};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt()
        .with_timer(LocalTime::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();
    tracing::info!("Hello, world!");
    Ok(())
}
