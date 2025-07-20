fn main() {
    let x: i32 = 42;
    let result: i32 = negate(x);
    println!("{}", result);
}

pub fn negate(x: i32) -> i32 {
    x * -1
}
