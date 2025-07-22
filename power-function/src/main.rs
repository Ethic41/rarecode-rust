fn main() {
    let base: u32 = 2;
    let exponent: u32 = 4;
    let result: u32 = power(base, exponent);

    println!("{}", result);
}

pub fn power(base: u32, exponent: u32) -> u32 {
    let mut acc: u32 = 1;

    if exponent == 0 {
        return 1;
    }

    for _i in 0..exponent {
        acc *= base;
    }

    acc
}
