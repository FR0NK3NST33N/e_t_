use askama::Template;

#[derive(Template)]
#[template(path = "components/alert.html")]
pub struct AlertTemplate {
    pub id: String,
    pub error: String
}