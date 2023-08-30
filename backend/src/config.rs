use dotenv;
use std::env::VarError;
use std::{env, io};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Environment variable error")]
    EnvVarError(#[from] VarError),
    #[error("Empty variable error `{0}`")]
    VarEmpty(String),
    #[error("Serde json error")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("IO error")]
    IOError(#[from] io::Error),
    #[error("Invalid bool `{0}`")]
    InvalidBool(String),
}

pub type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub log_db_statements: bool,
}

impl Config {
    pub fn new() -> ConfigResult<Config> {
        dotenv::dotenv().ok();

        Ok(Config {
            database_url: load_env_str(String::from("DATABASE_URL"))?,
            log_db_statements: load_env_bool(String::from("LOG_DB_STATEMENTS"))?,
        })
    }
}

fn load_env_str(key: String) -> ConfigResult<String> {
    let var = env::var(&key)?;

    if var.is_empty() {
        return Err(ConfigError::VarEmpty(key));
    }

    Ok(var)
}

fn load_env_bool(key: String) -> ConfigResult<bool> {
    let var = load_env_str(key)?;
    match var.as_str() {
        "false" => Ok(false),
        "true" => Ok(true),
        _ => Err(ConfigError::InvalidBool(var)),
    }
}
