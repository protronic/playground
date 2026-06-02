let now = ts();
let sum = 0;
for i in 0..10000 { sum += i; }
print(`sum=${sum} t=${(ts()-now)/32768.0}s`);
