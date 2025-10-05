use rand::Rng;
use anyhow::Result;

use crate::types::*;
use super::GameEngine;

pub struct AIEngine;

impl AIEngine {
    /// Execute AI actions for all AI players
    pub fn tick_all(engine: &mut GameEngine) {
        let ai_players: Vec<_> = engine.state.players
            .iter()
            .filter(|p| p.is_ai && p.is_alive)
            .map(|p| (p.id, p.ai_personality.unwrap()))
            .collect();

        for (player_id, personality) in ai_players {
            Self::execute_ai_turn(engine, player_id.into(), personality);
        }
    }

    fn execute_ai_turn(engine: &mut GameEngine, player_id: PlayerId, personality: AIPersonality) {
        // Update ratios based on personality
        Self::update_ratios(engine, player_id, personality);

        // Decide whether to build
        if let Err(_) = Self::try_build(engine, player_id, personality) {
            // Building failed, that's ok
        }

        // Decide whether to attack
        if let Err(_) = Self::try_attack(engine, player_id, personality) {
            // Attack failed, that's ok
        }
    }

    fn update_ratios(engine: &mut GameEngine, player_id: PlayerId, personality: AIPersonality) {
        let player = match engine.get_player(player_id) {
            Ok(p) => p,
            Err(_) => return,
        };

        let game_time = engine.state.game_time_seconds;
        let territory_count = player.territories_controlled;

        let (troop_ratio, attack_ratio) = match personality {
            AIPersonality::Turtle => {
                // High workers early, transition to balanced
                let troop_ratio = if game_time < 180 {
                    0.3
                } else {
                    0.5
                };
                (troop_ratio, 0.15)
            }
            AIPersonality::Aggressor => {
                // Always high troops, aggressive attacks
                (0.7, 0.4)
            }
            AIPersonality::Balanced => {
                // Adjust based on territory count
                let troop_ratio = if territory_count < 5 {
                    0.5
                } else {
                    0.6
                };
                (troop_ratio, 0.25)
            }
            AIPersonality::Opportunist => {
                // Medium troops, lower attack ratio (pick battles carefully)
                (0.5, 0.2)
            }
            AIPersonality::Rusher => {
                // All troops, all the time
                (1.0, 0.5)
            }
        };

        let _ = engine.set_troop_ratio(player_id, troop_ratio);
        let _ = engine.set_attack_ratio(player_id, attack_ratio);
    }

    fn try_build(engine: &mut GameEngine, player_id: PlayerId, personality: AIPersonality) -> Result<()> {
        let mut rng = rand::thread_rng();

        let player = engine.get_player(player_id)?;
        let gold = player.gold;

        // Decide what to build based on personality
        let building_priority = match personality {
            AIPersonality::Turtle => vec![BuildingType::DefensePost, BuildingType::City, BuildingType::GoldMine],
            AIPersonality::Aggressor => vec![BuildingType::City, BuildingType::GoldMine, BuildingType::DefensePost],
            AIPersonality::Balanced => vec![BuildingType::GoldMine, BuildingType::City, BuildingType::DefensePost],
            AIPersonality::Opportunist => vec![BuildingType::GoldMine, BuildingType::DefensePost, BuildingType::City],
            AIPersonality::Rusher => vec![BuildingType::City, BuildingType::GoldMine, BuildingType::DefensePost],
        };

        // Find affordable building
        for building_type in building_priority {
            if gold >= building_type.cost() {
                // Find a territory without a building
                let territories: Vec<_> = engine.state.territories
                    .iter()
                    .filter(|t| t.owner == Some(player_id.into()) && t.building.is_none())
                    .map(|t| t.id)
                    .collect();

                if !territories.is_empty() {
                    let territory_id = territories[rng.gen_range(0..territories.len())];
                    return engine.build_structure(player_id, territory_id.into(), building_type);
                }
            }
        }

        Ok(())
    }

    fn try_attack(engine: &mut GameEngine, player_id: PlayerId, personality: AIPersonality) -> Result<()> {
        let mut rng = rand::thread_rng();

        // Find owned territories
        let owned_territories: Vec<_> = engine.state.territories
            .iter()
            .filter(|t| t.owner == Some(player_id.into()))
            .map(|t| (t.id, t.neighbors.clone()))
            .collect();

        if owned_territories.is_empty() {
            return Ok(());
        }

        // Build list of possible attacks
        let mut attack_options = Vec::new();

        for (territory_id, neighbors) in owned_territories {
            for neighbor_id in neighbors {
                let neighbor = engine.get_territory(neighbor_id.into())?;

                // Skip if we own it
                if neighbor.owner == Some(player_id.into()) {
                    continue;
                }

                // Get defender info
                if let Some(defender_id) = neighbor.owner {
                    let defender = engine.get_player(defender_id.into())?;
                    let defender_troops = neighbor.troops;

                    attack_options.push((
                        territory_id,
                        neighbor_id,
                        defender_troops,
                        defender.territories_controlled,
                    ));
                }
            }
        }

        if attack_options.is_empty() {
            return Ok(());
        }

        // Choose target based on personality
        let target = match personality {
            AIPersonality::Turtle => {
                // Rarely attack, only if heavily outnumber
                let player = engine.get_player(player_id)?;
                let our_troops = player.troops();

                attack_options
                    .iter()
                    .filter(|(_, _, defender_troops, _)| our_troops > *defender_troops * 3)
                    .min_by_key(|(_, _, troops, _)| *troops)
            }
            AIPersonality::Aggressor => {
                // Attack anyone, prefer weakest
                attack_options.iter().min_by_key(|(_, _, troops, _)| *troops)
            }
            AIPersonality::Balanced => {
                // Attack if we have advantage
                let player = engine.get_player(player_id)?;
                let our_troops = player.troops();

                attack_options
                    .iter()
                    .filter(|(_, _, defender_troops, _)| our_troops > *defender_troops)
                    .min_by_key(|(_, _, troops, _)| *troops)
            }
            AIPersonality::Opportunist => {
                // Attack weakest player
                attack_options
                    .iter()
                    .min_by_key(|(_, _, troops, territory_count)| (*troops, *territory_count))
            }
            AIPersonality::Rusher => {
                // Attack randomly, frequently
                attack_options.get(rng.gen_range(0..attack_options.len()))
            }
        };

        if let Some((from, to, _, _)) = target {
            // Execute with probability based on personality
            let attack_chance = match personality {
                AIPersonality::Turtle => 0.1,
                AIPersonality::Aggressor => 0.8,
                AIPersonality::Balanced => 0.4,
                AIPersonality::Opportunist => 0.5,
                AIPersonality::Rusher => 0.9,
            };

            if rng.gen::<f32>() < attack_chance {
                let _ = engine.execute_attack(player_id, (*from).into(), (*to).into());
            }
        }

        Ok(())
    }
}

impl GameEngine {
    /// Run AI logic for all AI players
    pub fn tick_ai(&mut self) {
        // Distribute troops for all players at beginning of each tick
        let all_player_ids: Vec<_> = self.state.players
            .iter()
            .filter(|p| p.is_alive)
            .map(|p| p.id)
            .collect();

        for player_id in all_player_ids {
            self.distribute_troops(player_id.into());
        }

        // Run AI decision making
        AIEngine::tick_all(self);
    }
}
