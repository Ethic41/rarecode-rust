fn main() {
    let x: i32 = 42;
    let y: i32 = 43;
    let result: i32 = max(x, y);
    println!("{}", result);
}

pub fn max(x: i32, y: i32) -> i32 {
    if x > y {
        return x;
    }

    y
}
