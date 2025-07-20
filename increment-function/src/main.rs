fn main() {
    let x: i32 = 42;
    let result: i32 = increment(x);
    println!("{}", result);
}

pub fn increment(x: i32) -> i32 {
    x + 1
}
