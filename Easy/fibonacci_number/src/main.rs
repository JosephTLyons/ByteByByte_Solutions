// Given an integer n, write a function to compute the nth Fibonacci number
// Solved

// Recursive version
fn fib_rec(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_rec(n - 1) + fib_rec(n - 2)
    }
}

// Non-recursive version
fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    let mut result: u32 = 0;
    let mut last: u32 = 1;
    let mut last_2: u32 = 0;

    for _ in 2..=n {
        result = last + last_2;
        last_2 = last;
        last = result;
    }

    result
}

fn main() {
    for i in 0..15 {
        println!("fib({}) = {}", i, fib(i));
    }

    for i in 0..15 {
        println!("fib({}) = {}", i, fib_rec(i));
    }
}
