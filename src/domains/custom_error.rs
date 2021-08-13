#[derive(Debug)]
pub enum Error {
    Reqwest(String),
    Other(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Self::Reqwest(err.to_string())
    }
}

impl From<Box<(dyn std::error::Error)>> for Error {
    fn from(err: Box<(dyn std::error::Error)>) -> Error {
        Self::Other(err.to_string())
    }
}
