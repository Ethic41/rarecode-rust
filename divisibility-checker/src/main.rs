fn main() {
    let x: i32 = 42;
    let result: i32 = divisible_by(x);
    println!("{}", result);
}

pub fn divisible_by(x: i32) -> i32 {
    if x % 2 == 0 {
        return 2;
    } else if x % 3 == 0 {
        return 3;
    } else if x % 5 == 0 {
        return 5;
    }

    0
}
