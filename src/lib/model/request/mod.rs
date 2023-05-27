pub mod fields;

use crate::model::request::fields::environment::EnvironmentError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum FieldError {
    #[error("bad argument error: {0}")]
    BadArg(#[from] EnvironmentError),
}

// #[derive(Serialize, Deserialize, Debug, Default)] //TODO Git it up
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
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

pub fn deserialize_data(path: &Path, env: &str) -> Result<crate::domain::Request, FieldError> {
    let mut file = File::open(path).expect("Error:");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error:");
    let request: Request = serde_json::from_str(&contents).expect("Error:");
    request.from(env)

    //TODO
}

impl Request {
    pub fn from(self, env: &str) -> Result<crate::domain::Request, FieldError> {
        use crate::domain::Request;

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
