use std::{io::Read, path::Path};

const CONFIG_DIR: &str = "/etc/marine_greetd/config.toml";

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub envs: Vec<ConfigDesktop>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigDesktop {
    #[serde(rename = "useIn")]
    use_in: String,
    values: Vec<Env>,
    #[serde(rename = "cmdAlias")]
    cmd_alias: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Env {
    key: String,
    value: String,
}

impl std::fmt::Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

impl ConfigDesktop {
    pub fn get_envs(&self) -> Vec<String> {
        self.values.iter().map(|v| v.to_string()).collect()
    }
    pub fn get_real_cmd(&self) -> Option<&str> {
        self.cmd_alias.as_deref()
    }
}

impl Config {
    pub fn new() -> Self {
        if !Path::new(&CONFIG_DIR).exists() {
            return Self::default();
        }
        let Ok(mut file) = std::fs::OpenOptions::new().read(true).open(CONFIG_DIR) else {
            return Self::default();
        };
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_err() {
            return Self::default();
        };
        toml::from_str::<Config>(&buf).unwrap_or_default()
    }

    pub fn get_config_desktop(&self, wm_name: &str) -> Option<&ConfigDesktop> {
        self.envs.iter().find(|it| it.use_in == wm_name)
    }
}
