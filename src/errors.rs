use failure::Fail;
use serde_json::Value as JsonValue;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    RequestError(reqwest::Error),

    #[fail(display = "Login Failed: {}", _0)]
    LoginFailed(JsonValue),

    #[fail(display = "error: {}", _0)]
    ResponseError(JsonValue),

    #[fail(display = "Internal Server Error")]
    InternalServerError,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::RequestError(error)
    }
}
