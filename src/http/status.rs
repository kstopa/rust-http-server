use std::fmt::{Display, Formatter, Result as FmtResult};


/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status
/// TODO suport other status codes that the main ones.
#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad request",
            Self::NotFound => "Not found",
            Self::InternalServerError => "Internal server error",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
