use axum::{
    extract::{Json, State, Path},
    http::StatusCode,
    response::IntoResponse,
};

use sqlx::{Pool, Postgres};
use uuid::Uuid;

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

// GET /rooms/{id}
pub async fn get_room(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match RoomRepository::find_by_id(&pool, &id).await {
        Ok(Some(room)) => (StatusCode::OK, Json(room)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Room not found").into_response(),
        Err(err) => {
            eprintln!("Error fetching room: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch room").into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateRoomRequest {
    pub name: String,
}

// PUT /rooms/{id}
pub async fn update_room(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRoomRequest>,
) -> impl IntoResponse {
    match RoomRepository::update_room(&pool, &id, &payload.name).await {
        Ok(room) => (StatusCode::OK, Json(room)).into_response(),
        Err(err) => {
            eprintln!("Error updating room: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update room").into_response()
        }
    }
}

// DELETE /rooms/{id}
pub async fn delete_room(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match RoomRepository::delete_room(&pool, &id).await {
        Ok(true) => (StatusCode::NO_CONTENT).into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, "Room not found").into_response(),
        Err(err) => {
            eprintln!("Error deleting room: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete room").into_response()
        }
    }
}