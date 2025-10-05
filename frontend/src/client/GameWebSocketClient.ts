/**
 * WebSocket client for the strategy game
 * Uses auto-generated types from the OpenAPI SDK
 *
 * This file will be created after running: make sdk
 *
 * Usage:
 * ```typescript
 * import { GameWebSocketClient } from './client/GameWebSocketClient';
 * import type { ClientMessage, ServerMessage, GameState } from './sdk';
 *
 * const client = new GameWebSocketClient('ws://localhost:3000/ws');
 *
 * client.on('game_state_update', (state: GameState) => {
 *   console.log('Game state:', state);
 * });
 *
 * await client.connect();
 * client.attack(fromId, toId);
 * ```
 */

// Import types from generated SDK
// These will be available after running: make sdk
import type {
  ServerMessage,
  GameState,
} from '../api';
import { ClientMessage, BuildingType } from '../api';

type ServerMessageType = ServerMessage['type'];

type EventHandlers = {
  [K in ServerMessageType]?: (message: ServerMessage) => void;
} & {
  connect?: () => void;
  disconnect?: () => void;
  error?: (error: Error) => void;
};

export class GameWebSocketClient {
  private ws: WebSocket | null = null;
  private handlers: EventHandlers = {};
  private reconnectAttempts = 0;
  private readonly maxReconnectAttempts = 5;
  private readonly reconnectDelay = 1000;

  constructor(private url: string) {}

  /**
   * Connect to the game server
   */
  async connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        this.ws = new WebSocket(this.url);

        this.ws.onopen = () => {
          console.log('🎮 Connected to game server');
          this.reconnectAttempts = 0;
          this.handlers.connect?.();
          resolve();
        };

        this.ws.onmessage = (event) => {
          try {
            const message = JSON.parse(event.data) as ServerMessage;
            this.handleMessage(message);
          } catch (error) {
            console.error('Failed to parse message:', error);
          }
        };

        this.ws.onerror = (error) => {
          console.error('WebSocket error:', error);
          this.handlers.error?.(new Error('WebSocket error'));
          reject(error);
        };

        this.ws.onclose = () => {
          console.log('Disconnected from game server');
          this.handlers.disconnect?.();
          this.attemptReconnect();
        };
      } catch (error) {
        reject(error);
      }
    });
  }

  /**
   * Disconnect from the server
   */
  disconnect(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  /**
   * Register an event handler
   */
  on<K extends keyof EventHandlers>(event: K, handler: EventHandlers[K]): void {
    this.handlers[event] = handler;
  }

  /**
   * Unregister an event handler
   */
  off<K extends keyof EventHandlers>(event: K): void {
    delete this.handlers[event];
  }

  /**
   * Check if connected
   */
  isConnected(): boolean {
    return this.ws?.readyState === WebSocket.OPEN;
  }

  // ===== Private Methods =====

  private handleMessage(message: ServerMessage): void {
    const handler = this.handlers[message.type];
    if (handler) {
      (handler as any)(message);
    }
  }

  private attemptReconnect(): void {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      const delay = this.reconnectDelay * this.reconnectAttempts;

      console.log(`Reconnecting in ${delay}ms... (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);

      setTimeout(() => {
        this.connect().catch(console.error);
      }, delay);
    }
  }

  private send(message: ClientMessage): void {
    if (!this.isConnected()) {
      console.error('Cannot send message: not connected');
      return;
    }
    this.ws!.send(JSON.stringify(message));
  }

  // ===== Game Actions (Type-safe API) =====

  /**
   * Attack a neighboring territory
   */
  attack(from: string, to: string): void {
    this.send({
      type: ClientMessage.type.ATTACK,
      from,
      to,
    } as ClientMessage);
  }

  /**
   * Build a structure in a territory
   */
  buildStructure(territory: string, buildingType: BuildingType): void {
    this.send({
      type: 'build_structure' as any,
      territory,
      building_type: buildingType,
    } as ClientMessage);
  }

  /**
   * Set troop/worker ratio (0.0 - 1.0)
   */
  setTroopRatio(ratio: number): void {
    this.send({
      type: 'set_troop_ratio' as any,
      ratio: Math.max(0, Math.min(1, ratio)),
    } as ClientMessage);
  }

  /**
   * Set attack commitment ratio (0.0 - 1.0)
   */
  setAttackRatio(ratio: number): void {
    this.send({
      type: 'set_attack_ratio' as any,
      ratio: Math.max(0, Math.min(1, ratio)),
    } as ClientMessage);
  }

  /**
   * Pause the game
   */
  pauseGame(): void {
    this.send({ type: 'pause_game' as any } as ClientMessage);
  }

  /**
   * Resume the game
   */
  resumeGame(): void {
    this.send({ type: 'resume_game' as any } as ClientMessage);
  }

  /**
   * Set game speed multiplier
   */
  setGameSpeed(speed: number): void {
    this.send({
      type: 'set_game_speed' as any,
      speed,
    } as ClientMessage);
  }

  /**
   * Request current game state
   */
  requestGameState(): void {
    this.send({ type: 'get_game_state' as any } as ClientMessage);
  }
}

// Re-export types for convenience
export type {
  ClientMessage,
  ServerMessage,
  GameState,
  BuildingType,
};
