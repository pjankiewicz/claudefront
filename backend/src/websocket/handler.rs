use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::types::*;
use super::session::GameSession;

/// WebSocket connection handler
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(game_session): State<Arc<GameSession>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, game_session))
}

async fn handle_socket(socket: WebSocket, game_session: Arc<GameSession>) {
    let (mut sender, mut receiver) = socket.split();

    // Create channel for outgoing messages
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();

    // Get human player ID (first non-AI player)
    let player_id = {
        let engine = game_session.engine.read().await;
        engine.state.players
            .iter()
            .find(|p| !p.is_ai)
            .map(|p| p.id.into())
            .expect("No human player found")
    };

    // Register client
    game_session.add_client(player_id, tx).await;

    info!("Client connected: {:?}", player_id);

    // Send initial game state
    {
        let engine = game_session.engine.read().await;
        let initial_state = ServerMessage::GameStateUpdate {
            state: engine.state.clone(),
        };

        if let Ok(json) = serde_json::to_string(&initial_state) {
            let _ = sender.send(Message::Text(json)).await;
        }
    }

    // Spawn task to handle outgoing messages
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Spawn task to handle incoming messages
    let session_clone = game_session.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(client_msg) => {
                        if let Err(e) = session_clone.handle_message(player_id, client_msg).await {
                            error!("Error handling message: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse client message: {}", e);
                    }
                }
            } else if let Message::Close(_) = msg {
                break;
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    // Clean up
    game_session.remove_client(player_id).await;
    info!("Client disconnected: {:?}", player_id);
}
