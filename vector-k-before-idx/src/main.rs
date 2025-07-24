fn main() {
    println!("Hello, world!");
}

pub fn k_appears_before_idx(v: Vec<i32>, k: i32, idx: usize) -> bool {
    for i in 0..idx {
        if v[i] == k {
            return true;
        }
    }

    false
}
