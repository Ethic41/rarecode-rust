/* -=-<[ Bismillahirrahmanirrahim ]>-=-
 -*- coding: utf-8 -*-
 @Date    : 2025-08-03 08:18:55
 @Author  : Dahir Muhammad Dahir (dahirmuhammad3@gmail.com)
 @About   : Learning Rust Ownership II
*/


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

pub fn find_idx_of(v: &Vec<i32>, k: i32) -> usize {
    for i in 0..v.len() {
        if v[i] == k {
            return i;
        }
    }

    v.len()
}

pub fn first_common_index(v1: &Vec<i32>, v2: &Vec<i32>) -> usize {
    let v1_len: usize = v1.len();
    let v2_len: usize = v2.len();

    if v1_len < 1 {
        return 0;
    }

    if v2_len < 1 {
        return v1_len;
    }

    let min_len: usize = if v1_len < v2_len {v1_len} else {v2_len};

    for i in 0..min_len {
        if v1[i] == v2[i] {
            return i;
        }
    }

    v2_len
}

pub fn append_sum(v: &Vec<i32>) -> Vec<i32> {
    let mut sum = 0;

    for i in 0..v.len() {
        sum = sum + v[i];
    }

    let mut my_v: Vec<i32> = v.clone();
    my_v.push(sum);
    my_v
}

pub fn increment_by(v: &Vec<i32>, a: i32) -> Vec<i32> {
    let mut container: Vec<i32> = v.clone();

    for i in 0..container.len() {
        container[i] = container[i] + a;
    }

    container
}

pub fn remove_max(v: &Vec<u32>) -> Vec<u32> {
    let mut container: Vec<u32> = v.clone();

    if container.len() < 2 {
        return vec![];
    }

    let mut max_idx: usize = 0;
    
    for i in 0..container.len(){
        max_idx = if container[i] > container[max_idx] { i } else {max_idx};
    }

    container.remove(max_idx);
    container
}

pub fn filter_even_odd(v: &Vec<i32>, filter_even: bool) -> Vec<i32> {
    if v.len() < 1 {
        return vec![];
    }

    let mut container: Vec<i32> = vec![];

    if filter_even {
        for i in 0..v.len() {
            if v[i] % 2 != 0 {
                container.push(v[i]);
            }
        }
    }
    else {
        for i in 0..v.len() {
            if v[i] % 2 == 0 {
                container.push(v[i]);
            }
        }
    }

    container
}
