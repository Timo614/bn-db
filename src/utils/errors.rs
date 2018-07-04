use std::error::Error;
use std::fmt;
use std::fmt::Display;

pub enum ErrorCode {
    InvalidInput,
    MissingInput,
    NoResults,
    QueryError,
    InsertError,
    ConnectionError,
    InternalError,
    Unknown,
}

pub fn get_error_message(code: ErrorCode) -> (i32, String) {
    use self::ErrorCode::*;
    let (code, msg) = match code {
        InvalidInput => (1000, "Invalid input"),
        MissingInput => (1100, "Missing input"),
        NoResults => (2000, "No results"),
        QueryError => (3000, "Query Error"),
        InsertError => (3100, "Could not insert record"),
        ConnectionError => (4000, "Connection Error"),
        InternalError => (5000, "Internal error"),
        Unknown => (10, "Unknown database error"),
    };
    (code, msg.to_string())
}

#[derive(Debug)]
pub struct DatabaseError {
    code: i32,
    message: String,
    cause: Option<String>,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)?;
        if let Some(ref cause) = self.cause {
            write!(f, "\nCaused by: {}", cause)?;
        }
        Ok(())
    }
}

impl Error for DatabaseError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl DatabaseError {
    pub fn new(error_code: ErrorCode, cause: Option<&str>) -> DatabaseError {
        let (code, message) = get_error_message(error_code);
        let description = match cause {
            Some(err) => Some(String::from(err)),
            None => None,
        };
        DatabaseError {
            code,
            message,
            cause: description,
        }
    }

    /// Wraps the error from a Result into a DatabaseError
    pub fn wrap<T, E: Display>(
        error_code: ErrorCode,
        message: &str,
        res: Result<T, E>,
    ) -> Result<T, DatabaseError> {
        match res {
            Ok(val) => Ok(val),
            Err(e) => Err(DatabaseError::new(
                error_code,
                Some(&format!("{}, {}", message, e.to_string())),
            )),
        }
    }
}

#[test]
fn error_with_unknown_code() {
    let err = DatabaseError::new(ErrorCode::Unknown, None);
    assert_eq!(err.description(), err.message);
    assert_eq!(err.code, 10);
    assert!(err.cause.is_none());
    assert_eq!(format!("{}", err), "[10] Unknown database error");
}

#[test]
fn error_with_known_code() {
    let err = DatabaseError::new(ErrorCode::InvalidInput, None);
    assert_eq!(err.description(), "Invalid input");
    assert_eq!(err.code, 1000);
    assert!(err.cause.is_none());
    assert_eq!(format!("{}", err), "[1000] Invalid input");
}

#[test]
fn unknown_error_with_cause() {
    let cause = DatabaseError::new(ErrorCode::Unknown, None);
    let err = DatabaseError::new(ErrorCode::InvalidInput, Some(cause.description()));
    assert_eq!(err.description(), "Invalid input");
    assert_eq!(err.code, 1000);
    assert!(err.cause.is_some());
    assert_eq!(
        format!("{}", err),
        "\
[1000] Invalid input
Caused by: Unknown database error"
    );
}

#[test]
fn nested_causes() {
    let cause1 = DatabaseError::new(ErrorCode::Unknown, None);
    let cause2 = DatabaseError::new(ErrorCode::NoResults, Some(&format!("{}", cause1)));
    let err = DatabaseError::new(ErrorCode::InvalidInput, Some(&format!("{}", cause2)));
    assert_eq!(err.code, 1000);
    assert!(err.cause.is_some());
    assert_eq!(
        format!("{}", err),
        "\
[1000] Invalid input
Caused by: [2000] No results
Caused by: [10] Unknown database error"
    );
}
