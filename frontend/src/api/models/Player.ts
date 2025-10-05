/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AIPersonality } from './AIPersonality';
/**
 * A player in the game (human or AI)
 */
export type Player = {
    ai_personality?: (null | AIPersonality);
    /**
     * Percentage of troops committed per attack
     */
    attack_ratio: number;
    color: string;
    gold: number;
    id: string;
    is_ai: boolean;
    is_alive: boolean;
    max_population: number;
    name: string;
    population: number;
    territories_controlled: number;
    /**
     * Percentage of population used as troops (rest are workers)
     */
    troop_ratio: number;
};

