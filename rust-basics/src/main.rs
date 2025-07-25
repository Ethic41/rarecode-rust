fn main() {
    println!("Hello, world!");
}

pub fn between_50_and_100(x: i32) -> bool {
    if x > 50 && x < 100 {
        return true;
    }

    false
}

pub fn calculator(x: i32, y: i32, op: i32) -> i32 {
    if op == 0 {
        return x + y;
    }
    if op == 1 {
        return x - y;
    }
    if op == 2 {
        return x * y;
    }
    if op == 3 {
        return x / y;
    }
    x % y
}

pub fn count_evens(start: u32, end: u32) -> u32 {
    let mut count: u32 = 0;
    for _i in start..end {
        if _i % 2 == 0 {
            count += 1
        }
    }
    count
}

pub fn count_down() {
    let n: u32 = 10;

    for i in 0..n {
        let backward_i: u32 = n - i;
        println!("{}", backward_i);
    }
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

pub fn double(x: i32) -> i32 {
    x * 2
}

pub fn factorial(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial(n - 1)
}

pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

pub fn floor100(x: i32) -> i32 {
    if x < 100 {
        return 100;
    }

    x
}

pub fn for_loop() {
    for i in 0..6 {
        println!("{}", i);
    }
}

pub fn x_and_y_greater_than_20(x: i32, y: i32) -> bool {
    x > 20 && y > 20
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

pub fn hello_rarecode() {
    println!("hello RareCode");
}

pub fn increment(x: i32) -> i32 {
    x + 1
}

pub fn is_divisible_by(x: i32, y: i32) -> bool {
    x % y == 0
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

pub fn x_or_y_less_than_10(x: i32, y: i32) -> bool {
    x < 10 || y < 10
}

pub fn less_than_100(x: i32) -> bool {
    if x < 100 {
        return true;
    }

    false
}

pub fn loop_counter() {
    let mut x: i32 = 1;
    for _i in 0..3 {
        x = x + 1;
    }
    println!("{}", x);
}

pub fn max(x: i32, y: i32) -> i32 {
    if x > y {
        return x;
    }

    y
}

pub fn mut_var() {
    let mut x: i32 = 3;
    x = x + 1;
    println!("{}", x);
}

pub fn negate(x: i32) -> i32 {
    x * -1
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

pub fn sum_evens(start: u32, end: u32) -> u32 {
    let mut sum: u32 = 0;

    for i in start..end {
        if i % 2 == 0 {
            sum += i
        }
    }

    sum
}

pub fn sum(x: i32, y: i32) -> i32 {
    x + y
}

pub fn range_loop() {
    let start: i32 = 1;
    let end: i32 = 5;

    for i in start..end {
        println!("{}", i);
    }
}
