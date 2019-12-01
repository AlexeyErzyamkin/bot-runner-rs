use std::{
    result,
    fmt::{Display, Formatter, Error as FmtError}
};

use actix_web::{
    HttpResponse,
    ResponseError
};
use actix::MailboxError;

#[derive(Debug)]
pub enum Error {
    Unauthorized,
    InternalError
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Unauthorized => HttpResponse::Unauthorized().json(""),
            Error::InternalError => HttpResponse::InternalServerError().json("")
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> result::Result<(), FmtError> {
        match self {
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::InternalError => write!(f, "InternalError")
        }
    }
}

impl From<actix::MailboxError> for Error {
    fn from(_: MailboxError) -> Self {
        Error::InternalError
    }
}

pub type Result<T, E = Error> = result::Result<T, E>;