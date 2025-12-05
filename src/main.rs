use rss_parser::config::load_config;

#[tokio::main]
async fn main() {
    println!("Starting!");
    let config = load_config("./config.toml").await.expect("File error");
    println!("config: {:#?}", config);
}
