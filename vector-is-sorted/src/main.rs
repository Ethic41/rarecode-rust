fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 0];

    let result: bool = is_sorted(v);
    println!("{}", result);
}

pub fn is_sorted(v: Vec<i32>) -> bool {
    for i in 0..v.len() - 1 {
        if v[i + 1] < v[i] {
            return false;
        }
    }

    true
}
