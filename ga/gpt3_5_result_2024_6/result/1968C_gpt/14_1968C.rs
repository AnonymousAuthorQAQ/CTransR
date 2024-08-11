const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut x = [0; N];
    let mut a = [0; N];
    
    let n: usize;
    scanf("%d", &n);
    
    for i in 1..n {
        scanf("%d", &mut x[i]);
    }
    
    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
            j += 1;
        }
    }
    
    for i in 0..n {
        printf("%d ", a[i]);
    }
    printf("\n");
}

fn main() {
    let mut t: i32;
    scanf("%d", &mut t);
    
    for _ in 0..t {
        proc();
    }
}