/* -=-<[ Bismillahirrahmanirrahim ]>-=-
 -*- coding: utf-8 -*-
 @Date    : 2025-07-22 08:03:05
 @Author  : Dahir Muhammad Dahir (dahirmuhammad3@gmail.com)
 @About   : Sum of Evens
*/

fn main() {
    let start: u32 = 0;
    let end: u32 = 5;

    let result: u32 = sum_evens(start, end);
    println!("{}", result);
}

pub fn sum_evens(start: u32, end: u32) -> u32 {
    let mut sum: u32 = 0;

    for i in start..end {
        if i % 2 == 0 {
            sum += i
        }
    }

    sum
}
