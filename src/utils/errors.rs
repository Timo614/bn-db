use std::error::Error;
use std::fmt;

pub enum ErrorCode {
    InvalidInput,
    NoResults,
    Unknown,
}

pub fn get_error_message(code: ErrorCode) -> (i32, String) {
    use self::ErrorCode::*;
    let (code, msg) = match code {
        InvalidInput => (1000, "Invalid input"),
        NoResults => (2000, "No results"),
        Unknown => (10, "Unknown database error"),
    };
    (code, msg.to_string())
}

#[derive(Debug)]
pub struct DatabaseError<'a> {
    code: i32,
    message: String,
    cause: Option<&'a Error>,
}

impl<'a> fmt::Display for DatabaseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)?;
        if let Some(cause) = self.cause {
            write!(f, "\nCaused by:\n")?;
            fmt::Display::fmt(&cause, f)?;
        }
        Ok(())
    }
}

impl<'a> Error for DatabaseError<'a> {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&Error> {
        self.cause
    }
}

impl<'a> DatabaseError<'a> {
    fn new(error_code: ErrorCode, cause: Option<&Error>) -> DatabaseError {
        let (code, message) = get_error_message(error_code);
        DatabaseError {
            code,
            message,
            cause,
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
    let err = DatabaseError::new(ErrorCode::InvalidInput, Some(&cause));
    assert_eq!(err.description(), "Invalid input");
    assert_eq!(err.code, 1000);
    assert!(err.cause.is_some());
    assert_eq!(
        format!("{}", err),
        "\
[1000] Invalid input
Caused by:
[10] Unknown database error"
    );
}

#[test]
fn nested_causes() {
    let cause1 = DatabaseError::new(ErrorCode::Unknown, None);
    let cause2 = DatabaseError::new(ErrorCode::NoResults, Some(&cause1));
    let err = DatabaseError::new(ErrorCode::InvalidInput, Some(&cause2));
    assert_eq!(err.description(), "Invalid input");
    assert_eq!(err.code, 1000);
    assert!(err.cause.is_some());
    assert_eq!(
        format!("{}", err),
        "\
[1000] Invalid input
Caused by:
[2000] No results
Caused by:
[10] Unknown database error"
    );
}
