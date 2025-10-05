# Frontend SDK

Auto-generated TypeScript SDK from Rust backend OpenAPI specification.

## Overview

This project uses **type-safe code generation** to ensure the frontend and backend never drift apart:

1. **Rust Backend** → Defines types with `utoipa` annotations
2. **OpenAPI Spec** → Generated from Rust types at runtime
3. **TypeScript SDK** → Auto-generated from OpenAPI spec
4. **WebSocket Client** → Uses generated types for type safety

## Generating the SDK

### Quick Start

```bash
# Generate complete SDK (exports OpenAPI + generates TS types)
make sdk
```

This will:
1. Start the Rust backend temporarily
2. Download the OpenAPI spec from `http://localhost:3000/api-docs/openapi.json`
3. Stop the backend
4. Generate TypeScript types, models, and services in `src/sdk/`

### Individual Steps

```bash
# Export OpenAPI spec only
make export-openapi

# Generate SDK from existing OpenAPI spec
make generate-sdk
```

## Generated SDK Structure

After running `make sdk`, you'll have:

```
src/
├── api/              # Auto-generated (DO NOT EDIT)
│   ├── index.ts      # Main SDK exports
│   ├── models/       # TypeScript type definitions
│   │   ├── ClientMessage.ts
│   │   ├── ServerMessage.ts
│   │   ├── GameState.ts
│   │   ├── Territory.ts
│   │   ├── Player.ts
│   │   └── ...
│   ├── services/     # API service classes (for REST, if applicable)
│   └── core/         # SDK core utilities
│
└── client/           # Custom WebSocket client (manually maintained)
    ├── GameWebSocketClient.ts  # Type-safe WebSocket wrapper
    ├── useGameClient.tsx       # React hook
    └── index.ts                # Client exports
```

## Usage

### Basic WebSocket Client

```typescript
import { GameWebSocketClient } from './client';

const client = new GameWebSocketClient('ws://localhost:3000/ws');

// Register event handlers
client.on('game_state_update', (message) => {
  console.log('Game state:', message.state);
});

client.on('attack_result', (message) => {
  console.log('Attack result:', message.result);
});

// Connect and send actions
await client.connect();
client.attack(fromTerritoryId, toTerritoryId);
client.setTroopRatio(0.7);
client.buildStructure(territoryId, 'city');
```

### React Hook

```typescript
import { useGameClient } from './client';

function GameComponent() {
  const { client, gameState, isConnected, error } = useGameClient({
    autoConnect: true,
  });

  if (error) return <div>Error: {error.message}</div>;
  if (!isConnected) return <div>Connecting...</div>;
  if (!gameState) return <div>Loading...</div>;

  return (
    <div>
      <h1>Strategy Game</h1>
      <div>Tick: {gameState.tick}</div>
      <div>Players: {gameState.players.length}</div>
      <div>Territories: {gameState.territories.length}</div>

      <button onClick={() => client?.pauseGame()}>
        {gameState.is_paused ? 'Resume' : 'Pause'}
      </button>

      <input
        type="range"
        min="0"
        max="1"
        step="0.1"
        onChange={(e) => client?.setTroopRatio(parseFloat(e.target.value))}
      />
    </div>
  );
}
```

### Type-Safe Messages

All messages are fully typed:

```typescript
import type {
  ClientMessage,
  ServerMessage,
  GameState,
  Territory,
  Player,
} from './client';

// TypeScript knows the exact shape of each message
const attackMessage: ClientMessage = {
  type: 'attack',
  from: territoryId1,
  to: territoryId2,
};

// Discriminated unions for server messages
client.on('game_state_update', (message) => {
  // TypeScript knows `message` has `state: GameState`
  console.log(message.state.tick);
});

client.on('attack_result', (message) => {
  // TypeScript knows `message` has `result: CombatResult`
  console.log(message.result.territory_conquered);
});
```

## Type Safety Benefits

### Compile-Time Errors

If the backend types change:

```typescript
// ❌ TypeScript error if backend removes/renames a field
client.buildStructure(territoryId, 'invalid_building_type');

// ❌ TypeScript error if message structure changes
const message: ClientMessage = {
  type: 'attack',
  from: territoryId,
  // Missing required 'to' field - compile error!
};
```

### Auto-Complete

Your IDE will provide autocomplete for:
- All message types
- All fields in GameState, Territory, Player, etc.
- All enum values (BuildingType, TerrainType, AIPersonality)

### Refactoring Safety

Rename a field in Rust → regenerate SDK → TypeScript shows errors everywhere that needs updating!

## Development Workflow

### 1. Backend Changes

```bash
# In backend: modify types in src/types/
# Example: Add new field to Player struct

cd backend
cargo check  # Ensure it compiles
```

### 2. Regenerate SDK

```bash
# Export new OpenAPI spec + regenerate TypeScript SDK
make sdk
```

### 3. Fix Frontend

```bash
cd frontend
npm run dev

# TypeScript will show errors if anything broke
# Fix all type errors before continuing
```

## Troubleshooting

### "Cannot find module './api/models'"

**Solution**: You need to generate the SDK first.

```bash
make sdk
```

### Backend Server Won't Start

**Solution**: Check if port 3000 is already in use.

```bash
# Kill any existing process on port 3000
lsof -ti:3000 | xargs kill -9

# Or change the port in backend/src/main.rs
```

### Types Don't Match Backend

**Solution**: Regenerate the SDK.

```bash
make sdk
```

## CI/CD Integration

Add to your CI pipeline:

```yaml
# .github/workflows/ci.yml
- name: Generate SDK
  run: |
    cd backend && cargo build
    make export-openapi
    make generate-sdk

- name: Type Check Frontend
  run: |
    cd frontend
    npm install
    npm run build  # Will fail if types don't match
```

## Notes

- **Never edit files in `src/api/`** - they're auto-generated
- **Do edit `src/client/`** - custom WebSocket logic goes here
- Run `make sdk` after any backend type changes
- The SDK uses `openapi-typescript-codegen` with `--client fetch --useOptions` flags
- Generated SDK is in `src/api/` directory
- See `backend/README.md` for backend documentation
