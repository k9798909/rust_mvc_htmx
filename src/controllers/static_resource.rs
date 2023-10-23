use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::common::{HtmlTemplate, http::Page404Template};

static LAYOUT_CSS_PATH: &str = include_str!("../../assets/css/layout.css");

pub async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if path == "css/sidebars.css" {
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
        (StatusCode::OK, headers, LAYOUT_CSS_PATH)
    } else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}



pub async fn handle_404() -> impl IntoResponse {
    let index = Page404Template {};
    return HtmlTemplate(index);
}
