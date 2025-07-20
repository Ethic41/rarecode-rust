fn main() {
    let x: i32 = 42;
    let result: bool = less_than_100(x);
    println!("{}", result);
}

pub fn less_than_100(x: i32) -> bool {
    if x < 100 {
        return true;
    }

    false
}
