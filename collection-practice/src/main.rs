use std::collections::HashSet;

fn main() {
    let mut s1: HashSet<i32> = HashSet::new();
    s1.insert(1);
    s1.insert(2);
    s1.insert(3);

    let s2: HashSet<i32> = HashSet::from([1,2,3]);
    s1.extend(s2);

    let len: usize = size_of_set(&s1);
    println!("{}", len);
}

pub fn size_of_set(s: &HashSet<i32>) -> usize {
    s.len()
}
