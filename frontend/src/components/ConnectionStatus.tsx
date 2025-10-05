import '../styles/ConnectionStatus.css';

interface ConnectionStatusProps {
  isConnected: boolean;
}

export function ConnectionStatus({ isConnected }: ConnectionStatusProps) {
  return (
    <div className="connection-status">
      <div className={`status-indicator ${isConnected ? 'connected' : 'disconnected'}`} />
      <span>{isConnected ? 'Connected' : 'Disconnected'}</span>
    </div>
  );
}
