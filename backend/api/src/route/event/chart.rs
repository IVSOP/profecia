use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{AppState, error::AppResult, state::market_snapshot::EventChartDto};

#[debug_handler]
pub async fn handle(
    Path(event_id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> AppResult<Json<EventChartDto>> {
    let chart = app_state.get_event_chart(event_id).await?;
    Ok(Json(chart))
}
