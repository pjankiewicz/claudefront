import type { Player } from '../api';
import '../styles/PlayerStats.css';

interface PlayerStatsProps {
  players: Player[];
}

export function PlayerStats({ players }: PlayerStatsProps) {
  const sortedPlayers = [...players].sort((a, b) =>
    b.territories_controlled - a.territories_controlled
  );

  return (
    <div className="player-stats">
      <h2>Players</h2>
      <div className="player-list">
        {sortedPlayers.map((player) => (
          <div
            key={player.id}
            className={`player-card ${!player.is_alive ? 'eliminated' : ''} ${!player.is_ai ? 'human' : ''}`}
          >
            <div className="player-header">
              <div
                className="player-color"
                style={{ backgroundColor: player.color }}
              />
              <div className="player-info">
                <span className="player-name">
                  {player.name} {!player.is_ai && '(You)'}
                </span>
                {player.ai_personality && (
                  <span className="player-personality">{player.ai_personality}</span>
                )}
              </div>
            </div>

            {player.is_alive ? (
              <div className="player-details">
                <div className="detail-row">
                  <span>🏰 Territories</span>
                  <span className="detail-value">{player.territories_controlled}</span>
                </div>
                <div className="detail-row">
                  <span>👥 Population</span>
                  <span className="detail-value">{Math.floor(player.population)}</span>
                </div>
                <div className="detail-row">
                  <span>⚔️ Army</span>
                  <span className="detail-value">{Math.floor(player.population * player.troop_ratio)}</span>
                </div>
                <div className="detail-row">
                  <span>💰 Gold</span>
                  <span className="detail-value">{Math.floor(player.gold)}</span>
                </div>
              </div>
            ) : (
              <div className="eliminated-label">ELIMINATED</div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
