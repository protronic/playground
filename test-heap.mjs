// Local Node.js test for byte-accurate Rhai allocation tracking
// Usage: node test-heap.mjs
import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __dirname = dirname(fileURLToPath(import.meta.url));

const wasmBuffer = readFileSync(join(__dirname, 'pkg/index_bg.wasm'));

import pkg from './pkg/index.js';
const { initSync, Playground, alloc_live_bytes, alloc_peak_bytes, alloc_reset_peak } = pkg;

initSync({ module: wasmBuffer });

function makeScript(N) {
    return `
let now=ts();
const N=${N};
let mask=[];
for i in 0..=N { mask+=[ i >= 2 ]; }
let cnt=0;
for p in 2..=N {
    if !mask[p] { continue; }
    cnt+=1;
    for i in range(2*p, N+1, p) { mask[i]=false; }
}
print(\`\${cnt} primes in \${(ts()-now)/32768.0}s\`);
`;
}

const sizes = [1_000, 10_000, 100_000, 1_000_000];

console.log(`Live bytes after init: ${alloc_live_bytes().toLocaleString()} B\n`);

for (let pass = 1; pass <= 2; pass++) {
    console.log(`--- Pass ${pass} ---`);
    for (const N of sizes) {
        alloc_reset_peak();
        const liveBefore = alloc_live_bytes();
        const pg = new Playground();
        let printed = '';
        pg.runScript(makeScript(N), s => { printed = s; }, () => {}, null, null);
        pg.free();
        const liveAfter = alloc_live_bytes();
        const peak = alloc_peak_bytes();
        const peakNet = peak - liveBefore;
        const liveRetained = liveAfter - liveBefore;
        console.log(
            `N=${String(N).padStart(7)}: ` +
            `peak: ${(peakNet / 1024).toFixed(1).padStart(8)} KB  ` +
            `retained: ${liveRetained >= 0 ? '+' : ''}${liveRetained} B  ` +
            `[${printed}]`
        );
    }
    console.log();
}
