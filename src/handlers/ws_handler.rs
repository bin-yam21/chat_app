use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::repository::message_repository::MessageRepository;
use crate::models::message::Message as ChatMessage; // your message model

/// Shared app state used by all WebSocket connections
#[derive(Clone)]
pub struct ChatState {
    pub pool: Pool<Postgres>,
    pub tx: broadcast::Sender<(Uuid, ChatMessage)>, // (room_id, message)
}

/// The main WebSocket route handler.
/// Called when a client connects to `/ws/{room_id}`
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(room_id): Path<Uuid>,
    State(state): State<Arc<ChatState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, room_id))
}

/// Handle a single WebSocket connection.
async fn handle_socket(socket: WebSocket, state: Arc<ChatState>, room_id: Uuid) {
    println!("🔌 Client connected to room: {}", room_id);

    // Each client subscribes to the broadcast channel
    let mut rx = state.tx.subscribe();

    // Split the socket into sender (for sending messages) and receiver (for receiving messages)
    let (mut sender, mut receiver) = socket.split();

    // TASK 1: Forward new messages (from the broadcast channel) to this client
    let send_task = {
        let room_id = room_id.clone();
        let mut rx = rx;
        tokio::spawn(async move {
            while let Ok((msg_room, msg)) = rx.recv().await {
                // Only send messages that belong to this room
                if msg_room == room_id {
                    if let Ok(json_msg) = serde_json::to_string(&msg) {
                        if sender.send(Message::Text(json_msg.into())).await.is_err() {
                            println!("⚠️ Failed to send to client in room {}", room_id);
                            break;
                        }
                    }
                }
            }
        })
    };

    // TASK 2: Receive messages from this client and store in DB, then broadcast to others
    let recv_task = {
        let pool = state.pool.clone();
        let tx = state.tx.clone();
        let room_id = room_id.clone();

        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                println!("💬 Received: {}", text);

                // Expected JSON format: {"user_id": "uuid", "content": "hello"}
                #[derive(serde::Deserialize)]
                struct Incoming {
                    user_id: Uuid,
                    content: String,
                }

                if let Ok(incoming) = serde_json::from_str::<Incoming>(&text) {
                    // Store message in Postgres
                    match MessageRepository::create_message(
                        &pool,
                        room_id,
                        incoming.user_id,
                        Some(&incoming.content),
                    )
                    .await
                    {
                        Ok(saved_message) => {
                            println!("💾 Message saved: {:?}", saved_message.content);
                            // Broadcast saved message to all other clients
                            let _ = tx.send((room_id, saved_message));
                        }
                        Err(e) => eprintln!("❌ Failed to save message: {:?}", e),
                    }
                } else {
                    eprintln!("⚠️ Invalid JSON from client: {}", text);
                }
            }
        })
    };

    // Wait until either task finishes (client disconnects, etc.)
    tokio::select! {
        _ = send_task => println!("🛑 Send task ended for room {}", room_id),
        _ = recv_task => println!("👋 Client disconnected from room {}", room_id),
    }
}
