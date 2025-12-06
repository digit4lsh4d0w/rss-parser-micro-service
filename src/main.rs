use rss_parser::config::load_config;

#[tokio::main]
async fn main() {
    println!("Starting!");
    let config = match load_config("./config.toml").await {
        Ok(c) => c,
        Err(e) => panic!("Error: {e}"),
    };
    println!("config: {:#?}", config);
}
