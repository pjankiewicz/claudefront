use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::types::*;
use super::GameEngine;

impl GameEngine {
    /// Execute an attack from one territory to another
    pub fn execute_attack(
        &mut self,
        attacker_id: PlayerId,
        from_territory: TerritoryId,
        to_territory: TerritoryId,
    ) -> Result<CombatResult> {
        // Validate attacker owns the from territory
        let from = self.get_territory(from_territory)?;
        if from.owner != Some(attacker_id.into()) {
            return Err(anyhow!("You don't own the attacking territory"));
        }

        // Validate territories are neighbors
        if !from.neighbors.contains(&to_territory.into()) {
            return Err(anyhow!("Territories are not neighbors"));
        }

        // Get defender
        let to = self.get_territory(to_territory)?;

        // Check if attacking own territory
        if to.owner == Some(Into::<Uuid>::into(attacker_id)) {
            return Err(anyhow!("Can't attack your own territory"));
        }

        let defender_id = to.owner; // Can be None for neutral territories

        // Calculate attacking force
        let attacker = self.get_player(attacker_id)?;
        let total_attacker_troops = attacker.troops();
        let attacker_troops = (total_attacker_troops as f32 * attacker.attack_ratio) as u32;

        if attacker_troops == 0 {
            return Err(anyhow!("No troops available to attack"));
        }

        // Get defender troops
        let defender_troops = to.troops;

        // Calculate combat result
        let (attacker_losses, defender_losses, territory_conquered) =
            self.calculate_combat(
                attacker_troops,
                defender_troops,
                to_territory,
            );

        // Apply losses to attacker
        let attacker = self.get_player_mut(attacker_id)?;
        attacker.population = attacker.population.saturating_sub(attacker_losses);

        // Apply losses to defender (if they have an owner)
        if let Some(defender_player_id) = defender_id {
            let defender = self.get_player_mut(defender_player_id.into())?;
            defender.population = defender.population.saturating_sub(defender_losses);
        }

        // Update territory
        let to = self.get_territory_mut(to_territory)?;

        if territory_conquered {
            to.owner = Some(attacker_id.into());
            to.troops = attacker_troops - attacker_losses;
        } else {
            to.troops = defender_troops.saturating_sub(defender_losses);
        }

        Ok(CombatResult {
            attacker_id: attacker_id.into(),
            defender_id: defender_id.unwrap_or(Uuid::nil()), // Use nil UUID for neutral
            from_territory: from_territory.into(),
            to_territory: to_territory.into(),
            attacker_troops_committed: attacker_troops,
            defender_troops,
            attacker_losses,
            defender_losses,
            territory_conquered,
        })
    }

    /// Calculate combat outcome based on troop counts and modifiers
    fn calculate_combat(
        &self,
        attacker_troops: u32,
        defender_troops: u32,
        defender_territory: TerritoryId,
    ) -> (u32, u32, bool) {
        // Get terrain and building bonuses
        let territory = self.get_territory(defender_territory).unwrap();
        let mut defense_multiplier = territory.terrain.defense_multiplier();

        if let Some(building) = territory.building {
            defense_multiplier *= building.defense_multiplier();
        }

        // Base combat formula from design doc
        let (base_attacker_losses, base_defender_losses) = if attacker_troops > defender_troops {
            // Attacker wins
            let attacker_losses = (defender_troops as f32 * 0.3) as u32;
            let defender_losses = defender_troops;
            (attacker_losses, defender_losses)
        } else if attacker_troops < defender_troops {
            // Defender wins
            let attacker_losses = attacker_troops;
            let defender_losses = (attacker_troops as f32 * 0.5) as u32;
            (attacker_losses, defender_losses)
        } else {
            // Equal forces
            let attacker_losses = (attacker_troops as f32 * 0.7) as u32;
            let defender_losses = (defender_troops as f32 * 0.7) as u32;
            (attacker_losses, defender_losses)
        };

        // Apply defense multiplier (reduces defender losses)
        let defender_losses = (base_defender_losses as f32 * defense_multiplier) as u32;
        let attacker_losses = base_attacker_losses;

        // Territory is conquered if defender loses all troops
        let territory_conquered = defender_troops <= defender_losses;

        (attacker_losses, defender_losses, territory_conquered)
    }

    /// Distribute troops across all player territories
    pub fn distribute_troops(&mut self, player_id: PlayerId) {
        let player = match self.get_player(player_id) {
            Ok(p) => p,
            Err(_) => return,
        };

        let total_troops = player.troops();
        let territory_count = player.territories_controlled;

        if territory_count == 0 {
            return;
        }

        let troops_per_territory = total_troops / territory_count;

        // Update all territories owned by this player
        for territory in &mut self.state.territories {
            if territory.owner == Some(player_id.into()) {
                territory.troops = troops_per_territory;
            }
        }
    }
}
