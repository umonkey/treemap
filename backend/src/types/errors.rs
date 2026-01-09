use jsonwebtoken::errors::Error as JwtError;
use libsql::Error as LibSqlError;
use std::convert::From;
use std::fmt;

use actix_web::{error::ResponseError, http::header::ContentType, http::StatusCode, HttpResponse};

#[derive(Debug)]
pub enum Error {
    AddressNotFound,
    AccessDenied,
    BadAuthToken,
    BadAuthorizationHeader,
    BadCallback,
    BadImage,
    #[allow(unused)]
    BadRequest,
    Config(String),
    DatabaseConnect,
    DatabaseQuery,
    DatabaseStructure,
    DependencyLoad,
    DuplicateTree,
    EnvNotSet,
    FileDownload,
    FileNotFound,
    FileUpload,
    GoogleUserInfo,
    ImageResize,
    MissingAuthorizationHeader,
    OsmExchange,
    Queue,
    RemoteAddrNotSet,
    TreeNotFound,
    UniqueId,
    UserAgentNotSet,
    UserNotFound,
}

impl Error {
    fn payload(&self) -> &str {
        match self {
            Error::AddressNotFound => {
                r#"{"error":{"code":"AddressNotFound","description":"Could not find an address for the given coordinates."}}"#
            }
            Error::AccessDenied => {
                r#"{"error":{"code":"AccessDenied","description":"Access denied."}}"#
            }
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
            Error::BadRequest => r#"{"error":{"code":"BadRequest","description":"Bad request."}}"#,
            Error::Config(_) => {
                r#"{"error":{"code":"Config","description":"Configuration error."}}"#
            }
            Error::DatabaseConnect => {
                r#"{"error":{"code":"DatabaseConnect","description":"Error connecting to the database."}}"#
            }
            Error::DatabaseQuery => {
                r#"{"error":{"code":"DatabaseQuery","description":"There was a database error while processing your request."}}"#
            }
            Error::DatabaseStructure => {
                r#"{"error":{"code":"DatabaseStructure","description":"Database structure error."}}"#
            }
            Error::DependencyLoad => {
                r#"{"error":{"code":"DependencyLoad","description":"Error loading a dependency."}}"#
            }
            Error::DuplicateTree => {
                r#"{"error":{"code":"DuplicateTree","description":"A tree with these coordinates already exists."}}"#
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
            Error::RemoteAddrNotSet => {
                r#"{"error":{"code":"RemoteAddrNotSet","description":"Remote address not set."}}"#
            }
            Error::TreeNotFound => {
                r#"{"error":{"code":"TreeNotFound","description":"The specified tree does not exist in the database."}}"#
            }
            Error::UniqueId => {
                r#"{"error":{"code":"UniqueId","description":"Could not generate a unique id to assign to an object."}}"#
            }
            Error::UserAgentNotSet => {
                r#"{"error":{"code":"UserAgentNotSet","description":"User agent not set."}}"#
            }
            Error::UserNotFound => {
                r#"{"error":{"code":"UserNotFound","description":"User record not found, or deleted."}}"#
            }
        }
    }
}

impl From<LibSqlError> for Error {
    fn from(_: LibSqlError) -> Self {
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
            Error::AccessDenied => StatusCode::FORBIDDEN,
            Error::AddressNotFound => StatusCode::NOT_FOUND,
            Error::BadAuthToken => StatusCode::UNAUTHORIZED,
            Error::BadAuthorizationHeader => StatusCode::BAD_REQUEST,
            Error::BadCallback => StatusCode::BAD_REQUEST,
            Error::BadImage => StatusCode::BAD_REQUEST,
            Error::BadRequest => StatusCode::BAD_REQUEST,
            Error::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseConnect => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseQuery => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseStructure => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DependencyLoad => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DuplicateTree => StatusCode::CONFLICT,
            Error::EnvNotSet => StatusCode::INTERNAL_SERVER_ERROR,
            Error::FileDownload => StatusCode::NOT_FOUND,
            Error::FileNotFound => StatusCode::NOT_FOUND,
            Error::FileUpload => StatusCode::INTERNAL_SERVER_ERROR,
            Error::GoogleUserInfo => StatusCode::UNAUTHORIZED,
            Error::ImageResize => StatusCode::INTERNAL_SERVER_ERROR,
            Error::MissingAuthorizationHeader => StatusCode::UNAUTHORIZED,
            Error::OsmExchange => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Queue => StatusCode::INTERNAL_SERVER_ERROR,
            Error::RemoteAddrNotSet => StatusCode::INTERNAL_SERVER_ERROR,
            Error::TreeNotFound => StatusCode::NOT_FOUND,
            Error::UniqueId => StatusCode::INTERNAL_SERVER_ERROR,
            Error::UserAgentNotSet => StatusCode::UNAUTHORIZED,
            Error::UserNotFound => StatusCode::UNAUTHORIZED,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::AccessDenied => write!(f, "AccessDenied"),
            Error::AddressNotFound => write!(f, "AddressNotFound"),
            Error::BadAuthToken => write!(f, "BadAuthToken"),
            Error::BadAuthorizationHeader => write!(f, "BadAuthorizationHeader"),
            Error::BadCallback => write!(f, "BadCallback"),
            Error::BadImage => write!(f, "BadImage"),
            Error::BadRequest => write!(f, "BadRequest"),
            Error::Config(s) => write!(f, "Config error: {s}"),
            Error::DatabaseConnect => write!(f, "DatabaseConnect"),
            Error::DatabaseQuery => write!(f, "DatabaseQuery"),
            Error::DatabaseStructure => write!(f, "DatabaseStructure"),
            Error::DependencyLoad => write!(f, "DependencyLoad"),
            Error::DuplicateTree => write!(f, "DuplicateTree"),
            Error::EnvNotSet => write!(f, "EnvNotSet"),
            Error::FileDownload => write!(f, "FileDownload"),
            Error::FileNotFound => write!(f, "FileNotFound"),
            Error::FileUpload => write!(f, "FileUpload"),
            Error::GoogleUserInfo => write!(f, "GoogleUserInfo"),
            Error::ImageResize => write!(f, "ImageResize"),
            Error::MissingAuthorizationHeader => write!(f, "MissingAuthorizationHeader"),
            Error::OsmExchange => write!(f, "OsmExchange"),
            Error::Queue => write!(f, "Queue"),
            Error::RemoteAddrNotSet => write!(f, "RemoteAddrNotSet"),
            Error::TreeNotFound => write!(f, "TreeNotFound"),
            Error::UniqueId => write!(f, "UniqueId"),
            Error::UserAgentNotSet => write!(f, "UserAgentNotSet"),
            Error::UserNotFound => write!(f, "UserNotFound"),
        }
    }
}
