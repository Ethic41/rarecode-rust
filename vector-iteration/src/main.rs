fn main() {
    let my_vec: Vec<u32> = vec![1, 2, 3];

    for i in 0..my_vec.len() {
        let element: u32 = my_vec[i];
        println!("{}", element);
    }
}
