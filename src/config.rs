use toml;
use std::io;
use std::io::prelude::*;
use std::fs::File;

pub enum GitService {
    GitHub,
    GitLab,
}

#[derive(Deserialize)]
pub struct Config {
    pub github: Option<String>,
    pub gitlab: Option<String>
}

#[derive(Debug, Fail)]
pub enum ConfigError {
    #[fail(display = "failed to read API config: {}", why)]
    IO { why: io::Error},
    #[fail(display = "failed to parse config: {}", why)]
    Parse { why: toml::de::Error },

}

impl From<io::Error> for ConfigError {
    fn from(why: io::Error) -> Self {
        ConfigError::IO { why }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(why: toml::de::Error) -> Self {
        ConfigError::Parse { why }
    }
}

impl Config {
    pub fn new(path: &str) -> Result<Config, ConfigError> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(toml::from_slice::<Config>(&buffer)?)
    }
}
