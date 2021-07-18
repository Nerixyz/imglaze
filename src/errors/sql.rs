use crate::errors::json_error::JsonError;
use actix_web::http::StatusCode;
use serde::Serialize;
use sqlx::Error;

pub type SqlResult<T> = Result<T, JsonError<SqlReason>>;

#[derive(Debug, Serialize, derive_more::Display)]
pub enum SqlReason {
    #[display(fmt = "SQL: NotFound")]
    NotFound,
    #[display(fmt = "SQL: Duplicate")]
    Duplicate(String),
    #[display(fmt = "SQL: Internal")]
    Internal,
}

impl From<sqlx::Error> for JsonError<SqlReason> {
    fn from(e: Error) -> Self {
        match e {
            Error::RowNotFound | Error::TypeNotFound { .. } | Error::ColumnNotFound(_) => {
                log::warn!("NotFound sql-error: {}", e);
                Self::new(SqlReason::NotFound, StatusCode::NOT_FOUND)
            }
            Error::Database(db_err) if db_err.constraint().is_some() => Self::new(
                SqlReason::Duplicate(db_err.constraint().unwrap().to_string()),
                StatusCode::CONFLICT,
            ),
            _ => {
                log::warn!("Internal sql-error: {}", e);
                Self::new(SqlReason::Internal, StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
