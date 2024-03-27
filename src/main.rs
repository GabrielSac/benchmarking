use num::BigInt;
fn main() {
    println!("{}", factorial_iterative(BigInt::from(10000)));
    println!("{}", factorial_recursive(BigInt::from(10000)));
}

fn factorial_recursive(n: BigInt) -> BigInt {
    if n == BigInt::from(0) {
        BigInt::from(1)
    } else {
        n.clone() * factorial_recursive(n - 1)
    }
}

fn factorial_iterative(n: BigInt) -> BigInt {
    let mut res = BigInt::from(1);
    let mut i = BigInt::from(1);
    while i <= n {
        res *= i.clone();
        i += 1;
    }
    res
}
