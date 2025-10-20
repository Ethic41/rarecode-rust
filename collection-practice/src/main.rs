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

pub fn largest_set(v: &Vec<HashSet<i32>>) -> usize {
    let len: usize = v.len();
    if len < 1 {
        return len;
    }

    let mut largest: usize = 0;

    for i in 0..len {
        if v[i].len() > largest {
            largest = v[i].len(); 
        }
    }
    
    largest
}

pub fn mul_table(n: u32) -> Vec<Vec<u32>> {
    let mut mul_table: Vec<Vec<u32>> = vec![];

    for i in 1..n+1 {
        let mut cur_table: Vec<u32> = vec![];
        
        for j in 1..n+1 {
            cur_table.push(i * j);
        }

        mul_table.push(cur_table);
    }

    mul_table
}

pub fn remove_smaller_than_k(v: &Vec<HashSet<i32>>, k: usize) -> Vec<HashSet<i32>> {
    let mut my_v: Vec<HashSet<i32>> = vec![];

    for i in 0..v.len() {
        if v[i].len() >= k {
            my_v.push(v[i].clone());
        }
    }

    my_v
}
