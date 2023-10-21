use std::{io::Read, path::Path};

const CONFIG_DIR: &str = "/etc/marine_greetd/config.toml";

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    pub envs: Vec<ConfigDesktop>,
}

#[derive(Debug, Deserialize)]
struct ConfigDesktop {
    #[serde(rename = "useIn")]
    use_in: String,
    values: Vec<Env>,
}

#[derive(Debug, Deserialize)]
struct Env {
    key: String,
    value: String,
}

impl ToString for Env {
    fn to_string(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

impl ConfigDesktop {
    fn get_envs(&self) -> Vec<String> {
        self.values.iter().map(|v| v.to_string()).collect()
    }
}

pub fn read_config_from_user(wm_name: &str) -> Vec<String> {
    if !Path::new(&CONFIG_DIR).exists() {
        return Vec::new();
    }
    let Ok(mut file) = std::fs::OpenOptions::new().read(true).open(&CONFIG_DIR) else {
        return Vec::new();
    };
    let mut buf = String::new();
    if file.read_to_string(&mut buf).is_err() {
        return Vec::new();
    };
    let Ok(config) = toml::from_str::<Config>(&buf) else {
        return Vec::new();
    };
    let Some(env_config) = config.envs.iter().find(|it| it.use_in == wm_name) else {
        return Vec::new();
    };
    env_config.get_envs()
}
