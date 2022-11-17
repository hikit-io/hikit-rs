use std::sync::Arc;

use axum::{
    extract::Query,
    http::{Request, StatusCode},
    middleware::Next,
    response, Extension, Json,
};

use crate::service::{self, model::*};

pub async fn admin_api_auth<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<response::Response, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn delete_app(
    Extension(app): Extension<Arc<service::App>>,
    Query(DeleteApplicationParams { client_id }): Query<DeleteApplicationParams>,
) -> Json<Response> {
    match app.delete_app(&client_id).await {
        Ok(_) => Json(ResponseBuilder::default().build().unwrap()),
        Err(e) => Json(e.into()),
    }
}

pub async fn create_app(
    Extension(app): Extension<Arc<service::App>>,
    Json(params): Json<CreateAppParams>,
) -> Json<Response<App>> {
    match app.create_app(&params.name).await {
        Ok(app) => Json(ResponseBuilder::default().data(Some(app)).build().unwrap()),
        Err(e) => Json(e.into()),
    }
}

pub async fn fetch_applications(
    Extension(app): Extension<Arc<service::App>>,
) -> Json<Response<Vec<App>>> {
    match app.fetch_apps().await {
        Ok(apps) => Json(ResponseBuilder::default().data(Some(apps)).build().unwrap()),
        Err(e) => Json(e.into()),
    }
}

pub async fn status(Extension(app): Extension<Arc<service::App>>) -> Json<Response<Running>> {
    let resp = ResponseBuilder::default()
        .data(Some(Running {
            ch_ids: app.running_ch_ids().await,
        }))
        .msg("client_id".to_string())
        .build()
        .unwrap();
    Json(resp)
}

pub async fn admin_login(Extension(app): Extension<Arc<service::App>>) -> Json<Response> {
    
    Json(ResponseBuilder::default().build().unwrap())
}

