/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
/**
 * Combat result after an attack
 */
export type CombatResult = {
    attacker_id: string;
    attacker_losses: number;
    attacker_troops_committed: number;
    defender_id: string;
    defender_losses: number;
    defender_troops: number;
    from_territory: string;
    territory_conquered: boolean;
    to_territory: string;
};

