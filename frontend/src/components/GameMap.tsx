import { useState, useRef, useEffect, useMemo } from 'react';
import type { GameState, Territory } from '../api';
import type { GameWebSocketClient } from '../client/GameWebSocketClient';
import '../styles/GameMap.css';

interface GameMapProps {
  gameState: GameState;
  client: GameWebSocketClient | null;
  humanPlayerId?: string;
}

interface VoronoiCell {
  territory: Territory;
  polygon: [number, number][];
}

// Simple Voronoi diagram generator using Fortune's algorithm (simplified)
function generateVoronoiCells(territories: Territory[]): VoronoiCell[] {
  const cells: VoronoiCell[] = [];
  const width = 1200;
  const height = 800;

  // For each territory, generate a polygon using pixel-based Voronoi
  for (const territory of territories) {
    const polygon: [number, number][] = [];
    const centerX = territory.position[0] * width;
    const centerY = territory.position[1] * height;

    // Generate polygon points in a circle around the territory
    const numPoints = 32; // More points = smoother polygons
    const baseRadius = 60; // Base radius for polygons

    for (let i = 0; i < numPoints; i++) {
      const angle = (i / numPoints) * Math.PI * 2;
      let radius = baseRadius;

      // Sample points outward to find Voronoi boundary
      for (let r = 20; r < 150; r += 5) {
        const testX = centerX + Math.cos(angle) * r;
        const testY = centerY + Math.sin(angle) * r;

        // Check if this point is closer to another territory
        let closestDist = Infinity;
        let closestTerritory = territory;

        for (const other of territories) {
          const otherX = other.position[0] * width;
          const otherY = other.position[1] * height;
          const dist = Math.sqrt((testX - otherX) ** 2 + (testY - otherY) ** 2);

          if (dist < closestDist) {
            closestDist = dist;
            closestTerritory = other;
          }
        }

        if (closestTerritory.id !== territory.id) {
          radius = r;
          break;
        }
      }

      const x = centerX + Math.cos(angle) * radius;
      const y = centerY + Math.sin(angle) * radius;
      polygon.push([x, y]);
    }

    cells.push({ territory, polygon });
  }

  return cells;
}

export function GameMap({ gameState, client, humanPlayerId }: GameMapProps) {
  const [selectedTerritory, setSelectedTerritory] = useState<Territory | null>(null);
  const [hoveredTerritory, setHoveredTerritory] = useState<Territory | null>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  // Generate Voronoi polygons for territories
  const voronoiCells = useMemo(() => {
    return generateVoronoiCells(gameState.territories);
  }, [gameState.territories]);

  // Find player colors
  const getPlayerColor = (playerId: string | null) => {
    if (!playerId) return '#1a1a1a'; // Neutral territories - dark gray
    const player = gameState.players.find((p) => p.id === playerId);
    return player?.color || '#2a2a2a';
  };

  // Get terrain color
  const getTerrainColor = (terrain: Territory['terrain']) => {
    switch (terrain) {
      case 'plains':
        return '#7cb342';
      case 'mountains':
        return '#757575';
      case 'forests':
        return '#388e3c';
      case 'water':
        return '#1976d2';
      default:
        return '#424242';
    }
  };

  // Handle territory click
  const handleTerritoryClick = (territory: Territory) => {
    if (!client || !humanPlayerId) return;

    console.log('Territory clicked:', territory.id, 'Owner:', territory.owner, 'Human:', humanPlayerId);

    // If clicking on own territory, select it
    if (territory.owner === humanPlayerId) {
      setSelectedTerritory(territory);
      console.log('Selected own territory:', territory.id);
      return;
    }

    // If we have a selected territory, try to attack (can attack neutral or enemy territories)
    if (selectedTerritory && selectedTerritory.owner === humanPlayerId) {
      // Check if territories are neighbors
      const isNeighbor = selectedTerritory.neighbors?.includes(territory.id);
      console.log('Attacking from', selectedTerritory.id, 'to', territory.id, 'IsNeighbor:', isNeighbor);

      // Can attack if it's a neighbor and not your own territory
      if (isNeighbor && territory.owner !== humanPlayerId) {
        console.log('Sending attack command...');
        client.attack(selectedTerritory.id, territory.id);
        setSelectedTerritory(null);
      } else {
        console.log('Cannot attack: not a neighbor or own territory');
      }
    }
  };

  // Check if point is inside polygon
  const isPointInPolygon = (x: number, y: number, polygon: [number, number][]) => {
    let inside = false;
    for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
      const xi = polygon[i][0];
      const yi = polygon[i][1];
      const xj = polygon[j][0];
      const yj = polygon[j][1];

      const intersect =
        yi > y !== yj > y && x < ((xj - xi) * (y - yi)) / (yj - yi) + xi;
      if (intersect) inside = !inside;
    }
    return inside;
  };

  // Draw the map
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear canvas
    ctx.fillStyle = '#0a0e27';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw territory connection lines (optional - can be removed)
    // Commenting out to avoid clutter
    /*
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.1)';
    ctx.lineWidth = 1;
    const drawnConnections = new Set<string>();

    for (const cell of voronoiCells) {
      const { territory } = cell;

      territory.neighbors?.forEach((neighborId) => {
        // Only draw each connection once
        const connectionKey = [territory.id, neighborId].sort().join('-');
        if (drawnConnections.has(connectionKey)) return;
        drawnConnections.add(connectionKey);

        const neighborCell = voronoiCells.find((c) => c.territory.id === neighborId);
        if (neighborCell) {
          const centerX = territory.position[0] * canvas.width;
          const centerY = territory.position[1] * canvas.height;
          const neighborX = neighborCell.territory.position[0] * canvas.width;
          const neighborY = neighborCell.territory.position[1] * canvas.height;

          ctx.beginPath();
          ctx.moveTo(centerX, centerY);
          ctx.lineTo(neighborX, neighborY);
          ctx.stroke();
        }
      });
    }
    */

    // PASS 1: Draw territory fills
    for (const cell of voronoiCells) {
      const { territory, polygon } = cell;

      // Draw polygon
      ctx.beginPath();
      ctx.moveTo(polygon[0][0], polygon[0][1]);
      for (let i = 1; i < polygon.length; i++) {
        ctx.lineTo(polygon[i][0], polygon[i][1]);
      }
      ctx.closePath();

      // Fill with player color
      const playerColor = getPlayerColor(territory.owner);
      ctx.fillStyle = playerColor;
      ctx.fill();

      // Draw terrain texture overlay
      ctx.globalAlpha = 0.3;
      ctx.fillStyle = getTerrainColor(territory.terrain);
      ctx.fill();
      ctx.globalAlpha = 1.0;
    }

    // PASS 2: Draw borders between different owners (shared borders)
    const drawnBorders = new Set<string>();

    for (const cell of voronoiCells) {
      const { territory, polygon } = cell;

      // For each edge of the polygon, check if it's a border with another owner
      for (let i = 0; i < polygon.length; i++) {
        const nextI = (i + 1) % polygon.length;
        const p1 = polygon[i];
        const p2 = polygon[nextI];

        // Check if this edge is shared with a neighbor of different owner
        const edgeKey = [p1, p2].sort().join('-');
        if (drawnBorders.has(edgeKey)) continue;

        territory.neighbors?.forEach((neighborId) => {
          const neighbor = voronoiCells.find((c) => c.territory.id === neighborId)?.territory;
          if (neighbor && neighbor.owner !== territory.owner) {
            // Draw border line between different owners
            ctx.beginPath();
            ctx.moveTo(p1[0], p1[1]);
            ctx.lineTo(p2[0], p2[1]);
            ctx.strokeStyle = 'rgba(255, 255, 255, 0.6)';
            ctx.lineWidth = 2;
            ctx.stroke();
            drawnBorders.add(edgeKey);
          }
        });
      }
    }

    // PASS 2b: Draw special highlighting for selected/hovered/player-owned
    for (const cell of voronoiCells) {
      const { territory, polygon } = cell;
      const isSelected = selectedTerritory?.id === territory.id;
      const isHovered = hoveredTerritory?.id === territory.id;
      const isPlayerOwned = territory.owner === humanPlayerId;

      // Only draw special border if selected, hovered, or player-owned
      if (isSelected || isHovered || isPlayerOwned) {
        ctx.beginPath();
        ctx.moveTo(polygon[0][0], polygon[0][1]);
        for (let i = 1; i < polygon.length; i++) {
          ctx.lineTo(polygon[i][0], polygon[i][1]);
        }
        ctx.closePath();

        if (isSelected) {
          ctx.strokeStyle = '#ffd700';
          ctx.lineWidth = 5;
          ctx.shadowBlur = 20;
          ctx.shadowColor = '#ffd700';
        } else if (isHovered) {
          ctx.strokeStyle = '#ffffff';
          ctx.lineWidth = 4;
          ctx.shadowBlur = 15;
          ctx.shadowColor = '#ffffff';
        } else if (isPlayerOwned) {
          ctx.strokeStyle = 'rgba(102, 126, 234, 0.8)';
          ctx.lineWidth = 3;
          ctx.shadowBlur = 0;
        }

        ctx.stroke();
        ctx.shadowBlur = 0;
      }
    }

    // PASS 3: Draw text and icons on top of everything
    for (const cell of voronoiCells) {
      const { territory } = cell;
      const centerX = territory.position[0] * canvas.width;
      const centerY = territory.position[1] * canvas.height;

      // Draw troop count
      ctx.fillStyle = '#ffffff';
      ctx.font = 'bold 18px Arial';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.shadowColor = 'rgba(0, 0, 0, 0.8)';
      ctx.shadowBlur = 4;
      ctx.fillText(Math.floor(territory.troops).toString(), centerX, centerY);
      ctx.shadowBlur = 0;

      // Draw building indicator
      if (territory.building) {
        ctx.fillStyle = '#ffd700';
        ctx.font = 'bold 24px Arial';
        ctx.shadowColor = 'rgba(0, 0, 0, 0.8)';
        ctx.shadowBlur = 4;
        ctx.fillText('⭐', centerX, centerY - 25);
        ctx.shadowBlur = 0;
      }
    }
  }, [gameState, selectedTerritory, hoveredTerritory, humanPlayerId, voronoiCells]);

  // Handle mouse move for hover effect
  const handleMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    // Account for canvas scaling
    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;
    const x = (e.clientX - rect.left) * scaleX;
    const y = (e.clientY - rect.top) * scaleY;

    // Find territory under cursor using polygon containment
    const cell = voronoiCells.find((c) => isPointInPolygon(x, y, c.polygon));
    setHoveredTerritory(cell?.territory || null);
  };

  const handleCanvasClick = () => {
    if (hoveredTerritory) {
      handleTerritoryClick(hoveredTerritory);
    } else {
      setSelectedTerritory(null);
    }
  };

  return (
    <div className="game-map">
      <canvas
        ref={canvasRef}
        width={1200}
        height={800}
        onMouseMove={handleMouseMove}
        onClick={handleCanvasClick}
        style={{ cursor: hoveredTerritory ? 'pointer' : 'default' }}
      />

      {hoveredTerritory && (
        <div className="territory-tooltip">
          <h3>Territory {hoveredTerritory.id.substring(0, 8)}</h3>
          <p>Owner: {hoveredTerritory.owner ? gameState.players.find(p => p.id === hoveredTerritory.owner)?.name : 'Neutral'}</p>
          <p>Terrain: {hoveredTerritory.terrain}</p>
          <p>Troops: {Math.floor(hoveredTerritory.troops)}</p>
          {hoveredTerritory.building && <p>Building: {hoveredTerritory.building}</p>}
        </div>
      )}

      {selectedTerritory && (
        <div className="selected-info">
          <h3>Selected: {selectedTerritory.id.substring(0, 8)}</h3>
          <p>Click a neighboring territory to attack!</p>
          <button onClick={() => setSelectedTerritory(null)}>Cancel</button>
        </div>
      )}
    </div>
  );
}
