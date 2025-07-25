fn main() {
    println!("Hello, world!");
}

pub fn contains_k_twice(v: Vec<i32>, k: i32) -> bool {
    let mut count: usize = 0;
    for i in 0..v.len() {
        if v[i] == k {
            count += 1;
            if count >= 2 {
                return true;
            }
        }
    }

    false
}
