fn main() {
    let x: i32 = 42;
    let y: i32 = 43;
    let result: bool = x_and_y_greater_than_20(x, y);
    println!("{}", result);
}

pub fn x_and_y_greater_than_20(x: i32, y: i32) -> bool {
    x > 20 && y > 20
}
