use axum::{http::StatusCode, response::IntoResponse};
use eyre::Report;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Report);

impl<E> From<E> for Error
where
    E: Into<Report>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = if matches!(
            self.0.downcast_ref::<sqlx::Error>(),
            Some(sqlx::Error::RowNotFound)
        ) {
            StatusCode::NOT_FOUND
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };

        (status, self.0.to_string()).into_response()
    }
}
