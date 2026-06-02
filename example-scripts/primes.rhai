let now=ts();
const N=1000;
let mask=[];
for i in 0..=N { mask+=[ i >= 2 ]; }
let cnt=0;
for p in 2..=N{
    if!mask[p]{
        continue;
    }
    cnt+=1;
    for i in range(2*p,N+1,p){
        mask[i]=false;
    }
}
print(`${cnt} primes in ${(ts()-now)/32768.0}s`);