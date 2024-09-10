mod config;

use config::{get_config};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_values = get_config()?;
    Ok(())
}
