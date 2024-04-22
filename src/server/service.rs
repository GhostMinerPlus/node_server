use axum::http::StatusCode;

pub async fn http_index() -> (StatusCode, String) {
    (StatusCode::OK, "Hello".to_string())
}
