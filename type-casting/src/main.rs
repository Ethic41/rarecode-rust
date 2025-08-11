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

pub fn can_downcast(x: u32) -> bool {
    if x <= u8::MAX as u32 {
        true
    }
    else {
        false
    }
}

pub fn can_downcast_1(x: i32) -> bool {
    if x <= i8::MAX as i32 && x >= i8::MIN as i32 {
        true
    }
    else {
        false
    }
}

pub fn max_of_three(x: i32, y: i32, z: i32) -> i32 {
    let mut max: i32 =  if x > y { x } else { y };

    max = if z > max { z } else { max };

    max
}

pub fn vector_can_be_downcasted(v: &Vec<i32>) -> bool {
    for i in 0..v.len(){
        if v[i] > i8::MAX as i32 || v[i] < i8::MIN as i32 {
            return false;
        }
    }
    
    true
}

pub fn clamp(x: i32) -> i8 {
    if x > i8::MAX as i32 {
        return i8::MAX;
    }

    if x < i8::MIN as i32 {
        return i8::MIN;
    }

    x as i8
}
