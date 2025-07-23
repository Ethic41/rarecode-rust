fn main() {
    let n: u32 = 5;
    let result: u32 = factorial(n);
    println!("{}", result);
}

pub fn factorial(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial(n - 1)
}
