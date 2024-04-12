use jsonwebtoken::errors::Error as JwtError;
use std::convert::From;
use std::fmt;

use actix_web::{error::ResponseError, http::header::ContentType, http::StatusCode, HttpResponse};
use async_sqlite::Error as SqliteError;

#[derive(Debug)]
pub enum Error {
    BadAuthToken,
    BadAuthorizationHeader,
    DatabaseConnect,
    DatabaseQuery,
    EnvNotSet,
    GoogleUserInfo,
    MissingAuthorizationHeader,
    TreeNotFound,
    UniqueId,
}

impl Error {
    fn payload(&self) -> &str {
        match self {
            Error::BadAuthToken => {
                r#"{"error":{"code":"BadAuthToken","description":"Bad authentication token."}}"#
            }
            Error::BadAuthorizationHeader => {
                r#"{"error":{"code":"BadAuthorizationHeader","description":"Bad authorization header."}}"#
            }
            Error::DatabaseConnect => {
                r#"{"error":{"code":"DatabaseConnect","description":"Error connecting to the database."}}"#
            }
            Error::DatabaseQuery => {
                r#"{"error":{"code":"DatabaseQuery","description":"There was a database error while processing your request."}}"#
            }
            Error::EnvNotSet => {
                r#"{"error":{"code":"EnvNotSet","description":"Environment variable not set."}}"#
            }
            Error::GoogleUserInfo => {
                r#"{"error":{"code":"GoogleUserInfo","description":"Could not get user info from Google."}}"#
            }
            Error::MissingAuthorizationHeader => {
                r#"{"error":{"code":"MissingAuthorizationHeader","description":"Authentication required for this call."}}"#
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

impl From<JwtError> for Error {
    fn from(_: JwtError) -> Self {
        Error::BadAuthToken
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
            Error::BadAuthToken => StatusCode::BAD_REQUEST,
            Error::BadAuthorizationHeader => StatusCode::BAD_REQUEST,
            Error::DatabaseConnect => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseQuery => StatusCode::INTERNAL_SERVER_ERROR,
            Error::EnvNotSet => StatusCode::INTERNAL_SERVER_ERROR,
            Error::GoogleUserInfo => StatusCode::UNAUTHORIZED,
            Error::MissingAuthorizationHeader => StatusCode::UNAUTHORIZED,
            Error::TreeNotFound => StatusCode::NOT_FOUND,
            Error::UniqueId => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BadAuthToken => write!(f, "BadAuthToken"),
            Error::BadAuthorizationHeader => write!(f, "BadAuthorizationHeader"),
            Error::DatabaseConnect => write!(f, "DatabaseConnect"),
            Error::DatabaseQuery => write!(f, "DatabaseQuery"),
            Error::EnvNotSet => write!(f, "EnvNotSet"),
            Error::GoogleUserInfo => write!(f, "GoogleUserInfo"),
            Error::MissingAuthorizationHeader => write!(f, "MissingAuthorizationHeader"),
            Error::TreeNotFound => write!(f, "TreeNotFound"),
            Error::UniqueId => write!(f, "UniqueId"),
        }
    }
}
