fn main() {
    let x: i32 = 2;
    let y: i32 = inc(x);

    println!("{}", y);
    println!("{}", x);
}

pub fn inc(x: i32) -> i32 {
    x + 1
}
