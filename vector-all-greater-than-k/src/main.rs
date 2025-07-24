fn main() {
    println!("Hello, world!");
}

pub fn greater_than_k(v: Vec<u32>, k: u32) -> bool {
    for i in 0..v.len() {
        if !(v[i] > k) {
            return false;
        }
    }

    true
}
