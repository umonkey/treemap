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
    DatabaseConnect(String),
    DatabaseQuery(String),
    DatabaseStructure(String),
    DuplicateTree,
    EnvNotSet,
    FileDownload,
    FileNotFound,
    FileUpload,
    GoogleUserInfo,
    ImageResize,
    MissingAuthorizationHeader,
    OsmExchange(String),
    Queue,
    RemoteAddrNotSet,
    TreeNotFound,
    UniqueId,
    UserAgentNotSet,
    UserNotFound,
}

impl Error {
    fn payload(&self) -> String {
        match self {
            Error::AddressNotFound => {
                r#"{"error":{"code":"AddressNotFound","description":"Could not find an address for the given coordinates."}}"#.to_string()
            }
            Error::AccessDenied => {
                r#"{"error":{"code":"AccessDenied","description":"Access denied."}}"#.to_string()
            }
            Error::BadAuthToken => {
                r#"{"error":{"code":"BadAuthToken","description":"Bad authentication token."}}"#.to_string()
            }
            Error::BadAuthorizationHeader => {
                r#"{"error":{"code":"BadAuthorizationHeader","description":"Bad authorization header."}}"#.to_string()
            }
            Error::BadCallback => {
                r#"{"error":{"code":"BadCallback","description":"Bad callback URL, unable to authenticate."}}"#.to_string()
            }
            Error::BadImage => {
                r#"{"error":{"code":"BadImage","description":"Bad image file, cannot work with it."}}"#.to_string()
            }
            Error::BadRequest => r#"{"error":{"code":"BadRequest","description":"Bad request."}}"#.to_string(),
            Error::Config(_) => {
                r#"{"error":{"code":"Config","description":"Configuration error."}}"#.to_string()
            }
            Error::DatabaseConnect(_) => {
                r#"{"error":{"code":"DatabaseConnect","description":"Error connecting to the database."}}"#.to_string()
            }
            Error::DatabaseQuery(_) => {
                r#"{"error":{"code":"DatabaseQuery","description":"There was a database error while processing your request."}}"#.to_string()
            }
            Error::DatabaseStructure(s) => {
                format!(
                    r#"{{"error":{{"code":"DatabaseStructure","description":"Database structure error: {s}"}}}}"#
                )
            }
            Error::DuplicateTree => {
                r#"{"error":{"code":"DuplicateTree","description":"A tree with these coordinates already exists."}}"#.to_string()
            }
            Error::EnvNotSet => {
                r#"{"error":{"code":"EnvNotSet","description":"Environment variable not set."}}"#.to_string()
            }
            Error::FileDownload => {
                r#"{"error":{"code":"FileDownload","description":"Error downloading the file."}}"#.to_string()
            }
            Error::FileNotFound => {
                r#"{"error":{"code":"FileNotFound","description":"File not found file."}}"#.to_string()
            }
            Error::FileUpload => {
                r#"{"error":{"code":"FileUpload","description":"Error preparing file upload."}}"#.to_string()
            }
            Error::GoogleUserInfo => {
                r#"{"error":{"code":"GoogleUserInfo","description":"Could not get user info from Google."}}"#.to_string()
            }
            Error::ImageResize => {
                r#"{"error":{"code":"ImageResize","description":"Image reading or resizing failed."}}"#.to_string()
            }
            Error::MissingAuthorizationHeader => {
                r#"{"error":{"code":"MissingAuthorizationHeader","description":"Authentication required for this call."}}"#.to_string()
            }
            Error::OsmExchange(_) => {
                r#"{"error":{"code":"OsmExchange","description":"OSM exchange failed."}}"#.to_string()
            }
            Error::Queue => {
                r#"{"error":{"code":"Queue","description":"Error processing queue request."}}"#.to_string()
            }
            Error::RemoteAddrNotSet => {
                r#"{"error":{"code":"RemoteAddrNotSet","description":"Remote address not set."}}"#.to_string()
            }
            Error::TreeNotFound => {
                r#"{"error":{"code":"TreeNotFound","description":"The specified tree does not exist in the database."}}"#.to_string()
            }
            Error::UniqueId => {
                r#"{"error":{"code":"UniqueId","description":"Could not generate a unique id to assign to an object."}}"#.to_string()
            }
            Error::UserAgentNotSet => {
                r#"{"error":{"code":"UserAgentNotSet","description":"User agent not set."}}"#.to_string()
            }
            Error::UserNotFound => {
                r#"{"error":{"code":"UserNotFound","description":"User record not found, or deleted."}}"#.to_string()
            }
        }
    }
}

impl From<LibSqlError> for Error {
    fn from(e: LibSqlError) -> Self {
        Error::DatabaseQuery(e.to_string())
    }
}

impl From<JwtError> for Error {
    fn from(_: JwtError) -> Self {
        Error::BadAuthToken
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        if status.is_server_error() {
            log::error!("{self}");
        }

        HttpResponse::build(status)
            .insert_header(ContentType::json())
            .body(self.payload())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::AccessDenied => StatusCode::FORBIDDEN,
            Error::AddressNotFound
            | Error::FileDownload
            | Error::FileNotFound
            | Error::TreeNotFound => StatusCode::NOT_FOUND,
            Error::BadAuthToken
            | Error::GoogleUserInfo
            | Error::MissingAuthorizationHeader
            | Error::UserAgentNotSet
            | Error::UserNotFound => StatusCode::UNAUTHORIZED,
            Error::BadAuthorizationHeader
            | Error::BadCallback
            | Error::BadImage
            | Error::BadRequest => StatusCode::BAD_REQUEST,
            Error::DuplicateTree => StatusCode::CONFLICT,
            Error::DatabaseQuery(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
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
            Error::DatabaseConnect(s) => write!(f, "DatabaseConnect: {s}"),
            Error::DatabaseQuery(s) => write!(f, "Database error: {s}"),
            Error::DatabaseStructure(s) => write!(f, "DatabaseStructure: {s}"),
            Error::DuplicateTree => write!(f, "DuplicateTree"),
            Error::EnvNotSet => write!(f, "EnvNotSet"),
            Error::FileDownload => write!(f, "FileDownload"),
            Error::FileNotFound => write!(f, "FileNotFound"),
            Error::FileUpload => write!(f, "FileUpload"),
            Error::GoogleUserInfo => write!(f, "GoogleUserInfo"),
            Error::ImageResize => write!(f, "ImageResize"),
            Error::MissingAuthorizationHeader => write!(f, "MissingAuthorizationHeader"),
            Error::OsmExchange(s) => write!(f, "OsmExchange: {s}"),
            Error::Queue => write!(f, "Queue"),
            Error::RemoteAddrNotSet => write!(f, "RemoteAddrNotSet"),
            Error::TreeNotFound => write!(f, "TreeNotFound"),
            Error::UniqueId => write!(f, "UniqueId"),
            Error::UserAgentNotSet => write!(f, "UserAgentNotSet"),
            Error::UserNotFound => write!(f, "UserNotFound"),
        }
    }
}
