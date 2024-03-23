use std::convert::From;
use std::fmt;

use async_sqlite::Error as SqliteError;
use actix_web::{error::ResponseError, http::header::ContentType, http::StatusCode, HttpResponse};

#[derive(Debug)]
pub enum Error {
    DatabaseConnect,
    DatabaseQuery,
    EnvNotSet,
    TreeNotFound,
    UniqueId,
}

impl Error {
    fn payload(&self) -> &str {
        match self {
            Error::DatabaseConnect => {
                r#"{"error":{"code":"DatabaseConnect","description":"Error connecting to the database."}}"#
            }
            Error::DatabaseQuery => {
                r#"{"error":{"code":"DatabaseQuery","description":"There was a database error while processing your request."}}"#
            }
            Error::EnvNotSet => {
                r#"{"error":{"code":"EnvNotSet","description":"Environment variable not set."}}"#
            }
            Error::TreeNotFound => {
                r#"{"error":{"code":"TreeNotFound","description":"The specified tree does not exist in the database."}}"#
            }
            Error::UniqueId => {
                r#"{"error":{"code":"UniqueId","description":"Could not generate a unique id to assign to an object."}}"#
            }
        }
    }
}

impl From<SqliteError> for Error {
    fn from(_: SqliteError) -> Self {
        Error::DatabaseQuery
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
            Error::DatabaseConnect => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseQuery => StatusCode::INTERNAL_SERVER_ERROR,
            Error::EnvNotSet => StatusCode::INTERNAL_SERVER_ERROR,
            Error::TreeNotFound => StatusCode::NOT_FOUND,
            Error::UniqueId => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DatabaseConnect => write!(f, "DatabaseConnect"),
            Error::DatabaseQuery => write!(f, "DatabaseQuery"),
            Error::EnvNotSet => write!(f, "EnvNotSet"),
            Error::TreeNotFound => write!(f, "TreeNotFound"),
            Error::UniqueId => write!(f, "UniqueId"),
        }
    }
}
