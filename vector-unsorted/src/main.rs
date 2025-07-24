fn main() {
    let v: Vec<i32> = vec![1, 2, 1, 3, 4];
    let result: usize = first_unsorted(v);
    println!("{}", result);
}

pub fn first_unsorted(v: Vec<i32>) -> usize {
    for i in 0..v.len() - 1 {
        if v[i + 1] < v[i] {
            return i + 1;
        }
    }

    0
}
