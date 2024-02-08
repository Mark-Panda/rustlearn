// use axum::http::Request;
// use axum::{
//     async_trait,
//     extract::{rejection::JsonRejection, FromRequest},
//     BoxError, Json,
// };
// use serde::de::DeserializeOwned;
// use validator::Validate;

// use crate::server::error::Error;


use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, Json, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

/// Validate User Request.
pub struct ValidationExtractor<T>(pub T);

// #[async_trait]
// impl<T, S, B> FromRequest<S, B> for ValidationExtractor<T>
// where
//     T: DeserializeOwned + Validate,
//     S: Send + Sync,
//     Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
//     B: http_body::Body + Send + 'static,
//     B::Data: Send,
//     B::Error: Into<BoxError>,
// {
//     type Rejection = Error;

//     async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
//         let Json(value) = Json::<T>::from_request(req, state).await?;
//         value.validate()?;
//         Ok(ValidationExtractor(value))
//     }
// }

#[async_trait]
impl<T, S> FromRequest<S> for ValidationExtractor<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidationExtractor(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumJsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}
