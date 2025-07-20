fn main() {
    let x: i32 = 42;
    let y: i32 = 2;
    let op: i32 = 0;
    let result: i32 = calculator(x, y, op);
    println!("{}", result);
}

pub fn calculator(x: i32, y: i32, op: i32) -> i32 {
    if op == 0 { return x + y}
    if op == 1 {return x - y}
    if op == 2 {return x * y}
    if op == 3 {return x / y}
    x % y
}