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
