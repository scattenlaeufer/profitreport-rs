use clap::crate_name;
use kimai;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt, fs,
    path::{Path, PathBuf},
};
use toml;

#[derive(Debug)]
pub enum ProfitReportError {
    Kimai(String),
    IO(String),
    Toml(String),
    Xdg(String),
    Other(String),
}

impl std::error::Error for ProfitReportError {}

impl fmt::Display for ProfitReportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Kimai(e) => write!(f, "Kimai Error: {}", e),
            Self::IO(e) => write!(f, "IO Error: {}", e),
            Self::Toml(e) => write!(f, "TOML Error: {}", e),
            Self::Xdg(e) => write!(f, "XDG Error: {}", e),
            Self::Other(e) => write!(f, "Other Error: {}", e),
        }
    }
}

impl From<kimai::KimaiError> for ProfitReportError {
    fn from(error: kimai::KimaiError) -> Self {
        Self::Kimai(error.to_string())
    }
}

impl From<std::io::Error> for ProfitReportError {
    fn from(error: std::io::Error) -> Self {
        Self::IO(error.to_string())
    }
}

impl From<toml::de::Error> for ProfitReportError {
    fn from(error: toml::de::Error) -> Self {
        Self::Toml(error.to_string())
    }
}

impl From<xdg::BaseDirectoriesError> for ProfitReportError {
    fn from(error: xdg::BaseDirectoriesError) -> Self {
        Self::Xdg(error.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    default_account: String,
    accounts: Vec<HashMap<String, AccountConfig>>,
}

impl Config {
    fn load(path: Option<PathBuf>) -> Result<Self, ProfitReportError> {
        let config_path = if let Some(path) = path {
            path
        } else {
            let xdg_dirs = xdg::BaseDirectories::with_prefix(crate_name!())?;
            Path::new(
                &xdg_dirs
                    .find_config_file("config.toml")
                    .ok_or_else(|| ProfitReportError::Other("No config file found".into()))?,
            )
            .to_path_buf()
        };
        let config_string = fs::read_to_string(&*config_path)?;
        Ok(toml::from_str::<Self>(&config_string)?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AccountConfig {
    kimai: KimaiConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct KimaiConfig {
    host: String,
    auth_method: AuthorizationMethod,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
enum AuthorizationMethod {
    Password { user: String, password: String },
    Pass { user: String, pass_path: String },
}

pub fn print_profit_report(config_path: Option<PathBuf>) -> Result<(), ProfitReportError> {
    let config = Config::load(config_path)?;
    println!("{:#?}", config);

    Ok(())
}
