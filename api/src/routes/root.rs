use axum::response::IntoResponse;

pub async fn root() -> impl IntoResponse {
    "Jito Stakenet API"
}
