fn wait(ms){
    let t=ms*32768 / 1000;
    let s=ts();
    while ts()-s<t{}
}
let now=ts();
for i in 0..6{
    print(led(i%2==0));
    wait(300);
}
`Dauer: ${(ts()-now)/32768.0}s`