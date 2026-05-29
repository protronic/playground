let now = timestamp();
let sum = 0;
for i in 0..10000 { sum += i; }
print(`sum=${sum} t=${now.elapsed}s`);
