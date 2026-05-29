fn fib(n) { if n < 2 { n } else { fib(n-1) + fib(n-2) } }
let now = timestamp();
let r = fib(28);
print(`fib(28)=${r} in ${now.elapsed}s`);
