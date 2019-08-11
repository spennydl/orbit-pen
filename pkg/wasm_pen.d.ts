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
*/
  start(): void;
}

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        