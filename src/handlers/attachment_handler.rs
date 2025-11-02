use axum::{
    extract::{State, Path},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use sqlx::{Pool, Postgres};

use crate::{
    repository::attachment_repository::AttachmentRepository,
    models::attachment::{CreateAttachment},
};


// #[derive(Clone)]
// pub struct AppState {
//     pub db: Pool<Postgres>,
// }

pub async fn create_attachment(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateAttachment>,
) -> impl IntoResponse {
    match AttachmentRepository::create(&pool, payload).await {
        Ok(attachment) => (StatusCode::CREATED, Json(attachment)).into_response(),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create attachment").into_response()
        }
    }
}

pub async fn get_attachments_by_message(
    State(pool): State<Pool<Postgres>>,
    Path(message_id): Path<Uuid>,
) -> impl IntoResponse {
    match AttachmentRepository::get_by_message(&pool, message_id).await {
        Ok(list) => (StatusCode::OK, Json(list)).into_response(),
        Err(err) => {
            eprintln!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch attachments").into_response()
        }
    }
}

pub async fn delete_attachment(
     State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match AttachmentRepository::delete(&pool, id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(err) => {
            eprintln!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete attachment").into_response()
        }
    }
}
