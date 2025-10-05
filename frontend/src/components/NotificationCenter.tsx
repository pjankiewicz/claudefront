import { useState, useEffect } from 'react';
import '../styles/NotificationCenter.css';

interface Notification {
  id: string;
  message: string;
  level: 'info' | 'warning' | 'critical';
  timestamp: number;
}

export function NotificationCenter() {
  const [notifications, setNotifications] = useState<Notification[]>([]);

  // Auto-remove notifications after 5 seconds
  useEffect(() => {
    const interval = setInterval(() => {
      setNotifications((prev) =>
        prev.filter((n) => Date.now() - n.timestamp < 5000)
      );
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  if (notifications.length === 0) return null;

  return (
    <div className="notification-center">
      {notifications.map((notification) => (
        <div
          key={notification.id}
          className={`notification ${notification.level}`}
        >
          <div className="notification-content">
            {notification.message}
          </div>
          <button
            className="notification-close"
            onClick={() => setNotifications((prev) => prev.filter((n) => n.id !== notification.id))}
          >
            ×
          </button>
        </div>
      ))}
    </div>
  );
}
