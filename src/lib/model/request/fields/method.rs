use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Method(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Method {
    GET,
    POST,
    UNDEFINED,
}

impl Default for Method {
    fn default() -> Self {
        Self::UNDEFINED
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
