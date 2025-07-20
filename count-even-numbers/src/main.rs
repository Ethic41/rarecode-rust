fn main() {
    let start: u32 = 1;
    let end: u32 = 5;

    let result: u32 = count_evens(start, end);
    println!("{}", result);
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
