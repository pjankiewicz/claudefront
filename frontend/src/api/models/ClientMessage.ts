/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BuildingType } from './BuildingType';
/**
 * Messages sent from client to server
 */
export type ClientMessage = ({
    from: string;
    to: string;
    type: ClientMessage.type;
} | {
    building_type: BuildingType;
    territory: string;
    type: ClientMessage.type;
} | {
    ratio: number;
    type: ClientMessage.type;
} | {
    type: ClientMessage.type;
} | {
    speed: number;
    type: ClientMessage.type;
});
export namespace ClientMessage {
    export enum type {
        ATTACK = 'attack',
    }
}

