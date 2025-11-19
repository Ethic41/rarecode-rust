use std::collections::HashSet;

fn main() {
    let s = HashSet::from([2, 4, 8, 10]);

    let s_iter = s.into_iter();

    for item in s_iter {
        println!("{}", item);
    }

    let v = Vec::from([2, 4, 8, 10]);

    let v_clone = v.clone();

    let v_iter = v_clone.into_iter();

    let v_clone2 = v.clone();
    let s_iter = v_clone2.into_iter();

    let set: HashSet<i32> = s_iter.collect();
    println!("{:?}", set);

    for item in v_iter {
        println!("{}", item);
    }

    println!("{:?}", v);
}

pub fn size_of_set(s: &HashSet<i32>) -> usize {
    s.len()
}
