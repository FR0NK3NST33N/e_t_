use std::fmt::Debug;

use axum::body::Body;
use axum::extract::{Extension, Host};
// use axum::http::Response;
use axum::response::{Redirect, Response, IntoResponse};
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use serde::Deserialize;
use tracing::info;

use crate::app::{ApiContext, Error, Result, templates};
use crate::app::components;
use crate::util::response;

pub fn router() -> Router {
    Router::new()
        .route("/login", get(login))
        .route("/login", post(login_post))
        
}

pub fn api_router() -> Router {
    Router::new()
        .route("/login", post(api_login_post))
        
}

pub async fn login() -> impl IntoResponse {
    info!("[/login] page requested");
    let template = templates::LoginTemplate {};
    response::HtmlTemplate(template)
}

#[derive(Deserialize)]
pub struct AuthenticateUser {
    username: String,
    password: String,
}

// TODO: Implement salted and hashed pass, plus check.
pub async fn login_post(Host(host): Host, ctx: Extension<ApiContext>, Form(auth): Form<AuthenticateUser>) -> Result<Response> {
    info!("[/login] post requested");
    let template = components::AlertTemplate { id: String::from("alert"), error: String::from("something went wrong")};
    match fetch_user_by_username(ctx, auth.username).await {
        Ok(user) => {
            if auth.password == user.password { 
                info!("auth successful");
                // for if 303 starts working. e
                // let redirect = "http://".to_owned() + &host;
                // let mut res = Redirect::to("").into_response();
                let mut res = Response::new(Body::new("".to_string()));
                res.headers_mut().insert("HX-Redirect", "/".parse().unwrap());
                return Ok(res);
            }
        },
        // TODO: Need this to either catch or fail gracefully so we can respond with
        // alert template. currently only responds with successful fetch_user
        Err(_err) => {
            return Ok(response::HtmlTemplate(template).into_response())
        }
    }
    Ok(response::HtmlTemplate(template).into_response())
}

pub async fn api_login_post() -> &'static str {
    info!("[/api/login] post requested");
    "test"
}

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    id: i64,
    username: String,
    password: String
}

async fn fetch_user_by_username(ctx: Extension<ApiContext>, username: String) -> Result<Json<User>> {
    let user = sqlx::query!(
        r#"
            select id, email, username, password 
            from "users" where username = $1
        "#,
        username,
    )
    .fetch_optional(&ctx.db)
    .await?.ok_or(Error::unprocessable_entity([("error", "query error")]))?;
    
    Ok(Json(User {
        id: user.id.unwrap(), 
            username: user.username,
            password: user.password
        }))
}