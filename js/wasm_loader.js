// This file is for loading the WASM module produced by `wasm-pack` for the
// "web" target. The reason why we don't use the default "bundler" target is
// that the WASM support in webpack v4 always mangles the module and produces
// the file in different filenames. When the module is used on both the web and
// webworker targets, this causes the module to be needlessly duplicated,
// leading to the client having to download the module twice. By bypassing
// webpack's WASM support and loading it the "manual" way, we stop webpack from
// mangling the WASM module so that the module won't be duplicated.

import wasmPath from "../pkg/index_bg.wasm";
import wasmInit, * as wasm from "../pkg/index.js";

let _wasmInstance = null;
const wasmLoadPromise = wasmInit(wasmPath).then(instance => {
    _wasmInstance = instance;
    return instance;
});
const wasmImport = wasmLoadPromise.then(() => wasm);

/**
 * Returns the current WASM heap size in bytes, or null if not yet initialized.
 * @returns {number|null}
 */
export function getWasmHeapBytes() {
    return _wasmInstance ? _wasmInstance.memory.buffer.byteLength : null;
}

/** Returns currently live (not freed) bytes from the Rust allocator. */
export function getAllocLiveBytes() {
    return _wasmInstance ? wasm.alloc_live_bytes() : null;
}

/** Returns peak live bytes since the last allocResetPeak() call. */
export function getAllocPeakBytes() {
    return _wasmInstance ? wasm.alloc_peak_bytes() : null;
}

/** Resets the peak counter. Call immediately before each script run. */
export function allocResetPeak() {
    if (_wasmInstance) wasm.alloc_reset_peak();
}

export { wasm, wasmImport, wasmLoadPromise };
