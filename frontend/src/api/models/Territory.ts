/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BuildingType } from './BuildingType';
import type { TerrainType } from './TerrainType';
/**
 * A territory on the map
 */
export type Territory = {
    building?: (null | BuildingType);
    id: string;
    /**
     * Neighboring territory IDs
     */
    neighbors: any[] | null;
    owner: string | null;
    /**
     * Visual position for rendering (x, y normalized 0-1)
     */
    position: any[];
    terrain: TerrainType;
    /**
     * Current troops stationed in this territory
     */
    troops: number;
};

