use std::sync::Arc;

use axum::{routing::post, Extension, Json};

use crate::service::{self, model::*};

pub async fn register_token(
    Extension(app): Extension<Arc<service::App<'_>>>,
    Json(params): Json<RegisterTokenParams>,
) -> Json<Response<RegisterTokenResp>> {
    let res = app
        .register_token(&params.group, &params.chan, &params.token, params._override)
        .await;

    let resp = ResponseBuilder::default().build().unwrap();

    Json(resp)
}

pub async fn revoke_token(
    Extension(app): Extension<Arc<service::App<'_>>>,
    Json(params): Json<RegisterTokenParams>,
) -> Json<Response<RegisterTokenResp>> {
    let res = app
        .revoke_token(&params.group, &params.chan, &params.token, params._override)
        .await;

    let resp = ResponseBuilder::default().build().unwrap();

    Json(resp)
}

pub async fn push_transparent(
    Extension(app): Extension<Arc<service::App<'_>>>,
    Json(params): Json<PushTransparentParams>,
) -> Json<Response<RegisterTokenResp>> {
    let res = app
        .push_message(service::Message::Transparent(params))
        .await;

    let resp = ResponseBuilder::default().build().unwrap();
    Json(resp)
}

pub async fn push_notification(
    Extension(app): Extension<Arc<service::App<'_>>>,
    Json(params): Json<PushNotificationParams>,
) -> Json<Response<RegisterTokenResp>> {
    let res = app
        .push_message(service::Message::Notification(params))
        .await;

    let resp = ResponseBuilder::default().build().unwrap();
    Json(resp)
}

pub fn api_router() -> axum::Router {
    axum::Router::new()
        .route("/register", post(register_token))
        .route("/revoke", post(revoke_token))
        .route("/transparent", post(push_transparent))
        .route("/notification", post(push_notification))
}

pub async fn start() -> anyhow::Result<()> {
    let router = axum::Router::new().nest("/api", api_router());
    Ok(axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await?)
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_main() {
        super::start().await.expect("start failed");
    }
}
