#![allow(dead_code)]
#![allow(unused_variables)]
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    Ok(envy::from_env::<Config>()?)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_config_parsing() {
        // Arrange
        env::set_var("HOST", "127.0.0.1");
        env::set_var("PORT", "8080");

        // Act
        let config = get_config().unwrap();

        // Assert
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
    }
}
