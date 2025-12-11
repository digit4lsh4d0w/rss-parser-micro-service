use rss_parser::config::load_config;

#[tokio::main]
async fn main() {
    println!("Starting!");
    let config = match load_config("./config.toml").await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };
    println!("config: {:#?}", config);
}
