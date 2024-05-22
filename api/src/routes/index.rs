use axum::response::IntoResponse;

pub async fn index() -> impl IntoResponse {
    "Jito Stakenet API"
}
