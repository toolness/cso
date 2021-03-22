
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
*/
export class WebLevel {

    static __wrap(ptr) {
        const obj = Object.create(WebLevel.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_weblevel_free(ptr);
    }
    /**
    * @param {Uint8Array} bmp_bytes
    * @returns {WebLevel}
    */
    static new(bmp_bytes) {
        var ptr0 = passArray8ToWasm0(bmp_bytes, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.weblevel_new(ptr0, len0);
        return WebLevel.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    get_width() {
        var ret = wasm.weblevel_get_width(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    get_height() {
        var ret = wasm.weblevel_get_height(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {boolean} enable
    */
    set_enable_water_factories(enable) {
        wasm.weblevel_set_enable_water_factories(this.ptr, enable);
    }
    /**
    * @param {number | undefined} count
    */
    set_override_water_factory_count(count) {
        wasm.weblevel_set_override_water_factory_count(this.ptr, isLikeNone(count) ? 0xFFFFFF : count);
    }
    /**
    * @param {Uint8Array} arr
    */
    draw(arr) {
        try {
            var ptr0 = passArray8ToWasm0(arr, wasm.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.weblevel_draw(this.ptr, ptr0, len0);
        } finally {
            arr.set(getUint8Memory0().subarray(ptr0 / 1, ptr0 / 1 + len0));
            wasm.__wbindgen_free(ptr0, len0 * 1);
        }
    }
    /**
    */
    tick() {
        wasm.weblevel_tick(this.ptr);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('web_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

