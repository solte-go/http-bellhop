pub mod fields;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// #[derive(Serialize, Deserialize, Debug, Default)] //TODO Git it up
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub env: fields::environment::Configs,
    #[serde()]
    pub method: fields::method::Method,
    #[serde(rename = "headers")]
    pub headers: fields::headers::Headers,
    pub body: fields::Body,
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

pub fn deserialize_data(path: &Path, env: &str) -> crate::domain::Request {
    let mut file = File::open(path).expect("Error:");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error:");
    let request: Request = serde_json::from_str(&contents).expect("Error:");
    // println!("{:#?}", request);
    request.from(env)

    //TODO
}

impl Request {
    pub fn from(self, env: &str) -> crate::domain::Request {
        use crate::domain::Request;
        Request {
            host: self.env.into_inner(env),
            method: self.method,
            headers: self.headers.into_inner(),
            body: self.body.into(),
        }
    }
}
