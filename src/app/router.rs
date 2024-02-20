use axum::{routing::get, routing::post, Router};
use crate::login::controllers::{login, login_post};
use crate::dashboard::controllers::dashboard;

pub fn create_app_router() -> Router {
    Router::new()
        .route("/", get(login))
        .route("/dashboard", get(dashboard))
        .route("/login", post(login_post))
}