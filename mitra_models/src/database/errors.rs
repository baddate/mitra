use std::error::{Error as _ErrorTrait};
use thiserror::Error;
use tokio_postgres::error::{Error as PostgresError, SqlState};

#[derive(Debug, Error)]
#[error("database type error")]
pub struct DatabaseTypeError;

// Restores the behavior of tokio-postgres 0.7.14
// https://github.com/rust-postgres/rust-postgres/issues/1284
fn describe_postgres_error(error: &PostgresError) -> String {
    let mut message = error.to_string();
    if let Some(source) = error.source() {
        message += &format!(": {source}");
    };
    message
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database pool error")]
    PoolError(#[from] deadpool_postgres::PoolError),

    #[error("database query error")]
    QueryError(#[from] postgres_query::Error),

    #[error("{}", describe_postgres_error(.0))]
    ClientError(#[from] PostgresError),

    #[error(transparent)]
    TypeError(#[from] DatabaseTypeError),

    #[error("{0} not found")]
    NotFound(&'static str), // object type

    #[error("{0} already exists")]
    AlreadyExists(&'static str), // object type
}

impl DatabaseError {
    pub fn type_error() -> Self {
        Self::from(DatabaseTypeError)
    }
}

pub fn catch_unique_violation(
    object_type: &'static str,
) -> impl Fn(PostgresError) -> DatabaseError {
    move |err| {
        if let Some(code) = err.code() {
            if code == &SqlState::UNIQUE_VIOLATION {
                return DatabaseError::AlreadyExists(object_type);
            };
        };
        err.into()
    }
}
