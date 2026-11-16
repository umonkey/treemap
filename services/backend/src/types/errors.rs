use jsonwebtoken::errors::Error as JwtError;
use libsql::Error as LibSqlError;
use std::convert::From;
use std::fmt;

use actix_web::{error::ResponseError, http::header::ContentType, http::StatusCode, HttpResponse};

// SQLite extended result codes for a transient lock: BUSY (5), LOCKED (6) and
// BUSY_SNAPSHOT (517, which is BUSY with the snapshot bit set). The code stays
// in the message, so a report tells lock contention apart from a stale
// snapshot. A statement retry recovers the first two in autocommit mode; a
// BUSY_SNAPSHOT requires restarting the transaction.
const SQLITE_BUSY: i32 = 5;
const SQLITE_LOCKED: i32 = 6;
const SQLITE_BUSY_SNAPSHOT: i32 = 517;

// Prefixes a database message that carries the SQLite extended result code.
const SQLITE_CODE_MARKER: &str = "SQLite failure (code ";

#[derive(Debug)]
pub enum Error {
    AddressNotFound,
    AlertNotFound,
    AccessDenied,
    BadAuthToken,
    BadAuthorizationHeader,
    BadCallback,
    BadImage,
    BadRequest,
    BadRequestMessage(String),
    Config(String),
    DatabaseConnect(String),
    DatabaseQuery(String),
    DatabaseStructure(String),
    DuplicateRecord,
    DuplicateTree,
    EnvNotSet,
    FileDownload,
    FileNotFound,
    FileUpload,
    GoogleUserInfo,
    ImageResize,
    InstanceDisabled,
    InstanceNotFound,
    Io(String),

    MissingAuthorizationHeader,
    OsmExchange(String),
    PanoramaFailure(String),
    PanoramaNotFound,
    Queue,
    RemoteAddrNotSet,
    SpeciesNotFound,
    TreeNotFound,
    UniqueId,
    UserAgentNotSet,
    UserNotFound,
    WaterNotFound,
}

impl Error {
    fn payload(&self) -> String {
        match self {
            Error::AddressNotFound => {
                r#"{"error":{"code":"AddressNotFound","description":"Could not find an address for the given coordinates."}}"#.to_string()
            }
            Error::AlertNotFound => {
                r#"{"error":{"code":"AlertNotFound","description":"The specified alert does not exist in the database."}}"#.to_string()
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
            Error::BadRequestMessage(s) => {
                format!(r#"{{"error":{{"code":"BadRequest","description":"{s}"}}}}"#)
            }
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
            Error::DuplicateRecord => {
                r#"{"error":{"code":"DuplicateRecord","description":"This record already exists."}}"#.to_string()
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
            Error::InstanceDisabled => {
                r#"{"error":{"code":"InstanceDisabled","description":"This instance is currently disabled."}}"#.to_string()
            }
            Error::InstanceNotFound => {
                r#"{"error":{"code":"InstanceNotFound","description":"The specified instance does not exist."}}"#.to_string()
            }
            Error::Io(_) => {
                r#"{"error":{"code":"Io","description":"Input/output error."}}"#.to_string()
            }

            Error::MissingAuthorizationHeader => {
                r#"{"error":{"code":"MissingAuthorizationHeader","description":"Authentication required for this call."}}"#.to_string()
            }
            Error::OsmExchange(_) => {
                r#"{"error":{"code":"OsmExchange","description":"OSM exchange failed."}}"#.to_string()
            }
            Error::PanoramaFailure(s) => {
                format!(r#"{{"error":{{"code":"PanoramaFailure","description":"{s}"}}}}"#)
            }
            Error::PanoramaNotFound => {
                r#"{"error":{"code":"PanoramaNotFound","description":"The specified panorama does not exist in the database."}}"#.to_string()
            }
            Error::Queue => {
                r#"{"error":{"code":"Queue","description":"Error processing queue request."}}"#.to_string()
            }
            Error::RemoteAddrNotSet => {
                r#"{"error":{"code":"RemoteAddrNotSet","description":"Remote address not set."}}"#.to_string()
            }
            Error::SpeciesNotFound => {
                r#"{"error":{"code":"SpeciesNotFound","description":"The specified species does not exist in the database."}}"#.to_string()
            }
            Error::TreeNotFound => {
                r#"{"error":{"code":"TreeNotFound","description":"The specified tree does not exist in the database."}}"#.to_string()
            }
            Error::WaterNotFound => {
                r#"{"error":{"code":"WaterNotFound","description":"The specified water source does not exist in the database."}}"#.to_string()
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

    // Classifies a database error caused by a transient lock, which a repeated
    // write of the same statement can recover from. Only classifies: the error
    // is logged once at the response boundary.
    //
    // The retry is valid only for an autocommit statement. Retrying a statement
    // inside an open transaction cannot recover: the snapshot that caused the
    // failure is still stale, and the transaction has to be restarted instead.
    pub fn is_transient_lock(&self) -> bool {
        let Error::DatabaseQuery(message) = self else {
            return false;
        };

        let transient_code = Self::sqlite_result_code(message)
            .is_some_and(|code| matches!(code, SQLITE_BUSY | SQLITE_LOCKED | SQLITE_BUSY_SNAPSHOT));

        if transient_code {
            return true;
        }

        // Fallback for messages without a result code: other libsql error
        // variants only expose the lock state in the text.
        let message = message.to_lowercase();

        message.contains("database is locked")
            || message.contains("database table is locked")
            || message.contains("sqlite_busy")
    }

    // Extracts the SQLite extended result code from a database message, when
    // the message carries one.
    fn sqlite_result_code(message: &str) -> Option<i32> {
        let rest = message.strip_prefix(SQLITE_CODE_MARKER)?;

        rest.split_once(')')?.0.parse().ok()
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

impl From<LibSqlError> for Error {
    fn from(e: LibSqlError) -> Self {
        // Include the SQLite extended result code, otherwise every lock looks
        // like the same opaque "database is locked" message.
        let message = match e {
            LibSqlError::SqliteFailure(code, message) => {
                format!("{SQLITE_CODE_MARKER}{code}): {message}")
            }
            other => other.to_string(),
        };

        if message.contains("UNIQUE constraint failed") {
            return Error::DuplicateRecord;
        }

        Error::DatabaseQuery(message)
    }
}

impl From<JwtError> for Error {
    fn from(_: JwtError) -> Self {
        Error::BadAuthToken
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::BadRequestMessage(format!("JSON error: {e}"))
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
            Error::AccessDenied | Error::InstanceDisabled => StatusCode::FORBIDDEN,
            Error::AddressNotFound
            | Error::AlertNotFound
            | Error::FileDownload
            | Error::FileNotFound
            | Error::PanoramaNotFound
            | Error::InstanceNotFound
            | Error::SpeciesNotFound
            | Error::TreeNotFound
            | Error::WaterNotFound => StatusCode::NOT_FOUND,
            Error::BadAuthToken
            | Error::GoogleUserInfo
            | Error::MissingAuthorizationHeader
            | Error::UserAgentNotSet
            | Error::UserNotFound => StatusCode::UNAUTHORIZED,
            Error::BadAuthorizationHeader
            | Error::BadCallback
            | Error::BadImage
            | Error::BadRequest
            | Error::BadRequestMessage(_)
            | Error::PanoramaFailure(_) => StatusCode::BAD_REQUEST,
            Error::DuplicateRecord | Error::DuplicateTree => StatusCode::CONFLICT,
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
            Error::AlertNotFound => write!(f, "AlertNotFound"),
            Error::BadAuthToken => write!(f, "BadAuthToken"),
            Error::BadAuthorizationHeader => write!(f, "BadAuthorizationHeader"),
            Error::BadCallback => write!(f, "BadCallback"),
            Error::BadImage => write!(f, "BadImage"),
            Error::BadRequest => write!(f, "BadRequest"),
            Error::BadRequestMessage(s) => write!(f, "BadRequest: {s}"),
            Error::Config(s) => write!(f, "Config error: {s}"),
            Error::DatabaseConnect(s) => write!(f, "DatabaseConnect: {s}"),
            Error::DatabaseQuery(s) => write!(f, "Database error: {s}"),
            Error::DatabaseStructure(s) => write!(f, "DatabaseStructure: {s}"),
            Error::DuplicateRecord => write!(f, "DuplicateRecord"),
            Error::DuplicateTree => write!(f, "DuplicateTree"),
            Error::EnvNotSet => write!(f, "EnvNotSet"),
            Error::FileDownload => write!(f, "FileDownload"),
            Error::FileNotFound => write!(f, "FileNotFound"),
            Error::FileUpload => write!(f, "FileUpload"),
            Error::GoogleUserInfo => write!(f, "GoogleUserInfo"),
            Error::ImageResize => write!(f, "ImageResize"),
            Error::InstanceDisabled => write!(f, "InstanceDisabled"),
            Error::InstanceNotFound => write!(f, "InstanceNotFound"),
            Error::Io(s) => write!(f, "IO error: {s}"),

            Error::MissingAuthorizationHeader => write!(f, "MissingAuthorizationHeader"),
            Error::OsmExchange(s) => write!(f, "OsmExchange: {s}"),
            Error::PanoramaFailure(s) => write!(f, "PanoramaFailure: {s}"),
            Error::PanoramaNotFound => write!(f, "PanoramaNotFound"),
            Error::Queue => write!(f, "Queue"),
            Error::RemoteAddrNotSet => write!(f, "RemoteAddrNotSet"),
            Error::SpeciesNotFound => write!(f, "SpeciesNotFound"),
            Error::TreeNotFound => write!(f, "TreeNotFound"),
            Error::UniqueId => write!(f, "UniqueId"),
            Error::UserAgentNotSet => write!(f, "UserAgentNotSet"),
            Error::UserNotFound => write!(f, "UserNotFound"),
            Error::WaterNotFound => write!(f, "WaterNotFound"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use libsql::Error as LibSqlError;

    #[test]
    fn test_is_transient_lock_detects_busy_codes() {
        for code in [5, 6, 517] {
            let error = Error::DatabaseQuery(format!("SQLite failure (code {code}): locked"));

            assert!(error.is_transient_lock(), "code {code} should be transient");
        }
    }

    #[test]
    fn test_is_transient_lock_detects_busy_messages() {
        let locked = Error::DatabaseQuery("SQLite failure: `database is locked`".to_string());
        let table_locked =
            Error::DatabaseQuery("SQLite failure: `database table is locked`".to_string());
        let busy = Error::DatabaseQuery("SQLite failure: SqliteFailure(SQLITE_BUSY)".to_string());

        assert!(locked.is_transient_lock());
        assert!(table_locked.is_transient_lock());
        assert!(busy.is_transient_lock());
    }

    #[test]
    fn test_is_transient_lock_ignores_other_errors() {
        let unique =
            Error::DatabaseQuery("SQLite failure: UNIQUE constraint failed: files.id".to_string());
        let syntax = Error::DatabaseQuery("syntax error near FROM".to_string());
        let persistent =
            Error::DatabaseQuery("SQLite failure (code 26): file is not a database".to_string());

        assert!(!unique.is_transient_lock());
        assert!(!syntax.is_transient_lock());
        assert!(!persistent.is_transient_lock());
        assert!(!Error::DuplicateRecord.is_transient_lock());
        assert!(!Error::DatabaseConnect("database is locked".to_string()).is_transient_lock());
    }

    #[test]
    fn test_from_libsql_error_keeps_result_code() {
        let error = Error::from(LibSqlError::SqliteFailure(
            517,
            "database is locked".to_string(),
        ));

        let Error::DatabaseQuery(message) = &error else {
            panic!("expected a database query error");
        };

        assert!(message.contains("517"));
        assert!(error.is_transient_lock());
    }

    #[test]
    fn test_from_libsql_error_maps_unique_constraint() {
        let error = Error::from(LibSqlError::SqliteFailure(
            2067,
            "UNIQUE constraint failed: files.id".to_string(),
        ));

        assert!(matches!(error, Error::DuplicateRecord));
    }
}
