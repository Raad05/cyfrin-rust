use std::collections::HashMap;

pub fn filter_non_zero(v: Vec<u32>) -> Vec<u32> {
    let result: Vec<u32> = v.into_iter().filter(|&val| val > 0).map(|x| x).collect();
    result
}

pub fn to_string(v: Vec<&str>) -> Vec<String> {
    let result: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    result
}

pub fn to_hash_map(v: Vec<(String, u32)>) -> HashMap<String, u32> {
    let result: HashMap<String, u32> = v.iter().map(|x| (x.0.to_string(), x.1)).collect();
    result
}
