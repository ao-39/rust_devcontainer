mod common;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    common::logger::init();
    tracing::info!("Hello, world!");
    Ok(())
}
