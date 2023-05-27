pub mod actions;
pub use actions::Request;

#[derive(thiserror::Error, Debug)]
pub enum ResultError {
    #[error("request error: {0}")]
    ConnectionError(String),
    #[error("unexpected error: {0}")]
    UnexpectedError(String),
}
