fn mm(a, b) {
    let c = [];
    for (row, i) in a {
        let r = [];

        for (v, j) in b[0] {
            r += [0.0];  
        }
        c += [r];
    }
    for (row, i) in a {
        for (v, j) in b[0] {
            let sum = 0.0;

            for (vk, k) in row {
                sum += a[i][k] * b[k][j];
            }

            c[i][j] = sum;
        }
    }
    c
}

let a = [[1.0, 2.0], [3.0, 4.0]];
let b = [[5.0, 6.0], [7.0, 8.0]];
print(mm(a, b));
