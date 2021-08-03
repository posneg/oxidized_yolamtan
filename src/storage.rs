use serde::Deserialize;
use serde::Serialize;
use serde::de;

use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub struct Environment
{
    pub owner: String,
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct Server
{
    pub bot_channels: Option<Vec<u64>>,
    pub pronoun_roles: Option<HashMap<String, u64>>,
}

#[derive(Deserialize, Serialize)]
pub struct Storage
{
    notice: String,
    pub servers: HashMap<String, Server>,
}


pub fn parse_toml<T>(path: &str) -> T 
where
    T: de::DeserializeOwned,
{
    let contents = fs::read(&path)
        .expect("Something went wrong reading a file");
    let parsed_data: T = toml::from_slice(&contents).unwrap();
    return parsed_data;
}

pub fn write_toml<T>(path: &str, data: &T)
where
    T: Serialize,
{
    let toml = toml::to_string_pretty(&data).unwrap();
    fs::write(&path, toml);
}