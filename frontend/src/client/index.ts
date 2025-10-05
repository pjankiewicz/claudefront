/**
 * Game Client Exports
 *
 * This module provides a type-safe WebSocket client for the strategy game,
 * using types generated from the OpenAPI specification.
 *
 * Run `make sdk` to generate the SDK types before using this client.
 */

export { GameWebSocketClient } from './GameWebSocketClient';
export { useGameClient } from './useGameClient';

// Re-export types for convenience
export type {
  ClientMessage,
  ServerMessage,
  GameState,
  BuildingType,
} from './GameWebSocketClient';
