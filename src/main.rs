use rss_parser::config::load_config;
use tracing::{Level, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("Starting!");
    let config = match load_config("./config.toml").await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };
    info!("{:#?}", config);

    Ok(())
}
