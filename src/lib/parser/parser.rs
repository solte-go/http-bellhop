use serde::{Deserialize, Serialize};
use std::fs::{DirEntry, File};
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub host: String,
    pub method: String,
    #[serde(rename = "content-type")]
    pub content_type: String,
    pub body: Vec<String>,
}

pub fn get_cfg_data(path: &DirEntry) -> Request {
    let mut file = File::open(path.path()).expect("Error:");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error:");
    let request: Request = serde_json::from_str(&contents).expect("Error:");
    request
}
