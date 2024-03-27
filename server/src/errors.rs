use core::fmt;

use ntex::{web::{WebResponseError, HttpRequest, HttpResponse}, http::StatusCode};

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum CustomError {
    NotFound(String),
    InternalError(String),
}

impl WebResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        HttpResponse::new(self.status_code()).set_body(
            match self {
                Self::NotFound(e) => e,
                Self::InternalError(e) => e,
            }.into()
        )
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(e) => write!(f,"{e}"),
            Self::InternalError(e) => write!(f,"{e}"),
        }
    }
}