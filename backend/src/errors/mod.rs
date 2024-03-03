use std::fmt;

use actix_web::{error::ResponseError, http::header::ContentType, http::StatusCode, HttpResponse};

#[derive(Debug)]
pub enum Error {
    TreeNotFound,
}

impl Error {
    fn payload(&self) -> &str {
        match self {
            Error::TreeNotFound => {
                r#"{"error":{"code":"TreeNotFound","description":"The specified tree does not exist in the database."}}"#
            }
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.payload().to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::TreeNotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::TreeNotFound => write!(f, "TreeNotFound"),
        }
    }
}
