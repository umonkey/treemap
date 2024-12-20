use jsonwebtoken::errors::Error as JwtError;
use std::convert::From;
use std::fmt;

use actix_web::{error::ResponseError, http::header::ContentType, http::StatusCode, HttpResponse};
use async_sqlite::Error as SqliteError;

#[derive(Debug)]
pub enum Error {
    BadAuthToken,
    BadAuthorizationHeader,
    BadCallback,
    BadImage,
    DatabaseConnect,
    DatabaseQuery,
    DependencyLoad,
    EnvNotSet,
    FileDownload,
    FileNotFound,
    FileUpload,
    GoogleUserInfo,
    ImageResize,
    MissingAuthorizationHeader,
    OsmExchange,
    Queue,
    TreeNotFound,
    UniqueId,
    UserNotFound,
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
            Error::BadCallback => {
                r#"{"error":{"code":"BadCallback","description":"Bad callback URL, unable to authenticate."}}"#
            }
            Error::BadImage => {
                r#"{"error":{"code":"BadImage","description":"Bad image file, cannot work with it."}}"#
            }
            Error::DatabaseConnect => {
                r#"{"error":{"code":"DatabaseConnect","description":"Error connecting to the database."}}"#
            }
            Error::DatabaseQuery => {
                r#"{"error":{"code":"DatabaseQuery","description":"There was a database error while processing your request."}}"#
            }
            Error::DependencyLoad => {
                r#"{"error":{"code":"DependencyLoad","description":"Error loading a dependency."}}"#
            }
            Error::EnvNotSet => {
                r#"{"error":{"code":"EnvNotSet","description":"Environment variable not set."}}"#
            }
            Error::FileDownload => {
                r#"{"error":{"code":"FileDownload","description":"Error downloading the file."}}"#
            }
            Error::FileNotFound => {
                r#"{"error":{"code":"FileNotFound","description":"File not found file."}}"#
            }
            Error::FileUpload => {
                r#"{"error":{"code":"FileUpload","description":"Error preparing file upload."}}"#
            }
            Error::GoogleUserInfo => {
                r#"{"error":{"code":"GoogleUserInfo","description":"Could not get user info from Google."}}"#
            }
            Error::ImageResize => {
                r#"{"error":{"code":"ImageResize","description":"Image reading or resizing failed."}}"#
            }
            Error::MissingAuthorizationHeader => {
                r#"{"error":{"code":"MissingAuthorizationHeader","description":"Authentication required for this call."}}"#
            }
            Error::OsmExchange => {
                r#"{"error":{"code":"OsmExchange","description":"OSM exchange failed."}}"#
            }
            Error::Queue => {
                r#"{"error":{"code":"Queue","description":"Error processing queue request."}}"#
            }
            Error::TreeNotFound => {
                r#"{"error":{"code":"TreeNotFound","description":"The specified tree does not exist in the database."}}"#
            }
            Error::UniqueId => {
                r#"{"error":{"code":"UniqueId","description":"Could not generate a unique id to assign to an object."}}"#
            }
            Error::UserNotFound => {
                r#"{"error":{"code":"UserNotFound","description":"User record not found, or deleted."}}"#
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
            Error::BadAuthToken => StatusCode::UNAUTHORIZED,
            Error::BadAuthorizationHeader => StatusCode::BAD_REQUEST,
            Error::BadCallback => StatusCode::BAD_REQUEST,
            Error::BadImage => StatusCode::BAD_REQUEST,
            Error::DatabaseConnect => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseQuery => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DependencyLoad => StatusCode::INTERNAL_SERVER_ERROR,
            Error::EnvNotSet => StatusCode::INTERNAL_SERVER_ERROR,
            Error::FileDownload => StatusCode::NOT_FOUND,
            Error::FileNotFound => StatusCode::NOT_FOUND,
            Error::FileUpload => StatusCode::INTERNAL_SERVER_ERROR,
            Error::GoogleUserInfo => StatusCode::UNAUTHORIZED,
            Error::ImageResize => StatusCode::INTERNAL_SERVER_ERROR,
            Error::MissingAuthorizationHeader => StatusCode::UNAUTHORIZED,
            Error::OsmExchange => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Queue => StatusCode::INTERNAL_SERVER_ERROR,
            Error::TreeNotFound => StatusCode::NOT_FOUND,
            Error::UniqueId => StatusCode::INTERNAL_SERVER_ERROR,
            Error::UserNotFound => StatusCode::UNAUTHORIZED,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BadAuthToken => write!(f, "BadAuthToken"),
            Error::BadAuthorizationHeader => write!(f, "BadAuthorizationHeader"),
            Error::BadCallback => write!(f, "BadCallback"),
            Error::BadImage => write!(f, "BadImage"),
            Error::DatabaseConnect => write!(f, "DatabaseConnect"),
            Error::DatabaseQuery => write!(f, "DatabaseQuery"),
            Error::DependencyLoad => write!(f, "DependencyLoad"),
            Error::EnvNotSet => write!(f, "EnvNotSet"),
            Error::FileDownload => write!(f, "FileDownload"),
            Error::FileNotFound => write!(f, "FileNotFound"),
            Error::FileUpload => write!(f, "FileUpload"),
            Error::GoogleUserInfo => write!(f, "GoogleUserInfo"),
            Error::ImageResize => write!(f, "ImageResize"),
            Error::MissingAuthorizationHeader => write!(f, "MissingAuthorizationHeader"),
            Error::OsmExchange => write!(f, "OsmExchange"),
            Error::Queue => write!(f, "Queue"),
            Error::TreeNotFound => write!(f, "TreeNotFound"),
            Error::UniqueId => write!(f, "UniqueId"),
            Error::UserNotFound => write!(f, "UserNotFound"),
        }
    }
}
