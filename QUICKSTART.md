# Quick Start Guide

Get the strategy game backend and SDK running in 5 minutes.

## Prerequisites

- Rust (latest stable)
- Node.js (v18+)
- npm

## Setup

### 1. Generate the TypeScript SDK

```bash
# This will:
# - Start backend temporarily
# - Export OpenAPI spec
# - Generate TypeScript types/models/services
make sdk
```

Expected output:
```
Starting backend server...
Waiting for server to start...
Downloading OpenAPI spec...
Stopping backend server...
✅ OpenAPI spec exported to backend/openapi.json
Installing frontend dependencies...
Cleaning old SDK...
Generating TypeScript SDK (types, models, services) from OpenAPI spec...
✅ TypeScript SDK generated in frontend/src/api/
   - Types, models, and services are ready to use
✅ SDK generation complete!
```

### 2. Verify SDK Generation

Check that the SDK was generated:

```bash
ls -la frontend/src/api/
```

You should see:
```
frontend/src/api/
├── index.ts
├── models/
│   ├── AIPersonality.ts
│   ├── BuildingType.ts
│   ├── ClientMessage.ts
│   ├── ServerMessage.ts
│   ├── GameState.ts
│   └── ...
├── services/
└── core/
```

## Running

### Backend Only

```bash
make backend
```

Server will start on `http://localhost:3000`
- WebSocket: `ws://localhost:3000/ws`
- Swagger UI: `http://localhost:3000/swagger-ui`

### Frontend Only

```bash
make frontend
```

Dev server will start on `http://localhost:5173`

### Both (Development Mode)

```bash
make dev
```

Runs backend and frontend in parallel.

## Using the SDK

### Example: WebSocket Client

Create `frontend/src/App.tsx`:

```typescript
import { useGameClient } from './client';

function App() {
  const { client, gameState, isConnected, error } = useGameClient({
    autoConnect: true,
  });

  if (error) {
    return <div>Error: {error.message}</div>;
  }

  if (!isConnected) {
    return <div>Connecting to game server...</div>;
  }

  if (!gameState) {
    return <div>Loading game state...</div>;
  }

  const humanPlayer = gameState.players.find(p => !p.is_ai);

  return (
    <div style={{ padding: '20px', fontFamily: 'monospace' }}>
      <h1>🎮 Strategy Game</h1>

      <div style={{ marginBottom: '20px' }}>
        <strong>Game Status:</strong>
        <div>Tick: {gameState.tick}</div>
        <div>Time: {gameState.game_time_seconds}s</div>
        <div>Speed: {gameState.game_speed}x</div>
        <div>Paused: {gameState.is_paused ? 'Yes' : 'No'}</div>
      </div>

      <div style={{ marginBottom: '20px' }}>
        <strong>Your Status:</strong>
        {humanPlayer && (
          <>
            <div>Population: {humanPlayer.population} / {humanPlayer.max_population}</div>
            <div>Gold: {humanPlayer.gold}</div>
            <div>Territories: {humanPlayer.territories_controlled}</div>
            <div>Troops: {Math.floor(humanPlayer.population * humanPlayer.troop_ratio)}</div>
            <div>Workers: {Math.floor(humanPlayer.population * (1 - humanPlayer.troop_ratio))}</div>
          </>
        )}
      </div>

      <div style={{ marginBottom: '20px' }}>
        <strong>Controls:</strong>
        <div>
          <button onClick={() => client?.pauseGame()}>
            {gameState.is_paused ? 'Resume' : 'Pause'}
          </button>
          <button onClick={() => client?.setGameSpeed(1)}>1x</button>
          <button onClick={() => client?.setGameSpeed(2)}>2x</button>
          <button onClick={() => client?.setGameSpeed(4)}>4x</button>
        </div>
      </div>

      <div style={{ marginBottom: '20px' }}>
        <strong>Troop Ratio:</strong>
        <input
          type="range"
          min="0"
          max="1"
          step="0.1"
          value={humanPlayer?.troop_ratio || 0.5}
          onChange={(e) => client?.setTroopRatio(parseFloat(e.target.value))}
        />
        <span>{Math.round((humanPlayer?.troop_ratio || 0.5) * 100)}%</span>
      </div>

      <div>
        <strong>Players:</strong>
        {gameState.players.map(player => (
          <div key={player.id} style={{
            padding: '5px',
            margin: '5px 0',
            background: player.is_ai ? '#f0f0f0' : '#e0ffe0',
            border: `2px solid ${player.color}`
          }}>
            {player.name} - Pop: {player.population} - Territories: {player.territories_controlled}
            {!player.is_alive && ' (Eliminated)'}
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
```

### Run the Example

```bash
# Terminal 1: Start backend
make backend

# Terminal 2: Start frontend
make frontend
```

Open `http://localhost:5173` in your browser.

## Available Commands

```bash
make help              # Show all commands
make backend           # Run backend server
make frontend          # Run frontend dev server
make export-openapi    # Export OpenAPI spec
make generate-sdk      # Generate TypeScript SDK
make sdk              # Export + Generate (complete)
make dev              # Run backend + frontend
make clean            # Clean build artifacts
```

## What's Included

### Backend (Rust)
- ✅ Game engine with 100ms tick rate
- ✅ Procedural map generation
- ✅ Combat system with terrain bonuses
- ✅ 5 AI personality types
- ✅ WebSocket server
- ✅ OpenAPI/utoipa documentation

### Frontend (TypeScript)
- ✅ Auto-generated types from backend
- ✅ Type-safe WebSocket client
- ✅ React hooks for easy integration
- ✅ Full type safety (no runtime surprises!)

## Next Steps

1. **Explore the API**: Visit `http://localhost:3000/swagger-ui`
2. **Read the docs**: Check `frontend/SDK_README.md` for detailed usage
3. **Build UI**: Create React components using `useGameClient` hook
4. **Customize**: Modify game logic in `backend/src/game/`

## Troubleshooting

**Port 3000 already in use:**
```bash
lsof -ti:3000 | xargs kill -9
```

**SDK not found:**
```bash
make sdk
```

**Frontend won't compile:**
```bash
cd frontend
rm -rf node_modules package-lock.json
npm install
```

**Backend won't compile:**
```bash
cd backend
cargo clean
cargo build
```

## Documentation

- **Backend**: `backend/README.md`
- **SDK Usage**: `frontend/SDK_README.md`
- **Game Design**: `docs/brief_expanded.md`
- **Full Summary**: `SDK_SUMMARY.md`

## Success! 🎉

You now have:
- ✅ Rust backend running with game logic
- ✅ TypeScript SDK auto-generated from backend
- ✅ Type-safe WebSocket client
- ✅ React hooks ready to use
- ✅ Full compile-time type safety

Start building your game UI!
