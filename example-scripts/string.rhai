print("hello\nworld");
print("\u2764 \U0001F603 \x40");
let s = "hello, world!";
print(`len=${s.len}`);
s[s.len-1] = '?';
print(s);
let t = `interp: ${"nested " + s}`;
print(t);
