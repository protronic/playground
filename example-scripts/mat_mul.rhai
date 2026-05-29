fn mm(a,b){let c=[];for i in 0..a.len{let r=[];for j in 0..b[0].len{let v=0.0;for k in 0..b.len{v+=a[i][k]*b[k][j];}r.push(v);}c.push(r);}c}
let a=[[1.0,2.0],[3.0,4.0]];
let b=[[5.0,6.0],[7.0,8.0]];
print(mm(a,b));
