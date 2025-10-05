import { useGameClient } from './client/useGameClient';
import { GameMap } from './components/GameMap';
import { ControlPanel } from './components/ControlPanel';
import { PlayerStats } from './components/PlayerStats';
import { NotificationCenter } from './components/NotificationCenter';
import { ConnectionStatus } from './components/ConnectionStatus';
import './styles/App.css';

function App() {
  const { client, gameState, isConnected, error } = useGameClient({
    autoConnect: true,
  });

  if (error) {
    return (
      <div className="error-screen">
        <div className="error-container">
          <h1>Connection Error</h1>
          <p>{error.message}</p>
          <button onClick={() => window.location.reload()}>Retry</button>
        </div>
      </div>
    );
  }

  if (!isConnected || !gameState) {
    return (
      <div className="loading-screen">
        <div className="loading-spinner"></div>
        <p>Connecting to game server...</p>
      </div>
    );
  }

  const humanPlayer = gameState.players.find((p: any) => !p.is_ai);

  return (
    <div className="app">
      <header className="app-header">
        <h1>Strategy Game</h1>
        <ConnectionStatus isConnected={isConnected} />
      </header>

      <div className="game-layout">
        <aside className="left-sidebar">
          <PlayerStats players={gameState.players} />
        </aside>

        <main className="game-main">
          <GameMap
            gameState={gameState}
            client={client}
            humanPlayerId={humanPlayer?.id}
          />
        </main>

        <aside className="right-sidebar">
          <ControlPanel
            client={client}
            gameState={gameState}
            humanPlayer={humanPlayer}
          />
        </aside>
      </div>

      <NotificationCenter />
    </div>
  );
}

export default App;
