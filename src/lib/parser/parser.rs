use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    GET,
    POST,
    UNDEFINED,
}

// #[derive(Serialize, Deserialize, Debug, Default)] //TODO Git it up
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub host: String,
    #[serde()]
    pub method: Method,
    #[serde(rename = "content-type")]
    pub content_type: String,
    pub body: Vec<String>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            body: vec![],
            method: Method::GET,
            host: String::new(),
            content_type: String::new(),
        }
    }
}

impl From<String> for Method {
    fn from(value: String) -> Self {
        if value.trim().to_lowercase() == "get" {
            return Method::GET;
        }
        if value.trim().to_lowercase() == "post" {
            return Method::POST;
        }
        Method::UNDEFINED
    }
}

pub fn get_cfg_data(path: &Path) -> Request {
    let mut file = File::open(path).expect("Error:");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error:");
    let request: Request = serde_json::from_str(&contents).expect("Error:");
    request
}
