use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::state::AppState;

pub async fn validator_history(
    State(app_state): State<AppState>,
    Path(vote_account): Path<String>,
) -> impl IntoResponse {
    let app_state = app_state.lock().await;
    let entry = app_state.cache.validator_history_map.get(&vote_account);

    match entry {
        Some(_) => "Ok",
        None => "None",
    }
}
