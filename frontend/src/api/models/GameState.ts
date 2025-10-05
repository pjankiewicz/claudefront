/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Player } from './Player';
import type { Territory } from './Territory';
/**
 * Complete game state
 */
export type GameState = {
    game_speed: number;
    game_time_seconds: number;
    is_paused: boolean;
    players: Array<Player>;
    territories: Array<Territory>;
    tick: number;
};

