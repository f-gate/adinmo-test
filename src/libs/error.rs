use actix_web::{ResponseError};
use derive_more::{Error, Display};
use crate::Debug;
use sea_orm::DbErr;

// shamelessly copied from seaorm docs.
#[derive(Debug, Error, Display)]
pub struct ErrorResponder {
    message: String,
}

impl ResponseError for ErrorResponder {}

// The following impl's are for easy conversion of error types.
impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        ErrorResponder {
            message: err.to_string(),
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        str.to_owned().into()
    }
}

impl From<serde_json::Error> for ErrorResponder {
    fn from(err: serde_json::Error) -> ErrorResponder {
        ErrorResponder {
            message: err.to_string(),
        }
    }
}
