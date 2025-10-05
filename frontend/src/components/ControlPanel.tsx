import { useState } from 'react';
import type { GameState, Player } from '../api';
import type { GameWebSocketClient } from '../client/GameWebSocketClient';
import '../styles/ControlPanel.css';

interface ControlPanelProps {
  client: GameWebSocketClient | null;
  gameState: GameState;
  humanPlayer?: Player;
}

export function ControlPanel({ client, gameState, humanPlayer }: ControlPanelProps) {
  const [troopRatio, setTroopRatio] = useState(humanPlayer?.troop_ratio || 0.5);
  const [attackRatio, setAttackRatio] = useState(humanPlayer?.attack_ratio || 0.5);

  if (!humanPlayer || !client) {
    return <div className="control-panel">No player data</div>;
  }

  const handleTroopRatioChange = (value: number) => {
    setTroopRatio(value);
    client.setTroopRatio(value);
  };

  const handleAttackRatioChange = (value: number) => {
    setAttackRatio(value);
    client.setAttackRatio(value);
  };

  const handleSpeedChange = (speed: number) => {
    client.setGameSpeed(speed);
  };

  const togglePause = () => {
    if (gameState.is_paused) {
      client.resumeGame();
    } else {
      client.pauseGame();
    }
  };

  const currentTroops = Math.floor(humanPlayer.population * troopRatio);
  const currentWorkers = Math.floor(humanPlayer.population * (1 - troopRatio));
  const goldPerSecond = Math.floor(currentWorkers / 10);

  return (
    <div className="control-panel">
      <div className="panel-section">
        <h2>Your Empire</h2>
        <div className="stat-grid">
          <div className="stat-item">
            <span className="stat-label">Gold</span>
            <span className="stat-value gold">{Math.floor(humanPlayer.gold)}</span>
          </div>
          <div className="stat-item">
            <span className="stat-label">Income</span>
            <span className="stat-value">{goldPerSecond}/s</span>
          </div>
          <div className="stat-item">
            <span className="stat-label">Population</span>
            <span className="stat-value">{Math.floor(humanPlayer.population)}/{humanPlayer.max_population}</span>
          </div>
          <div className="stat-item">
            <span className="stat-label">Territories</span>
            <span className="stat-value">{humanPlayer.territories_controlled}</span>
          </div>
        </div>
      </div>

      <div className="panel-section">
        <h3>Army Composition</h3>
        <div className="slider-container">
          <div className="slider-header">
            <span>Workers: {currentWorkers}</span>
            <span>Troops: {currentTroops}</span>
          </div>
          <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={troopRatio}
            onChange={(e) => handleTroopRatioChange(parseFloat(e.target.value))}
            className="ratio-slider"
          />
          <div className="slider-bar">
            <div className="slider-fill workers" style={{ width: `${(1 - troopRatio) * 100}%` }} />
            <div className="slider-fill troops" style={{ width: `${troopRatio * 100}%` }} />
          </div>
        </div>
      </div>

      <div className="panel-section">
        <h3>Attack Intensity</h3>
        <div className="slider-container">
          <div className="slider-header">
            <span>{Math.floor(attackRatio * 100)}% of troops</span>
          </div>
          <input
            type="range"
            min="0"
            max="1"
            step="0.05"
            value={attackRatio}
            onChange={(e) => handleAttackRatioChange(parseFloat(e.target.value))}
            className="ratio-slider"
          />
          <div className="slider-bar">
            <div className="slider-fill attack" style={{ width: `${attackRatio * 100}%` }} />
          </div>
        </div>
      </div>

      <div className="panel-section">
        <h3>Game Controls</h3>
        <div className="button-group">
          <button onClick={togglePause} className="control-button primary">
            {gameState.is_paused ? '▶️ Resume' : '⏸️ Pause'}
          </button>
        </div>
        <div className="speed-controls">
          <span>Speed:</span>
          <button
            onClick={() => handleSpeedChange(1)}
            className={`speed-button ${gameState.game_speed === 1 ? 'active' : ''}`}
          >
            1x
          </button>
          <button
            onClick={() => handleSpeedChange(2)}
            className={`speed-button ${gameState.game_speed === 2 ? 'active' : ''}`}
          >
            2x
          </button>
          <button
            onClick={() => handleSpeedChange(4)}
            className={`speed-button ${gameState.game_speed === 4 ? 'active' : ''}`}
          >
            4x
          </button>
        </div>
      </div>

      <div className="panel-section">
        <h3>Game Time</h3>
        <div className="game-time">
          {Math.floor(gameState.game_time_seconds / 60)}:{String(Math.floor(gameState.game_time_seconds % 60)).padStart(2, '0')}
        </div>
      </div>
    </div>
  );
}
