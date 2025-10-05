# Strategy Game - Frontend

Beautiful, modern UI for the strategy game built with React, TypeScript, and Canvas API.

## Features

- **Real-time Game Map**: Interactive canvas-based map with territory visualization
- **Type-Safe WebSocket Client**: Auto-generated types from Rust backend via OpenAPI
- **Responsive Control Panel**: Adjust troop ratios, attack intensity, and game speed
- **Live Player Statistics**: Track all players, their territories, and resources
- **Notification System**: Real-time game events and alerts
- **Modern UI Design**: Dark theme with gradient accents and smooth animations

## Tech Stack

- **React 18** with hooks
- **TypeScript** for type safety
- **Vite** for fast development and building
- **Canvas API** for map rendering
- **WebSocket** for real-time communication
- **CSS3** for modern styling

## Project Structure

```
frontend/
├── src/
│   ├── api/              # Auto-generated SDK types from backend
│   ├── client/           # WebSocket client and React hook
│   ├── components/       # UI components
│   │   ├── GameMap.tsx
│   │   ├── ControlPanel.tsx
│   │   ├── PlayerStats.tsx
│   │   ├── NotificationCenter.tsx
│   │   └── ConnectionStatus.tsx
│   ├── styles/           # Component-specific CSS
│   ├── App.tsx           # Main application component
│   └── main.tsx          # Entry point
├── index.html
└── package.json
```

## Getting Started

### Prerequisites

- Node.js 18+ and npm
- Backend server running on `http://localhost:3000`

### Installation

```bash
cd frontend
npm install
```

### Generate SDK Types

```bash
npm run generate-sdk
```

This reads `../backend/openapi.json` and generates TypeScript types in `src/api/`.

### Development

```bash
npm run dev
```

Opens at `http://localhost:5173`

### Build for Production

```bash
npm run build
```

Output in `dist/` folder.

## Components Overview

### GameMap

Interactive canvas-based map that renders:
- Territories as circles with owner colors
- Terrain types as inner circle colors
- Troop counts as text
- Building indicators (stars)
- Connections between neighboring territories

**Interactions:**
- Click your territory to select it
- Click a neighboring territory to attack
- Hover to see detailed information

### ControlPanel

Right sidebar with controls:
- **Empire Stats**: Gold, income, population, territories
- **Army Composition Slider**: Adjust troops vs. workers ratio
- **Attack Intensity Slider**: Control percentage of troops committed per attack
- **Game Controls**: Pause/resume and speed multiplier (1x, 2x, 4x)
- **Game Timer**: Current game time

### PlayerStats

Left sidebar showing all players:
- Player color and name
- AI personality type
- Territory count, population, army size, gold
- Visual indication of eliminated players
- Highlight for human player

### NotificationCenter

Top-right overlay for game events:
- Info notifications (territory captured)
- Warning notifications (under attack)
- Critical notifications (player eliminated)
- Auto-dismiss after 5 seconds

### ConnectionStatus

Header component showing WebSocket connection status with animated indicator.

## WebSocket Client API

The `GameWebSocketClient` provides type-safe methods for game actions:

```typescript
import { useGameClient } from './client/useGameClient';

function MyComponent() {
  const { client, gameState, isConnected } = useGameClient({
    autoConnect: true,
  });

  // Attack territory
  client?.attack('territory_1', 'territory_2');

  // Build structure
  client?.buildStructure('territory_1', BuildingType.CITY);

  // Adjust ratios
  client?.setTroopRatio(0.7); // 70% troops, 30% workers
  client?.setAttackRatio(0.5); // Use 50% of troops per attack

  // Game controls
  client?.pauseGame();
  client?.resumeGame();
  client?.setGameSpeed(2); // 2x speed

  return <div>...</div>;
}
```

## Styling

The UI uses a modern dark theme with:
- Custom CSS variables for consistent colors
- Gradient accents (purple/blue)
- Smooth transitions and animations
- Responsive layout with CSS Grid
- Custom scrollbars

### Color Palette

- **Background**: `#0a0e27` (primary), `#1a1f3a` (secondary)
- **Accent**: `#667eea` (purple/blue gradient)
- **Success**: `#48bb78` (green)
- **Warning**: `#ed8936` (orange)
- **Danger**: `#f56565` (red)

## Type Safety

The frontend is fully type-safe thanks to auto-generated types from the Rust backend:

1. Backend defines types with `utoipa` annotations
2. Backend generates `openapi.json`
3. Frontend runs `npm run generate-sdk`
4. TypeScript types generated in `src/api/`
5. Components import and use these types

Any changes to backend types automatically propagate to frontend after regenerating SDK.

## Performance

- Canvas rendering optimized for 50-100 territories
- React components use proper memoization
- WebSocket messages are typed and validated
- CSS animations use GPU acceleration (transform, opacity)

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+

Requires WebSocket and Canvas API support.
