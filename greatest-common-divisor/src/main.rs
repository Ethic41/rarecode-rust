fn main() {
    let x: u32 = 48;
    let y: u32 = 18;
    let result: u32 = gcd(x, y);
    println!("{}", result);
}

pub fn gcd(x: u32, y: u32) -> u32 {
    let mut min: u32 = min(x, y);

    loop {
        if x % min == 0 && y % min == 0 {
            return min;
        }

        min -= 1;
    }
}

pub fn min(x: u32, y: u32) -> u32 {
    if x > y {
        return y;
    }

    x
}
