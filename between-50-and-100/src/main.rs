fn main() {
    let x: i32 = 42;
    let result: bool = between_50_and_100(x);
    println!("{}", result);
}

pub fn between_50_and_100(x: i32) -> bool {
    if x > 50 && x < 100 {
        return true;
    }

    false
}
