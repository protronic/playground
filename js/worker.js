import { wasmImport, getWasmHeapBytes, getAllocLiveBytes, getAllocPeakBytes, allocResetPeak } from "./wasm_loader.js";

const playgroundPromise = wasmImport.then(wasm => new wasm.Playground);

async function runScript(script) {
    const playground = await playgroundPromise;
    function output(line) {
        self.postMessage({
            req: "runScript/output",
            output: line,
        });
    }
    allocResetPeak();
    const liveBefore = getAllocLiveBytes() || 0;
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
    const peakKB = Math.round((peakBytes - liveBefore) / 1024);
    postMessage({
        req: "runScript/end",
        heapKB,
        peakKB,
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
