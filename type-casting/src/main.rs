/* -=-<[ Bismillahirrahmanirrahim ]>-=-
 -*- coding: utf-8 -*-
 @Date    : 2025-08-03 10:51:56
 @Author  : Dahir Muhammad Dahir (dahirmuhammad3@gmail.com)
 @About   : Type Casting Exercises
*/

fn main() {
    println!("Hello, world!");
}

pub fn max(x: i32, y: i32) -> i32 {
    let m: i32 = if x > y { x } else { y };

    m
}

pub fn absolute_value(x: i32) -> i32 {
    let ans: i32 = if x < 0 { x * -1 } else { x };

    ans
}

pub fn absolute_value_1(x: i32) -> u32 {
    let ans: i32 = if x < 0 { x * -1 } else { x };

    ans as u32
}

pub fn foo(x: i32) -> u32 {
    x as u32
}

pub fn find(v: &Vec<i32>, k: i32) -> i32 {
	for i in 0..v.len() {
	    if k == v[i] {
	        return i as i32;
	    }
	}
	-1
}