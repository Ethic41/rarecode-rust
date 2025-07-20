fn main() {
    let x: i32 = 42;
    let y: i32 = 0;
    let result: i32 = sum(x, y);
    println!("{}", result);
}

pub fn sum(x: i32, y: i32) -> i32 {
    x + y
}
