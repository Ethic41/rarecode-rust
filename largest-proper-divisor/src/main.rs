fn main() {
    let x: u32 = 12;
    let result: u32 = largest_proper_divisor(x);
    println!("{}", result);
}

pub fn largest_proper_divisor(x: u32) -> u32 {
    if x == 1 {
        return x;
    }

    let mut i: u32 = x - 1;

    loop {
        if x % i == 0 {
            return i;
        }

        i -= 1;
    }
}
