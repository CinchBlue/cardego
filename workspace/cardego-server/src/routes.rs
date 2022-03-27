use std::{borrow::BorrowMut, sync::Arc};

use axum::{
    body::Body,
    extract::Extension,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use cardego_database::{APIConnection, QueryCardsInput};
use tokio::sync::Mutex;

use crate::{
    app::AppState,
    error::{AppError, AppResult, ServerError},
};

pub async fn index() -> AppResult<impl IntoResponse> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("health check ok"))
        .map_err(|err| AppError::Server(err.into()))
}

pub async fn query_cards(
    Json(payload): Json<cardego_database::QueryCardsInput>,
    Extension(app_state): Extension<Arc<Mutex<AppState>>>,
) -> AppResult<impl IntoResponse> {
    let app_state = app_state.lock().await;

    let result = app_state
        .api_conn
        .operation()
        .await
        .map_err(|err| AppError::from(ServerError::from(err)))?
        .query_cards(QueryCardsInput {
            ids: None,
            name_regex: None,
            desc_regex: None,
            image_url_regex: None,
        })
        .await
        .map_err(|err| AppError::from(ServerError::from(err)))?;

    Ok(Json(result))
}
