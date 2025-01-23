use axum::{
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

/// Validate User Request.
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
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

// use axum::{
//     extract::{rejection::JsonRejection, FromRequest, Json, Request},
//     http::StatusCode,
//     response::{IntoResponse, Response},
// };
// use rutils::error::Error;
// use serde::de::DeserializeOwned;
// use validator::Validate;
// /// Validate User Request.
// #[derive(Debug, Clone, Copy, Default)]
// pub struct ValidatedJson<T>(pub T);

// pub struct ErrorResponse(pub Error);

// impl<T, S> FromRequest<S> for ValidatedJson<T>
// where
//     T: DeserializeOwned + Validate,
//     S: Send + Sync,
//     Json<T>: FromRequest<S, Rejection = JsonRejection>,
// {
//     type Rejection = ErrorResponse;

//     async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
//         let Json(value) = Json::<T>::from_request(req, state)
//             .await
//             .map_err(|e| ErrorResponse(Error::from(e)))?;
//         value
//             .validate()
//             .map_err(|e| ErrorResponse(Error::ValidationError(e)))?;
//         Ok(ValidatedJson(value))
//     }
// }

// impl IntoResponse for ErrorResponse {
//     fn into_response(self) -> Response {
//         match self.0 {
//             Error::ValidationError(_) => {
//                 let message = format!("Input validation error: [{:?}]", self.0).replace('\n', ", ");
//                 (StatusCode::BAD_REQUEST, message)
//             }
//             Error::AxumJsonRejection(_) => (StatusCode::BAD_REQUEST, self.0.to_string()),
//             _ => (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()),
//         }
//         .into_response()
//     }
// }
