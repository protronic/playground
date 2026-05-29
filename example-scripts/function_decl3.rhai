const KEY = 38;
fn f(a, b, c, d, e, g) {
    let x = global::KEY;
    a - b*c - d*e - g + x
}
print(f(100, 5, 2, 9, 6, 32)); // 42
