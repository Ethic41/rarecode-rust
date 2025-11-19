/* -=-<[ Bismillahirrahmanirrahim ]>-=-
 -*- coding: utf-8 -*-
 @Date    : 2025-08-11 07:34:53
 @Author  : Dahir Muhammad Dahir (dahirmuhammad3@gmail.com)
 @About   : I will tell you later
*/

use std::collections::HashSet;

fn main() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(10);
    println!("{:?}", set);
}

pub fn membership() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(10);
    set.insert(8);

    println!("{}", set.contains(&10));
    println!("{}", set.contains(&11));
    println!("{}", (&set).contains(&10));
    println!("{}", (&set).contains(&11));
}

pub fn has_zero_to_n(set: &HashSet<i32>, n: i32) -> bool {
    for i in 0..n {
        if set.contains(&i) {
            return true;
        }
    }

    false
}

pub fn exists_in_set(set: &HashSet<i32>, v: &Vec<i32>) -> bool {
    for i in 0..v.len() {
        if set.contains(&v[i]) {
            return true;
        }
    }

    false
}

pub fn clone_set() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(3);

    let mut set2: HashSet<i32> = set.clone();
    set2.insert(4);

    println!("{:?}", set);
    println!("{:?}", set2);
}

pub fn clone_set_reference() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(3);

    let set_ref: &HashSet<i32> = &set;

    let mut set2: HashSet<i32> = set_ref.clone();
    set2.insert(4);

    println!("{:?}", set);
    println!("{:?}", set2);
}

pub fn remove_vector_elements(set: &HashSet<i32>, v: &Vec<i32>) -> HashSet<i32> {
    let mut set: HashSet<i32> = set.clone();

    for i in 0..v.len() {
        set.remove(&v[i]);
    }

    set
}

pub fn return_index_of_first_duplicate(v: &Vec<i32>) -> i32 {
    let mut set: HashSet<i32> = HashSet::new();

    for i in 0..v.len() {
        if set.contains(&v[i]) {
            return i as i32;
        }

        set.insert(v[i]);
    }

    -1
}

pub fn vector_to_set(v: Vec<i32>) -> HashSet<i32> {
    let mut set: HashSet<i32> = HashSet::new();

    for i in 0..v.len() {
        set.insert(v[i]);
    }

    set
}

pub fn is_disjoint(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    let set: HashSet<i32> = vector_to_set(v1.clone());

    for i in 0..v2.len() {
        if set.contains(&v2[i]) {
            return false;
        }
    }

    true
}

pub fn safe_cast_vec_to_set(v: &Vec<i64>) -> HashSet<i32> {
    let mut set: HashSet<i32> = HashSet::new();

    for i in 0..v.len() {
        if v[i] <= i32::MAX as i64 && v[i] >= i32::MIN as i64 {
            set.insert(v[i] as i32);
        }
    }

    set
}
