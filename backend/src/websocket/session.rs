use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use anyhow::Result;

use crate::game::GameEngine;
use crate::types::*;

pub type GameEngineRef = Arc<RwLock<GameEngine>>;

/// Represents a connected client session
pub struct ClientSession {
    pub player_id: PlayerId,
    pub tx: mpsc::UnboundedSender<ServerMessage>,
}

/// Manages all client connections and game state
pub struct GameSession {
    pub engine: GameEngineRef,
    pub clients: Arc<RwLock<Vec<ClientSession>>>,
}

impl GameSession {
    pub fn new(engine: GameEngine) -> Self {
        Self {
            engine: Arc::new(RwLock::new(engine)),
            clients: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add a new client connection
    pub async fn add_client(&self, player_id: PlayerId, tx: mpsc::UnboundedSender<ServerMessage>) {
        let session = ClientSession { player_id, tx };
        self.clients.write().await.push(session);
    }

    /// Remove a client connection
    pub async fn remove_client(&self, player_id: PlayerId) {
        let mut clients = self.clients.write().await;
        clients.retain(|c| c.player_id != player_id);
    }

    /// Broadcast a message to all clients
    pub async fn broadcast(&self, message: ServerMessage) {
        let clients = self.clients.read().await;
        for client in clients.iter() {
            let _ = client.tx.send(message.clone());
        }
    }

    /// Send a message to a specific client
    pub async fn send_to_client(&self, player_id: PlayerId, message: ServerMessage) {
        let clients = self.clients.read().await;
        if let Some(client) = clients.iter().find(|c| c.player_id == player_id) {
            let _ = client.tx.send(message);
        }
    }

    /// Handle a client message
    pub async fn handle_message(&self, player_id: PlayerId, message: ClientMessage) -> Result<()> {
        match message {
            ClientMessage::Attack { from, to } => {
                let mut engine = self.engine.write().await;
                match engine.execute_attack(player_id, from.into(), to.into()) {
                    Ok(result) => {
                        // Broadcast attack result
                        drop(engine);
                        self.broadcast(ServerMessage::AttackResult { result: result.clone() }).await;

                        if result.territory_conquered {
                            self.broadcast(ServerMessage::TerritoryConquered {
                                territory_id: result.to_territory,
                                old_owner: Some(result.defender_id),
                                new_owner: result.attacker_id,
                            })
                            .await;
                        }
                    }
                    Err(e) => {
                        self.send_to_client(
                            player_id,
                            ServerMessage::Error {
                                message: e.to_string(),
                            },
                        )
                        .await;
                    }
                }
            }
            ClientMessage::BuildStructure { territory, building_type } => {
                let mut engine = self.engine.write().await;
                match engine.build_structure(player_id, territory.into(), building_type) {
                    Ok(_) => {
                        drop(engine);
                        self.broadcast(ServerMessage::BuildingCompleted {
                            territory_id: territory,
                            building_type,
                            player_id: player_id.into(),
                        })
                        .await;

                        self.broadcast(ServerMessage::Notification {
                            message: format!("Building completed!"),
                            severity: NotificationLevel::Success,
                        })
                        .await;
                    }
                    Err(e) => {
                        self.send_to_client(
                            player_id,
                            ServerMessage::Error {
                                message: e.to_string(),
                            },
                        )
                        .await;
                    }
                }
            }
            ClientMessage::SetTroopRatio { ratio } => {
                let mut engine = self.engine.write().await;
                let _ = engine.set_troop_ratio(player_id, ratio);
            }
            ClientMessage::SetAttackRatio { ratio } => {
                let mut engine = self.engine.write().await;
                let _ = engine.set_attack_ratio(player_id, ratio);
            }
            ClientMessage::PauseGame => {
                let mut engine = self.engine.write().await;
                engine.set_paused(true);
            }
            ClientMessage::ResumeGame => {
                let mut engine = self.engine.write().await;
                engine.set_paused(false);
            }
            ClientMessage::SetGameSpeed { speed } => {
                let mut engine = self.engine.write().await;
                engine.set_game_speed(speed);
            }
            ClientMessage::GetGameState => {
                let engine = self.engine.read().await;
                self.send_to_client(
                    player_id,
                    ServerMessage::GameStateUpdate {
                        state: engine.state.clone(),
                    },
                )
                .await;
            }
        }

        Ok(())
    }

    /// Game tick loop
    pub async fn start_game_loop(self: Arc<Self>) {
        let tick_rate_ms = {
            let engine = self.engine.read().await;
            engine.tick_rate_ms
        };

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(tick_rate_ms));

            loop {
                interval.tick().await;

                // Update game state
                {
                    let mut engine = self.engine.write().await;
                    engine.tick();
                    engine.tick_ai();

                    // Check for game over
                    if let Some(stats) = engine.check_game_over() {
                        drop(engine);
                        self.broadcast(ServerMessage::GameOver { stats }).await;
                        break;
                    }
                }

                // Broadcast state update every 5 ticks (reduce network traffic)
                let tick = {
                    let engine = self.engine.read().await;
                    engine.state.tick
                };

                if tick % 5 == 0 {
                    let engine = self.engine.read().await;
                    self.broadcast(ServerMessage::GameStateUpdate {
                        state: engine.state.clone(),
                    })
                    .await;
                }
            }
        });
    }
}
