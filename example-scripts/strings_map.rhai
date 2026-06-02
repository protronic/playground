let now = ts();
let map = #{a: 1, b: 2, c: 3};
map.d = 4;
map.remove("b");
print(map.keys());
print(map.contains("a"));
let s = ["x", "y", "z"].join(", ");
print(`map=${map} s=${s} t=${(ts()-now)/32768.0}s`);
