fn main() {
    let i: usize = 0;
    let j: usize = 1;
    let v: Vec<i32> = vec![5, 3, 15, 21];

    let result: bool = index_greater(v, i, j);
    println!("{}", result);
}

pub fn index_greater(v: Vec<i32>, i: usize, j: usize) -> bool {
    v[i] >= v[j]
}
