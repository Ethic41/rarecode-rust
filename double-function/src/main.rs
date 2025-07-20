fn main() {
    let x: i32 = 42;
    let result: i32 = double(x);
    println!("{}", result);
}

pub fn double(x: i32) -> i32 {
    x * 2
}
