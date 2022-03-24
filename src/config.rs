use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr:String,
}
#[derive(Deserialize)]
pub struct Config {
    pub web:WebConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env()->Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
