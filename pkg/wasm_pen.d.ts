/* tslint:disable */
/**
*/
export class Simulation {
  free(): void;
/**
* @returns {Simulation} 
*/
  static new(): Simulation;
/**
* @param {number} mass 
* @param {number} radius 
* @param {number} x 
* @param {number} y 
* @param {number} vx 
* @param {number} vy 
* @returns {number} 
*/
  add_body(mass: number, radius: number, x: number, y: number, vx: number, vy: number): number;
/**
* @returns {void} 
*/
  start(): void;
}
