use std::error::Error;

use client::run;
use common::config::get_config;
use log::debug;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    match get_config() {
        Ok(config_values) => {
            let address = format!("{}:{}", config_values.host, config_values.port);

            match run(address).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    debug!("Error running the client: {}", e);
                    Err(e)
                }
            }
        }
        Err(_e) => {
            eprintln!("Error: Your environment variables are not set for HOST and PORT. Exiting.");
            Ok(())
        }
    }
}
