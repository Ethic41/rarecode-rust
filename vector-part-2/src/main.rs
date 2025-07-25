fn main() {
    println!("Hello, world!");
}

pub fn vector_modified() {
    let mut v: Vec<i32> = vec![5, 6, 7];

    v[0] = 99;

    println!("{:?}", v);
}

pub fn vector_append() {
    let mut v: Vec<i32> = vec![4, 5, 6];

    v.push(99);

    println!("{:?}", v);
}

pub fn simple_count(n: u32) -> Vec<u32> {
    let mut container: Vec<u32> = vec![];

    for i in 0..n{
        container.push(i+1);
    }

    container
}

pub fn countdown(n: u32) -> Vec<u32> {
    let mut container: Vec<u32> = vec![];

    for i in 0..n{
        container.push(n - i);
    }

    container
}

pub fn modify(v: Vec<i32>) -> Vec<i32> {
    let mut my_vec = vec![];

    for i in 0..v.len(){
        my_vec.push(v[i]);
    }

    my_vec.push(99);

    my_vec
}

pub fn make_copy(v: Vec<i32>) -> Vec<i32> {
    let mut container: Vec<i32> = vec![];

    for i in 0..v.len() {
        container.push(v[i]);
    }
    
    container
}