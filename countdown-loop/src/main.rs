fn main() {
    let n: u32 = 10;

    for i in 0..n {
        let backward_i: u32 = n - i;
        println!("{}", backward_i);
    }
}
