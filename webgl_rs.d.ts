/* tslint:disable */
/* eslint-disable */
/**
*/
export function sierpinski(): void;
/**
*/
export function sierpinski_3d(): void;
/**
*/
export function sierpinski_3d_points(): void;
/**
*/
export function sierpinski_points(): void;
/**
*/
export function square(): void;
/**
*/
export function triangle(): void;
/**
*/
export function twist(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly sierpinski: () => void;
  readonly sierpinski_3d: () => void;
  readonly sierpinski_3d_points: () => void;
  readonly sierpinski_points: () => void;
  readonly square: () => void;
  readonly triangle: () => void;
  readonly twist: () => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        