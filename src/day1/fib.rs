pub fn fib(a: u32) -> u32 {
    if a <= 1 { 1 } else { fib(a - 1) + fib(a - 2) }
}
