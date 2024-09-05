// Here you can see two fibonacci functions. One is recursive and one is iterative
// Let's just expect the input to be > 0. Makes the code easier to read
// If you run `cargo bench` you can run benchmarks on these two to catch performance regressions!

pub fn fibonacci(n: u32) -> u64 {
    if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}

pub fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => unreachable!(),
        1 | 2 => 1,
        3 => 2,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}
