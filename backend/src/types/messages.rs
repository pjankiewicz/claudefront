use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{
    BuildingType, CombatResult, GameState, GameStats, NotificationLevel,
};
use uuid::Uuid;

/// Messages sent from client to server
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    /// Attack a neighboring territory
    Attack {
        #[schema(value_type = String, format = "uuid")]
        from: Uuid,
        #[schema(value_type = String, format = "uuid")]
        to: Uuid,
    },
    /// Build a structure in a territory
    BuildStructure {
        #[schema(value_type = String, format = "uuid")]
        territory: Uuid,
        building_type: BuildingType,
    },
    /// Set the troop/worker ratio (0.0-1.0)
    SetTroopRatio {
        ratio: f32,
    },
    /// Set the attack commitment ratio (0.0-1.0)
    SetAttackRatio {
        ratio: f32,
    },
    /// Pause the game
    PauseGame,
    /// Resume the game
    ResumeGame,
    /// Set game speed multiplier
    SetGameSpeed {
        speed: f32,
    },
    /// Request full game state
    GetGameState,
}

/// Messages sent from server to client
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    /// Full game state update
    GameStateUpdate {
        state: GameState,
    },
    /// Result of a combat action
    AttackResult {
        result: CombatResult,
    },
    /// Territory ownership changed
    TerritoryConquered {
        #[schema(value_type = String, format = "uuid")]
        territory_id: Uuid,
        #[schema(value_type = String, format = "uuid", nullable = true)]
        old_owner: Option<Uuid>,
        #[schema(value_type = String, format = "uuid")]
        new_owner: Uuid,
    },
    /// Building was constructed
    BuildingCompleted {
        #[schema(value_type = String, format = "uuid")]
        territory_id: Uuid,
        building_type: BuildingType,
        #[schema(value_type = String, format = "uuid")]
        player_id: Uuid,
    },
    /// Player was eliminated
    PlayerEliminated {
        #[schema(value_type = String, format = "uuid")]
        player_id_test: Uuid,
        #[schema(value_type = String, format = "uuid")]
        eliminated_by: Uuid,
    },
    /// Game has ended
    GameOver {
        stats: GameStats,
    },
    /// General notification
    Notification {
        message: String,
        severity: NotificationLevel,
    },
    /// Error response
    Error {
        message: String,
    },
}
