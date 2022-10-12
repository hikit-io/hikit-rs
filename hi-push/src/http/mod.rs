use std::sync::Arc;

use axum::{
    routing::post,
    Extension,
    Json,
    routing::get,
};


use crate::service::{self, model::*};

pub async fn register_token(
    Extension(app): Extension<Arc<service::App>>,
    Json(params): Json<Vec<RegisterTokenParams>>,
) -> Json<Response<RegisterTokenResp>> {
    let mut success = 0;
    let mut failure = 0;
    let mut failure_tokens = Vec::new();
    for params in params {
        match app
            .register_token(&params.group, &params.ch_id, &params.token, params._override)
            .await {
            Ok(_) => {
                success += 1;
            }
            Err(e) => {
                failure += 1;
                failure_tokens.push(params.token);
            }
        }
    }

    let resp = ResponseBuilder::default()
        .data(Some(RegisterTokenResp {
            success,
            failure,
            failure_tokens,
        }))
        .build().unwrap();

    Json(resp)
}

pub async fn revoke_token(
    Extension(app): Extension<Arc<service::App>>,
    Json(params): Json<Vec<RegisterTokenParams>>,
) -> Json<Response<RevokeTokenResp>> {
    let mut success = 0;
    let mut failure = 0;
    let mut failure_tokens = Vec::new();

    for param in params.into_iter() {
        match app
            .revoke_token(&param.group, &param.ch_id, &param.token)
            .await {
            Ok(_) => {
                success += 1;
            }
            Err(e) => {
                failure += 1;
                failure_tokens.push(param.token);
            }
        }
    }


    let resp = ResponseBuilder::default()
        .data(Some(RevokeTokenResp {
            success,
            failure,
            failure_tokens,
        }))
        .build().unwrap();

    Json(resp)
}

pub async fn push_transparent(
    Extension(app): Extension<Arc<service::App>>,
    Json(params): Json<PushTransparentParams>,
) -> Json<Response<RevokeTokenResp>> {
    let res = app
        .push_message(service::Message::Transparent(params))
        .await;

    let resp = ResponseBuilder::default().build().unwrap();
    Json(resp)
}

pub async fn push_notification(
    Extension(app): Extension<Arc<service::App>>,
    Json(params): Json<PushNotificationParams>,
) -> Json<Response<RevokeTokenResp>> {
    let res = app
        .push_message(service::Message::Notification(params))
        .await;

    let resp = ResponseBuilder::default().build().unwrap();
    Json(resp)
}

pub async fn ping() -> Json<Response<String>> {
    let resp = ResponseBuilder::<String>::default().msg("PONG".to_string()).build().unwrap();
    Json(resp)
}

pub async fn create_channel(
    Extension(app): Extension<Arc<service::App>>,
    Json(params): Json<PublicChannel>,
) -> Json<Response> {
    let _ = app.create_channel("app_id", params).await;
    let resp = ResponseBuilder::default().build().unwrap();
    Json(resp)
}

pub async fn status(
    Extension(app): Extension<Arc<service::App>>,
) -> Json<Response<Running>> {
    let resp = ResponseBuilder::default()
        .data(Some(Running {
            ch_ids: app.running_ch_ids().await,
        }))
        .build().unwrap();
    Json(resp)
}


pub fn api_router() -> axum::Router {
    axum::Router::new()
        .route("/register", post(register_token))
        .route("/revoke", post(revoke_token))
        .route("/transparent", post(push_transparent))
        .route("/notification", post(push_notification))
        .route("/ping", get(ping))
        .route("/channel", post(create_channel))
        .route("/status", get(status))
}

pub async fn start(mut app: service::App) -> anyhow::Result<()> {
    app.init().await.expect("app init error");
    let router = axum::Router::new().nest("/api", api_router()).layer(Extension(Arc::new(app)));
    Ok(axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await?)
}

#[cfg(test)]
mod test {}
