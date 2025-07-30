fn main() {
    let v: Vec<i32> = vec![1, 2, 3];
    do_nothing(v); // usage will work here but not again
}

pub fn do_nothing(v: Vec<i32>) -> Vec<i32> {
    v
}

pub fn variables_consume() {
    let v: Vec<i32> = vec![1, 2, 3];
    let w: Vec<i32> = v;

    println!("{:?}", w);
    // println!("{:?}", v);
}

pub fn fix_ownership_misuse() {
    let v: Vec<i32> = vec![4, 5, 6];
    let u: Vec<i32> = vec![7, 8, 9];

    let a: Vec<i32> = v;
    let b: Vec<i32> = u;

    // println!("{:?}", v); fixed
    println!("{:?}", a);
    println!("{:?}", b);
}

pub fn vec_sum(v: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;

    for i in 0..v.len() {
        sum = sum + v[i];
    }

    sum
}

pub fn get_max(v: &Vec<i32>) -> i32 {
    let mut max: i32 = v[0];

    for i in 1..v.len() {
        if v[i] > max {
            max = v[i];
        }
    }

    max
}

pub fn absolute_value(v: &Vec<i32>) -> Vec<i32> {
    let mut my_v: Vec<i32> = vec![];

    for i in 0..v.len() {
        if v[i] < 0 {
            my_v.push(v[i] * -1);
            continue;
        }

        my_v.push(v[i]);
    }

    my_v
}

pub fn append_sum(v: &Vec<i32>) -> Vec<i32> {
    let mut container: Vec<i32> = v.clone();
    let mut sum: i32 = 0;

    for i in 0..v.len() {
        sum += v[i];
    }

    container.push(sum);
    container
}

pub fn elementwise_sum(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    let mut container: Vec<i32> = vec![];
    let v_len: usize = v1.len();

    for i in 0..v_len{
        container.push(v1[i] + v2[i]);
    }

    container
}

pub fn all_elements_less_than_k(v: &Vec<i32>, k: i32) -> bool {
    for i in 0..v.len(){
        if v[i] >= k {
            return false;
        }
    }

    true
}

pub fn sum_greater_than_s(v: &Vec<i32>, s: i32) -> bool{
    let mut sum: i32 = 0;

    for i in 0..v.len(){
        sum += v[i];
        if sum > s {
            return true;
        }
    }

    false
}

pub fn filter_lt_k(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let mut container: Vec<i32> = vec![];

    for i in 0..v.len(){
        if v[i] >= k{
            container.push(v[i]);
        }
    }

    container
}