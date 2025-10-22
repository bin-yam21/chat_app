use axum::{
    extract::{
        Json, State
    },
  http::StatusCode,
  response::IntoResponse
};

use sqlx::{Pool, Postgres};

use serde::Deserialize;
use crate::repository::room_repository::RoomRepository;

#[derive(Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub created_by: Option<uuid::Uuid>,
}

// POST /rooms
pub async fn create_room(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateRoomRequest>,
) -> impl IntoResponse {
    match RoomRepository::create_room(&pool, &payload.name, payload.created_by.as_ref()).await {
        Ok(room) => (StatusCode::CREATED, Json(room)).into_response(),
        Err(e) => {
            eprintln!("Error creating room: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create room").into_response()
        }
    }
}

// GET /rooms
pub async fn get_rooms(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    match RoomRepository::get_all_rooms(&pool).await {
        Ok(rooms) => Json(rooms).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch rooms").into_response(),
    }
}