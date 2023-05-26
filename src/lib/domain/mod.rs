use crate::domain::ResultError::{ConnectionError, UnexpectedError};
use crate::model::request::fields::method::CfgMethod;

use std::error::Error;

#[derive(Debug)]
pub struct Request {
    pub name: String,
    pub host: String,
    pub method: CfgMethod,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub expected_status: String,
}

#[derive(thiserror::Error, Debug)]
pub enum ResultError {
    #[error("request error: {0}")]
    ConnectionError(String),
    #[error("unexpected error: {0}")]
    UnexpectedError(String),
}

fn is_ok(status: &str, expected_status: &str) -> bool {
    status.eq(expected_status)
}

impl Request {
    pub fn do_request(self) -> Result<(), Box<dyn Error>> {
        let client = reqwest::blocking::Client::builder().build()?;
        let mut request = client.request(self.method.into(), self.host);
        if !self.headers.is_empty() {
            for (k, v) in self.headers {
                request = request.header(k, v);
            }
        }
        let r = request.json(self.body.as_str()).send();
        println!("-----");
        match r {
            Ok(res) => {
                if is_ok(res.status().as_str(), self.expected_status.as_str()) {
                    println!(
                        "Name: {} --- Ok\nResponse status code: {}",
                        self.name,
                        res.status().as_str()
                    )
                } else {
                    println!(
                        "Name: {} --- Failed!\nReason: Mismatch status code, expected: {} get: {}",
                        self.name.as_str(),
                        self.expected_status,
                        res.status().as_str()
                    )
                }
            }
            Err(e) => {
                if e.is_connect() {
                    println!(
                        "Name: {} --- Failed!\nReason: {}",
                        self.name.as_str(),
                        ConnectionError(e.to_string())
                    );
                    return Ok(());
                } else {
                    print!(
                        "Name: {} --- Failed!\nReason: {}",
                        self.name.as_str(),
                        UnexpectedError(e.source().unwrap().to_string())
                    );
                };
            }
        }
        Ok(())
    }
}
