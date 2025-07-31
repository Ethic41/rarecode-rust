use std::ops::Index;

fn main() {
    let x: i32 = 2;
    let y: i32 = inc(x);

    println!("{}", y);
    println!("{}", x);
}

pub fn inc(x: i32) -> i32 {
    x + 1
}

pub fn is_in_vector(v: &Vec<i32>, k: i32) -> bool {
    v.contains(&k)
}

pub fn k_is_at_idx(v: &Vec<i32>, k: i32, idx: usize) -> bool{
    for i in 0..v.len(){
        if i == idx && v[i] == k {
            return true;
        }
    }

    false
}