fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result: i32 = count_odds(v);
    println!("{}", result);
}

pub fn count_odds(v: Vec<i32>) -> i32 {
    let mut odd_count: i32 = 0;

    for i in 0..v.len() {
        if v[i] % 2 != 0 {
            odd_count += 1;
        }
    }

    odd_count
}
