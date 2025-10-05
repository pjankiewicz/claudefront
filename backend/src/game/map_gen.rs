use rand::Rng;
use uuid::Uuid;

use crate::types::*;

pub struct MapGenerator {
    pub territory_count: usize,
    pub player_count: usize,
}

impl MapGenerator {
    pub fn new(territory_count: usize, player_count: usize) -> Self {
        Self {
            territory_count,
            player_count,
        }
    }

    /// Generate a complete game with map and players
    pub fn generate(&self) -> GameState {
        let mut rng = rand::thread_rng();

        // Generate territories
        let mut territories = self.generate_territories(&mut rng);

        // Generate players
        let players = self.generate_players(&mut rng);

        // Assign starting territories to players
        self.assign_starting_territories(&mut territories, &players, &mut rng);

        GameState {
            territories,
            players,
            tick: 0,
            game_speed: 1.0,
            is_paused: false,
            game_time_seconds: 0,
        }
    }

    fn generate_territories(&self, rng: &mut impl Rng) -> Vec<Territory> {
        let mut territories = Vec::new();

        // Generate territories in a grid-like pattern for connectivity
        let grid_size = (self.territory_count as f32).sqrt().ceil() as usize;

        for i in 0..self.territory_count {
            let x = (i % grid_size) as f32 / grid_size as f32;
            let y = (i / grid_size) as f32 / grid_size as f32;

            // Add some randomness to positions
            let x = (x + rng.gen::<f32>() * 0.1 - 0.05).clamp(0.0, 1.0);
            let y = (y + rng.gen::<f32>() * 0.1 - 0.05).clamp(0.0, 1.0);

            let terrain = self.generate_terrain(x, y, rng);

            territories.push(Territory {
                id: Uuid::new_v4(),
                owner: None,
                terrain,
                building: None,
                troops: 0,
                neighbors: Vec::new(),
                position: (x, y),
            });
        }

        // Generate neighbors based on distance
        self.connect_territories(&mut territories);

        territories
    }

    fn generate_terrain(&self, _x: f32, _y: f32, rng: &mut impl Rng) -> TerrainType {
        let rand_val: f32 = rng.gen();

        // Terrain distribution: 40% Plains, 25% Mountains, 25% Forests, 10% Water
        if rand_val < 0.4 {
            TerrainType::Plains
        } else if rand_val < 0.65 {
            TerrainType::Mountains
        } else if rand_val < 0.9 {
            TerrainType::Forests
        } else {
            TerrainType::Water
        }
    }

    fn connect_territories(&self, territories: &mut [Territory]) {
        let n = territories.len();

        for i in 0..n {
            let pos_i = territories[i].position;
            let mut distances: Vec<(usize, f32)> = Vec::new();

            // Calculate distances to all other territories
            for j in 0..n {
                if i == j {
                    continue;
                }

                let pos_j = territories[j].position;
                let dx = pos_i.0 - pos_j.0;
                let dy = pos_i.1 - pos_j.1;
                let distance = (dx * dx + dy * dy).sqrt();

                distances.push((j, distance));
            }

            // Sort by distance
            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            // Connect to 3-6 nearest neighbors
            let neighbor_count = rand::thread_rng().gen_range(3..=6).min(distances.len());

            for (j, _) in distances.iter().take(neighbor_count) {
                territories[i].neighbors.push(territories[*j].id);
            }
        }

        // Ensure connectivity is bidirectional
        for i in 0..n {
            let neighbors: Vec<_> = territories[i].neighbors.clone();
            for neighbor_id in neighbors {
                if let Some(neighbor_idx) = territories.iter().position(|t| t.id == neighbor_id) {
                    if !territories[neighbor_idx].neighbors.contains(&territories[i].id) {
                        territories[neighbor_idx].neighbors.push(territories[i].id);
                    }
                }
            }
        }
    }

    fn generate_players(&self, rng: &mut impl Rng) -> Vec<Player> {
        let colors = vec![
            "#FF0000", "#00FF00", "#0000FF", "#FFFF00",
            "#FF00FF", "#00FFFF", "#FF8800", "#8800FF", "#00FF88",
        ];

        let ai_personalities = vec![
            AIPersonality::Turtle,
            AIPersonality::Aggressor,
            AIPersonality::Balanced,
            AIPersonality::Opportunist,
            AIPersonality::Rusher,
        ];

        let mut players = Vec::new();

        // First player is human
        players.push(Player {
            id: Uuid::new_v4(),
            name: "Player".to_string(),
            is_ai: false,
            ai_personality: None,
            color: colors[0].to_string(),
            population: 1000,
            max_population: 10_000,
            gold: 500,
            troop_ratio: 0.5,
            attack_ratio: 0.2,
            territories_controlled: 0,
            is_alive: true,
        });

        // Rest are AI
        for i in 1..self.player_count {
            let personality = ai_personalities[rng.gen_range(0..ai_personalities.len())];

            players.push(Player {
                id: Uuid::new_v4(),
                name: format!("AI {}", i),
                is_ai: true,
                ai_personality: Some(personality),
                color: colors[i % colors.len()].to_string(),
                population: 1000,
                max_population: 10_000,
                gold: 500,
                troop_ratio: match personality {
                    AIPersonality::Rusher => 1.0,
                    AIPersonality::Turtle => 0.3,
                    AIPersonality::Aggressor => 0.7,
                    _ => 0.5,
                },
                attack_ratio: 0.2,
                territories_controlled: 0,
                is_alive: true,
            });
        }

        players
    }

    fn assign_starting_territories(
        &self,
        territories: &mut [Territory],
        players: &[Player],
        rng: &mut impl Rng,
    ) {
        // Each player gets ONE starting territory
        // Find territories that are well-distributed across the map
        let territory_count = territories.len();
        let step = territory_count / players.len();

        for (i, player) in players.iter().enumerate() {
            // Pick a starting territory roughly evenly distributed
            let start_idx = (i * step + rng.gen_range(0..step.min(5))) % territory_count;

            territories[start_idx].owner = Some(player.id);
            // Start with 500 troops (half of starting population)
            territories[start_idx].troops = 500;
        }

        // All other territories remain neutral (owner = None)
        // They will have minimal troops for defense
        for territory in territories.iter_mut() {
            if territory.owner.is_none() {
                // Neutral territories have small defensive force
                territory.troops = rng.gen_range(50..150);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_generation() {
        let gen = MapGenerator::new(50, 5);
        let state = gen.generate();

        assert_eq!(state.territories.len(), 50);
        assert_eq!(state.players.len(), 5);
        assert_eq!(state.players[0].is_ai, false);

        // Check all territories have neighbors
        for territory in &state.territories {
            assert!(!territory.neighbors.is_empty());
            assert!(territory.neighbors.len() <= 6);
        }

        // Check starting territories assigned
        let owned_count = state.territories.iter().filter(|t| t.owner.is_some()).count();
        assert_eq!(owned_count, 5);
    }
}
