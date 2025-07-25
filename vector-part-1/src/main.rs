fn main() {
    println!("Hello, world!");
}

pub fn at_least_k_larger_than_t(v: Vec<i32>, k: usize, t: i32) -> bool {
    let mut count: usize = 0;

    for i in 0..v.len() {
        if v[i] > t {
            count += 1;
            if count >= k {
                return true;
            }
        }
    }
    false
}

pub fn greater_than_k(v: Vec<u32>, k: u32) -> bool {
    for i in 0..v.len() {
        if !(v[i] > k) {
            return false;
        }
    }

    true
}

pub fn count_odds(v: Vec<i32>) -> i32 {
    let mut odd_count: i32 = 0;

    for i in 0..v.len() {
        if v[i] % 2 != 0 {
            odd_count += 1;
        }
    }

    odd_count
}

pub fn vector_creation() {
    let my_vec: Vec<u32> = vec![1, 2, 3];

    println!("{:?}", my_vec);
}

pub fn find_k(v: Vec<i32>, k: i32) -> usize {
    for i in 0..v.len() {
        if v[i] == k {
            return i;
        }
    }

    0
}

pub fn get_index_at(v: Vec<i32>, i: usize) -> i32 {
    v[i]
}

pub fn index_greater(v: Vec<i32>, i: usize, j: usize) -> bool {
    v[i] >= v[j]
}

pub fn is_palindrome(v: Vec<i32>) -> bool {
    let v_len: usize = v.len();

    for i in 0..v_len / 2 {
        // pass
        if v[i] != v[v_len - 1 - i] {
            return false;
        }
    }

    true
}

pub fn is_sorted(v: Vec<i32>) -> bool {
    for i in 0..v.len() - 1 {
        if v[i + 1] < v[i] {
            return false;
        }
    }

    true
}

pub fn vector_iteration() {
    let my_vec: Vec<u32> = vec![1, 2, 3];

    for i in 0..my_vec.len() {
        let element: u32 = my_vec[i];
        println!("{}", element);
    }
}

pub fn k_appears_before_idx(v: Vec<i32>, k: i32, idx: usize) -> bool {
    for i in 0..idx {
        if v[i] == k {
            return true;
        }
    }

    false
}

pub fn contains_k_twice(v: Vec<i32>, k: i32) -> bool {
    let mut count: usize = 0;
    for i in 0..v.len() {
        if v[i] == k {
            count += 1;
            if count >= 2 {
                return true;
            }
        }
    }

    false
}

pub fn vector_length() {
    let my_vec: Vec<u32> = vec![1, 3];

    let length: usize = my_vec.len();
    println!("{}", length);
}

pub fn max(v: Vec<u32>) -> u32 {
    let mut biggest: u32 = 0;

    for i in 0..v.len() {
        if v[i] > biggest {
            biggest = v[i];
        }
    }

    biggest
}

pub fn first_unsorted(v: Vec<i32>) -> usize {
    for i in 0..v.len() - 1 {
        if v[i + 1] < v[i] {
            return i + 1;
        }
    }

    0
}
