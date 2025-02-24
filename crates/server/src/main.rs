use common::config::get_config;
use server::run;
mod user;
mod user_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_values = get_config()?;
    let address = format!("{}:{}", config_values.host, config_values.port);
    run(address).await
}
