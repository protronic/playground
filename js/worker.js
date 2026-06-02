import { wasmImport, getWasmHeapBytes, getAllocLiveBytes, getAllocPeakBytes, getAllocStackPeakBytes, allocResetPeak } from "./wasm_loader.js";

const playgroundPromise = wasmImport.then(wasm => new wasm.Playground);

// Stable baseline: live bytes right after WASM + Playground initialisation.
// Using a per-run snapshot would deflate the peak on subsequent runs because
// the Rhai Engine retains cached state between runs, raising live bytes each
// time without a corresponding rise in the run's own peak allocation.
let allocBaseline = null;
playgroundPromise.then(() => {
    allocBaseline = getAllocLiveBytes() || 0;
});

async function runScript(script) {
    const playground = await playgroundPromise;
    function output(line) {
        self.postMessage({
            req: "runScript/output",
            output: line,
        });
    }
    allocResetPeak();
    const baseline = allocBaseline !== null ? allocBaseline : (getAllocLiveBytes() || 0);
    const heapBefore = getWasmHeapBytes() || 0;
    try {
        let result = playground.runScript(script, s => {
            output(`[PRINT] ${s}`);
        }, s => {
            output(`[DEBUG] ${s}`);
        }, ops => {
            self.postMessage({
                req: "runScript/updateOps",
                ops,
            });
        }, on => {
            self.postMessage({
                req: "runScript/led",
                on,
            });
        });
        output(`\nScript returned: "${result}"`);
    } catch (ex) {
        output(`\nEXCEPTION: ${ex}`);
    }
    const heapBytes = getWasmHeapBytes() || 0;
    const heapKB = Math.round(heapBytes / 1024);
    const peakBytes = getAllocPeakBytes() || 0;
    const peakKB = Math.round((peakBytes - baseline) / 1024);
    const stackPeakBytes = getAllocStackPeakBytes() || 0;
    const stackPeakKB = Math.round(stackPeakBytes / 1024);
    postMessage({
        req: "runScript/end",
        heapKB,
        peakKB,
        stackPeakKB,
    });
}

self.onmessage = ev => {
    if (ev.data.req === "runScript") {
        runScript(ev.data.script);
    } else {
        console.log("Unknown message received by worker:", ev.data);
    }
};

wasmImport.then(() => {
    self.postMessage({ req: "_ready" });
});
