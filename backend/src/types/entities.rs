use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Territory identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
#[schema(as = String, description = "Territory identifier")]
pub struct TerritoryId(pub Uuid);

impl TerritoryId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<Uuid> for TerritoryId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<TerritoryId> for Uuid {
    fn from(territory_id: TerritoryId) -> Self {
        territory_id.0
    }
}

/// Player identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
#[schema(as = String, description = "Player identifier")]
pub struct PlayerId(pub Uuid);

impl PlayerId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<Uuid> for PlayerId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<PlayerId> for Uuid {
    fn from(player_id: PlayerId) -> Self {
        player_id.0
    }
}

/// Terrain type affecting territory bonuses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum TerrainType {
    /// +20% gold generation
    Plains,
    /// +30% defense bonus
    Mountains,
    /// +20% population growth
    Forests,
    /// No bonus, visual variety
    Water,
}

impl TerrainType {
    pub fn gold_multiplier(&self) -> f32 {
        match self {
            TerrainType::Plains => 1.2,
            _ => 1.0,
        }
    }

    pub fn defense_multiplier(&self) -> f32 {
        match self {
            TerrainType::Mountains => 0.7, // Reduces attacker damage by 30%
            _ => 1.0,
        }
    }

    pub fn population_growth_multiplier(&self) -> f32 {
        match self {
            TerrainType::Forests => 1.2,
            _ => 1.0,
        }
    }
}

/// Building types that can be constructed in territories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum BuildingType {
    /// +25,000 max population, costs 1000 gold
    City,
    /// +20% defense, costs 500 gold
    DefensePost,
    /// +50% gold generation, costs 750 gold
    GoldMine,
}

impl BuildingType {
    pub fn cost(&self) -> u32 {
        match self {
            BuildingType::City => 1000,
            BuildingType::DefensePost => 500,
            BuildingType::GoldMine => 750,
        }
    }

    pub fn max_population_bonus(&self) -> u32 {
        match self {
            BuildingType::City => 25_000,
            _ => 0,
        }
    }

    pub fn defense_multiplier(&self) -> f32 {
        match self {
            BuildingType::DefensePost => 0.8, // Reduces defender losses by 20%
            _ => 1.0,
        }
    }

    pub fn gold_multiplier(&self) -> f32 {
        match self {
            BuildingType::GoldMine => 1.5,
            _ => 1.0,
        }
    }
}

/// A territory on the map
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Territory {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[schema(value_type = String, format = "uuid", nullable = true)]
    pub owner: Option<Uuid>,
    pub terrain: TerrainType,
    pub building: Option<BuildingType>,
    /// Current troops stationed in this territory
    pub troops: u32,
    /// Neighboring territory IDs
    #[schema(nullable = true)]
    pub neighbors: Vec<Uuid>,
    /// Visual position for rendering (x, y normalized 0-1)
    pub position: (f32, f32),
}

/// AI personality type determining behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AIPersonality {
    /// High worker ratio, builds defense posts
    Turtle,
    /// High troop ratio, attacks constantly
    Aggressor,
    /// Balanced approach
    Balanced,
    /// Attacks weakest neighbors
    Opportunist,
    /// 100% troops, immediate attacks
    Rusher,
}

/// A player in the game (human or AI)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Player {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub name: String,
    pub is_ai: bool,
    pub ai_personality: Option<AIPersonality>,
    pub color: String, // Hex color like "#FF0000"

    // Resources
    pub population: u32,
    pub max_population: u32,
    pub gold: u32,

    // Ratios (0.0 to 1.0)
    /// Percentage of population used as troops (rest are workers)
    pub troop_ratio: f32,
    /// Percentage of troops committed per attack
    pub attack_ratio: f32,

    // Stats
    pub territories_controlled: u32,
    pub is_alive: bool,
}

impl Player {
    pub fn troops(&self) -> u32 {
        (self.population as f32 * self.troop_ratio) as u32
    }

    pub fn workers(&self) -> u32 {
        self.population - self.troops()
    }
}

/// Complete game state
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GameState {
    pub territories: Vec<Territory>,
    pub players: Vec<Player>,
    pub tick: u64,
    pub game_speed: f32, // 1.0 = normal, 2.0 = 2x speed, etc.
    pub is_paused: bool,
    pub game_time_seconds: u32,
}

/// Combat result after an attack
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CombatResult {
    #[schema(value_type = String, format = "uuid")]
    pub attacker_id: Uuid,
    #[schema(value_type = String, format = "uuid")]
    pub defender_id: Uuid,
    #[schema(value_type = String, format = "uuid")]
    pub from_territory: Uuid,
    #[schema(value_type = String, format = "uuid")]
    pub to_territory: Uuid,
    pub attacker_troops_committed: u32,
    pub defender_troops: u32,
    pub attacker_losses: u32,
    pub defender_losses: u32,
    pub territory_conquered: bool,
}

/// Game statistics at end of game
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GameStats {
    #[schema(value_type = String, format = "uuid")]
    pub winner: Uuid,
    pub game_duration_seconds: u32,
    pub territories_captured: u32,
    pub total_battles: u32,
    pub final_score: u32,
}

/// Notification severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
    Success,
}
