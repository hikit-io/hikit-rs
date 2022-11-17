mod api;
mod admin_api;

use api::*;
use admin_api::*;

use std::sync::Arc;

use axum::{
    middleware::{self},
    routing::post,
    routing::{delete, get},
    Extension, Router,
};

use crate::service;

#[inline]
pub fn api_router() -> axum::Router {
    axum::Router::new()
        .route("/register", post(register_token))
        .route("/revoke", post(revoke_token))
        .route("/transparent", post(push_transparent))
        .route("/notification", post(push_notification))
        .route("/ping", get(ping))
        .route("/channels", get(fetch_channels))
        .route("/channel", post(create_channel))
        .route("/channel", delete(delete_channel))
}

#[inline]
pub fn admin_route() -> axum::Router {
    axum::Router::new()
        .route("/applications", get(fetch_applications))
        .route("/application", post(create_app))
        .route("/application", delete(delete_app))
        .route("/status", get(status))
}

///Example:
///
///pub struct Addr(pub String);
///
///impl ServiceOption for Addr {
///    fn apply(&self, r: &mut Router) {}
///}
///
///pub struct SelfRoute;
///
///impl ServiceOption for SelfRoute {
///    fn apply(&self, r: &mut Router) {}
///}
pub trait ServiceOption {
    fn apply(&self, r: &mut Router);
}

pub async fn start(
    mut app: service::App,
    addr: impl Into<Option<&str>>,
    options: Option<Vec<Box<dyn ServiceOption>>>,
) -> anyhow::Result<()> {
    app.init().await.expect("app init error");
    let app = Arc::new(app);

    let mut api_router = axum::Router::new()
        .nest(
            "/api",
            api_router()
                .layer(middleware::from_fn(api_auth))
                .layer(Extension(app.clone())),
        )
        .nest(
            "/api/admin",
            admin_route()
                .layer(middleware::from_fn(admin_api_auth))
                .layer(Extension(app)),
        )
        .route("/api/admin/login", post(admin_login));

    for option in options.unwrap_or_default().into_iter() {
        let option: Box<dyn ServiceOption> = option.into();
        option.apply(&mut api_router);
    }

    Ok(
        axum::Server::bind(&addr.into().unwrap_or("0.0.0.0:8080").parse().unwrap())
            .serve(api_router.into_make_service())
            .await?,
    )
}

#[cfg(test)]
mod test {}
