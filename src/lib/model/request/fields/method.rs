use http::method::Method;
use serde::{Deserialize, Serialize};
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Method(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CfgMethod {
    GET,
    POST,
    PUT,
    CONNECT,
    DELETE,
    TRACE,
}

impl Default for CfgMethod {
    fn default() -> Self {
        Self::GET
    }
}

impl From<CfgMethod> for http::method::Method {
    fn from(value: CfgMethod) -> http::method::Method {
        match value {
            CfgMethod::GET => http::Method::GET,
            CfgMethod::POST => http::Method::POST,
            CfgMethod::PUT => http::Method::PUT,
            CfgMethod::CONNECT => http::Method::CONNECT,
            CfgMethod::DELETE => http::Method::DELETE,
            CfgMethod::TRACE => http::Method::TRACE,
        }
    }
}

impl From<String> for CfgMethod {
    fn from(value: String) -> Self {
        if value.trim().to_lowercase() == "get" {
            return CfgMethod::GET;
        }
        if value.trim().to_lowercase() == "post" {
            return CfgMethod::POST;
        }
        CfgMethod::GET
    }
}
