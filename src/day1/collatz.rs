pub fn collatz(n: u32) -> u32 {
    if n == 1 {
        1
    } else if n % 2 == 1 {
        collatz(3 * n + 1) + 1
    } else {
        collatz(n / 2) + 1
    }
}
