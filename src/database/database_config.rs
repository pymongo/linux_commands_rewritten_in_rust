//! database_config(eg. username, password) use in test

#[derive(serde::Deserialize)]
pub struct Config {
    pub mysql: MysqlConfig,
}

#[derive(serde::Deserialize)]
pub struct MysqlConfig {
    pub username: String,
    pub password: String,
    pub db_name: String,
}

impl Default for Config {
    fn default() -> Self {
        let config_filename = format!("{}/database_config.toml", env!("CARGO_MANIFEST_DIR"));
        let toml_str = std::fs::read_to_string(config_filename).unwrap();
        toml::de::from_str(&toml_str).unwrap()
    }
}
