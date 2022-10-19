use std::sync::Arc;

use axum::{
    extract::Query,
    http::{header, Request, StatusCode},
    middleware::{self, Next},
    response,
    routing::post,
    routing::{delete, get},
    Extension, Json, Router,
};

use crate::service::{self, model::*};

#[derive(Clone)]
pub struct Auth(String, String);

async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<response::Response, StatusCode> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let mut tokens = token.split(" ").collect::<Vec<_>>();

    let token = if tokens.len() != 2 {
        return Err(StatusCode::UNAUTHORIZED);
    } else {
        tokens.pop().unwrap()
    };

    let token = String::from_utf8(base64::decode(token).map_err(|e| StatusCode::UNAUTHORIZED)?)
        .map_err(|e| StatusCode::UNAUTHORIZED)?;

    let user_pass = token.split(":").collect::<Vec<_>>();
    if user_pass.len() != 2 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if let [client_id, client_secret] = user_pass[..] {
        let app = req
            .extensions()
            .get::<Arc<service::App>>()
            .expect("missing extension `service::App`");

        let _ = app
            .validate_app(client_id, client_secret)
            .await
            .map_err(|e| StatusCode::UNAUTHORIZED)?;

        req.extensions_mut()
            .insert(Auth(user_pass[0].to_string(), user_pass[1].to_string()));
        return Ok(next.run(req).await);
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn register_token(
    Extension(app): Extension<Arc<service::App>>,
    Extension(Auth(client_id, _)): Extension<Auth>,
    Json(params): Json<Vec<RegisterTokenParams>>,
) -> Json<Response<RegisterTokenResp>> {
    let mut success = 0;
    let mut failure = 0;
    let mut failure_tokens = Vec::new();
    let mut errors = Vec::new();
    for params in params {
        match app
            .register_token(
                &client_id,
                &params.group,
                &params.ch_id,
                &params.token,
                params._override,
            )
            .await
        {
            Ok(_) => {
                success += 1;
            }
            Err(e) => {
                failure += 1;
                failure_tokens.push(params.token);
                errors.push(e.to_string());
            }
        }
    }

    let resp = ResponseBuilder::default()
        .data(Some(RegisterTokenResp {
            success,
            failure,
            failure_tokens,
        }))
        .errors(Some(errors))
        .build()
        .unwrap();

    Json(resp)
}

pub async fn revoke_token(
    Extension(app): Extension<Arc<service::App>>,
    Extension(Auth(client_id, _)): Extension<Auth>,
    Json(params): Json<Vec<RevokeTokenParams>>,
) -> Json<Response<RevokeTokenResp>> {
    let mut success = 0;
    let mut failure = 0;
    let mut failure_tokens = Vec::new();

    let mut errors = Vec::new();

    for param in params.into_iter() {
        match app
            .revoke_token(&client_id, &param.group, &param.ch_id, &param.token)
            .await
        {
            Ok(_) => {
                success += 1;
            }
            Err(e) => {
                failure += 1;
                failure_tokens.push(param.token);
                errors.push(e.to_string());
            }
        }
    }

    let resp = ResponseBuilder::default()
        .data(Some(RevokeTokenResp {
            success,
            failure,
            failure_tokens,
        }))
        .errors(Some(errors))
        .build()
        .unwrap();

    Json(resp)
}

pub async fn push_transparent(
    Extension(app): Extension<Arc<service::App>>,
    Extension(Auth(client_id, _)): Extension<Auth>,
    Json(params): Json<PushTransparentParams>,
) -> Json<Response> {
    let res = app
        .push_message(&client_id, service::Message::Transparent(params))
        .await;

    let mut resp = ResponseBuilder::default();
    match res {
        Ok(_) => {}
        Err(e) => {
            resp.errors(Some(vec![e.to_string()]));
        }
    }

    Json(resp.build().unwrap())
}

pub async fn push_notification(
    Extension(app): Extension<Arc<service::App>>,
    Extension(Auth(client_id, _)): Extension<Auth>,
    Json(params): Json<PushNotificationParams>,
) -> Json<Response<PushResp>> {
    let resp = match app
        .push_message(&client_id, service::Message::Notification(params))
        .await
    {
        Ok(res) => Response {
            data: Some(res),
            code: Default::default(),
            msg: Default::default(),
            errors: Default::default(),
        },
        Err(e) => e.into(),
    };

    Json(resp)
}

pub async fn ping() -> Json<Response<String>> {
    let resp = ResponseBuilder::<String>::default()
        .msg("PONG".to_string())
        .build()
        .unwrap();
    Json(resp)
}

pub async fn create_channel(
    Extension(app): Extension<Arc<service::App>>,
    Extension(Auth(client_id, _)): Extension<Auth>,
    Json(params): Json<PublicChannel>,
) -> Json<Response> {
    let resp = match app.create_channel(&client_id, params).await {
        Ok(ch_id) => ResponseBuilder::default()
            .data(Some(ch_id))
            .build()
            .unwrap(),
        Err(e) => e.into(),
    };
    Json(resp)
}

pub async fn delete_channel(
    Extension(app): Extension<Arc<service::App>>,
    Extension(Auth(client_id, _)): Extension<Auth>,
    Query(params): Query<DeleteChannelParams>,
) -> Json<Response> {
    let resp = match app.delete_channel(&client_id, &params.ch_id).await {
        Ok(_) => ResponseBuilder::default().build().unwrap(),
        Err(e) => e.into(),
    };
    Json(resp)
}

pub async fn fetch_channels(
    Extension(app): Extension<Arc<service::App>>,
    Extension(Auth(client_id, _)): Extension<Auth>,
) -> Json<Response<Vec<Channel>>> {
    let resp = match app.fetch_channels(&client_id).await {
        Ok(chans) => ResponseBuilder::default()
            .data(Some(chans))
            .build()
            .unwrap(),
        Err(e) => e.into(),
    };
    Json(resp)
}

pub async fn delete_app(
    Extension(app): Extension<Arc<service::App>>,
    Extension(Auth(client_id, client_secret)): Extension<Auth>,
) -> Json<Response> {
    match app.delete_app(&client_id, &client_secret).await {
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

pub async fn status(
    Extension(app): Extension<Arc<service::App>>,
    Extension(Auth(client_id, _)): Extension<Auth>,
) -> Json<Response<Running>> {
    let resp = ResponseBuilder::default()
        .data(Some(Running {
            ch_ids: app.running_ch_ids().await,
        }))
        .msg(client_id)
        .build()
        .unwrap();
    Json(resp)
}

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
    let mut router = axum::Router::new()
        .nest("/api", api_router())
        .layer(middleware::from_fn(auth))
        .layer(Extension(Arc::new(app)));

    for option in options.unwrap_or_default().into_iter() {
        let option: Box<dyn ServiceOption> = option.into();
        option.apply(&mut router);
    }

    Ok(
        axum::Server::bind(&addr.into().unwrap_or("0.0.0.0:8080").parse().unwrap())
            .serve(router.into_make_service())
            .await?,
    )
}

#[cfg(test)]
mod test {}
