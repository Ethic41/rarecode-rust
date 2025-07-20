fn main() {
    let x: i32 = 42;
    let y: i32 = 8;
    let result: bool = x_or_y_less_than_10(x, y);
    println!("{}", result);
}

pub fn x_or_y_less_than_10(x: i32, y: i32) -> bool {
    x < 10 || y < 10
}
