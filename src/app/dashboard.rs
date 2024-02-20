use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tracing::info;

use crate::app::templates;
use crate::util::response;

pub fn router() -> Router {
    Router::new()
    .route("/", get(dashboard))
        
}

// TODO: Implement cookie check and redirect to login
pub async fn dashboard() -> impl IntoResponse {
    info!("[/] page requested");
    let template = templates::DashboardTemplate {};
    response::HtmlTemplate(template)
}
