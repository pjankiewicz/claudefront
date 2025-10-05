import { useEffect, useRef, useState, useCallback } from 'react';
import { GameWebSocketClient } from './GameWebSocketClient';
import type { GameState } from '../api';
import { ServerMessage } from '../api';

interface UseGameClientOptions {
  url?: string;
  autoConnect?: boolean;
  onConnect?: () => void;
  onDisconnect?: () => void;
  onError?: (error: Error) => void;
}

interface UseGameClientResult {
  client: GameWebSocketClient | null;
  gameState: GameState | null;
  isConnected: boolean;
  error: Error | null;
  connect: () => Promise<void>;
  disconnect: () => void;
}

/**
 * React hook for managing game client connection and state
 *
 * @example
 * ```tsx
 * function Game() {
 *   const { client, gameState, isConnected } = useGameClient({
 *     autoConnect: true,
 *   });
 *
 *   if (!isConnected) return <div>Connecting...</div>;
 *
 *   return (
 *     <div>
 *       <button onClick={() => client?.pauseGame()}>Pause</button>
 *       <div>Tick: {gameState?.tick}</div>
 *     </div>
 *   );
 * }
 * ```
 */
export function useGameClient(options: UseGameClientOptions = {}): UseGameClientResult {
  const {
    url = 'ws://localhost:3000/ws',
    autoConnect = false,
    onConnect,
    onDisconnect,
    onError,
  } = options;

  const [gameState, setGameState] = useState<GameState | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const clientRef = useRef<GameWebSocketClient | null>(null);

  useEffect(() => {
    // Initialize client
    const client = new GameWebSocketClient(url);

    // Register event handlers
    client.on(ServerMessage.type.GAME_STATE_UPDATE as any, (message: any) => {
      if ('state' in message) {
        setGameState(message.state);
      }
    });

    client.on('connect', () => {
      setIsConnected(true);
      setError(null);
      onConnect?.();
    });

    client.on('disconnect', () => {
      setIsConnected(false);
      onDisconnect?.();
    });

    client.on('error', (err) => {
      setError(err);
      onError?.(err);
    });

    clientRef.current = client;

    // Auto-connect if requested
    if (autoConnect) {
      client.connect().catch((err) => {
        setError(err);
        console.error('Failed to connect:', err);
      });
    }

    // Cleanup on unmount
    return () => {
      client.disconnect();
    };
  }, [url, autoConnect, onConnect, onDisconnect, onError]);

  const connect = useCallback(async () => {
    if (clientRef.current) {
      try {
        await clientRef.current.connect();
      } catch (err) {
        const error = err instanceof Error ? err : new Error('Connection failed');
        setError(error);
        throw error;
      }
    }
  }, []);

  const disconnect = useCallback(() => {
    clientRef.current?.disconnect();
  }, []);

  return {
    client: clientRef.current,
    gameState,
    isConnected,
    error,
    connect,
    disconnect,
  };
}
