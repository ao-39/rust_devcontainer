fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    println!("Hello, world!");
    Ok(())
}
