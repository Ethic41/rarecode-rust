fn main() {
    let x: i32 = 42;
    let result: i32 = floor100(x);
    println!("{}", result);
}

pub fn floor100(x: i32) -> i32 {
    if x < 100 {
        return 100;
    }

    x
}
