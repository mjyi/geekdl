use failure::Error;

pub enum GeekErrorKind {
    LoginError,
    RequestError,
    ParseError,
}
