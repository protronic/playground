let list = [];
for i in 0..10 { list.push(|| i ** 2); }
let sum = 0;
for f in list { sum += f.call(); }
print(`sum=${sum}`); // should be 285
