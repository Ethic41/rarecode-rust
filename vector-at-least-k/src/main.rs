fn main() {
    println!("Hello, world!");
}

pub fn at_least_k_larger_than_t(v: Vec<i32>, k: usize, t: i32) -> bool {
    let mut count: usize = 0;

    for i in 0..v.len() {
        if v[i] > t {
            count += 1;
            if count >= k {
                return true;
            }
        }
    }
    false
}
