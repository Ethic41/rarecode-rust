fn main() {
    let x: u32 = 17;
    let result: bool = is_prime(x);
    println!("{}", result);
}

pub fn is_prime(x: u32) -> bool {
    if x < 2 {
        return false;
    }

    for i in 2..(x / 2 + 1) {
        if x % i == 0 {
            return false;
        }
    }

    true
}
