# Strategy Game Backend

Rust backend for a real-time strategy game with WebSocket communication and OpenAPI type generation.

## Features

- ✅ Real-time game engine with configurable tick rate
- ✅ Procedural map generation with terrain types
- ✅ Combat system with terrain and building bonuses
- ✅ 5 AI personality types (Turtle, Aggressor, Balanced, Opportunist, Rusher)
- ✅ WebSocket server for real-time communication
- ✅ OpenAPI/utoipa schema generation for type-safe frontend
- ✅ Swagger UI documentation

## Architecture

### Core Types (`src/types/`)
- **entities.rs**: Game entities (Territory, Player, Buildings, etc.)
- **messages.rs**: WebSocket message types (ClientMessage, ServerMessage)

### Game Logic (`src/game/`)
- **state.rs**: Game state management and main game loop
- **combat.rs**: Combat resolution with formulas from design doc
- **map_gen.rs**: Procedural map generation
- **ai.rs**: AI decision-making for 5 personality types

### WebSocket (`src/websocket/`)
- **session.rs**: Game session management and broadcasting
- **handler.rs**: WebSocket connection handling

## Running the Server

```bash
# Development mode
cargo run

# Release mode
cargo run --release

# Run tests
cargo test
```

Server will start on `http://localhost:3000`

## API Endpoints

- **WebSocket**: `ws://localhost:3000/ws` - Real-time game communication
- **Swagger UI**: `http://localhost:3000/swagger-ui` - Interactive API documentation
- **OpenAPI Spec**: `http://localhost:3000/api-docs/openapi.json` - Type definitions

## Generating TypeScript Types

```bash
# Export OpenAPI spec
./export_openapi.sh

# Or manually:
# 1. Start server: cargo run
# 2. Download spec: curl http://localhost:3000/api-docs/openapi.json > ../frontend/src/generated/openapi.json
# 3. Generate types: cd ../frontend && npm run generate-types
```

## Game Configuration

Edit `src/main.rs` to configure:
- Territory count (default: 75)
- Player count (default: 9 - 1 human + 8 AI)
- Tick rate (default: 100ms)

## WebSocket Message Format

### Client → Server
```json
{
  "type": "attack",
  "from": "territory-id",
  "to": "territory-id"
}
```

### Server → Client
```json
{
  "type": "game_state_update",
  "state": { ... }
}
```

See `src/types/messages.rs` for all message types.

## Game Balance

Current parameters (from `docs/brief_expanded.md`):
- Starting: 1000 population, 500 gold
- Population growth: 10/sec per territory
- Combat formula: Variable based on troop ratio
- Building costs: City (1000g), Defense Post (500g), Gold Mine (750g)

## Dependencies

Key crates:
- `axum` - Web framework
- `tokio` - Async runtime
- `utoipa` - OpenAPI generation
- `serde` - Serialization
- `rand` - Random generation
