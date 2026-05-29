let val = ();
let o = #{
    x: 42,
    get: || this.x,
    set: |v| { this.x=v; val=v; print(`x=${this.x}`); }
};
if o.get() > 0 { o.set(7); }
print(`val=${val}`); // 7
