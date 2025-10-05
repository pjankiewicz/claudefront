use std::collections::HashMap;
use anyhow::{anyhow, Result};

use crate::types::*;

pub struct GameEngine {
    pub state: GameState,
    territory_map: HashMap<TerritoryId, usize>,
    player_map: HashMap<PlayerId, usize>,
    pub tick_rate_ms: u64,
}

impl GameEngine {
    pub fn new(state: GameState, tick_rate_ms: u64) -> Self {
        let territory_map = state
            .territories
            .iter()
            .enumerate()
            .map(|(idx, t)| (t.id.into(), idx))
            .collect();

        let player_map = state
            .players
            .iter()
            .enumerate()
            .map(|(idx, p)| (p.id.into(), idx))
            .collect();

        Self {
            state,
            territory_map,
            player_map,
            tick_rate_ms,
        }
    }

    /// Update game state by one tick
    pub fn tick(&mut self) {
        if self.state.is_paused {
            return;
        }

        self.state.tick += 1;

        // Update game time based on speed
        let time_increment = (self.tick_rate_ms as f32 * self.state.game_speed) / 1000.0;
        self.state.game_time_seconds = (self.state.game_time_seconds as f32 + time_increment) as u32;

        // Update resources for all players
        self.update_resources();

        // Update territory control counts
        self.update_territory_counts();
    }

    /// Update population growth and gold generation
    fn update_resources(&mut self) {
        let tick_rate_sec = self.tick_rate_ms as f32 / 1000.0;

        // Collect player IDs first to avoid borrow issues
        let player_ids: Vec<PlayerId> = self.state.players
            .iter()
            .filter(|p| p.is_alive)
            .map(|p| p.id.into())
            .collect();

        for player_id in player_ids {
            let player = match self.get_player(player_id) {
                Ok(p) => p,
                Err(_) => continue,
            };

            let territories_controlled = player.territories_controlled;
            let workers = player.workers();

            // Population growth: 10/sec per territory + terrain bonuses
            let base_growth = 10.0 * territories_controlled as f32;
            let terrain_bonus = self.calculate_population_growth_bonus(player_id);
            let population_growth = (base_growth * terrain_bonus * tick_rate_sec * self.state.game_speed) as u32;

            // Gold generation: 1 gold per 10 workers per second + terrain/building bonuses
            let base_gold = workers as f32 / 10.0;
            let gold_bonus = self.calculate_gold_generation_bonus(player_id);
            let gold_generation = (base_gold * gold_bonus * tick_rate_sec * self.state.game_speed) as u32;

            // Apply updates
            if let Ok(player) = self.get_player_mut(player_id) {
                player.population = (player.population + population_growth).min(player.max_population);
                player.gold += gold_generation;
            }
        }
    }

    fn calculate_population_growth_bonus(&self, player_id: PlayerId) -> f32 {
        let mut total_multiplier = 1.0;
        let mut territory_count = 0;

        for territory in &self.state.territories {
            if territory.owner == Some(player_id.into()) {
                total_multiplier += territory.terrain.population_growth_multiplier();
                territory_count += 1;
            }
        }

        if territory_count > 0 {
            total_multiplier / territory_count as f32
        } else {
            1.0
        }
    }

    fn calculate_gold_generation_bonus(&self, player_id: PlayerId) -> f32 {
        let mut total_multiplier = 1.0;
        let mut territory_count = 0;

        for territory in &self.state.territories {
            if territory.owner == Some(player_id.into()) {
                let mut multiplier = territory.terrain.gold_multiplier();
                if let Some(building) = territory.building {
                    multiplier *= building.gold_multiplier();
                }
                total_multiplier += multiplier;
                territory_count += 1;
            }
        }

        if territory_count > 0 {
            total_multiplier / territory_count as f32
        } else {
            1.0
        }
    }

    fn update_territory_counts(&mut self) {
        // Reset all counts
        for player in &mut self.state.players {
            player.territories_controlled = 0;
        }

        // Count territories
        for territory in &self.state.territories {
            if let Some(owner_id) = territory.owner {
                if let Some(player_idx) = self.player_map.get(&owner_id.into()) {
                    self.state.players[*player_idx].territories_controlled += 1;
                }
            }
        }

        // Check for eliminated players
        for player in &mut self.state.players {
            if player.territories_controlled == 0 && player.is_alive {
                player.is_alive = false;
            }
        }
    }

    /// Get territory by ID
    pub fn get_territory(&self, id: TerritoryId) -> Result<&Territory> {
        let idx = self.territory_map.get(&id)
            .ok_or_else(|| anyhow!("Territory not found"))?;
        Ok(&self.state.territories[*idx])
    }

    /// Get mutable territory by ID
    pub fn get_territory_mut(&mut self, id: TerritoryId) -> Result<&mut Territory> {
        let idx = self.territory_map.get(&id)
            .ok_or_else(|| anyhow!("Territory not found"))?;
        Ok(&mut self.state.territories[*idx])
    }

    /// Get player by ID
    pub fn get_player(&self, id: PlayerId) -> Result<&Player> {
        let idx = self.player_map.get(&id)
            .ok_or_else(|| anyhow!("Player not found"))?;
        Ok(&self.state.players[*idx])
    }

    /// Get mutable player by ID
    pub fn get_player_mut(&mut self, id: PlayerId) -> Result<&mut Player> {
        let idx = self.player_map.get(&id)
            .ok_or_else(|| anyhow!("Player not found"))?;
        Ok(&mut self.state.players[*idx])
    }

    /// Set troop ratio for a player
    pub fn set_troop_ratio(&mut self, player_id: PlayerId, ratio: f32) -> Result<()> {
        let ratio = ratio.clamp(0.0, 1.0);
        let player = self.get_player_mut(player_id)?;
        player.troop_ratio = ratio;
        Ok(())
    }

    /// Set attack ratio for a player
    pub fn set_attack_ratio(&mut self, player_id: PlayerId, ratio: f32) -> Result<()> {
        let ratio = ratio.clamp(0.0, 1.0);
        let player = self.get_player_mut(player_id)?;
        player.attack_ratio = ratio;
        Ok(())
    }

    /// Build a structure in a territory
    pub fn build_structure(&mut self, player_id: PlayerId, territory_id: TerritoryId, building_type: BuildingType) -> Result<()> {
        // Validate ownership
        let territory = self.get_territory(territory_id)?;
        if territory.owner != Some(player_id.into()) {
            return Err(anyhow!("You don't own this territory"));
        }

        // Check if already has a building
        if territory.building.is_some() {
            return Err(anyhow!("Territory already has a building"));
        }

        // Check if player has enough gold
        let player = self.get_player(player_id)?;
        let cost = building_type.cost();
        if player.gold < cost {
            return Err(anyhow!("Not enough gold"));
        }

        // Deduct gold and build
        let player = self.get_player_mut(player_id)?;
        player.gold -= cost;

        // Add building bonuses
        if building_type == BuildingType::City {
            player.max_population += building_type.max_population_bonus();
        }

        let territory = self.get_territory_mut(territory_id)?;
        territory.building = Some(building_type);

        Ok(())
    }

    /// Check if game is over
    pub fn check_game_over(&self) -> Option<GameStats> {
        let alive_players: Vec<_> = self.state.players.iter()
            .filter(|p| p.is_alive)
            .collect();

        if alive_players.len() == 1 {
            let winner = alive_players[0];
            return Some(GameStats {
                winner: winner.id,
                game_duration_seconds: self.state.game_time_seconds,
                territories_captured: winner.territories_controlled,
                total_battles: 0, // TODO: track this
                final_score: winner.territories_controlled * 100 + winner.gold / 10,
            });
        }

        None
    }

    /// Pause/unpause the game
    pub fn set_paused(&mut self, paused: bool) {
        self.state.is_paused = paused;
    }

    /// Set game speed
    pub fn set_game_speed(&mut self, speed: f32) {
        self.state.game_speed = speed.clamp(0.5, 4.0);
    }
}
