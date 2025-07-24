fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 2, 1];

    let result = is_palindrome(v);
    println!("{}", result);
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
