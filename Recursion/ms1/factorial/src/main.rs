#![feature(strict_overflow_ops)]

fn factorial(n: &i64) -> i64 {
    if *n == 0 || *n == 1 {
        return 1;
    }
    n.strict_mul(factorial(&(n - 1)))
}

fn main() {
    for n in 0..22i64 {
        println!("{}! = {}", n, factorial(&n));
    }
}
