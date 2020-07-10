/* tslint:disable */
/* eslint-disable */
/**
*/
export function square(): void;
/**
*/
export function triangle(): void;
/**
*/
export function sierpinski_points(): void;
/**
*/
export function sierpinski(): void;
/**
*/
export function sierpinski_3d_points(): void;
/**
*/
export function sierpinski_3d(): void;
/**
*/
export function twist(): void;
/**
*/
export function rotating_square(): void;
/**
*/
export function rotating_square_controls(): void;
/**
*/
export function pixels(): void;
/**
*/
export function triangles(): void;
/**
*/
export function cad(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly square: () => void;
  readonly triangle: () => void;
  readonly sierpinski_points: () => void;
  readonly sierpinski: () => void;
  readonly sierpinski_3d_points: () => void;
  readonly sierpinski_3d: () => void;
  readonly twist: () => void;
  readonly rotating_square: () => void;
  readonly rotating_square_controls: () => void;
  readonly pixels: () => void;
  readonly triangles: () => void;
  readonly cad: () => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc9eef77cfce2d0b8: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7ecea66a4a95b031: (a: number, b: number, c: number) => void;
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
        