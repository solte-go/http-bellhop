use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Headers(pub Vec<Header>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Header {
    Host(String),
    UserAgent(String),
    Accept(String),
    AcceptLanguage(String),
    AcceptEncoding(String),
    Connection(String),
    UpgradeInsecureRequests(String),
    CacheControl(String),
    Pragma(String),
    DNT(String),
    Referer(String),
    Origin(String),
    TE(String),
    #[serde(rename = "Content-Type")]
    ContentType(String),
    ContentLength(String),
    Cookie(String),
    Undefined(String),
}

impl Headers {
    pub fn into_inner(self) -> Vec<(String, String)> {
        let mut headers = Vec::new();
        for header in self.0 {
            headers.push(header.into_inner());
        }
        headers
    }
}

//TODO dig dipper into IntoIterator
impl IntoIterator for Headers {
    type Item = Header;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Default for Headers {
    fn default() -> Self {
        let headers = vec![];
        Self(headers)
    }
}

impl Header {
    pub fn into_inner(self) -> (String, String) {
        match self {
            // Header::Host(value) => value,
            // Header::UserAgent(value) => value,
            // Header::Accept(value) => value,
            // Header::AcceptLanguage(value) => value,
            // Header::AcceptEncoding(value) => value,
            // Header::Connection(value) => value,
            // Header::UpgradeInsecureRequests(value) => value,
            // Header::CacheControl(value) => value,
            // Header::Pragma(value) => value,
            // Header::DNT(value) => value,
            // Header::Referer(value) => value,
            // Header::Origin(value) => value,
            // Header::TE(value) => value,
            Header::ContentType(value) => ("Content-Type".to_string(), value),
            // Header::ContentLength(value) => value,
            // Header::Cookie(value) => value,
            // Header::Undefined(value) => value,
            _ => ("Undefined".to_string(), "Undefined".to_string()),
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Header::Undefined("Undefined".to_string())
    }
}
