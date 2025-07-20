fn main() {
    let x: i32 = 42;
    let result: bool = is_divisible_by(x, 7);
    println!("{}", result);
}

pub fn is_divisible_by(x: i32, y: i32) -> bool {
    x % y == 0
}
