use std::cmp::{min,max};

pub fn convert_to_int(some_vec: &Vec<String>) -> Vec<i32> {
    let numbers_vec: Vec<i32> = some_vec.iter()
            .map(|x| x.parse().unwrap()).collect();
    numbers_vec
}

pub fn get_max_int_value(some_vec: &Vec<i32>) -> i32 {
    let max_value: i32 = *some_vec.iter().max().unwrap();
    max_value
}

pub fn get_min_int_value(some_vec: &Vec<i32>) -> i32 {
    let min_value: i32 = *some_vec.iter().min().unwrap();
    min_value
}