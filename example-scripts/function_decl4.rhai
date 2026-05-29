fn action(x, y) {
    this = this.abs() + x * y;
}
let obj = -40;
obj.action(1, 2);
print(`obj=${obj}`); // 42
