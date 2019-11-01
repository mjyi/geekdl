use failure::Fail;
use serde_json::Value as JsonValue;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    RequestError(reqwest::Error),

    #[fail(display = "{}", _0)]
    SerdeError(serde_json::Error),

    #[fail(display = "{}", _0)]
    Io(std::io::Error),

    #[fail(display = "Login Failed: {}", _0)]
    LoginFailed(JsonValue),

    #[fail(display = "error: {}", _0)]
    ResponseError(JsonValue),

    #[fail(display = "error: {}", 0)]
    PlainError(&'static str),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::RequestError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Self {
        Error::PlainError(e)
    }
}
