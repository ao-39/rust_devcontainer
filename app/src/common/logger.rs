use tracing_subscriber::{fmt::time::LocalTime, EnvFilter};
pub fn init() {
    tracing_subscriber::fmt()
        .with_timer(LocalTime::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();
}
