let now=timestamp();
const N=100000;
let mask=[];mask.pad(N+1,true);
mask[0]=false;mask[1]=false;
let cnt=0;
for p in 2..=N{if!mask[p]{continue;}cnt+=1;for i in range(2*p,N+1,p){mask[i]=false;}}
print(`${cnt} primes in ${now.elapsed}s`);
