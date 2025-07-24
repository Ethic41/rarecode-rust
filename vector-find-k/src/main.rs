fn main() {
    let k: i32 = 8;
    let v: Vec<i32> = vec![1, 2, 8, 9, 3];
    let result = find_k(v, k);
    println!("{}", result);
}

pub fn find_k(v: Vec<i32>, k: i32) -> usize {
    for i in 0..v.len() {
        if v[i] == k {
            return i;
        }
    }

    0
}
