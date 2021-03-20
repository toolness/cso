/* tslint:disable */
/* eslint-disable */
/**
*/
export class WebLevel {
  free(): void;
/**
* @param {Uint8Array} bmp_bytes
* @returns {WebLevel}
*/
  static new(bmp_bytes: Uint8Array): WebLevel;
/**
* @returns {number}
*/
  get_width(): number;
/**
* @returns {number}
*/
  get_height(): number;
/**
* @param {boolean} enable
*/
  set_enable_water_factories(enable: boolean): void;
/**
* @param {Uint8Array} arr
*/
  draw(arr: Uint8Array): void;
/**
*/
  tick(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_weblevel_free: (a: number) => void;
  readonly weblevel_new: (a: number, b: number) => number;
  readonly weblevel_get_width: (a: number) => number;
  readonly weblevel_get_height: (a: number) => number;
  readonly weblevel_set_enable_water_factories: (a: number, b: number) => void;
  readonly weblevel_draw: (a: number, b: number, c: number) => void;
  readonly weblevel_tick: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
