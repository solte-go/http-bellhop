pub mod fields;

use crate::domain::config::fields::environment::EnvironmentError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum FieldError {
    #[error("bad argument error: {0}")]
    BadArg(#[from] EnvironmentError),
    #[error("can't convert data")]
    Deserialize(String),
}

impl From<Error> for FieldError {
    fn from(value: Error) -> Self {
        FieldError::Deserialize(value.to_string())
    }
}

impl From<serde_json::Error> for FieldError {
    fn from(value: serde_json::Error) -> Self {
        FieldError::Deserialize(value.to_string())
    }
}

// #[derive(Serialize, Deserialize, Debug, Default)] //TODO dig it up
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "config_name")]
    pub name: fields::Name,
    pub environment: fields::environment::Configs,
    #[serde()]
    pub method: fields::method::CfgMethod,
    #[serde(rename = "headers")]
    pub headers: fields::headers::Headers,
    pub body: fields::Body,
    pub expected_status: fields::expected_status::ExpectedStatus,
}

pub fn deserialize_data(path: &Path) -> Result<Config, FieldError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let new_config: Config = serde_json::from_str(&contents)?;
    Ok(new_config)
}

impl Config {
    pub fn to_request(self, env: &str) -> Result<crate::service::Request, FieldError> {
        use crate::service::Request;

        let host = self.environment.into_inner(env)?;
        Ok(Request {
            name: self.name.into_inner(),
            host,
            method: self.method,
            headers: self.headers.into_inner(),
            body: self.body.into(),
            expected_status: self.expected_status.into_inner(),
        })
    }
}

// impl Default for Request {
//     fn default() -> Self {
//         Self {
//             env: fields::environment::Configs::default(),
//             method: fields::Method::default(),
//             headers: fields::Headers::default(),
//             body: fields::Body::default(),
//         }
//     }
// }