let now = ts();
let x = 1_000_000;
let l = true;
print("Ready... Go!");

while x > 0 {
    x -= 1;
    if(x % 100_000 == 0) {
        print(x);
        l = led(!l);
    }    
}
print(`Finished. Run time = ${(ts() - now)/32768.0} seconds.`);