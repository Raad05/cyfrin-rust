pub fn sum(nums: Vec<i32>) -> i32 {
    let mut total: i32 = 0;

    for n in nums {
        total += n;
    }

    total
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();

    while v.len() < n {
        v.push(i);
    }

    v
}
