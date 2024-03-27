fn main() {
    println!("{}", factorial_iterative(10));
    println!("{}", factorial_recursive(10));
}

fn factorial_recursive(n: u128) -> u128 {
    if n == 0 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

fn factorial_iterative(n: u128) -> u128 {
    let mut res = 1;
    for i in 1..n + 1 {
        res *= i;
    }
    res
}
