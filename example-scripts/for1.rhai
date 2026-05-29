let a = [1, 2, 3, 4];
for (v, i) in a {
    print(`[${i}] = ${v}`);
    if v == 3 { break; }
}
for i in range(3, 0, -1) { print(i); }
