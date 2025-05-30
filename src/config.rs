use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .set_default("host", "0.0.0.0")?
            .set_default("port", 8080)?
            .set_default("log_level", "info")?
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        cfg.try_deserialize()
    }
}