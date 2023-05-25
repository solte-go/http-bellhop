use crate::domain::ResultError::ConnectionError;
use crate::model::request::fields::Method;
use std::error::Error;

#[derive(Debug)]
pub struct Request {
    pub host: String,
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

#[derive(thiserror::Error, Debug)]
pub enum ResultError {
    #[error("request error: {0}")]
    ConnectionError(String),
}

impl Request {
    pub fn do_request(self) -> Result<(), Box<dyn Error>> {
        let client = reqwest::blocking::Client::builder().build()?;
        println!("{:#?}", self.host);
        match self.method {
            Method::GET => {
                let mut request = client.get(self.host);
                if !self.headers.is_empty() {
                    for (k, v) in self.headers {
                        request = request.header(k, v);
                    }
                }

                let r = request.json(&self.body).send();
                match r.as_ref() {
                    Err(e) => {
                        if e.is_connect() {
                            println!("{:?}", ConnectionError(e.to_string()))
                        } else {
                            println!("{:?}", e)
                        }
                    }
                    Ok(_) => {
                        println!("{:#?}", r);
                    }
                }
            }

            Method::POST => {
                let mut request = client.post(self.host);
                let r = request.json(&self.body).send()?;
                println!("{:#?}", r)
            }
            _ => println!("wrong argument"),
        }
        Ok(())
    }
}
