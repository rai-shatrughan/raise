use std::fmt::Display;
use std::ops::Add;

pub fn loop_example(init_value: i64, max_value: i64) {
    println!("====");
    let mut i: i64 = init_value;
    println!("i init : {}", i);
    loop {
        i = i + 1;
        println!("i in loop : {}", i);
        if i >= max_value {
            break;
        }
    }
    println!("i after: {}", i);
    println!("====");
}

pub fn while_example(init_value: i64, max_value: i64) {
    println!("----");
    let mut i = init_value;
    println!("i init : {}", i);
    while i < max_value {
        i = i + 1;
        println!("i in loop : {}", i);
    }
    println!("i after: {}", i);
    println!("----");
}

pub fn for_example(init_value: i64, max_value: i64) {
    println!("++");
    println!("i init : {}", init_value);
    for i in init_value..max_value + 1 {
        println!("i in loop : {}", i);
    }
    println!("i after: {}", max_value);
    println!("++");
}

pub fn vec_example<T: Display>(array: &Vec<T>) {
    println!("--");
    for item in array {
        println!("Vec Item : {}", item)
    }
    println!("--");
}

pub fn transform_vec_example<T: Add<Output = T> + Copy>(array: &mut Vec<T>, increment_by: T) {
    for i in 0..array.len() {
        array[i] = array[i] + increment_by;
    }
}
