let a = 3;
fn add(a, b) {
    a = 42; // passed by value
    a + b
}
let r = add(a, 4);
print(`add(a,4)=${r}`); // 46
print(`a=${a}`);        // 3
