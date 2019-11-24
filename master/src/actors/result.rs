use std::result;
use actix_web::HttpResponse;
use actix::MailboxError;

pub enum Error {
    Unauthorized,
    InternalError
}

impl Error {
    pub fn error_response(&self) -> HttpResponse {
        match self {
            Error::Unauthorized => HttpResponse::Unauthorized().json(""),
            Error::InternalError => HttpResponse::InternalServerError().json("")
        }
    }
}

impl From<actix::MailboxError> for Error {
    fn from(_: MailboxError) -> Self {
        Error::InternalError
    }
}

pub type Result<T, E = Error> = result::Result<T, E>;