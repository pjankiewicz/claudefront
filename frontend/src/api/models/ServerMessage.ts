/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BuildingType } from './BuildingType';
import type { CombatResult } from './CombatResult';
import type { GameState } from './GameState';
import type { GameStats } from './GameStats';
import type { NotificationLevel } from './NotificationLevel';
/**
 * Messages sent from server to client
 */
export type ServerMessage = ({
    state: GameState;
    type: ServerMessage.type;
} | {
    result: CombatResult;
    type: ServerMessage.type;
} | {
    new_owner: string;
    old_owner: string | null;
    territory_id: string;
    type: ServerMessage.type;
} | {
    building_type: BuildingType;
    player_id: string;
    territory_id: string;
    type: ServerMessage.type;
} | {
    eliminated_by: string;
    player_id_test: string;
    type: ServerMessage.type;
} | {
    stats: GameStats;
    type: ServerMessage.type;
} | {
    message: string;
    severity: NotificationLevel;
    type: ServerMessage.type;
} | {
    message: string;
    type: ServerMessage.type;
});
export namespace ServerMessage {
    export enum type {
        GAME_STATE_UPDATE = 'game_state_update',
    }
}

