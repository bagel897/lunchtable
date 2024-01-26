#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub redis: deadpool_redis::Config,
}
impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}
