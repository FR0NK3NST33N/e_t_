use anyhow::Context;
use axum::body::Body;
use axum::extract::Host;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::middleware::Next;
use axum::{middleware, Extension, Router};
use sqlx::SqlitePool;
use std::sync::Arc;
use tracing::info;
use tower::ServiceBuilder;

mod login;
mod dashboard;
pub mod templates;
pub mod components;

use crate::config::Config;
use crate::util::error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

use tower_http::services::ServeDir;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: SqlitePool,
}

pub async fn serve(config: Config, db: SqlitePool) -> anyhow::Result<()> {
    let port = config.port;
    let assets_path = std::env::current_dir().unwrap();
    let app = app_router()
    .nest("/api", api_router())
    .layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext {
                config: Arc::new(config),
                db,
            }))
    )
    // TODO: I want to figure this out - make the syntax cleaner than this Extension route
    // .with_state(Arc::new(ApiContext {config: Arc::new(config), db}))
    .nest_service(
        "/assets",
         ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
    );
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    info!("Server running on port {}", port);
    axum::serve(listener, app.into_make_service())
        .await
        .context("error running HTTP server")
}

// TODO: Check cookie
async fn auth_check<T>(Host(host): Host, req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    info!("Middleware Ran");
    let authed = true;
    if !authed { 
        let redirect = "http://".to_owned() + &host + "/login";
        let mut res = Redirect::to(&redirect).into_response();
        // HTMX way not needed with complete 303 redirect url.
        // res.headers_mut().insert("HX-Redirect", "/".parse().unwrap());
        Ok(res)
    } else {
        Ok(next.run(req).await)
    }
}

fn app_router() -> Router {
    dashboard::router()
    .layer(middleware::from_fn(auth_check::<Request<Body>>))
    .merge(login::router())

}

fn api_router() -> Router {
    login::api_router()
}