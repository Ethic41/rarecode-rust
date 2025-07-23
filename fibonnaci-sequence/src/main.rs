fn main() {
    let n: u32 = 10;
    let result: u32 = fibonacci(n);
    println!("{}", result);
}

pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
